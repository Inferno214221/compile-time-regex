use std::any;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use regex_syntax::hir::{Capture, Class, ClassBytesRange, ClassUnicodeRange, Hir, HirKind, Literal, Look, Repetition};

use crate::{codegen::{CodegenItem, Group, Groups}, matcher::{Always as A, Or, Then}};

pub fn type_name<T>() -> &'static str {
    any::type_name::<T>()
        .split('<').next().unwrap()
        .rsplit("::").next().unwrap()
}

pub fn type_ident<T>() -> Ident {
    format_ident!("{}", type_name::<T>())
}

pub trait HirExtension {
    fn into_matcher<I: CodegenItem>(self) -> (TokenStream, Vec<Group>);
}

impl HirExtension for Hir {
    fn into_matcher<I: CodegenItem>(self) -> (TokenStream, Vec<Group>) {
        let mut caps = Groups::new();
        let tokens = self.into_matcher_expr::<I>(&mut caps);
        (tokens, caps.into_vec())
    }
}

pub trait IntoMatcherExpr {
    fn into_matcher_expr<I: CodegenItem>(self, caps: &mut Groups) -> TokenStream;
}

impl IntoMatcherExpr for Hir {
    fn into_matcher_expr<I: CodegenItem>(self, caps: &mut Groups) -> TokenStream {
        match self.into_kind() {
            HirKind::Empty              => Empty.into_matcher_expr::<I>(caps),
            HirKind::Literal(lit)       => lit.into_matcher_expr::<I>(caps),
            HirKind::Class(class)       => class.into_matcher_expr::<I>(caps),
            HirKind::Look(look)         => look.into_matcher_expr::<I>(caps),
            HirKind::Repetition(rep)    => rep.into_matcher_expr::<I>(caps),
            HirKind::Capture(cap)       => cap.into_matcher_expr::<I>(caps),
            HirKind::Concat(hirs)       => Concat(hirs).into_matcher_expr::<I>(caps),
            HirKind::Alternation(hirs)  => Alternation(hirs).into_matcher_expr::<I>(caps),
        }
    }
}

#[derive(Debug)]
struct Empty;

#[derive(Debug)]
struct Concat(pub Vec<Hir>);

#[derive(Debug)]
struct Alternation(pub Vec<Hir>);

#[derive(Debug)]
struct Backtrack {
    rep: Repetition,
    then: Vec<Hir>,
}

