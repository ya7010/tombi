use convert_case::{Case, Casing};
use proc_macro2::{Punct, Spacing};
use quote::{format_ident, quote};

use super::syntax_kind_src::{KEYWORDS, LITERALS, NODES, PUNCTUATIONS, TOKENS};

pub fn generate_syntax_kind() -> Result<String, anyhow::Error> {
    let punctuation_values = PUNCTUATIONS.iter().map(|(token, _)| match *token {
        "{" | "}" | "[" | "]" => {
            let c = token.chars().next().unwrap();
            quote! { #c }
        }
        "[[" | "]]" => {
            quote!(#token)
        }
        _ => {
            let cs = token.chars().map(|c| Punct::new(c, Spacing::Alone));
            quote! { #(#cs)* }
        }
    });
    let punctuations = PUNCTUATIONS
        .iter()
        .map(|(_, name)| format_ident!("{}", name))
        .collect::<Vec<_>>();
    let attr_punctuations = PUNCTUATIONS
        .iter()
        .map(|(token, name)| {
            let ident = format_ident!("{}", name);
            quote! { #[token(#token)] #ident }
        })
        .collect::<Vec<_>>();

    let keyword_idents = KEYWORDS
        .iter()
        .map(|kw| format_ident!("{}", kw))
        .collect::<Vec<_>>();
    let keywords = KEYWORDS
        .iter()
        .map(|kw| format_ident!("{}_KW", kw.to_case(Case::Upper)))
        .collect::<Vec<_>>();

    let attr_literals = LITERALS
        .iter()
        .map(|item| item.to_attr_token())
        .collect::<Vec<_>>();

    let attr_tokens = TOKENS
        .iter()
        .map(|item| item.to_attr_token())
        .collect::<Vec<_>>();

    let nodes = NODES
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let token = quote! {
        #[doc = r" The kind of syntax node, e.g. `WHITESPACE`, `COMMENT`, or `TABLE`."]
        #[derive(logos::Logos, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        #[repr(u16)]
        #[logos(error = crate::Error)]
        #[allow(non_camel_case_types)]
        pub enum SyntaxKind {
            #(#attr_punctuations,)*
            #(#keywords,)*
            #(#attr_literals,)*
            #(#attr_tokens,)*
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
            // Punctuation
            #([#punctuation_values] => { $crate::SyntaxKind::#punctuations };)*
            // Keywords
            #([#keyword_idents] => { $crate::SyntaxKind::#keywords };)*
            // Bare key
            [bare_key] => { $crate::SyntaxKind::BARE_KEY };
            // String
            [basic_string] => { $crate::SyntaxKind::BASIC_STRING };
            [multi_line_basic_string] => { $crate::SyntaxKind::MULTI_LINE_BASIC_STRING };
            [literal_string] => { $crate::SyntaxKind::LITERAL_STRING };
            [multi_line_literal_string] => { $crate::SyntaxKind::MULTI_LINE_LITERAL_STRING };
            // Integer
            [integer_dec] => { $crate::SyntaxKind::INTEGER_DEC };
            [integer_hex] => { $crate::SyntaxKind::INTEGER_HEX };
            [integer_oct] => { $crate::SyntaxKind::INTEGER_OCT };
            [integer_bin] => { $crate::SyntaxKind::INTEGER_BIN };
            // Float
            [float] => { $crate::SyntaxKind::FLOAT };
            // Date and time
            [offset_date_time] => { $crate::SyntaxKind::OFFSET_DATE_TIME };
            [local_date_time] => { $crate::SyntaxKind::LOCAL_DATE_TIME };
            [local_date] => { $crate::SyntaxKind::LOCAL_DATE };
            [local_time] => { $crate::SyntaxKind::LOCAL_TIME };
        }
    };

    crate::utils::reformat(token)
}
