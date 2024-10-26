use crate::models::{language::Language, section_header::SectionHeader, wiktionary_macro::RuNounPlus};



#[test]
fn cobaka() {
    let noun = RuNounPlus {
        page_id: 295351,
        page_title: "собака".to_string(),
        language: Language::Russian,
        section: SectionHeader::Noun,
        macro_text: "{{ru-noun+|соба́ка|a=an|adj=соба́чий|dim=соба́чка}}".to_string()
    };

    assert_eq!(
        noun.lemma(),
        "соба́ка"
    )
}

#[test]
fn uka3() {
    let noun = RuNounPlus {
        page_id: 295351,
        page_title: "указ".to_string(),
        language: Language::Russian,
        section: SectionHeader::Noun,
        macro_text: "{{ru-noun+|ука́з}}".to_string()
    };

    assert_eq!(
        noun.lemma(),
        "ука́з"
    )
}

#[test]
fn zont() {
    let noun = RuNounPlus {
        page_id: 295351,
        page_title: "зонт".to_string(),
        language: Language::Russian,
        section: SectionHeader::Noun,
        macro_text: "{{ru-noun+|b}}".to_string()
    };

    assert_eq!(
        noun.lemma(),
        "зонт"
    )
}

#[test]
fn кот() {
    let noun = RuNounPlus {
        page_id: 295351,
        page_title: "кот".to_string(),
        language: Language::Russian,
        section: SectionHeader::Noun,
        macro_text: "{{ru-noun+|b|a=an|f=ко́шка|adj=коша́чий|dim=ко́тик|dim2=кото́к|aug=коти́ще|aug2=котя́ра|pej=коша́к}}".to_string()
    };

    assert_eq!(
        noun.lemma(),
        "кот"
    )
}