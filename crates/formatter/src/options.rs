#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Options {
    /// Maximum length of a line.
    pub max_line_length: Option<u8>,

    /// Size of a tab in spaces.
    pub tab_size: Option<u8>,

    /// Prefer spaces over tabs.
    pub insert_space: Option<bool>,
}

impl Options {
    #[inline]
    pub fn tab(&self) -> String {
        if self.insert_space == Some(false) {
            "\t".to_string()
        } else {
            " ".repeat(self.tab_size.unwrap_or(2) as usize)
        }
    }

    pub fn merge(&mut self, other: &Options) -> &mut Self {
        if other.max_line_length.is_some() {
            self.max_line_length = other.max_line_length;
        }
        if other.tab_size.is_some() {
            self.tab_size = other.tab_size;
        }
        if other.insert_space.is_some() {
            self.insert_space = other.insert_space;
        }

        self
    }
}
