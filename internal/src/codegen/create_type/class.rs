#![allow(non_snake_case)]

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

use crate::{codegen::{self, CodegenItem, ExprMetadata}, matcher::ClassEntry};

impl<I: CodegenItem> ToTokens for ClassEntry<I> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ClassEntry = quote!(::ct_regex::internal::matcher::ClassEntry);

        let ClassEntry { value, is_upper_bound } = self;
        tokens.extend(quote! {
            #ClassEntry {
                value: #value,
                is_upper_bound: #is_upper_bound,
            }
        });
    }
}

pub(crate) fn impl_classes<I: CodegenItem>(metadata: &ExprMetadata<I>) -> TokenStream {
    metadata.classes.iter()
        .enumerate()
        .map(|(index, entries)| impl_class(&metadata.name, index, entries))
        .collect()
}

pub(crate) fn impl_class<I: CodegenItem>(name: &Ident, index: usize, entries: &[ClassEntry<I>]) -> TokenStream {
    let ClassTrait = quote!(::ct_regex::internal::matcher::Class);
    let ClassEntry = quote!(::ct_regex::internal::matcher::ClassEntry);

    let ItemTy = codegen::type_ident::<I>();

    let name = codegen::create_class_id::<I>(name, index);
    let entries = entries.iter().map(|entry| quote!(#entry));

    quote! {
        #[derive(Default, Clone, Copy)]
        pub struct #name;

        impl #ClassTrait<#ItemTy> for #name {
            const ENTRIES: &[#ClassEntry<#ItemTy>] = &[#(#entries),*];
        }
    }
}
