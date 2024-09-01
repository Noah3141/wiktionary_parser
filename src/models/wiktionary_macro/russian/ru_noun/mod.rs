use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

/// # Russian Noun (Not +, the old version)
/// ## The old macro for a Russian noun
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuNounOld {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl RuNounOld {
    pub const TAG: &'static str = "ru-noun";
}