/// TOML version.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub enum TomlVersion {
    /// TOML 1.0.0
    V1_0_0,

    #[default]
    /// TOML 1.1.0-preview
    V1_1_0_Preview,
}

impl TomlVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1_0_0 => "v1.0.0",
            Self::V1_1_0_Preview => "v1.1.0-preview",
        }
    }

    pub const fn help(self) -> &'static str {
        match self {
            Self::V1_0_0 => "TOML v1.0.0",
            Self::V1_1_0_Preview => "TOML v1.1.0-preview",
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for TomlVersion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TomlVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "v1.0.0" => Ok(Self::V1_0_0),
            "v1.1.0-preview" => Ok(Self::V1_1_0_Preview),
            _ => Err(serde::de::Error::custom("invalid TOML version")),
        }
    }
}

#[cfg(feature = "jsonschema")]
impl schemars::JsonSchema for TomlVersion {
    fn schema_name() -> String {
        "TOML version".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::{InstanceType, ObjectValidation, Schema, SchemaObject};

        let mut schema = SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            enum_values: Some(vec![
                serde_json::Value::String(Self::V1_0_0.as_str().to_string()),
                serde_json::Value::String(Self::V1_1_0_Preview.as_str().to_string()),
            ]),
            ..Default::default()
        };

        schema.metadata().description = Some("TOML version".to_string());

        Schema::Object(schema)
    }
}

#[cfg(feature = "clap")]
impl clap::ValueEnum for TomlVersion {
    fn value_variants<'a>() -> &'a [Self] {
        use TomlVersion::*;

        &[V1_0_0, V1_1_0_Preview]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.as_str()).help(self.help()))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn toml_version_comp() {
        assert!(crate::TomlVersion::V1_0_0 < crate::TomlVersion::V1_1_0_Preview);
    }
}
