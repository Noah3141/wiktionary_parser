#[cfg(test)]
mod test;

use crate::models::wiktionary_macro::Expand;
use super::UkADecl;

impl Expand for UkADecl {
    fn macro_text(&self) -> &str {
        &self.macro_text
    }
    fn page_title(&self) -> &str {
        &self.page_title
    }
}