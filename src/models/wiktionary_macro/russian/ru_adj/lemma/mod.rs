use super::RuAdj;

#[cfg(test)]
mod test;


impl RuAdj {
    pub fn lemma(&self) -> String {
        let without_brackets = self.macro_text
            .strip_prefix("{{")
            .expect("Starting brackets")
            .strip_suffix("}}")
            .expect("ending brackets");

        let mut parts = without_brackets.split("|")
            .into_iter()
            .filter(|segment| -> bool {
                !segment.contains("=") // named parameter segment
            })
            .filter(|segment| -> bool { // sometimes provided sometimes assumed stress pattern param
                !["a", "b", "c", "d", "e", "f", "b'", "d'", "f'", "f''"]
                    .iter().any(|param| -> bool { segment == param }) // not any is equal to a param flag
            });

        let tag_is_expected = parts.next() == Some("ru-adj") ;
        if !tag_is_expected { println!("\n\nUnexpected tag! {parts:#?}\n\n"); }

        let maybe_lemma = parts.next();

        match maybe_lemma {
            None => return self.page_title.to_string(),   // Single syllable words
            Some(mut text) => {
                if !text.contains("[") {
                    if let Some(l) = text.find("//") {
                        text = &text[..l]
                    }
                    return text.to_string() // probably the lemma
                } else { // Complex multi word syntax need lemma extracted
                    let mut segments = vec![text];
                    segments.extend(parts);

                    let text = segments.into_iter()
                        .filter_map(|seg| {
                            match seg {
                                "_" => Some(" "),
                                "-" => Some("-"),
                                "$" => None,
                                string => {
                                    if !string.ends_with("]]") { None }
                                    else {
                                        let mut word = string.strip_suffix("]]").expect("suffix in this context");
                                        word = word.strip_prefix("[[").unwrap_or(word);
                                        Some(word)
                                    }
                                }
                            }
                        })
                        .collect::<Vec<&str>>()
                        .join("");
                    text
                }
            } 
        }
    }

    pub fn form_and_lemma(&self) -> Vec<(&str, String)> {
        vec![(&self.page_title, self.lemma())]
    }
}