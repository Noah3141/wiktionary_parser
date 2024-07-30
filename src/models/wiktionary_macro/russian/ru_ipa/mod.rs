use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

///
/// # Russian IPA
/// ## Generates Russian IPA of word
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct RuIpa {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl RuIpa {
    pub const TAG: &'static str = "ru-IPA";
}