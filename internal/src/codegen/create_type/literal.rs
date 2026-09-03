use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::Ident;

use crate::codegen::{self, CodegenItem, ExprMetadata};

pub(crate) fn impl_literals<I: CodegenItem>(metadata: ExprMetadata<I>) -> TokenStream {
    metadata.literals.into_iter()
        .enumerate()
        .map(|(index, literal)| impl_literal(&metadata.name, index, literal))
        .collect()
}

pub(crate) fn impl_literal(name: &Ident, index: usize, literal: Box<[u8]>) -> TokenStream {
    #![allow(nonstandard_style)]
    let LiteralTrait = quote!(::ct_regex::internal::matcher::Literal);

    let name = codegen::create_literal_id(name, index);
    let literal = Literal::byte_string(&literal);

    quote! {
        #[derive(Default, Clone, Copy)]
        pub struct #name;

        impl #LiteralTrait for #name {
            const LITERAL: &[u8] = #literal;
        }
    }
}
