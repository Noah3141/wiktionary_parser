
use crate::models::{language::Language::{self, }, section_header::SectionHeader::*, wiktionary_macro::{BeConj, Expand}};
use super::super::class_selectors;
// BeConj {
//     page_id: 184406,
//     page_title: "аб'явіць",
//     language: Belarusian,
//     section: Verb,
//     macro_text: "{{be-conj|аб'яві\u{301}ць&lt;4c.pf.tr.ppp&gt;}}",
// }
// {{be-conj|піса́ць&lt;6c.impf.tr.ppp&gt;|pres_actv_part=пі́шучы}} - пісаць
// BeConj {
//     page_id: 4039867,
//     page_title: "пісаць",
//     language: Belarusian,
//     section: Verb,
//     macro_text: "{{be-conj|піса\u{301}ць&lt;6c.impf.tr.ppp&gt;|pres_actv_part=пі\u{301}шучы}}",
// }
// {{be-conj|заплаці́ць&lt;4c.pf.tr.ppp&gt;}} - заплаціць
// BeConj {
//     page_id: 4053205,
//     page_title: "заплаціць",
//     language: Belarusian,
//     section: Verb,
//     macro_text: "{{be-conj|заплаці\u{301}ць&lt;4c.pf.tr.ppp&gt;}}",
// }
// {{be-conj|дасягну́ць&lt;3b.pf.tr.ppp-&gt;}} - дасягнуць
// BeConj {
//     page_id: 4069452,
//     page_title: "дасягнуць",
//     language: Belarusian,
//     section: Verb,
//     macro_text: "{{be-conj|дасягну\u{301}ць&lt;3b.pf.tr.ppp-&gt;}}",
// }
// {{be-conj|спрача́цца&lt;1a.impf&gt;}} - спрачацца
// BeConj {
//     page_id: 4078689,
//     page_title: "спрачацца",
//     language: Belarusian,
//     section: Verb,
//     macro_text: "{{be-conj|спрача\u{301}цца&lt;1a.impf&gt;}}",
// }


#[tokio::test]
async fn yoink_particular_forms() {
    let table =  BeConj {
        page_id: 4078689,
        page_title: "спрачацца".to_string(),
        language: Language::Belarusian,
        section: Verb,
        macro_text: "{{be-conj|спрача\u{301}цца&lt;1a.impf&gt;}}".to_string(),
    };

    let client = reqwest::Client::new();
    let html = table.html(&client).await;
    println!("{}", html.html());

    let neut_past  = BeConj::get_form(&html, &class_selectors::NEUT_PAST).expect("presence of form");

    assert_eq!(neut_past, String::from("спрача\u{301}лася")) // sic, Belarusian spelling lol
}


#[tokio::test]
async fn nonpresent_forms_dont_break() {
    let table = BeConj {
        page_id: 4039867,
        page_title: "пісаць".to_string(),
        language: Language::Belarusian,
        section: Verb,
        macro_text: "{{be-conj|піса\u{301}ць&lt;6c.impf.tr.ppp&gt;|pres_actv_part=пі\u{301}шучы}}".to_string(),
    };

    let client = reqwest::Client::new();
    let html = table.html(&client).await;
    
    assert_eq!(
        BeConj::get_form(&html, &class_selectors::PAST_ACTIVE_PARTICIPLE),
        None
    );
    assert_eq!(
        BeConj::get_form(&html, &class_selectors::PRESENT_PASSIVE_PARTICIPLE),
        None
    );
    assert_eq!(
        BeConj::get_form(&html, &class_selectors::Я_PRESENT_INDICATIVE).unwrap(),
        "пішу́"
    );
    assert_eq!(
        BeConj::get_form(&html, &class_selectors::МЫ_PRESENT_INDICATIVE).unwrap(),
        "пі́шам"
    );
    assert_eq!(
        BeConj::get_form(&html, &class_selectors::ОНИ_PRESENT_INDICATIVE).unwrap(),
        "пі́шуць"
    );
}

#[tokio::test]
async fn perfectivity_detects() {
    let table =  BeConj {
        page_id: 4053205,
        page_title: "заплаціць".to_string(),
        language: Language::Belarusian,
        section: Verb,
        macro_text: "{{be-conj|заплаці\u{301}ць&lt;4c.pf.tr.ppp&gt;}}".into(),
    };

    let client = reqwest::Client::new();
    let html = table.html(&client).await;

    assert!(table.is_perfective(&html));
    assert!(!table.is_imperfective(&html));
}