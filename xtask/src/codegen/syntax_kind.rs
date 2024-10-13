use convert_case::{Case, Casing};
use proc_macro2::{Literal, Punct, Spacing};
use quote::{format_ident, quote};
use ungrammar::Grammar;

use super::syntax_kind_src::{KEYWORDS, PUNCTUATIONS};

pub fn generate_syntax_kind(_grammer: &Grammar) -> Result<String, anyhow::Error> {
    let punctuation_values = PUNCTUATIONS.iter().map(|(token, _)| {
        if "{}[]()".contains(token) {
            let c = token.chars().next().unwrap();
            quote! { #c }
        } else {
            let cs = token.chars().map(|c| Punct::new(c, Spacing::Joint));
            quote! { #(#cs)* }
        }
    });
    let punctuation = PUNCTUATIONS
        .iter()
        .map(|(_, name)| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let keyword_idents = KEYWORDS
        .iter()
        .map(|kw| format_ident!("{}", kw))
        .collect::<Vec<_>>();
    let keyword = KEYWORDS
        .iter()
        .map(|kw| format_ident!("{}_KW", kw.to_case(Case::Upper)))
        .collect::<Vec<_>>();

    let token = quote! {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        #[repr(u16)]
        pub enum SyntaxToken {
            #(#punctuation,)*
            #(#keyword,)*
            /// Marks the end of the file. May have trivia attached
            EOF,
        }

        use self::SyntaxToken::*;

        impl SyntaxToken {
            pub fn is_keyword(self) -> bool {
                match self {
                    #(#keyword)|* => true,
                    _ => false,
                }
            }
        }

        /// Utility macro for creating a SyntaxKind through simple macro syntax
        #[macro_export]
        macro_rules! T {
            #([#punctuation_values] => { $crate::SyntaxKind::#punctuation };)*
            #([#keyword_idents] => { $crate::SyntaxKind::#keyword };)*
            [ident] => { $crate::SyntaxKind::IDENT };
            [EOF] => { $crate::SyntaxKind::EOF };
        }
    };

    crate::utils::reformat(token)
}
