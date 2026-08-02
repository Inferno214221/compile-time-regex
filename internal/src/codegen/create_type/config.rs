use regex_syntax::ast::parse::ParserBuilder;
use regex_syntax::hir::translate::TranslatorBuilder;

#[derive(Debug, Default, Clone)]
pub(crate) struct ConfigExt {
    pub ast: ParserBuilder,
    pub hir: TranslatorBuilder,
    pub complex_classes: bool,
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
        swap_greed,
        unicode,
        utf8
    }

    pub fn ignore_whitespace(&mut self, flag: bool) -> &mut Self {
        self.ast.ignore_whitespace(flag);
        self
    }

    pub fn complex_classes(&mut self, flag: bool) -> &mut Self {
        self.complex_classes = flag;
        self
    }
}
