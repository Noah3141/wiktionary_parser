#[cfg(test)]
mod test;

use scraper::Html;
use crate::{models::wiktionary_macro::Expand, utils::select_from};
use super::BeADecl;

impl BeADecl {
    /// Parses the accented lemma from the macro text. Requires no expansion but has to navigate the template syntax.
    pub fn lemma(&self) -> &str {
        let without_brackets = self.macro_text
            .strip_prefix("{{")
            .expect("Starting brackets")
            .strip_suffix("}}")
            .expect("ending brackets");

        let parts: Vec<&str> = without_brackets.split("|").collect();

        let tag_is_expected = parts[0] == "be-adecl" ;
        if !tag_is_expected { println!("\n\nUnexpected tag! {parts:#?}\n\n"); }

        match parts.get(1) {
            Some(segment) => {
                if *segment == "" { return &self.page_title }

                if *segment == "-" && without_brackets.contains("manual") {
                    if let Some(lemma) = select_from(without_brackets, "nom_mp=", "|") {
                        return lemma.trim()
                    };
                    if let Some(lemma) = select_from(without_brackets, "nom_m=", "|") {
                        return lemma.trim()
                    };
                    if let Some(lemma) = select_from(without_brackets, "nom_sg=", "|") {
                        return lemma.trim()
                    };
                    panic!("Couldn't figure out this manual template: {{{without_brackets}}}")
                }

                if let Some(lt) = segment.find("&lt;") {
                    return &segment[..lt]
                } else {
                    return segment
                }
            },
            None => &self.page_title,
        }
    }

    pub async fn form_and_lemma(&self, client: &reqwest::Client) -> Vec<(String, &str)> {

        let mut form_lemma_tuples: Vec<(String, &str)> = Vec::with_capacity(6);
        let res = self.expand_with(&client).await;
        let fragment = Html::parse_fragment(&res);

        for classname in super::class_selectors::ALL {
            if let Some(form) = self.get_form(&fragment, classname) {
                form_lemma_tuples.push( (form, self.lemma()) )
            }
        }

        form_lemma_tuples
    }
}