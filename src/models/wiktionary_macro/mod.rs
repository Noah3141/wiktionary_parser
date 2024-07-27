use core::panic;
use std::fmt::format;
use audio::Audio;
use belarusian::{be_adecl::BeADecl, be_adj::BeAdj, be_adv::BeAdv, be_ipa::BeIpa, be_ndecl::BeNDecl, be_noun::BeNoun, be_verb::BeVerb};
use russian::{ru_adj::RuAdj, ru_conj::RuConj, ru_decl_adj::RuDeclAdj, ru_ipa::RuIpa, ru_noun_plus::RuNounPlus, ru_noun_table::RuNounTable, ru_verb::RuVerb};
use serde::{Deserialize, Serialize};
use ukrainian::{uk_adecl::UkADecl, uk_adj::UkAdj, uk_conj::UkConj, uk_ipa::UkIpa, uk_ndecl::UkNDecl, uk_noun::UkNoun, uk_verb::UkVerb};
use crate::utils::{select_from::select_from, select_unto_language_header::select_unto_language_header, select_unto_section_header::select_unto_section_header, split_sections::split_section};
use super::{language::Language, section_header::SectionHeader};

pub mod inflection_of;
pub mod related_terms;
pub mod head;
pub mod affix;
pub mod audio;

pub mod russian;
pub mod belarusian;
pub mod ukrainian;

use inflection_of::InflectionOf;
use head::Head;
use related_terms::RelatedTerms;

#[derive(Serialize, Deserialize)]
pub enum WiktionaryMacro {
    InflectionOf(InflectionOf),
    RelatedTerms(RelatedTerms),
    Head(Head),
    Audio(Audio),
    // Belarusian
    BeADecl(BeADecl),
    BeAdj(BeAdj),
    BeAdv(BeAdv),
    BeIpa(BeIpa),
    BeNDecl(BeNDecl),
    BeNoun(BeNoun),
    BeVerb(BeVerb),
    // Russian
    RuAdj(RuAdj),
    RuConj(RuConj),
    RuDeclAdj(RuDeclAdj),
    RuIpa(RuIpa),
    RuNounPlus(RuNounPlus),
    RuNounTable(RuNounTable),
    RuVerb(RuVerb),
    // Ukrainian
    UkADecl(UkADecl),
    UkAdj(UkAdj),
    UkIpa(UkIpa),
    UkNDecl(UkNDecl),
    UkNoun(UkNoun),
    UkVerb(UkVerb),
    UkConj(UkConj),
}


impl WiktionaryMacro {
    /// Take in <page> </page>
    pub fn parse_from_xml(page_xml: &str) -> Vec<Self> {
        let mut wiki_macros: Vec<Self> = Vec::with_capacity(30_000);
        let page_id = u64::from_str_radix(
            select_from(page_xml, "<id>", "</id>").expect("presence of page id"),
            10
        ).expect("radix");
        let page_title = select_from(page_xml, "<title>", "</title>").expect("page title").to_string();
        if page_xml.contains("==Russian==") {
            let language_section = select_unto_language_header(page_xml, "==Russian==").expect("successful language section extraction");
            let language = Language::Russian;
            let sections = split_section(language_section);
            for (section, section_text) in sections {
                let section_wiki_macros = WiktionaryMacro::find_macros_in(section_text);
                for macro_text in section_wiki_macros {
                    match WiktionaryMacro::new(
                        page_id, 
                        page_title.clone(), 
                        language, 
                        section, 
                        macro_text
                    ) {
                        Ok(m) => wiki_macros.push(m),
                        Err(e) => println!("{e}"),
                    };
                }
            }
        }
        wiki_macros
    }


