use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Ident, Visibility};

use crate::codegen::Group;

pub fn impl_captures(vis: &Visibility, name: &Ident, groups: Vec<Group>) -> TokenStream {
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

pub fn impl_capture_getters(index: usize, cap: &Group, cap_name: Ident) -> TokenStream {
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