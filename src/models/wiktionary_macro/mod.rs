use ipa::Ipa;
use reqwest::Request;
use serde::{Deserialize, Serialize};
use affix::Affix;
use label::Label;
use reference::Reference;
use audio::Audio;
use head::Head;
use synonyms::Synonyms;
use inflection_of::InflectionOf;
use related_terms::RelatedTerms;
use participle_of::ParticipleOf;
use belarusian::*;
use russian::*;
use ukrainian::*;
use crate::{constants, utils::{select_from, select_unto_language_header, split_sections}};
use super::{language::Language, section_header::SectionHeader};

pub mod inflection_of;
pub mod related_terms;
pub mod head;
pub mod affix;
pub mod audio;
pub mod label;
pub mod synonyms;
pub mod reference;
pub mod participle_of;
pub mod ipa;
//
pub mod russian;
pub mod belarusian;
pub mod ukrainian;

#[cfg(test)]
mod test;

#[derive(Serialize, Deserialize, Debug)]
pub enum WiktionaryMacro {
    InflectionOf(InflectionOf),
    Label(Label),
    RelatedTerms(RelatedTerms),
    Head(Head),
    ParticipleOf(ParticipleOf),
    Audio(Audio),
    Affix(Affix),
    Synonyms(Synonyms),
    Reference(Reference),
    Ipa(Ipa),
    // Belarusian
    BeADecl(BeADecl),
    BeConj(BeConj),
    BeAdj(BeAdj),
    BeAdv(BeAdv),
    BeIpa(BeIpa),
    BeNDecl(BeNDecl),
    BeNoun(BeNoun),
    BeVerb(BeVerb),
    // Russian
    RuAdj(RuAdj),
    RuConj(RuConj),
    RuAdv(RuAdv),
    RuDeclAdj(RuDeclAdj),
    RuIpa(RuIpa),
    RuNounOld(RuNounOld),
    RuNounPlus(RuNounPlus),
    RuNounTable(RuNounTable),
    RuVerb(RuVerb),
    // Ukrainian
    UkADecl(UkADecl),
    UkAdj(UkAdj),
    UkIpa(UkIpa),
    UkAdv(UkAdv),
    UkNDecl(UkNDecl),
    UkNoun(UkNoun),
    UkVerb(UkVerb),
    UkConj(UkConj),
}


impl WiktionaryMacro {
    /// Takes in <page></page> as `page_xml`
    /// `language` determines which language header to process in the provided text. Others are ignored.
    pub fn parse_from_xml(page_xml: &str, language: &Language) -> Result<Vec<Self>, String> {
        let mut wiki_macros: Vec<Self> = Vec::with_capacity(30_000);
        let page_id = u64::from_str_radix(
            select_from(page_xml, "<id>", "</id>").expect("presence of page id"),
            10
        ).expect("radix");
        let page_title = select_from(page_xml, "<title>", "</title>").expect("page title").to_string();
        if  page_title.starts_with("Wiktionary:") || 
            page_title.starts_with("User:") ||
            page_title.starts_with("Module:") ||
            page_title.starts_with("Template:") ||
            page_title.starts_with("Appendix:")
            { return Err("This page is not a contentful page, and likely has headers beyond the usual".to_string()) }
        if page_xml.contains(language.as_header()) {
            let language_section = select_unto_language_header(page_xml, language.as_header()).expect("successful language section extraction");
            let sections = split_sections(language_section);
            for (section, section_text) in sections {
                let section_wiki_macros = WiktionaryMacro::find_macros_in(section_text);
                for macro_text in section_wiki_macros {
                    match WiktionaryMacro::new(
                        page_id, 
                        page_title.clone(), 
                        *language, 
                        section, 
                        macro_text
                    ) {
                        Ok(m) => wiki_macros.push(m),
                        Err(e) => println!("\n{page_id} - {e}\n"),
                    };
                }
            }
        }
        Ok(wiki_macros)
    }


