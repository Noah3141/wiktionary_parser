use scraper::{Html, Selector};

use crate::models::wiktionary_macro::Expand;

use super::UkNDecl;

#[cfg(test)]
mod test;

impl UkNDecl {
    // todo Numerals like сто
    pub fn lemma(&self) -> &str {
        let without_brackets = self
            .macro_text
            .strip_prefix("{{")
            .expect("Starting brackets")
            .strip_suffix("}}")
            .expect("ending brackets");
        let tag_is_expected = without_brackets.starts_with("uk-ndecl");
        if !tag_is_expected {
            println!("\n\nUnexpected tag! {{{without_brackets}}}\n\n");
        }
        let after_name = without_brackets
            .strip_prefix("uk-ndecl|")
            .expect("prefix presence");
        
        if after_name.contains(" ") { // multi word stuff just give up these are peripheral concerns
            return &self.page_title
        }
        
        let mut lemma = match after_name.find("&lt;") {
            Some(lt) => &after_name[..lt],
            None => after_name,
        };
        if lemma.starts_with("((") {
            lemma = &lemma[2..]
        } 

        lemma

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
                form_lemma_tuples.push((text, self.lemma()))
            }
        }
        form_lemma_tuples
    }
}
