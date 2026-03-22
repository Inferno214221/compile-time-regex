use regex_syntax::hir::Class;

use crate::{codegen::IntoMatcherExpr, haystack::HaystackItem};

pub trait CodegenItem: HaystackItem + IntoMatcherExpr + NormalizeClass {}

impl CodegenItem for u8 {}
impl CodegenItem for char {}

pub trait NormalizeClass {
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