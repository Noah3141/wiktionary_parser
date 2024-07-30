use crate::{constants, models::wiktionary_macro::Expand};
use super::BeConj;

impl Expand for BeConj {
    fn macro_text(&self) -> &str {
        &self.macro_text
    }


    fn page_title(&self) -> &str {
        &self.page_title
    }
}