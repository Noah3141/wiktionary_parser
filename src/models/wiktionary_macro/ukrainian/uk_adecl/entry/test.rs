use crate::models::{language::Language, section_header::SectionHeader, wiktionary_macro::Expand};
use super::UkADecl;
use super::super::class_selectors;

#[tokio::test]
async fn хороший() {
    let table = UkADecl {
        page_id: 188329,
        page_title: "хороший".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Adjective,
        macro_text: "{{uk-adecl|хоро́ший}}".to_string()
    };
    let c = reqwest::Client::new();
    let html = table.html(&c).await;
    let form = table.get_form(&html, &class_selectors::GEN_P);
    assert_eq!( form.unwrap(), "хоро́ших" );

    let form = table.get_form(&html, &class_selectors::INSTR_F);
    assert_eq!( form.unwrap(), "хоро́шою" );
}

#[tokio::test]
async fn великий() {
    let table = UkADecl {
        page_id: 188329,
        page_title: "великий".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Adjective,
        macro_text: "{{uk-adecl|вели́кий}}".to_string()
    };
    let c = reqwest::Client::new();
    let html = table.html(&c).await;
    let form = table.get_form(&html, &class_selectors::GEN_N);
    assert_eq!( form.unwrap(), "вели́кого" );

    let form = table.get_form(&html, &class_selectors::AN_ACC_M);
    assert_eq!( form.unwrap(), "вели́кого" );
}


#[tokio::test]
async fn радий() {
    let table = UkADecl {
        page_id: 188329,
        page_title: "радий".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Adjective,
        macro_text: "{{uk-adecl|ра́дий&lt;old&gt;|short=рад}}".to_string()
    };
    let c = reqwest::Client::new();
    let html = table.html(&c).await;
    let form = table.get_form(&html, class_selectors::LOC_F);
    assert_eq!( form.expect("ра́дій"), "ра́дій" );
    
    // Todo This table ONLY has masc short form, so the damn class ignores mentionnig 'm' like the SHORT_M

    let form = table.get_form(&html, class_selectors::SHORT_F);
    assert_eq!( form, None );

    let form = table.get_form(&html, class_selectors::SHORT_N);
    assert_eq!( form, None );

}