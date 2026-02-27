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
        RegexArgType::Regex(args) => regex_internal(args, quote!(ct_regex_internal::general::Regex)).into(),
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

    // TODO: remember to fill cap 0 with the whole match
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

    let captures_impl = impl_captures(&vis, &captures_name, groups);

    quote! {
        #vis struct #name;

        impl #regex_trait<u8> for #name {
            type Pattern = #type_expr_byte;

            type Captures<'a> = #captures_name<'a>;
        }

        impl #regex_trait<char> for #name {
            type Pattern = #type_expr_scalar;

            type Captures<'a> = #captures_name<'a>;
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
    if groups.is_empty() {
        panic!("empty groups")
    }

    let capture_ty: TokenStream = quote!(ct_regex_internal::general::Capture);

    let inner = groups.iter().map(|cap| if cap.required {
        quote!(pub #capture_ty<'a>)
    } else {
        quote!(pub Option<#capture_ty<'a>>)
    });

    let named_groups = groups.iter().enumerate().filter_map(|(index, cap)|
        cap.name.as_ref().map(|cap_name| {
            let cap_name = Ident::new(cap_name, Span::call_site());
            let cap_name_full = format_ident!("{}_cap", cap_name);
            let index_name = Literal::usize_unsuffixed(index);

            if cap.required {
                quote! {
                    pub fn #cap_name(&'a self) -> &'a str {
                        self.#index_name.content
                    }

                    pub fn #cap_name_full(&'a self) -> &'a #capture_ty<'a> {
                        &self.#index_name
                    }
                }
            } else {
                quote! { 
                    pub fn #cap_name(&'a self) -> Option<&'a str> {
                        self.#index_name.as_ref().map(#capture_ty::content)
                    }

                    pub fn #cap_name_full(&'a self) -> Option<&'a #capture_ty<'a>> {
                        self.#index_name.as_ref()
                    }
                }
            }
        })
    );

    quote! {
        #[derive(Debug, Clone)]
        #vis struct #name<'a>(#(#inner),*);

        impl<'a> #name<'a> {
            #(#named_groups)*
        }
    }
}