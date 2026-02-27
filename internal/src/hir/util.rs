use std::any;

use regex_syntax::hir::{Class, Hir};

use crate::{haystack::HaystackItem, hir::write_matcher::WriteMatcher};

pub fn type_name<T>() -> &'static str {
    any::type_name::<T>().split('<').next().unwrap()
}

pub trait HirExtension {
    fn into_type_expr<I: HaystackItem>(self) -> String;
}

impl HirExtension for Hir {
    fn into_type_expr<I: HaystackItem>(self) -> String {
        let mut string = String::new();
        self.write_matcher::<I>(&mut string).unwrap();
        string
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