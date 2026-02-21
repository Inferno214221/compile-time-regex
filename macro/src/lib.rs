extern crate proc_macro;

use std::convert::TryFrom;

use litrs::StringLit;
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use regex::hir::HirExtension;
use regex_automata::util::syntax::{self, Config};

#[proc_macro]
pub fn regex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    regex2(TokenStream::from(input)).into()
}

fn regex2(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut iter = input.into_iter();
    let TokenTree::Ident(ident) = iter.next()
        .expect("input contains wrong number of arguments") else {
        panic!("first arg needs to be an identifier")
    };

    let TokenTree::Punct(eq) = iter.next()
        .expect("input contains wrong number of arguments") else {
        panic!("second arg needs to be an '='")
    };
    assert_eq!(eq.as_char(), '=', "second arg needs to be an '='");

    let pattern = iter.next().expect("input contains wrong number of arguments");
    let pattern_str = match StringLit::try_from(pattern) {
        Ok(lit) => lit.into_value(),
        Err(e) => return e.to_compile_error().into(),
    };

    let config = Config::new().unicode(false);

    let type_expr: TokenStream = syntax::parse_with(&pattern_str, &config)
        .expect("failed to parse regex")
        .into_type_expr()
        .parse()
        .expect("failed to parse type expression");

    // TODO: make pub optional like lazy_static
    quote! {
        pub struct #ident;

        impl regex::regex::Regex for #ident {
            type Pattern = #type_expr;
        }
    }
}