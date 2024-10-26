use crate::models::{language::Language, section_header::SectionHeader, wiktionary_macro::RuAdj};

#[test]
fn tayskiy() {
    let verb = RuAdj {
        page_id: 295351,
        page_title: "тайский".to_string(),
        language: Language::Russian,
        section: SectionHeader::Adjective,
        macro_text: "{{ru-adj|та́йский}}".to_string()
    };

    assert_eq!(
        verb.lemma(),
        "та́йский"
    )
}

#[test]
fn slabiy() {
    let verb = RuAdj {
        page_id: 295351,
        page_title: "слабый".to_string(),
        language: Language::Russian,
        section: SectionHeader::Adjective,
        macro_text: "{{ru-adj|сла́бый|слабе́е|слабе́йший|adv=сла́бо|absn=сла́бость|dim=сла́бенький}}".to_string()
    };

    assert_eq!(
        verb.lemma(),
        "сла́бый"
    )
}

#[test]
fn laskoviy() {
    let verb = RuAdj {
        page_id: 295351,
        page_title: "ласковый".to_string(),
        language: Language::Russian,
        section: SectionHeader::Adjective,
        macro_text: "{{ru-adj|ла́сковый|+}}".to_string()
    };

    assert_eq!(
        verb.lemma(),
        "ла́сковый"
    )
}

#[test]
fn el() {
    let verb = RuAdj {
        page_id: 295351,
        page_title: "Л".to_string(),
        language: Language::Russian,
        section: SectionHeader::Adjective,
        macro_text: "{{ru-adj|indecl=1}}".to_string()
    };

    assert_eq!(
        verb.lemma(),
        "Л"
    )
}