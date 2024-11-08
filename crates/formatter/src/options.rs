#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Options {}

impl Options {
    pub fn merge(&mut self, _other: &Options) -> &mut Self {
        self
    }
}
