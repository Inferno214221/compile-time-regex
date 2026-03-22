extern crate proc_macro;

mod args;
use args::*;

use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use regex_automata::util::syntax::{self, Config};
use syn::{Ident, Visibility, parse_macro_input};

use ct_regex_internal::{haystack::HaystackItem, hir::{Group, HirExtension}};

#[proc_macro]
pub fn regex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match parse_macro_input!(input as RegexArgType) {
        RegexArgType::Regex(args) => regex_internal(args, false).into(),
        RegexArgType::Anon(pat) => anon_regex_internal(pat).into(),
    }
}

fn parse_regex<I: HaystackItem>(pat: &str, config: &Config) -> (TokenStream, Vec<Group>) {
    let (type_expr_str, groups) = syntax::parse_with(&pat, &config)
        .expect("failed to parse regex")
        .into_matcher::<I>();

    let type_expr: TokenStream = type_expr_str.parse()
        .expect("failed to parse type expression");

    (type_expr, groups)
}

fn regex_internal(
    RegexArgs {
        vis,
        name,
        pat,
        flags,
    }: RegexArgs,
    impl_anon: bool
) -> TokenStream {
    // Aliases
    #![allow(nonstandard_style)]
    let fmt = quote!(::std::fmt);
    let Regex = quote!(::ct_regex::internal::expr::Regex);
    let AnonRegex = quote!(::ct_regex::internal::expr::AnonRegex);

    let pat_str = pat.value();

    let doc = format!("A macro-generated regular expression matching the pattern: `{}` with flags: \
        {}", pat_str, flags);

    let config = flags.create_config().unicode(false).utf8(false);

    let (type_expr_byte, groups) = parse_regex::<u8>(&pat_str, &config);

    let config = config.unicode(true).utf8(true);

    let (type_expr_scalar, _) = parse_regex::<char>(&pat_str, &config);

    let captures_name = format_ident!("{}Capture", name);
    let captures_len = Literal::usize_unsuffixed(groups.len());
    let captures_impl = impl_captures(&vis, &captures_name, groups);

    let anon_impl = if impl_anon {
        quote! {
            impl #AnonRegex<u8, #captures_len> for #name {}

            impl #AnonRegex<char, #captures_len> for #name {}
        }
    } else {
        quote!()
    };

    quote! {
        #[doc = #doc]
        #vis struct #name;

        impl #Regex<u8, #captures_len> for #name {
            type Pattern = #type_expr_byte;

            type Capture<'a> = #captures_name<'a, u8>;
        }

        impl #Regex<char, #captures_len> for #name {
            type Pattern = #type_expr_scalar;

            type Capture<'a> = #captures_name<'a, char>;
        }

        #anon_impl

        impl #fmt::Debug for #name {
            fn fmt(&self, f: &mut #fmt::Formatter<'_>) -> #fmt::Result {
                write!(f, "/{:?}/", <Self as #Regex<char, #captures_len>>::Pattern::default())
            }
        }

        #captures_impl
    }
}

fn anon_regex_internal(AnonRegexArgs { pat, flags }: AnonRegexArgs) -> TokenStream {
    let impl_tokens = regex_internal(
        RegexArgs {
            vis: Visibility::Inherited,
            name: Ident::new("__AnonRegex", Span::call_site()),
            pat,
            flags,
        },
        true
    );
    quote! {
        {
            #impl_tokens

            __AnonRegex
        }
    }
}

fn impl_captures(vis: &Visibility, name: &Ident, groups: Vec<Group>) -> TokenStream {
    // Aliases
    #![allow(nonstandard_style)]
    let CaptureFromRanges = quote!(::ct_regex::internal::expr::CaptureFromRanges);
    let Haystack = quote!(::ct_regex::internal::haystack::Haystack);
    let HaystackItem = quote!(::ct_regex::internal::haystack::HaystackItem);
    let Range = quote!(::std::ops::Range<usize>);
    let Option = quote!(::std::option::Option);

    let captures_len = Literal::usize_unsuffixed(groups.len());

    if groups.is_empty() {
        panic!("empty groups")
    }

    let inner = groups.iter().map(|cap| if cap.required {
        quote!(#Range)
    } else {
        quote!(#Option<#Range>)
    });

    let numbered_groups = groups.iter().enumerate().map(
            |(index, cap)| impl_capture_getters(index, cap, format_ident!("cap_{}", index))
        );

    let named_groups = groups.iter().enumerate().filter_map(
        |(index, cap)| cap.name.as_ref().map(
            |cap_name| impl_capture_getters(index, cap, Ident::new(cap_name, Span::call_site()))
        )
    );

    let capture_destructure = (0..groups.len()).map(|i| format_ident!("c{}", i));

    let capture_constructor = groups.iter().enumerate().map(|(index, cap)| {
        let ident = format_ident!("c{}", index);
        if cap.required {
            quote!(#ident?)
        } else {
            quote!(#ident)
        }
    });

    quote! {
        #[derive(Debug, Clone)]
        #vis struct #name<'a, I: #HaystackItem>(
            pub #Haystack<'a, I>,
            #(#inner),*
        );

        impl<'a, I: #HaystackItem> #name<'a, I> {
            #(#numbered_groups)*

            #(#named_groups)*
        }

        impl<'a, I: #HaystackItem> #CaptureFromRanges<'a, I, #captures_len> for #name<'a, I> {
            fn from_ranges(
                ranges: [#Option<#Range>; #captures_len],
                hay: #Haystack<'a, I>
            ) -> #Option<Self> {
                let [#(#capture_destructure),*] = ranges;
                #Option::Some(Self(
                    hay, #(#capture_constructor),*
                ))
            }
        }
    }
}

fn impl_capture_getters(index: usize, cap: &Group, cap_name: Ident) -> TokenStream {
    // Aliases
    #![allow(nonstandard_style)]
    let Range = quote!(::std::ops::Range<usize>);
    let Option = quote!(::std::option::Option);

    let index = index + 1;
    let cap_name_full = format_ident!("{}_range", cap_name);
    let index_name = Literal::usize_unsuffixed(index);

    if cap.required {
        quote! {
            pub fn #cap_name(&'a self) -> I::Slice<'a> {
                self.0.slice(self.#index_name.clone())
            }

            pub fn #cap_name_full(&'a self) -> &'a #Range {
                &self.#index_name
            }
        }
    } else {
        quote! {
            pub fn #cap_name(&'a self) -> #Option<I::Slice<'a>> {
                self.#index_name.as_ref().map(|c| self.0.slice(c.clone()))
            }

            pub fn #cap_name_full(&'a self) -> #Option<&'a #Range> {
                self.#index_name.as_ref()
            }
        }
    }
}