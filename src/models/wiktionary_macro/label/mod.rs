use serde::{Deserialize, Serialize};
use crate::models::section_header::SectionHeader;
use crate::models::language::Language;

/// 
/// # Label Macro
/// ## Labels word senses with extra attributes
/// 
/// 1. To label senses with restricted usage
/// 2. To label senses with grammatical information, in addition to that in the part-of-speech heading and headword line
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Label {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl Label {
    pub const TAG: &'static str = "lb"; 
}