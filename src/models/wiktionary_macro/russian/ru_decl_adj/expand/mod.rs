use crate::models::wiktionary_macro::Expand;
use super::RuDeclAdj;



impl Expand for RuDeclAdj {
    fn macro_text(&self) -> &str {
        &self.macro_text
    }
    fn page_title(&self) -> &str {
        &self.page_title
    }
}