use crate::models::{language::Language, section_header::SectionHeader};

use super::RuAdv;


#[test]
fn not_bad() {
    let table = RuAdv {
        page_id: 295351,
        page_title: "неплохо".to_string(),
        language: Language::Russian,
        section: SectionHeader::Adverb,
        macro_text: "{{ru-adv|непло́хо}}".to_string()
    };

    assert_eq!(table.lemma(), "непло́хо")
}

#[test]
fn somewhere() {
    let table = RuAdv {
        page_id: 295351,
        page_title: "где-то".to_string(),
        language: Language::Russian,
        section: SectionHeader::Adverb,
        macro_text: "{{ru-adv|[[где́]][[-то]]}}".to_string()
    };

    assert_eq!(table.lemma(), "где́-то")
}

#[test]
fn in_other_words() {
    let table = RuAdv {
        page_id: 295351,
        page_title: "иными словами".to_string(),
        language: Language::Russian,
        section: SectionHeader::Adverb,
        macro_text: "{{ru-adv|[[иной|ины́ми]] [[слово|слова́ми]]}}".to_string()
    };

    assert_eq!(table.lemma(), "иными словами")
}


#[test]
fn necessarily() {
    let table = RuAdv {
        page_id: 295351,
        page_title: "обязательно".to_string(),
        language: Language::Russian,
        section: SectionHeader::Adverb,
        macro_text: "{{ru-adv|обяза́тельно}}".to_string()
    };

    assert_eq!(table.lemma(), "обяза́тельно")
}

