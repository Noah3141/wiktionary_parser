use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Language {
    Russian,
    Ukrainian,
    Belarusian,
}

impl Language {
    /// Converts
    pub fn as_header(self) -> &'static str {
        match self {
            Language::Russian => "==Russian==",
            Language::Ukrainian => "==Ukrainian==",
            Language::Belarusian => "==Belarusian==",
        }
    }
}