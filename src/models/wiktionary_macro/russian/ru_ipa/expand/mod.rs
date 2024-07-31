use crate::models::wiktionary_macro::Expand;
use super::RuIpa;



impl Expand for RuIpa {
    fn page_title(&self) -> &str {
        &self.page_title
    }

    fn macro_text(&self) -> &str {
        &self.macro_text
    }
}