use crate::models::{language::Language, section_header::SectionHeader, wiktionary_macro::Expand};
use super::UkNDecl;
use super::super::class_selectors;

#[tokio::test]
async fn город() {
    let table =  UkNDecl {
        page_id: 56898,
        page_title: "город".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Noun,
        macro_text: "{{uk-ndecl|го́род&lt;c&gt;}}".to_string()
    };
    let c = reqwest::Client::new();
    let html = table.html(&c).await;
    let form = table.get_form(&html, class_selectors::DAT_P);
}

#[tokio::test]
async fn Бутан() {
    let table =  UkNDecl {
        page_id: 56898,
        page_title: "Бутан".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::ProperNoun,
        macro_text: "{{uk-ndecl|Бута́н&lt;sg.genu&gt;}}".to_string()
    };
    let c = reqwest::Client::new();
    let html = table.html(&c).await;
    let form = table.get_form(&html, class_selectors::DAT_P);
    assert_eq!(form , None);
    
    let form = table.get_form(&html, class_selectors::LOC_S);
    assert_eq!(form.unwrap() , "Бута́ну")
}

#[tokio::test]
async fn вода() {
    let table =  UkNDecl {
        page_id: 56898,
        page_title: "вода".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Noun,
        macro_text: "{{uk-ndecl|вода́&lt;d'&gt;}}".to_string()
    };
    let c = reqwest::Client::new();
    let html = table.html(&c).await;
    let form = table.get_form(&html, class_selectors::VOC_S);
    assert_eq!(form.unwrap() , "во́до");
    
    let form = table.get_form(&html, class_selectors::INSTR_P);
    assert_eq!(form.unwrap() , "во́дами")
}