
use crate::models::section_header::SectionHeader;
use super::RuDeclAdj;

#[tokio::test]
async fn basic_adj() {
    let adj = RuDeclAdj {
        page_id: 1504929,
        page_title: "знакомый".to_string(),
        language: crate::models::language::Language::Russian,
        section: SectionHeader::Adjective,
        macro_text: "{{ru-decl-adj|знако\u{301}мый|a}}".to_string(),
    };
    let client = reqwest::Client::new();

    let forms = adj.form_and_lemma(&client).await;
    dbg!(&forms);

    assert!(
        forms.contains(
            &("знако\u{301}мого".to_string(), "знако\u{301}мый")
        )
    );
    assert!(
        forms.contains(
            &("знако\u{301}мых".to_string(), "знако\u{301}мый")
        )
    );
    assert!(
        forms.contains(
            &("знако\u{301}мая".to_string(), "знако\u{301}мый")
        )
    )
}


#[test]
fn extracting_lemma1() {
    let adj = RuDeclAdj {
        page_id: 00000,
        page_title: "Not used".to_string(),
        language: crate::models::language::Language::Russian,
        section: SectionHeader::Adjective,
        macro_text: "{{ru-decl-adj|знако\u{301}мый|a}}".to_string(),
    };
    let accented_form = adj.lemma();
    assert_eq!(accented_form, "знако\u{301}мый");
}
#[test]
fn extracting_lemma2() {
    let adj = RuDeclAdj {
        page_id: 814497,
        page_title: "лихорадочный".to_string(),
        language: crate::models::language::Language::Russian,
        section: SectionHeader::Adjective,
        macro_text: "{{ru-decl-adj|лихора\u{301}дочный|a*}}".to_string(),
    };
    let accented_form = adj.lemma();
    assert_eq!(accented_form, "лихора\u{301}дочный");
}
#[test]
fn extracting_lemma3() {
    let adj = RuDeclAdj {
        page_id: 7512,
        page_title: "большой".to_string(),
        language: crate::models::language::Language::Russian,
        section: SectionHeader::Adjective,
        macro_text: "{{ru-decl-adj|большо\u{301}й|b:вели\u{301}к}}".to_string(),
    };
    let accented_form = adj.lemma();
    assert_eq!(accented_form, "большо\u{301}й");
}

#[test]
fn варёный() {
    let adj = RuDeclAdj {
        page_id: 7512,
        page_title: "варёный".to_string(),
        language: crate::models::language::Language::Russian,
        section: SectionHeader::Adjective,
        macro_text: "{{ru-decl-adj|}}".to_string(),
    };
    let accented_form = adj.lemma();
    assert_eq!(accented_form, "варёный");
}

#[test]
fn вооружённый() {
    let adj = RuDeclAdj {
        page_id: 7512,
        page_title: "вооружённый".to_string(),
        language: crate::models::language::Language::Russian,
        section: SectionHeader::Adjective,
        macro_text: "{{ru-decl-adj||a(1)}}".to_string(),
    };
    let accented_form = adj.lemma();
    assert_eq!(accented_form, "вооружённый");
}

#[test]
fn ordinal() {
    let adj = RuDeclAdj {
        page_id: 000,
        page_title: "девяносто два".to_string(),
        language: crate::models::language::Language::Russian,
        section: SectionHeader::Adjective,
        macro_text: "{{ru-decl-adj|-|manual|nom_mp=девяно́сто два́|nom_fp=девяно́сто две́|gen_p=девяно́ста дву́х|dat_p=девяно́ста дву́м|ins_p=девяно́ста двумя́|pre_p=девяно́ста дву́х|special=cdva}}".to_string(),
    };

    let accented_form = adj.lemma();
    assert_eq!(accented_form, "девяно\u{301}сто два\u{301}");
}
