#[cfg(test)]
mod test;

use scraper::{Html, Selector};
use crate::models::wiktionary_macro::Expand;
use super::BeConj;


impl BeConj {
    /// Parses the accented lemma from the macro text. Requires no expansion but has to navigate the template syntax.
    /// Docs for the template suggest that the accented lemma is a positional parameter reliably found at param 3 
    pub fn lemma(&self) -> &str {
        let without_brackets = self.macro_text
            .strip_prefix("{{")
            .expect("Starting brackets")
            .strip_suffix("}}")
            .expect("ending brackets");

        let parts: Vec<&str> = without_brackets.split("|").collect();

        let tag_is_expected = parts[0] == "be-conj" ;
        if !tag_is_expected { println!("\n\nUnexpected tag! {parts:#?}\n\n"); }

        let without_params = match parts[1].find("&lt") {
            Some(l) => &parts[1][..l],
            None => parts[1],
        };

        if without_params.starts_with("((") {
            &without_params[2..]
        } else {
            without_params
        }
    }
    pub async fn form_and_lemma(&self, client: &reqwest::Client) -> Vec<(String, &str)> {

        let mut form_lemma_tuples: Vec<(String, &str)> = Vec::with_capacity(25);
        let res = self.expand_with(&client).await;
        let html = Html::parse_fragment(&res);

        for form in super::class_selectors::ALL {
            let try_to_get = self.get_form(&html, form);
            if let Some(form) = try_to_get {
                form_lemma_tuples.push(
                    (form, self.lemma())
                )
            }
        }

        form_lemma_tuples
    }
}