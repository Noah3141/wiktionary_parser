#[cfg(test)]
mod test;

use super::class_selectors;
use scraper::Selector;
use super::BeADecl;

impl BeADecl {
    /// `form` needs to provide a wiktionary_macro::russian::ru_conj::class_labels
    pub fn get_form(&self, html: &scraper::Html, form_selector: &str) -> Option<String> {
        let selector = Selector::parse(form_selector).unwrap();
        if let Some(first_match) = html.select(&selector).next() {
            let inner_text = first_match.text().collect::<Vec<&str>>()[0];
            let text = inner_text.to_string();
            return Some(text)
        };

        let retry = match form_selector {
            class_selectors::AN_ACC_M => ".acc\\|m\\|s-form-of",
            class_selectors::AN_ACC_P => ".acc\\|p-form-of",
            _ => return None
        };

        let selector = Selector::parse(retry).unwrap();
        if let Some(first_match) = html.select(&selector).next() {
            let inner_text = first_match.text().collect::<Vec<&str>>()[0];
            let text = inner_text.to_string();
            return Some(text)
        };

        None
    }
}