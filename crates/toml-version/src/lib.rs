/// # TOML version.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub enum TomlVersion {
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = "v1.0.0"))]
    #[cfg_attr(feature = "clap", value(name = "v1.0.0"))]
    V1_0_0,

    #[cfg_attr(feature = "serde", serde(rename = "v1.1.0-preview"))]
    #[cfg_attr(feature = "clap", value(name = "v1.1.0-preview"))]
    V1_1_0_Preview,
}

impl std::fmt::Display for TomlVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V1_0_0 => write!(f, "v1.0.0"),
            Self::V1_1_0_Preview => write!(f, "v1.1.0-preview"),
        }
    }
}

impl TomlVersion {
    pub const fn latest() -> Self {
        Self::V1_1_0_Preview
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn toml_version_comp() {
        assert!(crate::TomlVersion::V1_0_0 < crate::TomlVersion::V1_1_0_Preview);
    }
}
