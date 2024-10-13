use convert_case::{Case, Casing};
use proc_macro2::{Punct, Spacing};
use quote::{format_ident, quote};
use ungrammar::Grammar;

use super::syntax_kind_src::{KEYWORDS, LITERALS, NODES, PUNCTUATIONS, TOKENS};

pub fn generate_syntax_kind(_grammar: &Grammar) -> Result<String, anyhow::Error> {
    let punctuation_values = PUNCTUATIONS.iter().map(|(token, _)| {
        if "{}[]()".contains(token) {
            let c = token.chars().next().unwrap();
            quote! { #c }
        } else {
            let cs = token.chars().map(|c| Punct::new(c, Spacing::Joint));
            quote! { #(#cs)* }
        }
    });
    let punctuations = PUNCTUATIONS
        .iter()
        .map(|(_, name)| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let keyword_idents = KEYWORDS
        .iter()
        .map(|kw| format_ident!("{}", kw))
        .collect::<Vec<_>>();
    let keywords = KEYWORDS
        .iter()
        .map(|kw| format_ident!("{}_KW", kw.to_case(Case::Upper)))
        .collect::<Vec<_>>();

    let literals = LITERALS
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let tokens = TOKENS
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let nodes = NODES
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let token = quote! {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        #[repr(u16)]
        #[allow(non_camel_case_types)]
        pub enum SyntaxKind {
            #(#punctuations,)*
            #(#keywords,)*
            #(#literals,)*
            #(#tokens,)*
            #(#nodes,)*
        }

        use self::SyntaxKind::*;

        impl SyntaxKind {
            pub fn is_keyword(self) -> bool {
                match self {
                    #(#keywords)|* => true,
                    _ => false,
                }
            }
        }

        impl From<SyntaxKind> for rowan::SyntaxKind {
            #[inline]
            fn from(k: SyntaxKind) -> Self {
                Self(k as u16)
            }
        }

        /// Utility macro for creating a SyntaxKind through simple macro syntax
        #[macro_export]
        macro_rules! T {
            #([#punctuation_values] => { $crate::SyntaxKind::#punctuations };)*
            #([#keyword_idents] => { $crate::SyntaxKind::#keywords };)*
        }
    };

    crate::utils::reformat(token)
}
