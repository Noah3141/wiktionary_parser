use super::RuAdv;

#[cfg(test)]
mod test;

impl RuAdv {
    pub fn lemma(&self, ) -> String {
        let without_brackets = self.macro_text
        .strip_prefix("{{")
        .expect("Starting brackets")
        .strip_suffix("}}")
        .expect("ending brackets");

    let tag_is_expected = without_brackets.starts_with("ru-adv") ;
    if !tag_is_expected { println!("\n\nUnexpected tag! {{{without_brackets}}}\n\n"); }

    let mut parts = without_brackets.split("|")
        .into_iter()
        .map(|segment| -> &str {
            if let Some(slashes) = segment.find("//") {
                &segment[..slashes]
            } else {
                segment
            }
        })
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
    
    if without_brackets.contains(" ") && without_brackets.contains("[[") {
        return self.page_title.clone()
    }

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
}