use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

///
/// # Ukrainian IPA
/// ## Generates Ukrainian IPA of word
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct UkIpa {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl UkIpa {
    pub const TAG: &'static str = "uk-IPA";
}
