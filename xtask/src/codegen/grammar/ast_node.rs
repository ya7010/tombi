use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use ungrammar::Grammar;

use crate::{
    codegen::grammar::{ast_src::AstEnumSrc, lower_enum},
    utils::reformat,
};

use super::{ast_src::AstSrc, syntax_kind_src::NODES};

pub fn generate_ast_node(ast: &AstSrc) -> Result<String, anyhow::Error> {
    let nodes = NODES.iter().map(|token| {
        let name = format_ident!("{}", token.to_case(Case::Pascal));
        let kind = format_ident!("{}", token.to_case(Case::UpperSnake));
        quote! {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct #name {
                pub(crate) syntax: SyntaxNode,
            }
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::Display::fmt(&self.syntax, f)
                }
            }
            impl AstNode for #name {
                fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::#kind }
                fn cast(syntax: SyntaxNode) -> Option<Self> {
                    if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
                }
                fn syntax(&self) -> &SyntaxNode { &self.syntax }
            }
        }
    });

    reformat(
        quote! {
            use crate::AstNode;
            use syntax::{SyntaxKind, SyntaxNode};
            #(#nodes)*
        }
        .to_string(),
    )
    .map(|content| content.replace("#[derive", "\n#[derive"))
}
