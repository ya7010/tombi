use quote::{format_ident, quote};

#[derive(Default, Debug)]
pub(crate) struct AstSrc {
    pub(crate) tokens: Vec<String>,
    pub(crate) nodes: Vec<AstNodeSrc>,
    pub(crate) enums: Vec<AstEnumSrc>,
}

#[derive(Debug)]
pub(crate) struct AstNodeSrc {
    #[allow(dead_code)]
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) fields: Vec<Field>,
}

#[derive(Debug)]
pub(crate) struct AstEnumSrc {
    #[allow(dead_code)]
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) variants: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Field {
    Token(String),
    Node {
        name: String,
        ty: String,
        cardinality: Cardinality,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Cardinality {
    Optional,
    Many,
}

impl Field {
    pub fn is_many(&self) -> bool {
        matches!(
            self,
            Field::Node {
                cardinality: Cardinality::Many,
                ..
            }
        )
    }
    pub fn token_kind(&self) -> Option<proc_macro2::TokenStream> {
        match self {
            Field::Token(token) => {
                let token: proc_macro2::TokenStream = match token.as_str() {
                    "[[" => quote! { "[[" },
                    "]]" => quote! { "]]" },
                    token => token.parse().unwrap(),
                };
                Some(quote! { T![#token] })
            }
            _ => None,
        }
    }

    pub fn method_name(&self) -> String {
        match self {
            Field::Token(name) => {
                let name = match name.as_str() {
                    "'{'" => "inline_table_open",
                    "'}'" => "inline_table_close",
                    "'['" => "array_open",
                    "']'" => "array_close",
                    "[[" => "array_table_open",
                    "]]" => "array_table_close",
                    "=" => "eq",
                    "." => "dot",
                    "," => "comma",
                    _ => name,
                };
                format!("{name}_token",)
            }
            Field::Node { name, .. } => {
                if name == "type" {
                    String::from("ty")
                } else {
                    name.to_owned()
                }
            }
        }
    }

    pub fn ty(&self) -> proc_macro2::Ident {
        match self {
            Field::Token(_) => format_ident!("SyntaxToken"),
            Field::Node { ty, .. } => format_ident!("{}", ty),
        }
    }
}
