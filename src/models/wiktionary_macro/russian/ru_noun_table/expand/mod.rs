use crate::{ models::wiktionary_macro::Expand};
use super::RuNounTable;

impl Expand for RuNounTable {
    fn macro_text(&self) -> &str {
        &self.macro_text
    }
    fn page_title(&self) -> &str {
        &self.page_title
    }
}