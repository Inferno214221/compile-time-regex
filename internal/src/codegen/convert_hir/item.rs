use quote::ToTokens;
use regex_syntax::hir::{Class, ClassBytes, ClassBytesRange, ClassUnicode, ClassUnicodeRange};

use crate::codegen::IntoMatcherExpr;
use crate::haystack::HaystackItem;

pub(crate) trait CodegenItem: HaystackItem + IntoMatcherExpr + ToTokens {
    type HirClass: ClassIter<Self>;

    fn upcast_byte(byte: u8) -> Self;
    fn upcast_char(scalar: char) -> Self;

    fn normalize_class(value: Class) -> Self::HirClass;
}

impl CodegenItem for u8 {
    type HirClass = ClassBytes;

    fn upcast_byte(byte: u8) -> Self {
        byte
    }

    fn upcast_char(_scalar: char) -> Self {
        panic!("failed to assert type equality u8 != char")
    }

    fn normalize_class(value: Class) -> Self::HirClass {
        match value {
            Class::Unicode(unicode) => {
                unicode.to_byte_class().expect("failed to convert to byte class")
            },
            Class::Bytes(bytes) => bytes,
        }
    }
}

impl CodegenItem for char {
    type HirClass = ClassUnicode;

    fn upcast_byte(_byte: u8) -> Self {
        panic!("failed to assert type equality char != u8")
    }

    fn upcast_char(scalar: char) -> Self {
        scalar
    }

    fn normalize_class(value: Class) -> Self::HirClass {
        match value {
            Class::Unicode(unicode) => unicode,
            Class::Bytes(bytes) => {
                bytes.to_unicode_class().expect("failed to convert to unicode class")
            },
        }
    }
}

pub(crate) trait ClassIter<I: CodegenItem> {
    type Range: ClassRange<I>;

    fn ranges(&self) -> impl Iterator<Item = &Self::Range>;
}

impl ClassIter<char> for ClassUnicode {
    type Range = ClassUnicodeRange;

    fn ranges(&self) -> impl Iterator<Item = &Self::Range> {
        self.ranges().iter()
    }
}

impl ClassIter<u8> for ClassBytes {
    type Range = ClassBytesRange;

    fn ranges(&self) -> impl Iterator<Item = &Self::Range> {
        self.ranges().iter()
    }
}

pub(crate) trait ClassRange<I: CodegenItem> {
    fn start(&self) -> I;
    fn end(&self) -> I;
}

impl ClassRange<char> for ClassUnicodeRange {
    fn start(&self) -> char {
        self.start()
    }

    fn end(&self) -> char {
        self.end()
    }
}

impl ClassRange<u8> for ClassBytesRange {
    fn start(&self) -> u8 {
        self.start()
    }

    fn end(&self) -> u8 {
        self.end()
    }
}