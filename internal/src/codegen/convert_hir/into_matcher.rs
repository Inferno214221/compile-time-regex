#![allow(non_snake_case)]

use match_vec::match_vec;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use regex_syntax::hir::{Capture, Class, Hir, HirKind, Literal, Look, Repetition};

use crate::codegen::{ClassIter, ClassRange, CodegenItem, ExprMetadata};
use crate::matcher::ClassEntry;

pub(crate) trait IntoMatcherExpr {
    fn into_matcher_expr<I: CodegenItem>(self, meta: &mut ExprMetadata<I>) -> TokenStream;
}

impl IntoMatcherExpr for Hir {
    fn into_matcher_expr<I: CodegenItem>(self, meta: &mut ExprMetadata<I>) -> TokenStream {
        match self.into_kind() {
            HirKind::Empty              => Empty.into_matcher_expr(meta),
            HirKind::Literal(lit)       => lit.into_matcher_expr(meta),
            HirKind::Class(class)       => class.into_matcher_expr(meta),
            HirKind::Look(look)         => look.into_matcher_expr(meta),
            HirKind::Repetition(rep)    => rep.into_matcher_expr(meta),
            HirKind::Capture(cap)       => cap.into_matcher_expr(meta),
            HirKind::Concat(hirs)       => Concat(hirs).into_matcher_expr(meta),
            HirKind::Alternation(hirs)  => Alternation(hirs).into_matcher_expr(meta),
        }
    }
}

#[derive(Debug, Clone)]
struct Empty;

#[derive(Debug, Clone)]
struct Concat(pub Vec<Hir>);

#[derive(Debug, Clone)]
struct Alternation(pub Vec<Hir>);

impl IntoMatcherExpr for Empty {
    fn into_matcher_expr<I: CodegenItem>(self, _meta: &mut ExprMetadata<I>) -> TokenStream {
        quote!(::ct_regex::internal::matcher::Always)
    }
}

