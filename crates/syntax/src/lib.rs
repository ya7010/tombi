mod builder;
mod error;
mod generated;

pub use builder::SyntaxTreeBuilder;
pub use error::{Error, SyntaxError};
pub use generated::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TomlLanguage {}

impl rg_tree::Language for TomlLanguage {
    type Kind = crate::SyntaxKind;

    fn kind_from_raw(raw: rg_tree::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= crate::SyntaxKind::__LAST as u16);
        unsafe { std::mem::transmute::<u16, crate::SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rg_tree::SyntaxKind {
        kind.into()
    }
}

/// TOML version.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub enum TomlVersion {
    #[default]
    /// TOML 1.0.0
    V1_0_0,

    /// TOML 1.1.0-preview
    V1_1_0_Preview,
}

#[cfg(feature = "clap")]
impl clap::ValueEnum for TomlVersion {
    fn value_variants<'a>() -> &'a [Self] {
        use TomlVersion::*;

        &[V1_0_0, V1_1_0_Preview]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        use TomlVersion::*;

        Some(match self {
            V1_0_0 => clap::builder::PossibleValue::new("1.0.0"),
            V1_1_0_Preview => clap::builder::PossibleValue::new("1.1.0-preview"),
        })
    }
}

pub type SyntaxNode = rg_tree::RedNode<TomlLanguage>;
pub type SyntaxToken = rg_tree::RedToken<TomlLanguage>;
pub type SyntaxElement = rg_tree::RedElement<TomlLanguage>;
pub type SyntaxNodeChildren = rg_tree::RedNodeChildren<TomlLanguage>;
pub type SyntaxElementChildren = rg_tree::RedElementChildren<TomlLanguage>;
pub type PreorderWithTokens = rg_tree::PreorderWithTokens<TomlLanguage>;

#[cfg(test)]
mod test {
    #[test]
    fn toml_version_comp() {
        assert!(crate::TomlVersion::V1_0_0 < crate::TomlVersion::V1_1_0_Preview);
    }
}
