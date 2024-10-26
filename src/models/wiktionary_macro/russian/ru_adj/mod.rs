use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

/// # Russian Adjective Macro
/// ## The main macro for Russian adjectives
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuAdj {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

pub mod lemma;

impl RuAdj {
    pub const TAG: &'static str = "ru-adj";
}