impl IntoMatcherExpr for Literal {
    fn into_matcher_expr<I: CodegenItem>(self, meta: &mut ExprMetadata<I>) -> TokenStream {
        let LiteralTy = meta.insert_literal(self.0);
        quote!(::ct_regex::internal::matcher::LiteralMatcher<#LiteralTy>)
    }
}

impl IntoMatcherExpr for Class {
    fn into_matcher_expr<I: CodegenItem>(self, meta: &mut ExprMetadata<I>) -> TokenStream {
        let normalized = I::normalize_class(self);

        let class = normalized.ranges().flat_map(|range| {
            let mut range_entries = vec![
                ClassEntry {
                    value: range.start(),
                    is_upper_bound: false,
                }
            ];
            if range.start() != range.end() {
                range_entries.push(
                    ClassEntry {
                        value: range.end(),
                        is_upper_bound: true,
                    }
                )
            }
            range_entries
        }).collect::<Box<[_]>>();

        let ClassTy = meta.insert_class(class);
        let ItemTy = I::type_ident();
        quote!(::ct_regex::internal::matcher::ClassMatcher<#ItemTy, #ClassTy>)
    }
}

impl IntoMatcherExpr for Look {
    fn into_matcher_expr<I: CodegenItem>(self, _meta: &mut ExprMetadata<I>) -> TokenStream {
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
    fn into_matcher_expr<I: CodegenItem>(self, meta: &mut ExprMetadata<I>) -> TokenStream {
        let Repetition { min, max, greedy, sub } = self;

        let required = meta.required;
        if min == 0 {
            meta.required = false;
        }

        let ItemTy = I::type_ident();
        let sub_matcher = sub.into_matcher_expr(meta);
        // I need to document this somewhere, might as well be here: usize is used for all generic
        // parameters, even though Hir types use u32, because it is used for array indexing during
        // the conversion process.
        let (min, max) = (min as usize, max.map(|m| m as usize));

        if min == 0 {
            meta.required = required;
        }

        let tokens = match max {
            None => {
                quote!(::ct_regex::internal::matcher::QuantifierNOrMore<#ItemTy, #sub_matcher, #min>)
            },
            Some(max) if min == max => {
                return quote!(::ct_regex::internal::matcher::QuantifierN<#ItemTy, #sub_matcher, #min>);
            },
            Some(max) => {
                quote!(::ct_regex::internal::matcher::QuantifierNToM<#ItemTy, #sub_matcher, #min, #max>)
            },
        };

        if greedy {
            tokens
        } else {
            quote!(::ct_regex::internal::matcher::Lazy<#ItemTy, #tokens>)
        }
    }
}

impl IntoMatcherExpr for Capture {
    fn into_matcher_expr<I: CodegenItem>(self, meta: &mut ExprMetadata<I>) -> TokenStream {
        meta.insert_group(self.index, self.name);
        let ItemTy = I::type_ident();
        let sub_matcher = self.sub.into_matcher_expr(meta);
        let index = self.index as usize;

        quote!(::ct_regex::internal::matcher::CaptureGroup<#ItemTy, #sub_matcher, #index>)
    }
}

impl IntoMatcherExpr for Alternation {
    fn into_matcher_expr<I: CodegenItem>(self, caps: &mut ExprMetadata<I>) -> TokenStream {
        let required = caps.required;
        caps.required = false;
        let tokens = write_chunked::<I, _>(&format_ident!("Or"), caps, self.0);
        caps.required = required;
        tokens
    }
}

impl IntoMatcherExpr for Concat {
    fn into_matcher_expr<I: CodegenItem>(self, meta: &mut ExprMetadata<I>) -> TokenStream {
        write_chunked::<I, _>(&format_ident!("Then"), meta, self.0)
    }
}

fn write_chunked<I: CodegenItem, W: IntoMatcherExpr>(
    matcher: &Ident,
    meta: &mut ExprMetadata<I>,
    items: Vec<W>,
) -> TokenStream {
    let ItemTy = I::type_ident();

    match_vec!(match items {
        [] => panic!("literal contains no items"),
        [item] => item.into_matcher_expr(meta),
        [first, second] => {
            let first = first.into_matcher_expr(meta);
            let second = second.into_matcher_expr(meta);

            quote!(::ct_regex::internal::matcher::#matcher<#ItemTy, #first, #second>)
        },
        [first, rem @ ..] if rem.len() == 2 => {
            let first = first.into_matcher_expr(meta);
            let chunked = write_chunked::<I, W>(matcher, meta, rem);

            quote!(::ct_regex::internal::matcher::#matcher<#ItemTy, #first, #chunked>)
        },
        chunk if matches!(chunk.len(), 4 | 8 | 16) => {
            let n = chunk.len();
            write_n_items::<I, W>(matcher, meta, chunk, n)
        },
        mut other => {
            // Take largest chunk that fits, combine with remainder
            let chunk_size = if other.len() > 16 {
                16
            } else if other.len() > 8 {
                8
            } else {
                4
            };

            let remainder = other.split_off(chunk_size);
            let n_matcher = write_n_items::<I, W>(matcher, meta, other, chunk_size);
            let chunked = write_chunked::<I, W>(matcher, meta, remainder);

            quote!(::ct_regex::internal::matcher::#matcher<#ItemTy, #n_matcher, #chunked>)
        },
    })
}

fn write_n_items<I: CodegenItem, W: IntoMatcherExpr>(
    matcher: &Ident,
    meta: &mut ExprMetadata<I>,
    items: Vec<W>,
    n: usize,
) -> TokenStream {
    let matcher_n = format_ident!("{}{}", matcher, n);
    let ItemTy = I::type_ident();

    let mut tokens = quote!(::ct_regex::internal::matcher::#matcher_n<#ItemTy);

    for item in items {
        tokens.extend(quote!(,));
        tokens.extend(item.into_matcher_expr(meta));
    }

    tokens.extend(quote!(>));
    tokens
}
