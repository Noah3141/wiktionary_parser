use crate::models::{language::Language, section_header::SectionHeader};

use super::BeNDecl;


#[test]
fn палец() {
    let noun = BeNDecl {
        page_id: 7514,
        page_title: "палец".to_string(),
        language: Language::Belarusian,
        section: SectionHeader::Noun,
        macro_text: "{{be-ndecl|па\u{301}лец&lt;*&gt;}}".to_string(),
    };

    assert_eq!(
        noun.lemma(),
        "па\u{301}лец"
    )
}

#[test]
fn брат() {
    let noun = BeNDecl {
        page_id: 106448,
        page_title: "брат".to_string(),
        language: Language::Belarusian,
        section: SectionHeader::Noun,
        macro_text: "{{be-ndecl|брат&lt;c.pr.voc:бра\u{301}це&gt;}}".to_string(),
    };

    assert_eq!(
        noun.lemma(),
        "брат"
    )
}

#[tokio::test]
async fn яйцо() {
    let noun = BeNDecl {
        page_id: 188822,
        page_title: "яйцо".to_string(),
        language: Language::Belarusian,
        section: SectionHeader::Noun,
        macro_text: "{{be-ndecl|яйцо\u{301}&lt;*#(-)&gt;}}".to_string(),
    };

    assert_eq!(
        noun.lemma(),
        "яйцо\u{301}"
    );

    let client = reqwest::Client::new();

    assert!(
        noun.form_and_lemma(&client).await.contains(
            &("яе\u{301}ц".to_string(),"яйцо\u{301}")
        )
    )
}

#[test]
fn сыр() {
    let noun = BeNDecl {
        page_id: 208769,
        page_title: "сыр".to_string(),
        language: Language::Belarusian,
        section: SectionHeader::Noun,
        macro_text: "{{be-ndecl|сыр&lt;sg&gt;}}".to_string(),
    };

    assert_eq!(
        noun.lemma(),
        "сыр"
    )
}

#[test]
fn туалет() {
    let noun = BeNDecl {
        page_id: 239394,
        page_title: "туалет".to_string(),
        language: Language::Belarusian,
        section: SectionHeader::Noun,
        macro_text: "{{be-ndecl|туале\u{301}т&lt;&gt;}}".to_string(),
    };

    assert_eq!(
        noun.lemma(),
        "туале\u{301}т"
    )
}