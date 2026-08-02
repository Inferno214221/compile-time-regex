use regex_syntax::ast::{
    Ast, ClassAscii, ClassAsciiKind, ClassBracketed, ClassPerl, ClassPerlKind, ClassSet,
    ClassSetBinaryOp, ClassSetItem,
};

pub(crate) fn simplify_classes(ast: &mut Ast) {
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
        kind: ClassSet::Item(ClassSetItem::Ascii(replacement)),
    }));
}

pub(crate) fn replace_in_class(class: &mut ClassSet) {
    match class {
        ClassSet::BinaryOp(ClassSetBinaryOp { lhs, rhs, .. }) => {
            replace_in_class(lhs);
            replace_in_class(rhs);
        },
        ClassSet::Item(item) => replace_in_class_set_item(item),
    }
}

pub(crate) fn replace_in_class_set_item(item: &mut ClassSetItem) {
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

pub(crate) fn replace_perl_class(class: &mut ClassPerl) -> ClassAscii {
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