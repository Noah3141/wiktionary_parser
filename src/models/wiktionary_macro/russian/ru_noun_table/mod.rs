use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

/// # Russian Noun Declension Table
/// ## 
#[derive(Debug, Serialize, Deserialize)]
pub struct RuNounTable {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl RuNounTable {
    pub const TAG: &'static str = "ru-noun-table";
}