    /// Takes macro text and determines which macro is contained
    pub fn new(
        page_id: u64,
        page_title: String, 
        language: Language, 
        section: SectionHeader,
        macro_text: String, 
    ) -> Result<Self, String> {
        if !macro_text.starts_with("{{") { return Err(format!("Expected the provided macro_text to start with brackets!, received: {macro_text:#?}"))}

        let mut macro_name = select_from(&macro_text, "{{", "|").expect("presence of macro start in macro_text");
        macro_name = macro_name.strip_suffix("}}").unwrap_or(macro_name);

        if macro_name.starts_with(Reference::TAG_INITIAL) {
            return Ok(Self::Reference(Reference { page_id, page_title, language, section, macro_text }))
        }

        let new_macro = match macro_name {
            InflectionOf::TAG1 | 
            InflectionOf::TAG2 => Self::InflectionOf(InflectionOf { page_id, page_title, language, section, macro_text }),
            RelatedTerms::TAG => Self::RelatedTerms(RelatedTerms { page_id, page_title, language, section, macro_text }),
            Head::TAG => Self::Head(Head { page_id, page_title, language, section, macro_text }),
            Audio::TAG => Self::Audio(Audio { page_id, page_title, language, section, macro_text }),
            Affix::TAG => Self::Affix(Affix { page_id, page_title, language, section, macro_text }),
            Label::TAG => Self::Label(Label { page_id, page_title, language, section, macro_text }),
            ParticipleOf::TAG => Self::ParticipleOf(ParticipleOf  { page_id, page_title, language, section, macro_text }),
            Ipa::TAG => Self::Ipa(Ipa { page_id, page_title, language, section, macro_text }),
            // Belarusian
            BeADecl::TAG => Self::BeADecl(BeADecl { page_id, page_title, language, section, macro_text }),
            BeAdj::TAG => Self::BeAdj(BeAdj { page_id, page_title, language, section, macro_text }),
            BeAdv::TAG => Self::BeAdv(BeAdv { page_id, page_title, language, section, macro_text }),
            BeIpa::TAG => Self::BeIpa(BeIpa { page_id, page_title, language, section, macro_text }),
            BeNDecl::TAG => Self::BeNDecl(BeNDecl { page_id, page_title, language, section, macro_text }),
            BeNoun::TAG => Self::BeNoun(BeNoun { page_id, page_title, language, section, macro_text }),
            BeVerb::TAG => Self::BeVerb(BeVerb { page_id, page_title, language, section, macro_text }),
            BeConj::TAG => Self::BeConj(BeConj { page_id, page_title, language, section, macro_text }),
            // Russian
            RuAdj::TAG => Self::RuAdj(RuAdj { page_id, page_title, language, section, macro_text }),
            RuConj::TAG => Self::RuConj(RuConj { page_id, page_title, language, section, macro_text }),
            RuDeclAdj::TAG => Self::RuDeclAdj(RuDeclAdj { page_id, page_title, language, section, macro_text }),
            RuIpa::TAG => Self::RuIpa(RuIpa { page_id, page_title, language, section, macro_text }),
            RuNounPlus::TAG => Self::RuNounPlus(RuNounPlus { page_id, page_title, language, section, macro_text }),
            RuVerb::TAG => Self::RuVerb(RuVerb { page_id, page_title, language, section, macro_text }),
            RuAdv::TAG => Self::RuAdv(RuAdv { page_id, page_title, language, section, macro_text }),
            RuNounOld::TAG => Self::RuNounOld(RuNounOld { page_id, page_title, language, section, macro_text }),
            RuNounTable::TAG => Self::RuNounTable(RuNounTable { page_id, page_title, language, section, macro_text }),
            // Ukrainian
            UkADecl::TAG => Self::UkADecl(UkADecl { page_id, page_title, language, section, macro_text }),
            UkAdj::TAG => Self::UkAdj(UkAdj { page_id, page_title, language, section, macro_text }),
            UkAdv::TAG => Self::UkAdv(UkAdv { page_id, page_title, language, section, macro_text }),
            UkIpa::TAG => Self::UkIpa(UkIpa { page_id, page_title, language, section, macro_text }),
            UkNDecl::TAG => Self::UkNDecl(UkNDecl { page_id, page_title, language, section, macro_text }),
            UkNoun::TAG => Self::UkNoun(UkNoun { page_id, page_title, language, section, macro_text }),
            UkVerb::TAG => Self::UkVerb(UkVerb { page_id, page_title, language, section, macro_text }),
            UkConj::TAG => Self::UkConj(UkConj { page_id, page_title, language, section, macro_text }),

            "rfap" => return Err(format!("`rfap` macro has no significance")),
            tag => return Err(format!("Unimplemented macro encountered!\n{tag:#?}")),
        };

        Ok(new_macro)
    }

    /// Takes a block of text and detects any contained `{{macros}}`, returning the text (with surrounding brackets) of each macro in a Vec
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



pub trait Expand {
    fn page_title(&self) -> &str;
    fn macro_text(&self) -> &str;

    /// Use client to `.expand_with()` and return the HTML of the API's response
    async fn html(&self, client: &reqwest::Client) -> scraper::Html {
        let res = self.expand_with(&client).await;
        let fragment = scraper::Html::parse_fragment(&res);
        fragment
    }

    fn check_head<'text>(&self, html: &scraper::Html, text: &'text str) -> Result<bool, &'text str> {
        let selector = scraper::Selector::parse(".NavHead").unwrap();
        let first_match = html.select(&selector).next().ok_or(text)?;
        let inner_text = first_match.text().collect::<Vec<&str>>().join(" ");
        let found = inner_text.to_string().contains(text);
        Ok(found)
    }

    /// See `.expand_with`, except this launches a new client inefficiently.
    async fn expand(&self) -> String {
        let client = reqwest::Client::new();
        self.expand_with(&client).await
    }

    /// Sends request to Wiktionary.org API to expand the self's macro_text template. Performs simple reversal of HTML escaping (otherwise the Lua on the other side crashes)
    /// API requests require the macro to be expanded in a context of the model/page that the macro was on, so this function pairs with .page_title()
    async fn expand_with(&self, client: &reqwest::Client) -> String {
        let macro_text =  &self.macro_text()
            .replace("&lt;", "<")
            .replace("&gt;", ">");
        let res = client.get(constants::wiki_api::BASE_URL)
            .query(&[
                ("action", "parse"),
                ("format", "json"),
                ("prop", "text|title"),
                ("title", &self.page_title()),
                ("text", macro_text),
            ])
            .send()
            .await
            .expect("successful res");
        
        // Parse the response body as JSON
        let json: serde_json::Value = res.json().await.expect("Failed to parse JSON response");

        let html = json
            .get("parse").unwrap()
            .get("text").unwrap()
            .get("*").unwrap()
            .to_string()
            .replace(r#"\""#, r#"""#)
            .replace(r#"\n"#, "\n");

        html_escape::decode_html_entities(&html).to_string()
    }
}