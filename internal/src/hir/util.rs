use std::any;

use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use regex_syntax::hir::{Class, Hir};

use crate::{haystack::HaystackItem, hir::{Groups, Group, write_matcher::WriteMatcher}};

pub fn type_name<T>() -> &'static str {
    any::type_name::<T>()
        .split('<').next().unwrap()
        .rsplit("::").next().unwrap()
}

pub fn type_ident<T>() -> Ident {
    format_ident!("{}", type_name::<T>())
}

pub trait HirExtension {
    fn into_matcher<I: HaystackItem>(self) -> (TokenStream, Vec<Group>);
}

impl HirExtension for Hir {
    fn into_matcher<I: HaystackItem>(self) -> (TokenStream, Vec<Group>) {
        let mut caps = Groups::new();
        let tokens = self.write_matcher::<I>(&mut caps);
        (tokens, caps.into_vec())
    }
}

pub trait CastClass {
    fn cast_class(value: Class) -> Class;
}

impl CastClass for u8 {
    fn cast_class(value: Class) -> Class {
        match value {
            Class::Unicode(unicode) => Class::Bytes(
                unicode.to_byte_class().expect("failed to convert to byte class")
            ),
            Class::Bytes(bytes) => Class::Bytes(bytes),
        }
    }
}

impl CastClass for char {
    fn cast_class(value: Class) -> Class {
        match value {
            Class::Unicode(unicode) => Class::Unicode(unicode),
            Class::Bytes(bytes) => Class::Unicode(
                bytes.to_unicode_class().expect("failed to convert to unicode class")
            ),
        }
    }
}