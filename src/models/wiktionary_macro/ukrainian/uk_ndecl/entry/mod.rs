use scraper::Selector;
use super::UkNDecl;
use crate::models::{gender::Gender, wiktionary_macro::Expand};

#[cfg(test)]
mod test;

impl UkNDecl {
    pub fn get_form(&self, html: &scraper::Html, mut form_selector: &str) -> Option<String> {
        let singulare_tantum = self.check_head(html, "sg-only").expect("check_head") | self.macro_text.contains("sg");
        let plurale_tantum = self.check_head(html, "pl-only").expect("check_head") | self.macro_text.contains("pl");

        if super::class_selectors::PLURAL.contains(&form_selector) && singulare_tantum {
            return None
        }
        if super::class_selectors::SINGULAR.contains(&form_selector) && plurale_tantum {
            return None
        }

        // if plurale_tantum { 
        //     form_selector = match form_selector {
        //         super::class_selectors::NOM_P => ".nom-form-of",
        //         super::class_selectors::GEN_P => ".gen-form-of",
        //         super::class_selectors::ACC_P => ".acc-form-of",
        //         super::class_selectors::DAT_P => ".dat-form-of",
        //         super::class_selectors::INSTR_P => ".ins-form-of",
        //         super::class_selectors::LOC_P => ".loc-form-of",
        //         super::class_selectors::VOC_P => ".voc-form-of",
        //         _ => unreachable!()
        //     }
        // }

        match form_selector {
            // This may fail because there are two different arrangements (or so) for noun tables, due to animacy. If it fails, assume animacy variant and grab that instead
            super::class_selectors::ACC_S |
            super::class_selectors::ACC_P |
            ".acc-form-of"  => {
                let selector = Selector::parse(form_selector).unwrap();
                if let Some(first_match) = html.select(&selector).next() {    
                    let inner_text = first_match.text().collect::<Vec<&str>>()[0];
                    let text = inner_text.to_string();
                    return Some(text)
                };

                // assert!(self.macro_text.contains(&"a=ai" || &"a=ia"))
                let animate_selector = match form_selector {
                    super::class_selectors::ACC_S => ".an\\|acc\\|s-form-of",
                    super::class_selectors::ACC_P => ".an\\|acc\\|p-form-of",
                    ".acc-form-of" => ".an\\|acc-form-of",
                    _ => unreachable!()
                };
                let selector = Selector::parse(animate_selector).unwrap();
                if let Some(first_match) = html.select(&selector).next() {
                    let inner_text = first_match.text().collect::<Vec<&str>>()[0];
                    let text = inner_text.to_string();
                    return Some(text)
                };  

                if html.select(&Selector::parse(".hypothetical").unwrap()).next().is_some() { // If the page balks and says the plural forms are *hypothetical, thus not having the expected classes, screw it, the form isn't recognized enough to keep
                    return None
                }

                panic!("acc: Failed to find a way to handle selection Some|None for: {form_selector} , {}", &self.macro_text)

            },
            _ => {
                let selector = Selector::parse(form_selector).unwrap();
                if let Some(first_match) = html.select(&selector).next() {
                    let inner_text = first_match.text().collect::<Vec<&str>>()[0];
                    let text = inner_text.to_string();
                    return Some(text)
                };

                if html.select(&Selector::parse(".hypothetical").unwrap()).next().is_some() { // If the page balks and says the plural forms are *hypothetical, thus not having the expected classes, screw it, the form isn't recognized enough to keep
                    return None
                }

                panic!("Failed to find a way to handle selection Some|None for: {form_selector} , {}", &self.macro_text)
            }
        }
    }

    pub fn gender(&self, html: &scraper::Html) -> Gender {
        if let true = self.check_head(&html, "masc-form").expect("check_head attempt") { return Gender::Masculine };
        if let true = self.check_head(&html, "fem-form").expect("check_head attempt") { return Gender::Feminine };
        if let true = self.check_head(&html, "neut-form").expect("check_head attempt") { return Gender::Neuter };
        panic!("Should not occur! Couldn't determine gender from NavHead!")
    }
}