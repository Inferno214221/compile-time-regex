use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex_syntax::hir::{Capture, Class, ClassBytesRange, ClassUnicodeRange, Hir, HirKind, Literal, Look, Repetition};

use crate::{haystack::HaystackItem, hir::{Groups, type_ident, util::type_name}, matcher::{Always as A, Or, Then}};

pub trait WriteMatcher {
    fn write_matcher<I: HaystackItem>(self, caps: &mut Groups) -> TokenStream;
}

impl WriteMatcher for Hir {
    fn write_matcher<I: HaystackItem>(self, caps: &mut Groups) -> TokenStream {
        match self.into_kind() {
            HirKind::Empty              => Empty.write_matcher::<I>(caps),
            HirKind::Literal(lit)       => lit.write_matcher::<I>(caps),
            HirKind::Class(class)       => class.write_matcher::<I>(caps),
            HirKind::Look(look)         => look.write_matcher::<I>(caps),
            HirKind::Repetition(rep)    => rep.write_matcher::<I>(caps),
            HirKind::Capture(cap)       => cap.write_matcher::<I>(caps),
            HirKind::Concat(hirs)       => Concat(hirs).write_matcher::<I>(caps),
            HirKind::Alternation(hirs)  => Alternation(hirs).write_matcher::<I>(caps),
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

impl WriteMatcher for u8 {
    fn write_matcher<I: HaystackItem>(self, _caps: &mut Groups) -> TokenStream {
        assert_eq!(type_name::<I>(), type_name::<u8>());
        quote!(::ct_regex::internal::matcher::Byte<#self>)
    }
}

impl WriteMatcher for &ClassBytesRange {
    fn write_matcher<I: HaystackItem>(self, caps: &mut Groups) -> TokenStream {
        assert_eq!(type_name::<I>(), type_name::<u8>());
        if self.start() == self.end() {
            self.start().write_matcher::<I>(caps)
        } else {
            let start = self.start();
            let end = self.end();
            quote!(::ct_regex::internal::matcher::ByteRange<#start, #end>)
        }
    }
}

impl WriteMatcher for char {
    fn write_matcher<I: HaystackItem>(self, _caps: &mut Groups) -> TokenStream {
        assert_eq!(type_name::<I>(), type_name::<char>());
        quote!(::ct_regex::internal::matcher::Scalar<#self>)
    }
}

impl WriteMatcher for &ClassUnicodeRange {
    fn write_matcher<I: HaystackItem>(self, caps: &mut Groups) -> TokenStream {
        assert_eq!(type_name::<I>(), type_name::<char>(), "{:?}", self);
        if self.start() == self.end() {
            self.start().write_matcher::<I>(caps)
        } else {
            let start = self.start();
            let end = self.end();
            quote!(::ct_regex::internal::matcher::ScalarRange<#start, #end>)
        }
    }
}

impl WriteMatcher for Empty {
    fn write_matcher<I: HaystackItem>(self, _caps: &mut Groups) -> TokenStream {
        quote!(::ct_regex::internal::matcher::Always)
    }
}

impl WriteMatcher for Literal {
    fn write_matcher<I: HaystackItem>(self, caps: &mut Groups) -> TokenStream {
        write_chunked::<Then<u8, A, A>, I, _>(
            caps,
            I::vec_from_str(
                str::from_utf8(&self.0)
                    .expect("failed to convert bytes to valid unicode")
            )
        )
    }
}

impl WriteMatcher for Class {
    fn write_matcher<I: HaystackItem>(self, caps: &mut Groups) -> TokenStream {
        match I::cast_class(self) {
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

impl WriteMatcher for Look {
    fn write_matcher<I: HaystackItem>(self, _caps: &mut Groups) -> TokenStream {
        match self {
            Look::Start => quote!(::ct_regex::internal::matcher::Beginning),
            Look::End => quote!(::ct_regex::internal::matcher::End),
            Look::StartLF => todo!("complex looking"),
            Look::EndLF => todo!("complex looking"),
            Look::StartCRLF => todo!("complex looking"),
            Look::EndCRLF => todo!("complex looking"),
            _ => unimplemented!("complex look arounds"),
        }
    }
}

impl WriteMatcher for Repetition {
    fn write_matcher<I: HaystackItem>(self, caps: &mut Groups) -> TokenStream {
        let Repetition { min, max, greedy, sub } = self;
        if !greedy {
            todo!("lazy repetition")
        }

        let required = caps.required;
        if min == 0 {
            caps.required = false;
        }

        let item_type = type_ident::<I>();
        let sub_matcher = sub.write_matcher::<I>(caps);
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

impl WriteMatcher for Capture {
    fn write_matcher<I: HaystackItem>(self, caps: &mut Groups) -> TokenStream {
        caps.insert(self.index, self.name);
        let item_type = type_ident::<I>();
        let sub_matcher = self.sub.write_matcher::<I>(caps);
        let index = self.index as usize;

        quote!(::ct_regex::internal::matcher::CaptureGroup<#item_type, #sub_matcher, #index>)
    }
}

impl WriteMatcher for Alternation {
    fn write_matcher<I: HaystackItem>(self, caps: &mut Groups) -> TokenStream {
        let required = caps.required;
        caps.required = false;
        let tokens = write_chunked::<Or<u8, A, A>, I, _>(caps, self.0);
        caps.required = required;
        tokens
    }
}

impl WriteMatcher for Concat {
    fn write_matcher<I: HaystackItem>(self, caps: &mut Groups) -> TokenStream {
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
                (0, _) => backtrack.write_matcher::<I>(caps),
                (_, _) => {
                    let item_type = type_ident::<I>();
                    let concat_matcher = concat.write_type_basic::<I>(caps);
                    let backtrack_matcher = backtrack.write_matcher::<I>(caps);

                    quote!(::ct_regex::internal::matcher::Then<#item_type, #concat_matcher, #backtrack_matcher>)
                },
            }
        } else {
            concat.write_type_basic::<I>(caps)
        }
    }
}

impl WriteMatcher for Backtrack {
    fn write_matcher<I: HaystackItem>(mut self, caps: &mut Groups) -> TokenStream {
        let item_type = type_ident::<I>();
        let rep_matcher = self.rep.write_matcher::<I>(caps);
        let then_matcher = match self.then.len() {
            0 => return rep_matcher,
            1 => self.then.pop().unwrap().write_matcher::<I>(caps),
            _ => Concat(self.then).write_matcher::<I>(caps)
        };

        quote!(::ct_regex::internal::matcher::QuantifierThen<#item_type, #rep_matcher, #then_matcher>)
    }
}

impl Concat {
    fn write_type_basic<I: HaystackItem>(self, caps: &mut Groups) -> TokenStream {
        write_chunked::<Then<u8, A, A>, I, _>(caps, self.0)
    }
}

fn write_chunked<T, I: HaystackItem, W: WriteMatcher>(
    caps: &mut Groups,
    mut items: Vec<W>,
) -> TokenStream {
    let n = items.len();
    let base = format_ident!("{}", type_name::<T>());
    let item_type = type_ident::<I>();

    match n {
        0 => panic!("literal contains no items"),
        1 => items.pop().unwrap().write_matcher::<I>(caps),
        2 => {
            let mut iter = items.into_iter();
            let first = iter.next().unwrap().write_matcher::<I>(caps);
            let second = iter.next().unwrap().write_matcher::<I>(caps);

            quote!(::ct_regex::internal::matcher::#base<#item_type, #first, #second>)
        }
        3 => {
            let mut iter = items.into_iter();
            let first = iter.next().unwrap().write_matcher::<I>(caps);
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

fn write_n_items<T, I: HaystackItem, W: WriteMatcher>(
    caps: &mut Groups,
    items: Vec<W>,
    n: usize,
) -> TokenStream {
    let name = format_ident!("{}{}", type_name::<T>(), n);
    let item_type = type_ident::<I>();

    let mut tokens = quote!(::ct_regex::internal::matcher::#name<#item_type);

    for item in items {
        tokens.extend(quote!(,));
        tokens.extend(item.write_matcher::<I>(caps));
    }

    tokens.extend(quote!(>));
    tokens
}