#[cfg(test)]
mod test;

use scraper::Selector;
use crate::models::{gender::Gender, wiktionary_macro::Expand};

use super::BeNDecl;

impl BeNDecl {

    /// `form` needs to provide a wiktionary_macro::russian::ru_conj::class_labels
    pub fn get_form(&self, html: &scraper::Html, form_selector: &str) -> Option<String> {
        let selector = Selector::parse(form_selector).unwrap();
        let first_match = html.select(&selector).next()?;
        let inner_text = first_match.text().collect::<Vec<&str>>()[0];
        let text = inner_text.to_string();
        Some(text)
    }


    pub fn gender(&self, html: &scraper::Html) -> Result<Gender, ()> {
        if let true = self.check_head(&html, "masc-form").expect("check_head attempt") { return Ok(Gender::Masculine) };
        if let true = self.check_head(&html, "fem-form").expect("check_head attempt") { return Ok(Gender::Feminine) };
        if let true = self.check_head(&html, "neut-form").expect("check_head attempt") { return Ok(Gender::Neuter) };
        
        if let true = self.check_head(&html, "masc ").expect("check_head attempt") { return Ok(Gender::Masculine) };
        if let true = self.check_head(&html, "fem ").expect("check_head attempt") { return Ok(Gender::Feminine) };
        if let true = self.check_head(&html, "neut ").expect("check_head attempt") { return Ok(Gender::Neuter) };
        
        if self.lemma().ends_with("ая") { return Ok(Gender::Feminine)}

        Err(())
    }

}