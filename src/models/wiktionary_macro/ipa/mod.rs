use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;
use super::Expand;

///
/// # IPA macro
/// ## Generates IPA I guess...
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct Ipa {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
    
}

pub mod expand;

impl Ipa {
    pub const TAG: &'static str = "IPA";
}


impl Ipa {
    pub async fn to_ipa_string(&self, client: &reqwest::Client) -> String {
        let html = self.html(&client).await;
        let selector = scraper::Selector::parse(".IPA").expect("selector parsing");
        let ipa_string = html
            .select(&selector)
            .next()
            .expect("`.IPA` span tag to be present")
            .text()
            .next()
            .expect("Inner text of IPA string");
        ipa_string.to_string()
    }
}