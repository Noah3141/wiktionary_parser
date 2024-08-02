
use crate::models::{language::Language, section_header::SectionHeader};

use super::UkConj;

#[test]
fn мати() {
    let conj = UkConj {
        page_id: 92698,
        page_title: "мати".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Etymology2,
        macro_text: "{{uk-conj|ма́ти&lt;1a.impf.tr.-ppp&gt;}}".to_string()
    };
    assert!(
        conj.lemma() == "ма́ти"
    )
} 
#[test]
fn представити() {
    let conj = UkConj {
        page_id: 192206,
        page_title: "представити".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Verb,
        macro_text: "{{uk-conj|предста́вити&lt;4a.pf.tr.ppp&gt;}}".to_string()
    };
    assert!(
        conj.lemma() == "предста́вити"
    )
} 
#[test]
fn провести() {
    let conj = UkConj {
        page_id: 760235,
        page_title: "провести".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Verb,
        macro_text: "{{uk-conj|провести́&lt;7b.pf.tr.ppp.д&gt;}}".to_string()
    };
    assert!(
        conj.lemma() == "провести́"
    )
} 


#[test]
fn рушити() {
    let conj = UkConj {
        page_id: 814836,
        page_title: "рушити".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Verb,
        macro_text: "{{uk-conj|ру́шити&lt;4a.pf.tr.ppp&gt;}}".to_string()
    };
    assert!(
        conj.lemma() == "ру́шити"
    )
} 