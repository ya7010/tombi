use nu_ansi_term::{Color, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Level {
    Error,
    Warning,
}

impl Level {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Level::Error => "Error",
            Level::Warning => "Warning",
        }
    }

    pub fn as_padded_str(&self) -> &'static str {
        match self {
            Level::Error => "  Error",
            Level::Warning => "Warning",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Level::Error => Color::Red,
            Level::Warning => Color::Yellow,
        }
    }
}

impl From<Level> for Style {
    fn from(val: Level) -> Self {
        match val {
            Level::Error => Style::new().bold().fg(Color::Red),
            Level::Warning => Style::new().bold().fg(Color::Yellow),
        }
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color().bold().paint(self.as_str()))
    }
}
