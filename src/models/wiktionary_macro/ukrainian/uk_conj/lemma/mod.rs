use scraper::Html;
use crate::models::wiktionary_macro::Expand;
use super::UkConj;

#[cfg(test)]
mod test;


impl UkConj {
    pub fn lemma(&self) -> &str {
        let without_brackets = self.macro_text
            .strip_prefix("{{")
            .expect("Starting brackets")
            .strip_suffix("}}")
            .expect("ending brackets");
        let tag_is_expected = without_brackets.starts_with("uk-conj") ;
        if !tag_is_expected { println!("\n\nUnexpected tag! {{{without_brackets}}}\n\n"); }
        let after_name = without_brackets.strip_prefix("uk-conj|").expect("prefix presence");
        let lemma = match after_name.find("&lt;") {
            Some(lt) => &after_name[..lt],
            None => after_name,
        };
        lemma
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