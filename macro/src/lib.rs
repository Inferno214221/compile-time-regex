extern crate proc_macro;

use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use regex_automata::util::syntax::{self, Config};
use syn::{Ident, LitStr, Token, Visibility, parse::{Parse, ParseStream}, parse_macro_input};

use ct_regex_internal::hir::{Group, HirExtension};

struct RegexArgs {
    vis: Visibility,
    name: Ident,
    pat: LitStr,
}

impl Parse for RegexArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility: Visibility = input.parse()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let pattern: LitStr = input.parse()?;
        Ok(RegexArgs {
            vis: visibility,
            name,
            pat: pattern,
        })
    }
}

enum RegexArgType {
    Regex(RegexArgs),
    Anon(LitStr),
}

impl Parse for RegexArgType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.fork().parse::<RegexArgs>().is_ok() {
            Ok(RegexArgType::Regex(input.parse()?))
        } else {
            Ok(RegexArgType::Anon(input.parse()?))
        }
    }
}

#[proc_macro]
pub fn regex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match parse_macro_input!(input as RegexArgType) {
        RegexArgType::Regex(args) => regex_internal(
            args,
            quote!(::ct_regex::internal::general::Regex)
        ).into(),
        RegexArgType::Anon(pat) => anon_regex_internal(pat).into(),
    }
}

fn regex_internal(
    RegexArgs {
        vis,
        name,
        pat
    }: RegexArgs,
    regex_trait: TokenStream
) -> TokenStream {
    let config = Config::new().unicode(false).utf8(false);

    let pat_str = pat.value();

    let (type_expr_byte_str, groups) = syntax::parse_with(&pat_str, &config)
        .expect("failed to parse regex")
        .into_matcher::<u8>();

    let type_expr_byte: TokenStream = type_expr_byte_str.parse()
        .expect("failed to parse type expression");

    let config = config.unicode(true).utf8(true);

    let type_expr_scalar: TokenStream = syntax::parse_with(&pat_str, &config)
        .expect("failed to parse regex")
        .into_matcher::<char>().0
        .parse()
        .expect("failed to parse type expression");

    let captures_name = format_ident!("{}Captures", name);
    let captures_len = Literal::usize_unsuffixed(groups.len());
    let captures_impl = impl_captures(&vis, &captures_name, groups);

    quote! {
        #vis struct #name;

        impl #regex_trait<u8, #captures_len> for #name {
            type Pattern = #type_expr_byte;

            type Captures<'a> = #captures_name<'a, u8>;
        }

        impl #regex_trait<char, #captures_len> for #name {
            type Pattern = #type_expr_scalar;

            type Captures<'a> = #captures_name<'a, char>;
        }

        impl ::std::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "/{:?}/", <Self as #regex_trait<char, #captures_len>>::Pattern::default())
            }
        }

        #captures_impl
    }
}

fn anon_regex_internal(pat: LitStr) -> TokenStream {
    let impl_tokens = regex_internal(
        RegexArgs {
            vis: Visibility::Inherited,
            name: Ident::new("__AnonRegex", Span::call_site()),
            pat
        },
        quote!(ct_regex_internal::general::AnonRegex)
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
    let haystack_mod = quote!(::ct_regex::internal::haystack);
    let general_mod = quote!(::ct_regex::internal::general);
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
        #vis struct #name<'a, I: #haystack_mod::HaystackItem>(pub Haystack<'a, I>, #(#inner),*);

        impl<'a, I: #haystack_mod::HaystackItem> #name<'a, I> {
            #(#numbered_groups)*

            #(#named_groups)*
        }

        impl<'a, I: #haystack_mod::HaystackItem> #general_mod::FromCaptures<'a, I, #captures_len> for #name<'a, I> {
            fn from_captures(
                captures: [#Option<#Range>; #captures_len],
                hay: #haystack_mod::Haystack<'a, I>
            ) -> #Option<Self> {
                let [#(#capture_destructure),*] = captures;
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