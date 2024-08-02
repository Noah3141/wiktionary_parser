
use super::super::class_selectors;
use crate::models::wiktionary_macro::{BeNDecl, Expand, RuNounTable};
use crate::models::language::Language::{self, Russian};
use crate::models::section_header::SectionHeader::{self, *};


#[tokio::test]
async fn yoink_particular_forms() {
    let table = BeNDecl {
        page_id: 106448,
        page_title: "брат".to_string(),
        language: Language::Belarusian,
        section: SectionHeader::Noun,
        macro_text: "{{be-ndecl|брат&lt;c.pr.voc:бра\u{301}це&gt;}}".to_string(),
    };

    let client = reqwest::Client::new();
    let html = table.html(&client).await;
    println!("{}", html.html());

    let gen_pl  = table.get_form(&html, &class_selectors::GEN_P).expect("gen p");
    assert_eq!(gen_pl, "брато\u{301}ў");

    let gen_pl  = table.get_form(&html, &class_selectors::DAT_P).expect("dat p");
    assert_eq!(gen_pl, "брата\u{301}м");
}