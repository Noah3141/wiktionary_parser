use super::BeConj;
use crate::models::{language::Language::Belarusian, section_header::SectionHeader};

#[test]
fn спрачацца() {
    let verb = BeConj {
        page_id: 4078689,
        page_title: "спрачацца".to_string(),
        language: Belarusian,
        section: SectionHeader::Verb,
        macro_text: "{{be-conj|спрача\u{301}цца&lt;1a.impf&gt;}}".to_string(),
    };

    assert_eq!(
        verb.lemma(),
        "спрача\u{301}цца"
    )
} 

#[test]
fn пачакаць() {
    let verb = BeConj {
        page_id: 4221833,
        page_title: "пачакаць".to_string(),
        language: Belarusian,
        section: SectionHeader::Verb,
        macro_text: "{{be-conj|пачака\u{301}ць&lt;1a.pf.intr&gt;}}".to_string(),
    };

    assert_eq!(
        verb.lemma(),
        "пачака\u{301}ць"
    )
} 

#[test]
fn паказаць() {
    let verb = BeConj {
        page_id: 4606813,
        page_title: "паказаць".to_string(),
        language: Belarusian,
        section: SectionHeader::Verb,
        macro_text: "{{be-conj|паказа\u{301}ць&lt;6c.pf.tr.ppp-&gt;}}".to_string(),
    };

    assert_eq!(
        verb.lemma(),
        "паказа\u{301}ць"
    )
} 

#[test]
fn паўтараць() {
    let verb = BeConj {
        page_id: 4784516,
        page_title: "паўтараць".to_string(),
        language: Belarusian,
        section: SectionHeader::Verb,
        macro_text: "{{be-conj|паўтара\u{301}ць&lt;1a.impf.tr.-ppp&gt;}}".to_string(),
    };

    assert_eq!(
        verb.lemma(),
        "паўтара\u{301}ць"
    )
} 

#[test]
fn пачуць() {
    let verb = BeConj {
        page_id: 7206019,
        page_title: "пачуць".to_string(),
        language: Belarusian,
        section: SectionHeader::Verb,
        macro_text: "{{be-conj|пачу\u{301}ць&lt;12a.pf.tr.ppp&gt;}}".to_string(),
    };

    assert_eq!(
        verb.lemma(),
        "пачу\u{301}ць"
    )
} 

#[test]
fn тыкаць() {
    let verb = BeConj {
        page_id: 8572148,
        page_title: "тыкаць".to_string(),
        language: Belarusian,
        section: SectionHeader::Verb,
        macro_text: "{{be-conj|ты\u{301}каць&lt;1a.impf.tr&gt;}}".to_string(),
    };

    assert_eq!(
        verb.lemma(),
        "ты\u{301}каць"
    )
} 

#[test]
fn hiphen_ймаць() {
    let verb = BeConj {
        page_id: 9640843,
        page_title: "-ймаць".to_string(),
        language: Belarusian,
        section: SectionHeader::Suffix,
        macro_text: "{{be-conj|-йма\u{301}ць&lt;1a.impf.tr.ppp&gt;}}".to_string(),
    };

    assert_eq!(
        verb.lemma(),
        "-йма\u{301}ць"
    )
}

#[test]
fn рахаваць() {
    let verb = BeConj {
        page_id: 10089032,
        page_title: "рахаваць".to_string(),
        language: Belarusian,
        section: SectionHeader::Verb,
        macro_text: "{{be-conj|рахава\u{301}ць&lt;2a.impf.tr.ppp&gt;}}".to_string(),
    };

    assert_eq!(
        verb.lemma(),
        "рахава\u{301}ць"
    )
} 
