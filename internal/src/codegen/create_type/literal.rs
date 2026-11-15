#![allow(non_snake_case)]

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::Ident;

use crate::codegen::{CodegenItem, ExprMetadata};

pub(crate) fn impl_literals<I: CodegenItem>(metadata: ExprMetadata<I>) -> TokenStream {
    metadata.literals.into_iter()
        .map(|(literal, name)| impl_literal(name, literal))
        .collect()
}

pub(crate) fn impl_literal(name: Ident, literal: Box<[u8]>) -> TokenStream {
    let LiteralTrait = quote!(::ct_regex::internal::matcher::Literal);

    let literal = Literal::byte_string(&literal);

    quote! {
        #[derive(Default, Clone, Copy)]
        pub struct #name;

        impl #LiteralTrait for #name {
            const LITERAL: &[u8] = #literal;
        }
    }
}
