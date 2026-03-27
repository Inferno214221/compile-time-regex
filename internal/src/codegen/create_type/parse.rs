use proc_macro2::TokenStream;
use regex_syntax::{ast::{Ast, ClassBracketed, ClassPerl, ClassPerlKind, ClassSet, ClassSetBinaryOp, ClassSetItem, ClassSetRange, ClassSetUnion, HexLiteralKind, Literal, LiteralKind, SpecialLiteralKind, parse::ParserBuilder}, hir::translate::TranslatorBuilder};

use crate::codegen::{CodegenItem, Group, HirExtension};

pub fn parse_regex<I: CodegenItem>(pat: &str, config: &ConfigExt) -> (TokenStream, Vec<Group>) {
    let mut ast = config.ast.build()
        .parse(&pat)
        .expect("failed to parse regex");

    if !config.complex_classes {
        simplify_classes(&mut ast);
    }

    config.hir.build()
        .translate(&pat, &ast)
        .expect("failed to parse regex")
        .into_matcher::<I>()
}

pub fn simplify_classes(ast: &mut Ast) {
    let replacement = match ast {
        Ast::ClassPerl(class) =>      replace_pearl_class(class),
        Ast::ClassBracketed(class) => return replace_in_class(&mut class.kind),
        Ast::Repetition(rep) =>       return simplify_classes(&mut rep.ast),
        Ast::Group(group) =>          return simplify_classes(&mut group.ast),
        Ast::Alternation(alt) =>      return alt.asts.iter_mut().for_each(simplify_classes),
        Ast::Concat(cat) =>           return cat.asts.iter_mut().for_each(simplify_classes),
        _ => return,
    };
    *ast = Ast::ClassBracketed(Box::new(replacement));
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
        ClassSetItem::Perl(class) =>      replace_pearl_class(class),
        ClassSetItem::Bracketed(class) => return replace_in_class(&mut class.kind),
        ClassSetItem::Union(class) => {
            return class.items.iter_mut().for_each(replace_in_class_set_item);
        },
        _ => return,
    };
    *item = ClassSetItem::Bracketed(Box::new(replacement));
}

pub fn replace_pearl_class(class: &mut ClassPerl) -> ClassBracketed {
    match class.kind {
        ClassPerlKind::Digit => ClassBracketed {
            span: class.span,
            negated: class.negated,
            kind: ClassSet::Item(ClassSetItem::Range(ClassSetRange {
                span: class.span,
                start: Literal {
                    span: class.span,
                    kind: LiteralKind::Verbatim,
                    c: '0',
                },
                end: Literal {
                    span: class.span,
                    kind: LiteralKind::Verbatim,
                    c: '9',
                },
            }))
        },
        ClassPerlKind::Space => ClassBracketed {
            span: class.span,
            negated: class.negated,
            kind: ClassSet::Item(ClassSetItem::Union(ClassSetUnion {
                span: class.span,
                items: vec![
                    ClassSetItem::Literal(Literal {
                        span: class.span,
                        kind: LiteralKind::Special(SpecialLiteralKind::FormFeed),
                        c: '\x0C'
                    }),
                    ClassSetItem::Literal(Literal {
                        span: class.span,
                        kind: LiteralKind::Special(SpecialLiteralKind::LineFeed),
                        c: '\n'
                    }),
                    ClassSetItem::Literal(Literal {
                        span: class.span,
                        kind: LiteralKind::Special(SpecialLiteralKind::CarriageReturn),
                        c: '\r'
                    }),
                    ClassSetItem::Literal(Literal {
                        span: class.span,
                        kind: LiteralKind::Special(SpecialLiteralKind::Tab),
                        c: '\t'
                    }),
                    ClassSetItem::Literal(Literal {
                        span: class.span,
                        kind: LiteralKind::Special(SpecialLiteralKind::VerticalTab),
                        c: '\x08'
                    }),
                    ClassSetItem::Literal(Literal {
                        span: class.span,
                        kind: LiteralKind::Verbatim,
                        c: ' '
                    }),
                    ClassSetItem::Literal(Literal {
                        span: class.span,
                        kind: LiteralKind::HexFixed(HexLiteralKind::X),
                        c: '\u{00a0}'
                    }),
                ],
            })),
        },
        ClassPerlKind::Word => ClassBracketed {
            span: class.span,
            negated: class.negated,
            kind: ClassSet::Item(ClassSetItem::Union(ClassSetUnion {
                span: class.span,
                items: vec![
                    ClassSetItem::Range(ClassSetRange {
                        span: class.span,
                        start: Literal {
                            span: class.span,
                            kind: LiteralKind::Verbatim,
                            c: 'A',
                        },
                        end: Literal {
                            span: class.span,
                            kind: LiteralKind::Verbatim,
                            c: 'Z',
                        },
                    }),
                    ClassSetItem::Range(ClassSetRange {
                        span: class.span,
                        start: Literal {
                            span: class.span,
                            kind: LiteralKind::Verbatim,
                            c: 'a',
                        },
                        end: Literal {
                            span: class.span,
                            kind: LiteralKind::Verbatim,
                            c: 'z',
                        },
                    }),
                    ClassSetItem::Range(ClassSetRange {
                        span: class.span,
                        start: Literal {
                            span: class.span,
                            kind: LiteralKind::Verbatim,
                            c: '0',
                        },
                        end: Literal {
                            span: class.span,
                            kind: LiteralKind::Verbatim,
                            c: '9',
                        },
                    }),
                    ClassSetItem::Literal(Literal {
                        span: class.span,
                        kind: LiteralKind::Verbatim,
                        c: '_'
                    }),
                ],
            })),
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