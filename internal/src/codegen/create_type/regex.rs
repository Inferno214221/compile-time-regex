#![allow(non_snake_case)]

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Visibility;

use crate::codegen::{AnonRegexArgs, RegexArgs, TypeExpressions, capture, class, literal};

pub fn make_regex(
    RegexArgs {
        vis,
        name,
        pat,
        flags,
    }: RegexArgs,
    impl_anon: bool,
) -> TokenStream {
    let fmt = quote!(::std::fmt);
    let HaystackSlice = quote!(::ct_regex::internal::haystack::HaystackSlice);
    let Regex = quote!(::ct_regex::internal::expr::Regex);
    let AnonRegex = quote!(::ct_regex::internal::expr::AnonRegex);

    let mod_name = format_ident!("__regex_{}", &name);

    let pat_str = pat.value();

    let doc = format!(
        "A macro-generated regular expression matching the pattern: `{pat_str}` with flags: \
        {flags}. See the [`Regex`](::ct_regex::internal::expr::Regex) trait for associated \
        matching and capturing functions."
    );

    let config = flags.create_config();
    // TODO: for empty regex, this can match non-utf8 strings.
    let TypeExpressions {
        matcher: byte_matcher,
        anchors: byte_anchors,
        meta: mut byte_meta,
    } = TypeExpressions::<u8>::parse_regex(&name, &pat_str, &config);

    let byte_classes_impl = class::impl_classes(&byte_meta);

    // TODO: if !config.enforces_utf8 then don't implement for string
    let scalar_impl = if config.enforce_utf8 {
        let TypeExpressions {
            matcher: scalar_matcher,
            anchors: scalar_anchors,
            meta: mut scalar_meta,
        } = TypeExpressions::<char>::parse_regex(&name, &pat_str, &config);

        assert_eq!(byte_meta, scalar_meta);

        let scalar_classes_impl = class::impl_classes(&scalar_meta);

        let (captures_name, captures_len) = capture::captures_info(
            &name,
            &scalar_meta.take_groups()
        );

        quote! {
            #scalar_classes_impl

            impl #Regex<char, #captures_len, true> for #name {
                type Pattern = #scalar_matcher;
                type Anchors = #scalar_anchors;
                type Capture<'a, S: 'a + #HaystackSlice + ?Sized> = #captures_name<'a, S, true>;
            }
        }
    } else {
        quote!()
    };

    let (captures_name, captures_len, captures_impl) = capture::impl_captures(
        &name,
        byte_meta.take_groups()
    );

    let literal_impl = literal::impl_literals(byte_meta);

    let anon_impl = if impl_anon {
        if config.enforce_utf8 {
            quote! {
                impl #AnonRegex<u8, #captures_len, true> for #name {}

                impl #AnonRegex<char, #captures_len, true> for #name {}
            }
        } else {
            quote! {
                impl #AnonRegex<u8, #captures_len, false> for #name {}
            }
        }
    } else {
        quote!()
    };

    let enforce_utf8 = config.enforce_utf8;

    quote! {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        mod #mod_name {
            #byte_classes_impl
            #literal_impl

            #[doc = #doc]
            #[derive(Clone, Copy)]
            pub struct #name;

            impl #Regex<u8, #captures_len, #enforce_utf8> for #name {
                type Pattern = #byte_matcher;
                type Anchors = #byte_anchors;
                type Capture<'a, S: 'a + #HaystackSlice + ?Sized> = #captures_name<'a, S, #enforce_utf8>;
            }

            #scalar_impl

            #anon_impl

            impl #fmt::Debug for #name {
                fn fmt(&self, f: &mut #fmt::Formatter<'_>) -> #fmt::Result {
                    write!(f, "/{:?}/", <Self as #Regex<char, #captures_len, true>>::Pattern::default())
                }
            }

            #captures_impl
        }

        #[doc(inline)]
        #[allow(unused)]
        #vis use #mod_name::{#name, #captures_name};
    }
}

pub fn make_anon_regex(AnonRegexArgs { pat, flags }: AnonRegexArgs) -> TokenStream {
    let impl_tokens = make_regex(
        RegexArgs {
            vis: Visibility::Inherited,
            name: format_ident!("__AnonRegex"),
            pat,
            flags,
        },
        true,
    );
    quote! {
        {
            #impl_tokens

            __AnonRegex
        }
    }
}
