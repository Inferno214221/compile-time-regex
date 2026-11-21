use match_vec::match_vec;
use proc_macro2::TokenStream;
use quote::quote;
use regex_syntax::{ast::Ast, hir::{Capture, Hir, HirKind, Look, Properties}};
use syn::Ident;

use crate::codegen::{CodegenItem, ConfigExt, ExprMetadata, IntoMatcherExpr, simplify_classes};

#[derive(Debug, Clone)]
pub(crate) struct TypeExpressions<I: CodegenItem> {
    pub matcher: TokenStream,
    pub anchors: TokenStream,
    pub meta: ExprMetadata<I>,
}

impl<I: CodegenItem> TypeExpressions<I> {
    pub fn parse_regex(name: &Ident, pat: &str, config: &ConfigExt) -> TypeExpressions<I> {
        let mut config = config.clone();
        I::set_item_type(&mut config);

        let mut ast = config.ast.build()
            .parse(pat)
            .expect("failed to parse regex");

        Self::process_ast(&mut ast, &config);

        let hir = config.hir.build()
            .translate(pat, &ast)
            .expect("failed to parse regex");

        TypeExpressions::<I>::create(hir, name, &config)
    }

    pub fn create(hir: Hir, name: &Ident, config: &ConfigExt) -> TypeExpressions<I> {
        let mut meta = ExprMetadata::new(name.clone());
        let anchors = TypeExpressions::<I>::create_anchor_expression(hir.properties());

        TypeExpressions {
            matcher: Self::process_hir(hir, config).into_matcher_expr::<I>(&mut meta),
            anchors,
            meta,
        }
    }

    pub fn create_anchor_expression(props: &Properties) -> TokenStream {
        let mut anchors = Vec::new();

        if props.look_set_prefix().contains(Look::Start) {
            anchors.push(quote!(::ct_regex::internal::anchor::Start));
        }
        if let Some(min) = props.minimum_len() && min != 0 {
            anchors.push(quote!(::ct_regex::internal::anchor::MinLen<#min>));
        }
        if let Some(max) = props.maximum_len() {
            if props.look_set_suffix().contains(Look::End) {
                anchors.push(quote!(::ct_regex::internal::anchor::EndAndMaxLen<#max>));
            } else {
                anchors.push(quote!(::ct_regex::internal::anchor::MaxLen<#max>));
            }
        }

        match_vec!(match anchors {
            [] => quote!(::ct_regex::internal::anchor::AnchorNone),
            [a] => a,
            [a, b] => quote!(::ct_regex::internal::anchor::AnchorPair<#a, #b>),
            [a, b, c] => quote!(::ct_regex::internal::anchor::AnchorSet<#a, #b, #c>),
            _ => panic!("an excessive number for anchor assertions were found"),
        })
    }

    pub fn process_ast(ast: &mut Ast, config: &ConfigExt) {
        Self::simplify_classes(ast, config)
    }

    pub fn simplify_classes(ast: &mut Ast, config: &ConfigExt) {
        if !config.complex_classes {
            simplify_classes::simplify_classes(ast);
        }
    }

    pub fn process_hir(mut hir: Hir, config: &ConfigExt) -> Hir {
        hir = Self::remove_redundant_lookarounds(hir);
        hir
    }

    pub fn remove_redundant_lookarounds(hir: Hir) -> Hir {
        // Smart constructors mean that we have to do this using references and clones.
        match hir.kind() {
            HirKind::Look(Look::Start) => Hir::empty(),
            HirKind::Concat(sub) => match &sub[..] {
                [first, other] if first.kind() == &HirKind::Look(Look::Start) => {
                    other.clone()
                },
                [first, remainder @ ..] if first.kind() == &HirKind::Look(Look::Start) => {
                    Hir::concat(remainder.to_vec())
                },
                _ => hir,
            },
            HirKind::Capture(cap) => Hir::capture(Capture {
                index: cap.index,
                name: cap.name.clone(),
                sub: Box::new(Self::remove_redundant_lookarounds((*cap.sub).clone())),
            }),
            _ => hir,
        }
    }
}
