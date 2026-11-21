use regex_syntax::ast::parse::ParserBuilder;
use regex_syntax::hir::translate::TranslatorBuilder;

#[derive(Debug, Clone)]
pub(crate) struct ConfigExt {
    pub enforce_utf8: bool,
    pub ast: ParserBuilder,
    pub hir: TranslatorBuilder,
    pub complex_classes: bool,
}

impl Default for ConfigExt {
    fn default() -> Self {
        Self {
            enforce_utf8: true,
            ast: Default::default(),
            hir: Default::default(),
            complex_classes: Default::default()
        }
    }
}

macro_rules! impl_hir_methods {
    ($name:ident) => {
        pub fn $name(&mut self, flag: bool) -> &mut Self {
            self.hir.$name(flag);
            self
        }
    };
    ($name:ident, $($others:ident),+) => {
        impl_hir_methods! { $name }
        impl_hir_methods! { $($others),+ }
    };
}

impl ConfigExt {
    impl_hir_methods! {
        case_insensitive,
        multi_line,
        dot_matches_new_line,
        crlf,
        swap_greed
    }

    pub fn ignore_whitespace(&mut self, flag: bool) -> &mut Self {
        self.ast.ignore_whitespace(flag);
        self
    }

    pub fn complex_classes(&mut self, flag: bool) -> &mut Self {
        self.complex_classes = flag;
        self
    }

    pub fn set_item_byte(&mut self) -> &mut Self {
        self.hir.unicode(false);
        self
    }

    pub fn set_item_char(&mut self) -> &mut Self {
        self.hir.unicode(true);
        self
    }

    pub fn enforce_utf8(&mut self, flag: bool) -> &mut Self {
        self.enforce_utf8 = flag;
        self.hir.utf8(flag);
        self
    }
}

/// The documentation on regex_syntax in kinda unclear so I'm going to summarize here:
///
/// - [`TranslatorBuilder::unicode`]: Determines whether the resulting expression matches on a `u8`
///   or `char` basis, with a value of true matching only `char`s.
/// - [`TranslatorBuilder::utf8`]: Determine whether the resulting expression can match a haystack
///   that contains invalid utf8, regardless of the item type. In Rust however, using only `char`s,
///   you can't match invalid utf8, so this is only relevant for matching bytes. This method is not
///   the flag, and setting it to true will not permit opting out within an expression. Setting this
///   to true has the effect of asserting that all `Hir`s return true when `Properties::is_utf8` is
///   called.
///
/// See [`Properties::is_utf8`](regex_syntax::hir::Properties::is_utf8).
mod note {}
