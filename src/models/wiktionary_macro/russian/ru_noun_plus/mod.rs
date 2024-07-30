use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

/// # Russian Noun Plus
/// ## The main macro for a Russian noun
/// 
/// Follows many of the same parameters as RuNounTable
#[derive(Debug, Serialize, Deserialize)]
pub struct RuNounPlus {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

pub mod lemma;

impl RuNounPlus {
    pub const TAG: &'static str = "ru-noun+";
}

