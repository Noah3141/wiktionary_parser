
use super::super::class_selectors;
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

    let gen_pl  = table.get_form(&html, &class_selectors::GEN_P);

    assert_eq!(gen_pl.unwrap(), String::from("шлю\u{301}х"))
}


#[tokio::test]
async fn animacy() {
    let table = RuNounTable {
        page_id: 197114,
        page_title: String::from("кошка"),
        language: Russian,
        section: Noun,
        macro_text: String::from("{{ru-noun-table|ко́шка|*|a=ai}}"),
    };

    let client = reqwest::Client::new();
    let html = table.html(&client).await;
    println!("{}", html.html());

    let gen_pl  = table.get_form(&html, &class_selectors::GEN_P);

    assert_eq!(gen_pl.unwrap(), String::from("ко́шек"))
}


#[tokio::test]
async fn plurale_tantum() {
    let table = RuNounTable {
        page_id: 197114,
        page_title: String::from("люди"),
        language: Russian,
        section: Noun,
        macro_text: String::from("{{ru-noun-table|лю́ди|m|gen_pl=люде́й|ins_pl=людьми́|a=an}}"),
    };

    let client = reqwest::Client::new();
    let html = table.html(&client).await;
    println!("{}", html.html());

    let singular_dative  = table.get_form(&html, &class_selectors::DAT_S);

    assert_eq!(singular_dative, None)
}
