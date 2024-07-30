use scraper::Selector;

use crate::models::wiktionary_macro::Expand;

use super::RuDeclAdj;

#[cfg(test)]
mod test;



impl RuDeclAdj {


    /// `form` needs to provide a wiktionary_macro::russian::ru_conj::class_labels
    pub fn get_form(html: &scraper::Html, form_selector: &str) -> Option<String> {
        let selector = Selector::parse(form_selector).unwrap();
        let first_match = html.select(&selector).next()?;
        let inner_text = first_match.text().collect::<Vec<&str>>()[0];
        let text = inner_text.to_string();
        Some(text)
    }
}