    /// Takes macro text and determines which macro is contained
    pub fn new(
        page_id: u64,
        page_title: String, 
        language: Language, 
        section: SectionHeader,
        macro_text: String, 
    ) -> Result<Self, String> {
        assert!(macro_text.starts_with("{{"));
        let mut macro_name = select_from(&macro_text, "{{", "|").expect("presence of macro start in macro_text");
        macro_name = macro_name.strip_suffix("}}").unwrap_or(macro_name);

        let new_macro = match macro_name {
            InflectionOf::TAG1 | 
            InflectionOf::TAG2 => Self::InflectionOf(InflectionOf { page_id, page_title, language, section, macro_text }),
            RelatedTerms::TAG => Self::RelatedTerms(RelatedTerms { page_id, page_title, language, section, macro_text }),
            Head::TAG => Self::Head(Head { page_id, page_title, language, section, macro_text }),
            // Belarusian
            BeADecl::TAG => Self::BeADecl(BeADecl { page_id, page_title, language, section, macro_text }),
            BeAdj::TAG => Self::BeAdj(BeAdj { page_id, page_title, language, section, macro_text }),
            BeAdv::TAG => Self::BeAdv(BeAdv { page_id, page_title, language, section, macro_text }),
            BeIpa::TAG => Self::BeIpa(BeIpa { page_id, page_title, language, section, macro_text }),
            BeNDecl::TAG => Self::BeNDecl(BeNDecl { page_id, page_title, language, section, macro_text }),
            BeNoun::TAG => Self::BeNoun(BeNoun { page_id, page_title, language, section, macro_text }),
            BeVerb::TAG => Self::BeVerb(BeVerb { page_id, page_title, language, section, macro_text }),
            // Russian
            RuAdj::TAG => Self::RuAdj(RuAdj { page_id, page_title, language, section, macro_text }),
            RuConj::TAG => Self::RuConj(RuConj { page_id, page_title, language, section, macro_text }),
            RuDeclAdj::TAG => Self::RuDeclAdj(RuDeclAdj { page_id, page_title, language, section, macro_text }),
            RuIpa::TAG => Self::RuIpa(RuIpa { page_id, page_title, language, section, macro_text }),
            RuNounPlus::TAG => Self::RuNounPlus(RuNounPlus { page_id, page_title, language, section, macro_text }),
            RuVerb::TAG => Self::RuVerb(RuVerb { page_id, page_title, language, section, macro_text }),
            // Ukrainian
            UkADecl::TAG => Self::UkADecl(UkADecl { page_id, page_title, language, section, macro_text }),
            UkAdj::TAG => Self::UkAdj(UkAdj { page_id, page_title, language, section, macro_text }),
            UkIpa::TAG => Self::UkIpa(UkIpa { page_id, page_title, language, section, macro_text }),
            UkNDecl::TAG => Self::UkNDecl(UkNDecl { page_id, page_title, language, section, macro_text }),
            UkNoun::TAG => Self::UkNoun(UkNoun { page_id, page_title, language, section, macro_text }),
            UkVerb::TAG => Self::UkVerb(UkVerb { page_id, page_title, language, section, macro_text }),
            UkConj::TAG => Self::UkConj(UkConj { page_id, page_title, language, section, macro_text }),

            tag => return Err(format!("Unimplemented macro encountered!\n{tag}")),
        };

        Ok(new_macro)
    }

    fn find_macros_in(text: &str) -> Vec<String> {
        let mut macros = Vec::new();
        let mut stack = Vec::new();
        let mut macro_start: Option<usize> = None;
        let mut chars = text.chars().enumerate();

        while let Some((index, c)) = chars.next() {
            match c {
                '{' => {
                    stack.push(index);
                    if stack.len() == 2 {
                        macro_start = Some(index - 1);
                    }
                },
                '}' => {
                    stack.pop();
                    if stack.len() == 1 && macro_start.is_some() {
                        let start = macro_start.unwrap();
                        let end = index + 2;
                        let macro_string = text.chars().skip(start).take(end - start).collect::<String>();
                        if macro_string.starts_with("{{") && macro_string.ends_with("}}") {
                            macros.push(macro_string);
                        }
                        macro_start = None;
                    }
                },
                _ => (),
            }
        }

        macros
    }

}