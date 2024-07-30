
use crate::models::wiktionary_macro::ru_noun_table::class_selectors;
use crate::models::wiktionary_macro::{Expand, RuNounTable};
use crate::models::language::Language::Russian;
use crate::models::section_header::SectionHeader::*;


#[tokio::test]
async fn yoink_particular_forms() {
    let table = RuNounTable {
        page_id: 197114,
        page_title: String::from("шлюха"),
        language: Russian,
        section: Noun,
        macro_text: String::from("{{ru-noun-table|шлю\u{301}ха|a=an}}"),
    };

    let client = reqwest::Client::new();
    let html = table.html(&client).await;
    println!("{}", html.html());

    let gen_pl  = RuNounTable::get_form(&html, &class_selectors::GEN_P);

    assert_eq!(gen_pl, String::from("шлю\u{301}х"))
}