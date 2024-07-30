#[cfg(test)]
mod test;

use scraper::Selector;
use crate::models::wiktionary_macro::Expand;
use super::RuNounTable;

impl RuNounTable {


    /// `form` needs to provide a wiktionary_macro::russian::ru_noun_table::class_labels
    pub fn get_form(html: &scraper::Html, form_selector: &str) -> String {
        let selector = Selector::parse(form_selector).unwrap();
        let first_match = html.select(&selector).next().expect("a first element selected by classname");
        let inner_text = first_match.text().collect::<Vec<&str>>()[0];
        let text = inner_text.to_string();
        text
    }

}