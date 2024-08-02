
use crate::models::{language::Language, section_header::SectionHeader, wiktionary_macro::Expand};
use super::super::class_selectors;
use super::UkConj;

#[tokio::test]
async fn мати() {
    let conj = UkConj {
        page_id: 92698,
        page_title: "мати".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Etymology2,
        macro_text: "{{uk-conj|ма́ти&lt;1a.impf.tr.-ppp&gt;}}".to_string()
    };
    let client = reqwest::Client::new();
    let html = conj.html(&client).await;
    assert!( conj.get_form(&html, class_selectors::ВІН_PRESENT_INDICATIVE).unwrap() == "ма́є" );
    assert!( conj.get_form(&html, class_selectors::INFINITIVE).unwrap() == "ма́ти" );
} 

#[tokio::test]
async fn представити() {
    let conj = UkConj {
        page_id: 192206,
        page_title: "представити".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Verb,
        macro_text: "{{uk-conj|предста́вити&lt;4a.pf.tr.ppp&gt;}}".to_string()
    };
    let client = reqwest::Client::new();
    let html = conj.html(&client).await;
    assert!( conj.get_form(&html, class_selectors::MASC_PAST).unwrap() == "предста́вив" );
    assert!( conj.get_form(&html, class_selectors::PLUR_PAST).unwrap() == "предста́вили" );
    assert!( conj.get_form(&html, class_selectors::ТИ_FUTURE_INDICATIVE).unwrap() == "предста́виш" );
    assert!( conj.get_form(&html, class_selectors::ВИ_PRESENT_INDICATIVE) == None );
}

// #[tokio::test]
// async fn провести() {
//     let conj = UkConj {
//         page_id: 760235,
//         page_title: "провести".to_string(),
//         language: Language::Ukrainian,
//         section: SectionHeader::Verb,
//         macro_text: "{{uk-conj|провести́&lt;7b.pf.tr.ppp.д&gt;}}".to_string()
//     };
//     let client = reqwest::Client::new();
//     let html = conj.html(&client).await;
//     assert!(
//         conj.get_form(&html, class_selectors::) == "провести́"
//     )
// } 



// #[tokio::test]
// async fn рушити() {
//     let conj = UkConj {
//         page_id: 814836,
//         page_title: "рушити".to_string(),
//         language: Language::Ukrainian,
//         section: SectionHeader::Verb,
//         macro_text: "{{uk-conj|ру́шити&lt;4a.pf.tr.ppp&gt;}}".to_string()
//     };
//     let client = reqwest::Client::new();
//     let html = conj.html(&client).await;
//     assert!(
//         conj.get_form(&html, class_selectors::) == "ру́шити"
//     )
// } 