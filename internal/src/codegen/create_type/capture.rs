use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Ident, Visibility};

use crate::codegen::Group;

pub fn impl_captures(
    vis: &Visibility,
    regex_name: &Ident,
    groups: Vec<Group>
) -> (Ident, Literal, TokenStream) {
    #![allow(nonstandard_style)]
    let CaptureFromRanges = quote!(::ct_regex::internal::expr::CaptureFromRanges);
    let Haystack = quote!(::ct_regex::internal::haystack::Haystack);
    let Range = quote!(::std::ops::Range<usize>);
    let Option = quote!(::std::option::Option);
    let PhantomData = quote!(::std::marker::PhantomData);

    let len_usize = groups.len();

    let name = format_ident!("{}Capture", regex_name);
    let len = Literal::usize_unsuffixed(len_usize);

    let doc = format!(
        "A macro-generated type that holds {len_usize} captures for the associated regex, \
        [`{regex_name}`]. If present, named groups can be retrieved through their associated \
        method.\n\n\
        Capture types include methods to retrieve individual captures as a slice of the original \
        haystack, or as the underlying [`Range<usize>`](::std::range::Range), which can be used \
        to manually slice the haystack (without risk of panicking in the case of `&str`).\n\n\
        As is common with regular expressions, group `0` refers to the whole match (and is \
        therefore aliased as [`{name}::whole_match`])."
    );

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

    let captures_impl = quote! {
        #[doc = #doc]
        #[derive(Debug, Clone)]
        #vis struct #name<'a, H: #Haystack<'a>>(
            pub H,
            #(#inner),*,
            #PhantomData<&'a ()>,
        );

        impl<'a, H: #Haystack<'a>> #name<'a, H> {
            #(#numbered_groups)*

            #(#named_groups)*
        }

        impl<'a, H: #Haystack<'a>> #CaptureFromRanges<'a, H, #len> for #name<'a, H> {
            fn from_ranges(
                ranges: [#Option<#Range>; #len],
                hay: H
            ) -> #Option<Self> {
                let [#(#capture_destructure),*] = ranges;
                #Option::Some(Self(
                    hay,
                    #(#capture_constructor),*,
                    #PhantomData,
                ))
            }
        }
    };

    return (name, len, captures_impl);
}

pub fn impl_capture_getters(index: usize, cap: &Group, cap_name: Ident) -> TokenStream {
    #![allow(nonstandard_style)]
    let Range = quote!(::std::ops::Range<usize>);
    let Option = quote!(::std::option::Option);

    let index = index + 1;
    let cap_name_full = format_ident!("{}_range", cap_name);
    let index_name = Literal::usize_unsuffixed(index);

    if cap.required {
        quote! {
            pub fn #cap_name(&self) -> H::Slice {
                self.0.slice(self.#index_name.clone())
            }

            pub fn #cap_name_full(&self) -> #Range {
                self.#index_name.clone()
            }
        }
    } else {
        quote! {
            pub fn #cap_name(&self) -> #Option<H::Slice> {
                self.#index_name.clone().map(|r| self.0.slice(r))
            }

            pub fn #cap_name_full(&self) -> #Option<#Range> {
                self.#index_name.clone()
            }
        }
    }
}