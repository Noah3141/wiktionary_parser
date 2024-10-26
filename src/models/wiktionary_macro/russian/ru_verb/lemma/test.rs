use crate::models::{language::Language, section_header::SectionHeader, wiktionary_macro::RuNounPlus};


#[test]
fn bayat() {
    let verb = RuNounPlus {
        page_id: 295351,
        page_title: "баять".to_string(),
        language: Language::Russian,
        section: SectionHeader::Verb,
        macro_text: "{{ru-verb|ба́ять|impf}}".to_string()
    };

    assert_eq!(
        verb.lemma(),
        "ба́ять"
    )
}

#[test]
fn znat() {
    let verb = RuNounPlus {
        page_id: 295351,
        page_title: "баять".to_string(),
        language: Language::Russian,
        section: SectionHeader::Verb,
        macro_text: "{{ru-verb|знать|impf|pf=узна́ть|vn=зна́ние}}".to_string()
    };

    assert_eq!(
        verb.lemma(),
        "знать"
    )
}

#[test]
fn igrat() {
    let verb = RuNounPlus {
        page_id: 295351,
        page_title: "играть".to_string(),
        language: Language::Russian,
        section: SectionHeader::Verb,
        macro_text: "{{ru-verb|игра́ть|impf|pf=сыгра́ть|pf2=поигра́ть}}".to_string()
    };

    assert_eq!(
        verb.lemma(),
        "игра́ть"
    )
}