use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Russian,
    Ukrainian,
    Belarusian,
}

impl Language {
    /// Converts Language to the double equal sign delimited header of Wikitext, e.g. `==Gothic==`
    pub fn as_header(self) -> &'static str {
        match self {
            Language::Russian => "==Russian==",
            Language::Ukrainian => "==Ukrainian==",
            Language::Belarusian => "==Belarusian==",
        }
    }

    /// Converts Language to the template-macro language code, e.g. `Russian` -> `ru`
    pub fn as_code(self) -> &'static str {
        match self {
            Language::Russian => "ru",
            Language::Ukrainian => "uk",
            Language::Belarusian => "be",
        }
    }
}