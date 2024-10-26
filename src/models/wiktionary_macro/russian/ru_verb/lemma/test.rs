use crate::models::{language::Language, section_header::SectionHeader, wiktionary_macro::{RuNounPlus, RuVerb}};


#[test]
fn bayat() {
    let verb = RuVerb {
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
    let verb = RuVerb {
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
    let verb = RuVerb {
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

#[test]
fn уступить() {
    let verb = RuVerb {
        page_id: 295351,
        page_title: "уступить".to_string(),
        language: Language::Russian,
        section: SectionHeader::Verb,
        macro_text: "{{ru-verb|уступи́ть|pf|impf=уступа́ть}} ''(object in the [[accusative case]], receiver in the [[dative case]])''".to_string()
    };

    assert_eq!(
        verb.lemma(),
        "уступи́ть"
    )
}

#[test]
fn достичь() {
    let verb = RuVerb {
        page_id: 295351,
        page_title: "достичь".to_string(),
        language: Language::Russian,
        section: SectionHeader::Verb,
        macro_text: "{{ru-verb|дости́чь|pf|impf=достига́ть}} (''+ [[genitive case]]'')".to_string()
    };

    assert_eq!(
        verb.lemma(),
        "дости́чь"
    )
}


#[test]
fn проливать() {
    let verb = RuVerb {
        page_id: 295351,
        page_title: "проливать свет".to_string(),
        language: Language::Russian,
        section: SectionHeader::Verb,
        macro_text: "{{ru-verb|[[пролива́ть]] [[све́т]]|impf|pf=проли́ть све́т}}".to_string()
    };

    assert_eq!(
        verb.lemma(),
        "пролива́ть све́т"
    )
}


