#[cfg(test)]
mod test;

use scraper::Selector;
use crate::models::{section_header::SectionHeader, wiktionary_macro::Expand};
use super::RuNounTable;

impl RuNounTable {


    /// `form` needs to provide a wiktionary_macro::russian::ru_noun_table::class_labels
    pub fn get_form(&self, html: &scraper::Html, mut form_selector: &str) -> Option<String> {
        let singulare_tantum = self.check_head(html, "sg-only") | self.macro_text.contains("|n=s");
        let plurale_tantum = self.check_head(html, "pl-only") | self.macro_text.contains("|n=p");

        if super::class_selectors::PLURAL.contains(&form_selector) && singulare_tantum {
            return None
        }
        if super::class_selectors::SINGULAR.contains(&form_selector) && plurale_tantum {
            return None
        }

        if plurale_tantum { //todo Make a test that notes this was needed due to 'деньги'
            form_selector = match form_selector {
                super::class_selectors::NOM_P => ".nom-form-of",
                super::class_selectors::GEN_P => ".gen-form-of",
                super::class_selectors::ACC_P => ".acc-form-of",
                super::class_selectors::DAT_P => ".dat-form-of",
                super::class_selectors::INSTR_P => ".ins-form-of",
                super::class_selectors::PREP_P => ".pre-form-of",
                _ => unreachable!()
            }
        }

        match form_selector {
            // This may fail because there are two different arrangements (or so) for noun tables, due to animacy. If it fails, assume animacy variant and grab that instead
            super::class_selectors::ACC_S |
            super::class_selectors::ACC_P => {
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

}