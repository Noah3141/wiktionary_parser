#[cfg(test)]
mod test;

use scraper::Selector;
use crate::models::wiktionary_macro::Expand;
use super::RuConj;

impl RuConj {

    /// `form` needs to provide a wiktionary_macro::russian::ru_conj::class_labels
    pub fn get_form(html: &scraper::Html, form_selector: &str) -> Option<String> {
        let selector = Selector::parse(form_selector).unwrap();
        let first_match: scraper::ElementRef = html.select(&selector).next()?;
        let inner_text = first_match.text().collect::<Vec<&str>>()[0];
        let text = inner_text.to_string();
        Some(text)
    }

    pub fn is_perfective(html: &scraper::Html,) -> bool {
        let selector = Selector::parse(".NavHead").unwrap();
        let first_match = html.select(&selector).next().expect("a first element selected by classname");
        let inner_text = first_match.text().collect::<Vec<&str>>().join(" ");
        let text = inner_text.to_string();
        text.contains("perfective")
    }
    pub fn is_imperfective(html: &scraper::Html,) -> bool {
        let selector = Selector::parse(".NavHead").unwrap();
        let first_match = html.select(&selector).next().expect("a first element selected by classname");
        let inner_text = first_match.text().collect::<Vec<&str>>().join(" ");
        let text = inner_text.to_string();
        text.contains("imperfective")
    }

}