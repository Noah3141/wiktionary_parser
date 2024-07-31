use scraper::{Html, Selector};
use super::RuDeclAdj;
use crate::models::wiktionary_macro::Expand;

#[cfg(test)]
mod test;


impl RuDeclAdj {
    /// Parses the accented lemma from the macro text. Requires no expansion but has to navigate the template syntax.
    pub fn lemma(&self) -> &str {
        let without_brackets = self.macro_text
            .strip_prefix("{{")
            .expect("Starting brackets")
            .strip_suffix("}}")
            .expect("ending brackets");

        let parts: Vec<&str> = without_brackets.split("|").collect();

        let tag_is_expected = parts[0] == "ru-decl-adj" ;
        if !tag_is_expected { println!("\n\nUnexpected tag! {parts:#?}\n\n"); }

        parts[1]
    }
    pub async fn form_and_lemma(&self, client: &reqwest::Client) -> Vec<(String, &str)> {

        let mut form_lemma_tuples: Vec<(String, &str)> = Vec::with_capacity(6);
        let res = self.expand_with(&client).await;
        let fragment = Html::parse_fragment(&res);

        for classname in super::class_selectors::ALL {
            if let Some(form) = RuDeclAdj::get_form(&fragment, classname) {
                form_lemma_tuples.push( (form, self.lemma()) )
            }
        }

        form_lemma_tuples
    }
}