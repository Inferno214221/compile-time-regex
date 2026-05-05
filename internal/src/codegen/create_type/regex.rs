use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Ident, Visibility};

use crate::codegen::{AnonRegexArgs, RegexArgs, capture, parse};

pub fn make_regex(
    RegexArgs {
        vis,
        name,
        pat,
        flags,
    }: RegexArgs,
    impl_anon: bool,
) -> TokenStream {
    #![allow(nonstandard_style)]
    let fmt = quote!(::std::fmt);
    let HaystackSlice = quote!(::ct_regex::internal::haystack::HaystackSlice);
    let Regex = quote!(::ct_regex::internal::expr::Regex);
    let AnonRegex = quote!(::ct_regex::internal::expr::AnonRegex);

    let mod_name = format_ident!("__regex_{}", &name);

    let pat_str = pat.value();

    let doc = format!(
        "A macro-generated regular expression matching the pattern: `{pat_str}` with flags: \
        {flags}. See the [`Regex`](::ct_regex::internal::expr::Regex) trait for associated \
        matching and capturing functions." // TODO: Can be used for &str or &[u8]...
    );

    let mut config = flags.create_config();
    config.unicode(false).utf8(false);

    let (type_expr_byte, groups) = parse::parse_regex::<u8>(&pat_str, &config);

    config.unicode(true).utf8(true);

    let (type_expr_scalar, _) = parse::parse_regex::<char>(&pat_str, &config);

    let (captures_name, captures_len, captures_impl) = capture::impl_captures(&name, groups);

    let anon_impl = if impl_anon {
        quote! {
            impl #AnonRegex<u8, #captures_len> for #name {}

            impl #AnonRegex<char, #captures_len> for #name {}
        }
    } else {
        quote!()
    };

    quote! {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        mod #mod_name {
            #[doc = #doc]
            pub struct #name;

            impl #Regex<u8, #captures_len> for #name {
                type Pattern = #type_expr_byte;

                type Capture<'a, S: #HaystackSlice<'a>> = #captures_name<'a, S>;
            }

            impl #Regex<char, #captures_len> for #name {
                type Pattern = #type_expr_scalar;
                type Capture<'a, S: #HaystackSlice<'a>> = #captures_name<'a, S>;
            }

            #anon_impl

            impl #fmt::Debug for #name {
                fn fmt(&self, f: &mut #fmt::Formatter<'_>) -> #fmt::Result {
                    write!(f, "/{:?}/", <Self as #Regex<char, #captures_len>>::Pattern::default())
                }
            }

            #captures_impl
        }

        #[doc(inline)]
        #vis use #mod_name::*;
    }
}

pub fn make_anon_regex(AnonRegexArgs { pat, flags }: AnonRegexArgs) -> TokenStream {
    let impl_tokens = make_regex(
        RegexArgs {
            vis: Visibility::Inherited,
            name: Ident::new("__AnonRegex", Span::call_site()),
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
