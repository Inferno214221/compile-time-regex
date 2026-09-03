use quote::ToTokens;
use regex_syntax::hir::Class;

use crate::codegen::IntoMatcherExpr;
use crate::haystack::HaystackItem;

pub(crate) trait CodegenItem: HaystackItem + IntoMatcherExpr + NormalizeClass + ToTokens {
    fn upcast_byte(byte: u8) -> Self;
    fn upcast_char(scalar: char) -> Self;
}

impl CodegenItem for u8 {
    fn upcast_byte(byte: u8) -> Self {
        byte
    }

    fn upcast_char(_scalar: char) -> Self {
        panic!("failed to assert type equality u8 != char")
    }
}

impl CodegenItem for char {
    fn upcast_byte(_byte: u8) -> Self {
        panic!("failed to assert type equality char != u8")
    }

    fn upcast_char(scalar: char) -> Self {
        scalar
    }
}

pub(crate) trait NormalizeClass {
    fn normalize_class(value: Class) -> Class;
}

impl NormalizeClass for u8 {
    fn normalize_class(value: Class) -> Class {
        match value {
            Class::Unicode(unicode) => Class::Bytes(
                unicode.to_byte_class().expect("failed to convert to byte class")
            ),
            Class::Bytes(bytes) => Class::Bytes(bytes),
        }
    }
}

impl NormalizeClass for char {
    fn normalize_class(value: Class) -> Class {
        match value {
            Class::Unicode(unicode) => Class::Unicode(unicode),
            Class::Bytes(bytes) => Class::Unicode(
                bytes.to_unicode_class().expect("failed to convert to unicode class")
            ),
        }
    }
}
