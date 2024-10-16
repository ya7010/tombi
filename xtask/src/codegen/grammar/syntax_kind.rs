use proc_macro2::{Punct, Spacing};
use quote::{format_ident, quote};

use super::syntax_kind_src::{LITERALS, NODES, PUNCTUATIONS, TOKENS};

pub fn generate_syntax_kind() -> Result<String, anyhow::Error> {
    let punctuation_values = PUNCTUATIONS.iter().map(|item| {
        let token = item.token;
        match token {
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
        }
    });
    let punctuations = PUNCTUATIONS
        .iter()
        .map(|item| format_ident!("{}", item.name))
        .collect::<Vec<_>>();
    let attr_punctuations = PUNCTUATIONS
        .iter()
        .map(|item| item.to_attr_token())
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
            #[doc(hidden)]
            TOMBSTONE,
            #[doc(hidden)]
            EOF,
            #(#attr_punctuations,)*
            #(#attr_literals,)*
            #(#attr_tokens,)*
            #(#nodes,)*
            #[doc(hidden)]
            INVALID_TOKENS,
            #[doc(hidden)]
            __LAST,
        }

        impl SyntaxKind {
            #[inline]
            pub fn is_trivia(self) -> bool {
                matches!(self, SyntaxKind::WHITESPACE)
            }
        }

        impl From<SyntaxKind> for rowan::SyntaxKind {
            #[inline]
            fn from(k: SyntaxKind) -> Self {
                Self(k as u16)
            }
        }

        impl From<u16> for SyntaxKind {
            #[inline]
            fn from(d: u16) -> SyntaxKind {
                assert!(d <= (SyntaxKind::__LAST as u16));
                unsafe { std::mem::transmute::<u16, SyntaxKind>(d) }
            }
        }

        impl From<SyntaxKind> for u16 {
            #[inline]
            fn from(k: SyntaxKind) -> u16 {
                k as u16
            }
        }

        fn lex_single_line_string(lex: &mut logos::Lexer<SyntaxKind>, quote: char) -> bool {
            let remainder: &str = lex.remainder();
            let mut total_len = 0;

            for c in remainder.chars() {
                total_len += c.len_utf8();

                if c == quote {
                    lex.bump(remainder[0..total_len].as_bytes().len());
                    return true;
                }
            }
            false
        }

        fn lex_multi_line_string(lex: &mut logos::Lexer<SyntaxKind>, quote: char) -> bool {
            let remainder: &str = lex.remainder();

            let mut total_len = 0;
            let mut quote_count = 0;
            let mut escaped = false;

            // As the string can contain ",
            // we can end up with more than 3 "-s at
            // the end, in that case we need to include all
            // in the string.
            let mut quotes_found = false;

            for c in remainder.chars() {
                if quotes_found {
                    if c != quote {
                        if quote_count >= 6 {
                            return false;
                        }

                        lex.bump(remainder[0..total_len].as_bytes().len());
                        return true;
                    } else {
                        quote_count += 1;
                        total_len += c.len_utf8();
                        continue;
                    }
                }
                total_len += c.len_utf8();

                if c == '\\' {
                    escaped = true;
                    continue;
                }

                if c == quote && !escaped {
                    quote_count += 1;
                } else {
                    quote_count = 0;
                }

                if quote_count == 3 {
                    quotes_found = true;
                }

                escaped = false;
            }

            // End of input
            if quotes_found {
                if quote_count >= 6 {
                    return false;
                }

                lex.bump(remainder[0..total_len].as_bytes().len());
                true
            } else {
                false
            }
        }

        /// Utility macro for creating a SyntaxKind through simple macro syntax
        #[macro_export]
        macro_rules! T {
            // Punctuation
            #([#punctuation_values] => { $crate::SyntaxKind::#punctuations };)*
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
