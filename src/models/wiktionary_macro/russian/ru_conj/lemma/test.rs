use crate::models::wiktionary_macro::RuConj;
use crate::models::language::Language::Russian;
use crate::models::section_header::SectionHeader::Verb;

#[test]
fn убавить() {
    let verb = RuConj {
        page_id: 4139065,
        page_title: "убавить".to_string(),
        language: Russian,
        section: Verb,
        macro_text: "{{ru-conj|pf|4a+p|уба\u{301}вить}}".to_string(),
    };

    assert_eq!(
        verb.lemma(),
        "уба\u{301}вить"
    );
}

#[test]
fn обойтись() {
    let verb = RuConj {
        page_id: 4189781,
        page_title: "обойтись".to_string(),
        language: Russian,
        section: Verb,
        macro_text: "{{ru-conj|pf|irreg|обойти\u{301}сь}}".to_string(),
    };

    assert_eq!(
        verb.lemma(),
        "обойти\u{301}сь"
    );
}

#[test]
fn балакать() {
    let verb = RuConj {
        page_id: 4282009,
        page_title: "балакать".to_string(),
        language: Russian,
        section: Verb,
        macro_text: "{{ru-conj|impf-intr|1a|бала\u{301}кать}}".to_string(),
    };

    assert_eq!(
        verb.lemma(),
        "бала\u{301}кать"
    );
}

#[test]
fn лишить() {
    let verb = RuConj {
        page_id: 4313120,
        page_title: "лишить".to_string(),
        language: Russian,
        section: Verb,
        macro_text: "{{ru-conj|pf|4b+p|лиши\u{301}ть}}".to_string(),
    };

    assert_eq!(
        verb.lemma(),
        "лиши\u{301}ть"
    );
}

#[test]
fn экспортировать() {
    let verb = RuConj {
        page_id: 4503608,
        page_title: "экспортировать".to_string(),
        language: Russian,
        section: Verb,
        macro_text: "{{ru-conj|pf|2a+p|экспорти\u{301}ровать}}".to_string(),
    };

    assert_eq!(
        verb.lemma(),
        "экспорти\u{301}ровать"
    );
}

#[test]
fn образовываться() {
    let verb = RuConj {
        page_id: 4654130,
        page_title: "образовываться".to_string(),
        language: Russian,
        section: Verb,
        macro_text: "{{ru-conj|impf|1a|образо\u{301}вываться}}".to_string(),
    };

    assert_eq!(
        verb.lemma(),
        "образо\u{301}вываться"
    );
}