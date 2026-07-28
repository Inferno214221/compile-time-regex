use proc_macro2::{Literal, TokenStream};
use quote::quote;

use crate::codegen;

pub fn impl_literals(literals: Vec<Box<[u8]>>) -> TokenStream {
    literals.into_iter()
        .enumerate()
        .map(impl_literal)
        .collect()
}

pub fn impl_literal((index, literal): (usize, Box<[u8]>)) -> TokenStream {
    #![allow(nonstandard_style)]
    let LiteralTrait = quote!(::ct_regex::internal::matcher::Literal);

    let name = codegen::create_literal_id(index);
    let literal = Literal::byte_string(&literal);

    quote! {
        #[derive(Default, Clone, Copy)]
        pub struct #name;

        impl #LiteralTrait for #name {
            const LITERAL: &[u8] = #literal;
        }
    }
}