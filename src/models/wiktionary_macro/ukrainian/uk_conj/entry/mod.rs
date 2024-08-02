use scraper::Selector;
use crate::models::wiktionary_macro::Expand;
use super::UkConj;

#[cfg(test)]
mod test;

impl UkConj {
    /// `form` needs to provide a wiktionary_macro::russian::uk_conj::class_labels
    pub fn get_form(&self, html: &scraper::Html, form_selector: &str) -> Option<String> {
        let selector = Selector::parse(form_selector).unwrap();
        let first_match: scraper::ElementRef = html.select(&selector).next()?;
        let inner_text = first_match.text().collect::<Vec<&str>>()[0];
        let text = inner_text.to_string();
        Some(text)
    }
    
    pub fn is_perfective(&self, html: &scraper::Html,) -> bool {
        self.check_head(html, " perfective").expect("is_perfective")
    }
    pub fn is_imperfective(&self, html: &scraper::Html,) -> bool {
        self.check_head(html, "imperfective").expect("is_imperfective")
    }
    pub fn is_transitive(&self, html: &scraper::Html) -> bool {
        self.check_head(html, " transitive").expect("is_transitive")
    }
}