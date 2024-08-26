use crate::models::{language::Language, section_header::SectionHeader};

use super::UkADecl;


#[test]
fn хороший() {
    let table = UkADecl {
        page_id: 188329,
        page_title: "хороший".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Adjective,
        macro_text: "{{uk-adecl|хоро́ший}}".to_string()
    };
    assert_eq!( table.lemma(), "хоро́ший" );
}

#[test]
fn великий() {
    let table = UkADecl {
        page_id: 188329,
        page_title: "великий".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Adjective,
        macro_text: "{{uk-adecl|вели́кий}}".to_string()
    };
    assert_eq!( table.lemma(), "вели́кий" );
}


#[test]
fn радий() {
    let table = UkADecl {
        page_id: 188329,
        page_title: "радий".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Adjective,
        macro_text: "{{uk-adecl|ра́дий&lt;old&gt;|short=рад}}".to_string()
    };
    assert_eq!( table.lemma(), "ра́дий" );
}

#[test]
fn стартовий() {
    let table = UkADecl {
        page_id: 188329,
        page_title: "радий".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Adjective,
        macro_text: "{{uk-adecl|((ста́ртовий,стартови́й))}}".to_string()
    };
    assert_eq!( table.lemma(), "ста́ртовий" );
}

