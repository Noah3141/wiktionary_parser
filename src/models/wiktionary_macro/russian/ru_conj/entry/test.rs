
use crate::models::wiktionary_macro::{Expand, RuConj, RuNounTable};
use crate::models::language::Language::Russian;
use crate::models::section_header::SectionHeader::*;
use super::super::class_selectors;

// RuConj {
//     page_id: 43762,
//     page_title: "знать",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|impf|1a|знать|ppp=-|pradp2=зна\u{301}ючи|notes=1 Dated}}",
// }
// RuConj {
//     page_id: 46362,
//     page_title: "лететь",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|impf-intr|5b|лете\u{301}ть}}",
// }
// RuConj {
//     page_id: 46589,
//     page_title: "спать",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|impf-intr|5b/c|спать}}",
// }
// RuConj {
//     page_id: 47435,
//     page_title: "играть",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|impf|1a+p|игра\u{301}ть|pradp2=игра\u{301}ючи1|notes=1 Dated}}",
// }
// RuConj {
//     page_id: 55067,
//     page_title: "отпочковываться",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|impf|1a|отпочко\u{301}вываться}}",
// }
// RuConj {
//     page_id: 57783,
//     page_title: "жалить",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|impf|4a+p|жа\u{301}лить}}",
// }
// RuConj {
//     page_id: 58477,
//     page_title: "презирать",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|impf|1a|презира\u{301}ть}}",
// }
// RuConj {
//     page_id: 58560,
//     page_title: "делать",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|impf|1a+p|де\u{301}лать}}",
// }
// RuConj {
//     page_id: 63950,
//     page_title: "быть",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|impf-intr|irreg/c|быть|prestailall=1△|pres_1sg2=есмь1|pres_2sg2=еси\u{301}2|pres_2sg3=есь2|pres_1pl2=есмы\u{301}2|pres_2pl2=е\u{301}сте2|pres_3pl2=суть1|notes=1 See usage notes.|notes2=2 Obsolete.}}",
// }
// RuConj {
//     page_id: 68271,
//     page_title: "уступать",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|impf|1a|уступа\u{301}ть}}",
// }

// RuConj {
//     page_id: 68912,
//     page_title: "держать",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|impf|5c+p|держа\u{301}ть}}",
// }
// RuConj {
//     page_id: 69083,
//     page_title: "деревенеть",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|impf-intr|1a|деревене\u{301}ть}}",
// }
// RuConj {
//     page_id: 70098,
//     page_title: "давать",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|impf|13b|дава\u{301}ть}}",
// }
// RuConj {
//     page_id: 73293,
//     page_title: "скучать",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|impf-intr|1a|скуча\u{301}ть}}",
// }
// RuConj {
//     page_id: 87071,
//     page_title: "брать",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|impf|6°b/c|брать|бер|ppp=-}}",
// }
// RuConj {
//     page_id: 95442,
//     page_title: "взять",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|pf|14b/c'+p|взять|возьм}}",
// }
// RuConj {
//     page_id: 95706,
//     page_title: "дать",
//     language: Russian,
//     section: Verb,
//     macro_text: "{{ru-conj|pf|irreg/c'+p|дать|past_n=дало\u{301}|past_n2=да\u{301}ло|past_n_tail=*|notes=* Less common.}}",
// }

#[tokio::test]
async fn yoink_particular_forms() {
    let table =  RuConj {
        page_id: 63950,
        page_title: "быть".to_string(),
        language: Russian,
        section: Verb,
        macro_text: "{{ru-conj|impf-intr|irreg/c|быть|prestailall=1△|pres_1sg2=есмь1|pres_2sg2=еси\u{301}2|pres_2sg3=есь2|pres_1pl2=есмы\u{301}2|pres_2pl2=е\u{301}сте2|pres_3pl2=суть1|notes=1 See usage notes.|notes2=2 Obsolete.}}".to_string(),
    };

    let client = reqwest::Client::new();
    let html = table.html(&client).await;
    println!("{}", html.html());

    let neut_past  = RuConj::get_form(&html, &class_selectors::NEUT_PAST).expect("presence of form");

    assert_eq!(neut_past, String::from("бы\u{301}ло"))
}


#[tokio::test]
async fn nonpresent_forms_dont_break() {
    let table = RuConj {
        page_id: 95442,
        page_title: "взять".to_string(),
        language: Russian,
        section: Verb,
        macro_text: "{{ru-conj|pf|14b/c'+p|взять|возьм}}".to_string(),
    };

    let client = reqwest::Client::new();
    let html = table.html(&client).await;
    
    assert_eq!(
        RuConj::get_form(&html, &class_selectors::ОН_PRESENT_INDICATIVE),
        None
    );
    assert_eq!(
        RuConj::get_form(&html, &class_selectors::МЫ_PRESENT_INDICATIVE),
        None
    );
    assert_eq!(
        RuConj::get_form(&html, &class_selectors::ОНИ_PRESENT_INDICATIVE),
        None
    );
    assert_eq!(
        RuConj::get_form(&html, &class_selectors::Я_FUTURE_INDICATIVE).unwrap(),
        "возьму\u{301}"
    );
    assert_eq!(
        RuConj::get_form(&html, &class_selectors::МЫ_FUTURE_INDICATIVE).unwrap(),
        "возьмём"
    );
    assert_eq!(
        RuConj::get_form(&html, &class_selectors::ОНИ_FUTURE_INDICATIVE).unwrap(),
        "возьму\u{301}т"
    );
}

#[tokio::test]
async fn perfectivity_detects() {
    let table = RuConj {
        page_id: 95442,
        page_title: "взять".to_string(),
        language: Russian,
        section: Verb,
        macro_text: "{{ru-conj|pf|14b/c'+p|взять|возьм}}".to_string(),
    };

    let client = reqwest::Client::new();
    let html = table.html(&client).await;

    assert!(RuConj::is_perfective(&html));
}