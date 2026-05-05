use proc_macro2::TokenStream;
use regex_syntax::ast::parse::ParserBuilder;
use regex_syntax::ast::{
    Ast, ClassAscii, ClassAsciiKind, ClassBracketed, ClassPerl, ClassPerlKind, ClassSet,
    ClassSetBinaryOp, ClassSetItem,
};
use regex_syntax::hir::translate::TranslatorBuilder;

use crate::codegen::{CodegenItem, Group, HirExtension};

pub fn parse_regex<I: CodegenItem>(pat: &str, config: &ConfigExt) -> (TokenStream, Vec<Group>) {
    let mut ast = config.ast.build()
        .parse(pat)
        .expect("failed to parse regex");

    if !config.complex_classes {
        simplify_classes(&mut ast);
    }

    config.hir.build()
        .translate(pat, &ast)
        .expect("failed to parse regex")
        .into_matcher::<I>()
}

pub fn simplify_classes(ast: &mut Ast) {
    let replacement = match ast {
        Ast::ClassPerl(class) =>      replace_perl_class(class),
        Ast::ClassBracketed(class) => return replace_in_class(&mut class.kind),
        Ast::Repetition(rep) =>       return simplify_classes(&mut rep.ast),
        Ast::Group(group) =>          return simplify_classes(&mut group.ast),
        Ast::Alternation(alt) =>      return alt.asts.iter_mut().for_each(simplify_classes),
        Ast::Concat(cat) =>           return cat.asts.iter_mut().for_each(simplify_classes),
        _ => return,
    };
    *ast = Ast::ClassBracketed(Box::new(ClassBracketed {
        span: *ast.span(),
        negated: false,
        kind: ClassSet::Item(ClassSetItem::Ascii(replacement))
    }));
}

pub fn replace_in_class(class: &mut ClassSet) {
    match class {
        ClassSet::BinaryOp(ClassSetBinaryOp { lhs, rhs, .. }) => {
            replace_in_class(lhs);
            replace_in_class(rhs);
        },
        ClassSet::Item(item) => replace_in_class_set_item(item),
    }
}

pub fn replace_in_class_set_item(item: &mut ClassSetItem) {
    let replacement = match item {
        ClassSetItem::Perl(class) =>      replace_perl_class(class),
        ClassSetItem::Bracketed(class) => return replace_in_class(&mut class.kind),
        ClassSetItem::Union(class) => {
            return class.items.iter_mut().for_each(replace_in_class_set_item);
        },
        _ => return,
    };
    *item = ClassSetItem::Ascii(replacement);
}

pub fn replace_perl_class(class: &mut ClassPerl) -> ClassAscii {
    ClassAscii {
        span: class.span,
        negated: class.negated,
        kind: match class.kind {
            ClassPerlKind::Digit => ClassAsciiKind::Digit,
            ClassPerlKind::Space => ClassAsciiKind::Space,
            ClassPerlKind::Word => ClassAsciiKind::Word,
        },
    }
}

#[derive(Debug, Default, Clone)]
pub struct ConfigExt {
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