impl IntoMatcherExpr for u8 {
    fn into_matcher_expr<I: CodegenItem>(self, _caps: &mut Groups) -> TokenStream {
        assert_eq!(type_name::<I>(), type_name::<u8>());
        quote!(::ct_regex::internal::matcher::Byte<#self>)
    }
}

impl IntoMatcherExpr for &ClassBytesRange {
    fn into_matcher_expr<I: CodegenItem>(self, caps: &mut Groups) -> TokenStream {
        assert_eq!(type_name::<I>(), type_name::<u8>());
        if self.start() == self.end() {
            self.start().into_matcher_expr::<I>(caps)
        } else {
            let start = self.start();
            let end = self.end();
            quote!(::ct_regex::internal::matcher::ByteRange<#start, #end>)
        }
    }
}

impl IntoMatcherExpr for char {
    fn into_matcher_expr<I: CodegenItem>(self, _caps: &mut Groups) -> TokenStream {
        assert_eq!(type_name::<I>(), type_name::<char>());
        quote!(::ct_regex::internal::matcher::Scalar<#self>)
    }
}

impl IntoMatcherExpr for &ClassUnicodeRange {
    fn into_matcher_expr<I: CodegenItem>(self, caps: &mut Groups) -> TokenStream {
        assert_eq!(type_name::<I>(), type_name::<char>(), "{:?}", self);
        if self.start() == self.end() {
            self.start().into_matcher_expr::<I>(caps)
        } else {
            let start = self.start();
            let end = self.end();
            quote!(::ct_regex::internal::matcher::ScalarRange<#start, #end>)
        }
    }
}

impl IntoMatcherExpr for Empty {
    fn into_matcher_expr<I: CodegenItem>(self, _caps: &mut Groups) -> TokenStream {
        quote!(::ct_regex::internal::matcher::Always)
    }
}

impl IntoMatcherExpr for Literal {
    fn into_matcher_expr<I: CodegenItem>(self, caps: &mut Groups) -> TokenStream {
        write_chunked::<Then<u8, A, A>, I, _>(
            caps,
            I::vec_from_str(
                str::from_utf8(&self.0)
                    .expect("failed to convert bytes to valid unicode")
            )
        )
    }
}

impl IntoMatcherExpr for Class {
    fn into_matcher_expr<I: CodegenItem>(self, caps: &mut Groups) -> TokenStream {
        match I::normalize_class(self) {
            Class::Unicode(unicode) => write_chunked::<Or<u8, A, A>, I, _>(
                caps,
                unicode.ranges().iter().collect()
            ),
            Class::Bytes(bytes) => write_chunked::<Or<u8, A, A>, I, _>(
                caps,
                bytes.ranges().iter().collect()
            ),
        }
    }
}

impl IntoMatcherExpr for Look {
    fn into_matcher_expr<I: CodegenItem>(self, _caps: &mut Groups) -> TokenStream {
        match self {
            Look::Start => quote!(::ct_regex::internal::matcher::Start),
            Look::End => quote!(::ct_regex::internal::matcher::End),
            Look::StartLF => quote!(::ct_regex::internal::matcher::LineStart),
            Look::EndLF => quote!(::ct_regex::internal::matcher::LineEnd),
            Look::StartCRLF => quote!(::ct_regex::internal::matcher::CRLFStart),
            Look::EndCRLF => quote!(::ct_regex::internal::matcher::CRLFEnd),
            _ => unimplemented!("complex look arounds"),
        }
    }
}

impl IntoMatcherExpr for Repetition {
    fn into_matcher_expr<I: CodegenItem>(self, caps: &mut Groups) -> TokenStream {
        let Repetition { min, max, greedy, sub } = self;
        if !greedy {
            todo!("lazy repetition")
        }

        let required = caps.required;
        if min == 0 {
            caps.required = false;
        }

        let item_type = type_ident::<I>();
        let sub_matcher = sub.into_matcher_expr::<I>(caps);
        // I need to document this somewhere, might as well be here: usize is used for all generic
        // parameters, even though Hir types use u32, because it is used for arrasy indexing during
        // the conversion process.
        let (min, max) = (min as usize, max.map(|m| m as usize));

        let tokens = match max {
            None => {
                quote!(::ct_regex::internal::matcher::QuantifierNOrMore<#item_type, #sub_matcher, #min>)
            },
            Some(max) if min == max => {
                quote!(::ct_regex::internal::matcher::QuantifierN<#item_type, #sub_matcher, #min>)
            },
            Some(max) => {
                quote!(::ct_regex::internal::matcher::QuantifierNToM<#item_type, #sub_matcher, #min, #max>)
            },
        };

        if min == 0 {
            caps.required = required;
        }

        tokens
    }
}

impl IntoMatcherExpr for Capture {
    fn into_matcher_expr<I: CodegenItem>(self, caps: &mut Groups) -> TokenStream {
        caps.insert(self.index, self.name);
        let item_type = type_ident::<I>();
        let sub_matcher = self.sub.into_matcher_expr::<I>(caps);
        let index = self.index as usize;

        quote!(::ct_regex::internal::matcher::CaptureGroup<#item_type, #sub_matcher, #index>)
    }
}

impl IntoMatcherExpr for Alternation {
    fn into_matcher_expr<I: CodegenItem>(self, caps: &mut Groups) -> TokenStream {
        let required = caps.required;
        caps.required = false;
        let tokens = write_chunked::<Or<u8, A, A>, I, _>(caps, self.0);
        caps.required = required;
        tokens
    }
}

impl IntoMatcherExpr for Concat {
    fn into_matcher_expr<I: CodegenItem>(self, caps: &mut Groups) -> TokenStream {
        let mut iter = self.0.into_iter();
        let mut rep_item = None;
        let concat = Concat(
            iter.by_ref()
                .take_while(|i| if let HirKind::Repetition(rep) = i.kind() {
                    rep_item = Some(rep.clone());
                    false
                } else {
                    true
                })
                .collect()
        );
        if let Some(rep) = rep_item {
            let backtrack = Backtrack {
                rep,
                then: iter.collect(),
            };

            match (concat.0.len(), backtrack.then.len()) {
                (0, 0) => unreachable!(),
                (0, _) => backtrack.into_matcher_expr::<I>(caps),
                (_, _) => {
                    let item_type = type_ident::<I>();
                    let concat_matcher = concat.write_type_basic::<I>(caps);
                    let backtrack_matcher = backtrack.into_matcher_expr::<I>(caps);

                    quote!(::ct_regex::internal::matcher::Then<#item_type, #concat_matcher, #backtrack_matcher>)
                },
            }
        } else {
            concat.write_type_basic::<I>(caps)
        }
    }
}

impl IntoMatcherExpr for Backtrack {
    fn into_matcher_expr<I: CodegenItem>(mut self, caps: &mut Groups) -> TokenStream {
        let item_type = type_ident::<I>();
        let rep_matcher = self.rep.into_matcher_expr::<I>(caps);
        let then_matcher = match self.then.len() {
            0 => return rep_matcher,
            1 => self.then.pop().unwrap().into_matcher_expr::<I>(caps),
            _ => Concat(self.then).into_matcher_expr::<I>(caps)
        };

        quote!(::ct_regex::internal::matcher::QuantifierThen<#item_type, #rep_matcher, #then_matcher>)
    }
}

impl Concat {
    fn write_type_basic<I: CodegenItem>(self, caps: &mut Groups) -> TokenStream {
        write_chunked::<Then<u8, A, A>, I, _>(caps, self.0)
    }
}

fn write_chunked<T, I: CodegenItem, W: IntoMatcherExpr>(
    caps: &mut Groups,
    mut items: Vec<W>,
) -> TokenStream {
    let n = items.len();
    let base = format_ident!("{}", type_name::<T>());
    let item_type = type_ident::<I>();

    match n {
        0 => panic!("literal contains no items"),
        1 => items.pop().unwrap().into_matcher_expr::<I>(caps),
        2 => {
            let mut iter = items.into_iter();
            let first = iter.next().unwrap().into_matcher_expr::<I>(caps);
            let second = iter.next().unwrap().into_matcher_expr::<I>(caps);

            quote!(::ct_regex::internal::matcher::#base<#item_type, #first, #second>)
        }
        3 => {
            let mut iter = items.into_iter();
            let first = iter.next().unwrap().into_matcher_expr::<I>(caps);
            let chunked = write_chunked::<T, I, W>(caps, iter.collect());

            quote!(::ct_regex::internal::matcher::#base<#item_type, #first, #chunked>)
        }
        4 | 8 | 16 => write_n_items::<T, I, W>(caps, items, n),
        _ => {
            // Take largest chunk that fits, combine with remainder
            let chunk_size = if n > 16 { 16 } else if n > 8 { 8 } else { 4 };
            let remainder = items.split_off(chunk_size);
            let n_matcher = write_n_items::<T, I, W>(caps, items, chunk_size);
            let chunked = write_chunked::<T, I, W>(caps, remainder);

            quote!(::ct_regex::internal::matcher::#base<#item_type, #n_matcher, #chunked>)
        }
    }
}

fn write_n_items<T, I: CodegenItem, W: IntoMatcherExpr>(
    caps: &mut Groups,
    items: Vec<W>,
    n: usize,
) -> TokenStream {
    let name = format_ident!("{}{}", type_name::<T>(), n);
    let item_type = type_ident::<I>();

    let mut tokens = quote!(::ct_regex::internal::matcher::#name<#item_type);

    for item in items {
        tokens.extend(quote!(,));
        tokens.extend(item.into_matcher_expr::<I>(caps));
    }

    tokens.extend(quote!(>));
    tokens
}