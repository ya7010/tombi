use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use ungrammar::Grammar;

use crate::{
    codegen::grammar::{ast_src::AstEnumSrc, lower_enum},
    utils::reformat,
};

use super::{ast_src::AstSrc, syntax_kind_src::NODES};

pub fn generate_ast_node(ast: &AstSrc) -> Result<String, anyhow::Error> {
    let (node_defs, node_boilerplate_impls): (Vec<_>, Vec<_>) = ast
        .nodes
        .iter()
        .map(|node| {
            let name = format_ident!("{}", node.name);
            let kind = format_ident!("{}", node.name.to_case(Case::UpperSnake));
            let traits = node.traits.iter().map(|trait_name| {
                let trait_name = format_ident!("{}", trait_name);
                quote!(impl ast::#trait_name for #name {})
            });

            let methods = node.fields.iter().map(|field| {
                let method_name = format_ident!("{}", field.method_name());
                let ty = field.ty();

                if field.is_many() {
                    quote! {
                        #[inline]
                        pub fn #method_name(&self) -> AstChildren<#ty> {
                            support::children(&self.syntax)
                        }
                    }
                } else if let Some(token_kind) = field.token_kind() {
                    quote! {
                        #[inline]
                        pub fn #method_name(&self) -> Option<#ty> {
                            support::token(&self.syntax, #token_kind)
                        }
                    }
                } else {
                    quote! {
                        #[inline]
                        pub fn #method_name(&self) -> Option<#ty> {
                            support::child(&self.syntax)
                        }
                    }
                }
            });
            (
                quote! {
                    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                    pub struct #name {
                        pub(crate) syntax: SyntaxNode,
                    }

                    #(#traits)*

                    impl #name {
                        #(#methods)*
                    }
                },
                quote! {
                    impl AstNode for #name {
                        #[inline]
                        fn can_cast(kind: SyntaxKind) -> bool {
                            kind == SyntaxKind::#kind
                        }
                        #[inline]
                        fn cast(syntax: SyntaxNode) -> Option<Self> {
                            if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
                        }
                        #[inline]
                        fn syntax(&self) -> &SyntaxNode { &self.syntax }
                    }
                },
            )
        })
        .unzip();

    reformat(
        quote! {
            use crate::AstNode;
            use syntax::{SyntaxKind, SyntaxNode, SyntaxToken, T};
            use crate::support;
            use crate::AstChildren;

            #(#node_defs)*
            #(#node_boilerplate_impls)*
        }
        .to_string(),
    )
    .map(|content| content.replace("#[derive", "\n#[derive"))
}
