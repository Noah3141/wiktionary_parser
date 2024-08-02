#[cfg(test)]
mod test;

use scraper::Selector;
use crate::models::wiktionary_macro::Expand;
use super::BeConj;

impl BeConj {

    /// `form` needs to provide a wiktionary_macro::russian::ru_conj::class_labels
    pub fn get_form(html: &scraper::Html, form_selector: &str) -> Option<String> {
        let selector = Selector::parse(form_selector).unwrap();
        let first_match = html.select(&selector).next()?;
        let inner_text = first_match.text().collect::<Vec<&str>>()[0];
        let text = inner_text.to_string();
        Some(text)
    }
    pub fn is_imperfective(&self, html: &scraper::Html,) -> bool {
        self.check_head(html, "imperfective").expect("check_head for imperfective")
    }
    pub fn is_perfective(&self, html: &scraper::Html,) -> bool {
        self.check_head(html, " perfective").expect("check_head for perfective")
    }
    pub fn is_transitive(&self, html: &scraper::Html) -> bool {
        self.check_head(html, " transitive").expect("check_head for transitive")
    }

}