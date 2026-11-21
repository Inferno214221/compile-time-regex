use quote::{ToTokens, format_ident};
use regex_syntax::hir::{Class, ClassBytes, ClassBytesRange, ClassUnicode, ClassUnicodeRange};
use syn::Ident;

use crate::{codegen::ConfigExt, haystack::HaystackItem};

pub(crate) trait CodegenItem: HaystackItem + ToTokens {
    type HirClass: ClassIter<Self>;
    fn set_item_type(config: &mut ConfigExt);
    // TODO: Classes aren't guaranteed to match the unicode flag, which will cause errors. See
    // `(?-u:a|\xc2\xa0)`
    fn normalize_class(value: Class) -> Self::HirClass;
    fn type_ident() -> Ident;
    fn ident_fragment() -> Ident;
}

impl CodegenItem for u8 {
    type HirClass = ClassBytes;

    fn set_item_type(config: &mut ConfigExt) {
        config.set_item_byte();
    }

    fn normalize_class(value: Class) -> Self::HirClass {
        match value {
            Class::Unicode(unicode) => {
                unicode.to_byte_class().expect("failed to convert to byte class")
            },
            Class::Bytes(bytes) => bytes,
        }
    }

    fn type_ident() -> Ident {
        format_ident!("u8")
    }

    fn ident_fragment() -> Ident {
        format_ident!("Byte")
    }
}

impl CodegenItem for char {
    type HirClass = ClassUnicode;

    fn set_item_type(config: &mut ConfigExt) {
        config.set_item_char();
    }

    fn normalize_class(value: Class) -> Self::HirClass {
        match value {
            Class::Unicode(unicode) => unicode,
            Class::Bytes(bytes) => {
                bytes.to_unicode_class().expect("failed to convert to unicode class")
            },
        }
    }

    fn type_ident() -> Ident {
        format_ident!("char")
    }

    fn ident_fragment() -> Ident {
        format_ident!("Scalar")
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