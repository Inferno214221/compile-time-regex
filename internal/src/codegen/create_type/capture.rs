use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use syn::Ident;

use crate::codegen::Group;

pub fn impl_captures(regex_name: &Ident, groups: Vec<Group>) -> (Ident, Literal, TokenStream) {
    #![allow(nonstandard_style)]
    let CaptureFromRanges = quote!(::ct_regex::internal::expr::CaptureFromRanges);
    let HaystackSlice = quote!(::ct_regex::internal::haystack::HaystackSlice);
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

    let inner = groups.iter().map(|cap| {
        if cap.required {
            quote!(#Range)
        } else {
            quote!(#Option<#Range>)
        }
    });

    let numbered_groups = groups.iter()
        .enumerate()
        .map(|(index, cap)| impl_capture_getters(index, cap, format_ident!("cap_{}", index)));

    let named_groups = groups.iter()
        .enumerate()
        .filter_map(|(index, cap)| {
            cap.name.as_ref().map(|cap_name| {
                impl_capture_getters(index, cap, Ident::new(cap_name, Span::call_site()))
            })
        });

    let capture_destructure = (0..groups.len()).map(|i| format_ident!("c{}", i));

    let capture_constructor = groups.iter().enumerate().map(|(index, cap)| {
        let ident = format_ident!("c{}", index);
        if cap.required {
            quote!(#ident?)
        } else {
            quote!(#ident)
        }
    });

    // TODO: Manual debug implementation

    let captures_impl = quote! {
        #[doc = #doc]
        #[derive(Debug, Clone)]
        pub struct #name<'a, S: #HaystackSlice<'a>>(
            S,
            #(#inner),*,
            #PhantomData<&'a ()>,
        );

        impl<'a, S: #HaystackSlice<'a>> #name<'a, S> {
            #(#numbered_groups)*

            #(#named_groups)*
        }

        impl<'a, S: #HaystackSlice<'a>> #CaptureFromRanges<'a, S, #len> for #name<'a, S> {
            fn from_ranges(
                ranges: [#Option<#Range>; #len],
                hay: S,
            ) -> #Option<Self> {
                let [#(#capture_destructure),*] = ranges;
                #Option::Some(Self(
                    hay,
                    #(#capture_constructor),*,
                    #PhantomData,
                ))
            }

            fn whole_match_range(&self) -> #Range {
                self.1.clone()
            }
        }
    };

    (name, len, captures_impl)
}

pub fn impl_capture_getters(index: usize, cap: &Group, cap_name: Ident) -> TokenStream {
    #![allow(nonstandard_style)]
    let Range = quote!(::std::ops::Range<usize>);
    let Option = quote!(::std::option::Option);

    let index = index + 1;
    let cap_name_full = format_ident!("{}_range", cap_name);
    let index_name = Literal::usize_unsuffixed(index);

    let numeric_impl = if cap.digits.end == 0 {
        quote!()
    } else {
        impl_numeric_getters(cap, &cap_name)
    };

    if cap.required {
        quote! {
            pub fn #cap_name(&self) -> S {
                self.0.slice_with(self.#index_name.clone())
            }

            pub fn #cap_name_full(&self) -> #Range {
                self.#index_name.clone()
            }

            #numeric_impl
        }
    } else {
        quote! {
            pub fn #cap_name(&self) -> #Option<S> {
                self.#index_name.clone().map(|r| self.0.slice_with(r))
            }

            pub fn #cap_name_full(&self) -> #Option<#Range> {
                self.#index_name.clone()
            }

            #numeric_impl
        }
    }
}

pub fn impl_numeric_getters(cap: &Group, cap_name: &Ident) -> TokenStream {
    #![allow(nonstandard_style)]
    let Option = quote!(::std::option::Option);
    let FromStr = quote!(::std::str::FromStr);

    let bit_count = (cap.digits.end as f64 * 10.0_f64.log2()).ceil();
    if bit_count > 128.0 {
        return quote!();
    }

    let integer_size = (2 ^ bit_count.log2().ceil() as usize).clamp(8, 128);
    let integer_name = format_ident!("u{}", integer_size);

    let cap_name_num = format_ident!("{}_{}", cap_name, integer_name);

    match (cap.digits.start == 0, cap.required) {
        (false, false) => quote! {
            pub fn #cap_name_num(&self) -> #Option<#integer_name> {
                <#integer_name as #FromStr>::from_str(
                    self.#cap_name()?
                ).unwrap()
            }
        },
        (false, true) => quote! {
            pub fn #cap_name_num(&self) -> #integer_name {
                <#integer_name as #FromStr>::from_str(
                    self.#cap_name()
                ).unwrap()
            }
        },
        (true, false) => quote! {
            pub fn #cap_name_num(&self) -> #Option<#Option<#integer_name>> {
                let Some(cap) = self.#cap_name() else {
                    return Some(None);
                };
                if cap.is_empty() {
                    return None;
                }
                <#integer_name as #FromStr>::from_str(
                    cap
                ).unwrap()
            }
        },
        (true, true) => quote! {
            pub fn #cap_name_num(&self) -> #Option<#integer_name> {
                let cap = self.#cap_name();
                if cap.is_empty() {
                    return None;
                }
                <#integer_name as #FromStr>::from_str(
                    cap
                ).unwrap()
            }
        },
    }
}