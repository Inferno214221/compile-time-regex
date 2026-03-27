use proc_macro2::{Literal, Span, TokenStream};
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
    impl_anon: bool
) -> TokenStream {
    // Aliases
    #![allow(nonstandard_style)]
    let fmt = quote!(::std::fmt);
    let Regex = quote!(::ct_regex::internal::expr::Regex);
    let AnonRegex = quote!(::ct_regex::internal::expr::AnonRegex);

    let pat_str = pat.value();

    let doc = format!("A macro-generated regular expression matching the pattern: `{}` with flags: \
        {}", pat_str, flags);

    let mut config = flags.create_config();
    config.unicode(false).utf8(false);

    let (type_expr_byte, groups) = parse::parse_regex::<u8>(&pat_str, &config);

    config.unicode(true).utf8(true);

    let (type_expr_scalar, _) = parse::parse_regex::<char>(&pat_str, &config);

    let captures_name = format_ident!("{}Capture", name);
    let captures_len = Literal::usize_unsuffixed(groups.len());
    let captures_impl = capture::impl_captures(&vis, &captures_name, groups);

    let anon_impl = if impl_anon {
        quote! {
            impl #AnonRegex<u8, #captures_len> for #name {}

            impl #AnonRegex<char, #captures_len> for #name {}
        }
    } else {
        quote!()
    };

    quote! {
        #[doc = #doc]
        #vis struct #name;

        impl #Regex<u8, #captures_len> for #name {
            type Pattern = #type_expr_byte;

            type Capture<'a> = #captures_name<'a, u8>;
        }

        impl #Regex<char, #captures_len> for #name {
            type Pattern = #type_expr_scalar;

            type Capture<'a> = #captures_name<'a, char>;
        }

        #anon_impl

        impl #fmt::Debug for #name {
            fn fmt(&self, f: &mut #fmt::Formatter<'_>) -> #fmt::Result {
                write!(f, "/{:?}/", <Self as #Regex<char, #captures_len>>::Pattern::default())
            }
        }

        #captures_impl
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
        true
    );
    quote! {
        {
            #impl_tokens

            __AnonRegex
        }
    }
}