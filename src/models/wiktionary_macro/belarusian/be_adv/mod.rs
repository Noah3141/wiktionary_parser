
use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

/// # Belarusian Adverb Macro
/// ##
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BeAdv {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl BeAdv {
    pub const TAG: &'static str = "be-adv";
}