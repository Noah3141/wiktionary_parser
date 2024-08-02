use crate::models::wiktionary_macro::Expand;
use super::UkIpa;

impl Expand for UkIpa {
    fn page_title(&self) -> &str {
        &self.page_title
    }

    fn macro_text(&self) -> &str {
        &self.macro_text
    }
}