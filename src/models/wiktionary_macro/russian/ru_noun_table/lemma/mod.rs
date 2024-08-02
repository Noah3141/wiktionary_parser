use scraper::{Html, Selector};
use super::RuNounTable;
use crate::models::wiktionary_macro::Expand;

#[cfg(test)]
mod test;


impl RuNounTable {
    // todo Numerals like сто
    pub fn lemma(&self) -> String { 
        let without_brackets = self.macro_text
            .strip_prefix("{{")
            .expect("Starting brackets")
            .strip_suffix("}}")
            .expect("ending brackets");

        let tag_is_expected = without_brackets.starts_with("ru-noun-table") ;
        if !tag_is_expected { println!("\n\nUnexpected tag! {{{without_brackets}}}\n\n"); }

        let mut parts = without_brackets.split("|")
            .into_iter()
            .filter(|segment| -> bool {
                !segment.contains("=") // named parameter segment
            })
            .filter(|segment| -> bool { // sometimes provided sometimes assumed stress pattern param
                ![
                    "a", "b", "c", "d", "e", "f", "b'", "d'", "f'", "f''", "d'",
                    "m"
                ]
                    .iter().any(|param| -> bool { segment.contains(param)}) // not any is equal to a param flag
            });



        let maybe_lemma = parts.next();

        match maybe_lemma {
            None => return self.page_title.to_string(),   // Single syllable words
            Some(mut text) => {
                if text == "" { return self.page_title.to_string() }

                if !text.contains("[") {
                    if let Some(l) = text.find("//") {
                        text = &text[..l]
                    }
                    return text.to_string() // probably the lemma
                } else { // Complex multi word syntax need lemma extracted
                    let mut segments = vec![text];
                    segments.extend(parts);

                    let text: String = segments.into_iter()
                        .filter_map(|seg| {
                            match seg {
                                "_" => Some(" ".to_string()),
                                "-" => Some("-".to_string()),
                                "$" => None,
                                string => {
                                    if !string.ends_with("]]") { None }
                                    else {
                                        let word = string.chars().filter(|c| *c != ']' && *c != '[').collect::<String>();
                                        Some(word)
                                    }
                                }
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("");
                    text
                }
            } 
        }
    }
    
    pub async fn form_and_lemma(&self, client: &reqwest::Client) -> Vec<(String, String)> {

        let mut form_lemma_tuples: Vec<(String, String)> = Vec::with_capacity(6);
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