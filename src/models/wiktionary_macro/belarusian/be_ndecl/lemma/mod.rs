#[cfg(test)]
mod test;

use scraper::{Html, Selector};
use super::BeNDecl;
use crate::models::wiktionary_macro::Expand;

impl BeNDecl {
    pub fn lemma(&self) -> &str {
        let without_brackets = self.macro_text
        .strip_prefix("{{")
        .expect("Starting brackets")
        .strip_suffix("}}")
        .expect("ending brackets");

        let parts: Vec<&str> = without_brackets.split("|").collect();

        let tag_is_expected = parts[0] == "be-ndecl" ;
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

        let mut form_lemma_tuples: Vec<(String, &str)> = Vec::with_capacity(6);
        let res = self.expand_with(&client).await;
        let fragment = Html::parse_fragment(&res);

        for classname in super::class_selectors::ALL {
            let selector = Selector::parse(classname).unwrap();
            
            for element in fragment.select(&selector).into_iter() {
                let inner_text = element.text().collect::<Vec<&str>>()[0];
                let text = inner_text.to_string();
                form_lemma_tuples.push( (text, self.lemma()) )
            }
        }

        form_lemma_tuples
    }
}