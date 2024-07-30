use super::InflectionOf;

#[cfg(test)]
mod test;


impl InflectionOf {
    pub fn lemma(&self) -> &str {
        let without_brackets = self.macro_text
            .strip_prefix("{{")
            .expect("Starting brackets")
            .strip_suffix("}}")
            .expect("ending brackets");

        let parts: Vec<&str> = without_brackets.split("|").collect();

        let tag_is_expected = parts[0] == "infl of" || parts[0] == "inflection of";
        if !tag_is_expected { println!("\n\nUnexpected tag! {parts:#?}\n\n"); }

        let language_matches = parts[1] == self.language.as_code();
        if !language_matches { println!("\n\nUnexpected language! {parts:#?}\n\n") }
        
        let expected_lemma_block = parts[2]; // This may or may not be a comma delimited concatenation of multiple lemmata

        let expected_lemmas = expected_lemma_block
            .split(",")
            .filter(|l| *l != "")
            .map(|lemma| { // inline <modifiers> are not important rn, remove them
                if lemma.contains("<") {
                    lemma.split_at(lemma.find("<").unwrap()).0
                } else {
                    lemma
                }
            })
            .collect::<Vec<&str>>();

        expected_lemmas[0]
    }

    pub fn form_and_lemma(&self) -> Vec<(&str, &str)> {
        let form = &self.page_title;
        let lemma = self.lemma();

        vec![(&form, lemma)]
    }
}