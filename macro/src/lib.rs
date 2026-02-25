extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use regex_automata::util::syntax::{self, Config};
use syn::{Ident, LitStr, Token, Visibility, parse::{Parse, ParseStream}, parse_macro_input};

use ct_regex_internal::hir::HirExtension;

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

#[proc_macro]
pub fn regex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let RegexArgs {
        vis,
        name,
        pat
    } = parse_macro_input!(input as RegexArgs);
    regex_internal(vis, name, pat, quote!(ct_regex_internal::traits::Regex)).into()
}

fn regex_internal(
    vis: Visibility,
    name: Ident,
    pat: LitStr,
    regex_trait: TokenStream
) -> TokenStream {
    let config = Config::new().unicode(false);

    let pat_str = pat.value();

    let type_expr_byte: TokenStream = syntax::parse_with(&pat_str, &config)
        .expect("failed to parse regex")
        .into_type_expr::<u8>()
        .parse()
        .expect("failed to parse type expression");

    let config = config.unicode(true);

    let type_expr_scalar: TokenStream = syntax::parse_with(&pat_str, &config)
        .expect("failed to parse regex")
        .into_type_expr::<char>()
        .parse()
        .expect("failed to parse type expression");

    quote! {
        #vis struct #name;

        impl #regex_trait<u8> for #name {
            type Pattern = #type_expr_byte;
        }

        impl #regex_trait<char> for #name {
            type Pattern = #type_expr_scalar;
        }
    }
}

#[proc_macro]
pub fn anon_regex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    anon_regex_interal(parse_macro_input!(input as LitStr)).into()
}

fn anon_regex_interal(pat: LitStr) -> TokenStream {
    let impl_tokens = regex_internal(
        Visibility::Inherited,
        Ident::new("__AnonRegex", Span::call_site()),
        pat,
        quote!(ct_regex_internal::traits::AnonRegex)
    );
    quote! {
        {
            #impl_tokens

            __AnonRegex
        }
    }.into()
}