use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

/// 
/// # Russian Conjugation Table 
/// ## Expands into the conjugation forms of a Russian verb
/// 
/// 1)
#[derive(Debug, Serialize, Deserialize)]
pub struct RuConj {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl RuConj {
    pub const TAG: &'static str = "ru-conj";
}