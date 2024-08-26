use crate::models::{language::Language, section_header::SectionHeader};
use super::UkNDecl;



#[test]
fn собака() {
    let table =  UkNDecl {
        page_id: 1880,
        page_title: "собака".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Noun,
        macro_text: "{{uk-ndecl|соба́ка&lt;MF.anml&gt;}}".to_string()
    };
    assert_eq!( table.lemma() , "соба́ка" );
}


#[test]
fn весь() {
    let table =  UkNDecl {
        page_id: 44950,
        page_title: "весь".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Noun,
        macro_text: "{{uk-ndecl|ве́сь&lt;F&gt;}}".to_string()
    };
    assert_eq!( table.lemma() , "ве́сь" );
}

#[test]
fn горілка() {
    let table =  UkNDecl {
        page_id: 44950,
        page_title: "горілка".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Noun,
        macro_text: "{{uk-ndecl|горі́лка&lt;a*,c*&gt;}}".to_string()
    };
    assert_eq!( table.lemma() , "горі́лка" );
}

#[test]
fn слава() {
    let table =  UkNDecl {
        page_id: 56898,
        page_title: "слава".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Noun,
        macro_text: "{{uk-ndecl|сла́ва&lt;sg&gt;}}".to_string()
    };
    assert_eq!( table.lemma() , "сла́ва" );
}


#[test]
fn СаудівськаАравія() {
    let table = UkNDecl {
        page_id: 188329,
        page_title: "Саудівська Аравія".to_string(),
        language: Language::Ukrainian,
        section: SectionHeader::Adjective,
        macro_text: "{{uk-ndecl|[[сау́дівський|Сау́дівська]]&lt;+&gt; [[Ара́вія]]&lt;sg&gt;}}".to_string()
    };
    assert_eq!( table.lemma(), "Саудівська Аравія" );
}
