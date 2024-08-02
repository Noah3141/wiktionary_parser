use crate::models::wiktionary_macro::Expand;
use super::Ipa;


impl Expand for Ipa {
    fn page_title(&self) -> &str {
        &self.page_title
    }

    fn macro_text(&self) -> &str {
        &self.macro_text
    }
}