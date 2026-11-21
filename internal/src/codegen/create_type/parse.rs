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
        let anchors = TypeExpressions::<I>::create_anchor_expression(hir.properties(), config);

        TypeExpressions {
            matcher: Self::process_hir(hir, config).into_matcher_expr::<I>(&mut meta),
            anchors,
            meta,
        }
    }

    pub fn create_anchor_expression(props: &Properties, config: &ConfigExt) -> TokenStream {
        let anchor = quote!(::ct_regex::internal::anchor);
        let mut anchors = Vec::new();

        if props.look_set_prefix().contains(Look::Start) {
            anchors.push(quote!(#anchor::Start));
        }
        match props.minimum_len() {
            None | Some(0) if config.enforce_utf8 => {
                anchors.push(quote!(#anchor::ZeroWidthBoundary))
            },
            Some(min) => {
                anchors.push(quote!(#anchor::MinLen<#min>))
            },
            _ => {}
        }
        if let Some(max) = props.maximum_len() {
            if props.look_set_suffix().contains(Look::End) {
                anchors.push(quote!(#anchor::EndAndMaxLen<#max>));
            } else {
                anchors.push(quote!(#anchor::MaxLen<#max>));
            }
        }

        match_vec!(match anchors {
            [] => quote!(#anchor::AnchorNone),
            [a] => a,
            [a, b] => quote!(#anchor::AnchorPair<#a, #b>),
            [a, b, c] => quote!(#anchor::AnchorSet<#a, #b, #c>),
            [a, b, c, d] => quote!(
                #anchor::AnchorPair<#anchor::AnchorPair<#a, #b>, #anchor::AnchorPair<#c, #d>>
            ),
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

    pub fn process_hir(hir: Hir, _config: &ConfigExt) -> impl IntoMatcherExpr {
        Self::remove_redundant_lookarounds(hir)
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

    pub fn wrap_zero_with(hir: Hir, config: &ConfigExt) -> CheckZeroWith {
        CheckZeroWith(
            config.enforce_utf8 && hir.properties().minimum_len().unwrap_or(0) == 0,
            hir
        )
    }

//     pub fn prevent_unicode_opt_out(hir: &Hir, config: &ConfigExt) {
//         dbg!(hir);
//         // Allow opting in if the expression itself isn't unicode.
//         if config.enforce_utf8 {
//             Self::assert_utf8(hir);
//         }
//     }
//
//     pub fn assert_utf8(hir: &Hir) {
//         assert!(hir.properties().is_utf8());
//         match hir.kind() {
//             HirKind::Repetition(rep)   => Self::assert_utf8(&rep.sub),
//             HirKind::Capture(cap)      => Self::assert_utf8(&cap.sub),
//             HirKind::Concat(hirs)      => hirs.iter().for_each(Self::assert_utf8),
//             HirKind::Alternation(hirs) => hirs.iter().for_each(Self::assert_utf8),
//             _ => {},
//         };
//     }
}
