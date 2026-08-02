use proc_macro2::TokenStream;
use quote::quote;
use regex_syntax::hir::{Hir, Properties};
use syn::Ident;

use crate::codegen::{CodegenItem, ConfigExt, ExprMetadata, IntoMatcherExpr, classes};

#[derive(Debug, Clone)]
pub struct TypeExpressions {
    pub matcher: TokenStream,
    pub anchors: TokenStream,
    pub meta: ExprMetadata,
}

impl TypeExpressions {
    pub fn parse_regex<I: CodegenItem>(name: &Ident, pat: &str, config: &ConfigExt) -> TypeExpressions {
        let mut ast = config.ast.build()
            .parse(pat)
            .expect("failed to parse regex");

        if !config.complex_classes {
            classes::simplify_classes(&mut ast);
        }

        let hir = config.hir.build()
            .translate(pat, &ast)
            .expect("failed to parse regex");

        TypeExpressions::create::<I>(hir, name)
    }

    pub fn create<I: CodegenItem>(hir: Hir, name: &Ident) -> TypeExpressions {
        let mut meta = ExprMetadata::new(name.clone());
        let anchors = TypeExpressions::create_anchor_expression(hir.properties());

        TypeExpressions {
            matcher: hir.into_matcher_expr::<I>(&mut meta),
            anchors,
            meta
        }
    }

    pub fn create_anchor_expression(props: &Properties) -> TokenStream {
        let mut anchors = Vec::new();

        if props.look_set_prefix().contains_anchor_haystack() {
            anchors.push(quote!(::ct_regex::internal::matcher::anchor::Start));
        }
        if let Some(min) = props.minimum_len() {
            anchors.push(quote!(::ct_regex::internal::matcher::anchor::MinLen<#min>));
        }
        if let Some(max) = props.maximum_len() {
            if props.look_set_suffix().contains_anchor_haystack() {
                anchors.push(quote!(::ct_regex::internal::matcher::anchor::EndAndMaxLen<#max>));
            } else {
                anchors.push(quote!(::ct_regex::internal::matcher::anchor::MaxLen<#max>));
            }
        }

        match &anchors[..] {
            [] => quote!(::ct_regex::internal::matcher::anchor::AnchorNone),
            [a] => quote!(#a),
            [a, b] => quote!(::ct_regex::internal::matcher::anchor::AnchorPair<#a, #b>),
            [a, b, c] => quote!(::ct_regex::internal::matcher::anchor::AnchorSet<#a, #b, #c>),
            _ => panic!("an excessive number for anchor assertions were found"),
        }
    }
}