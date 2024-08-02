#[cfg(test)]
mod test;

use crate::models::wiktionary_macro::Expand;
use super::UkConj;

impl Expand for UkConj {
    fn macro_text(&self) -> &str {
        &self.macro_text
    }
    fn page_title(&self) -> &str {
        &self.page_title
    }
}