use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

/// Belarusian Adjective Macro
#[derive(Debug, Serialize, Deserialize)]
pub struct BeAdj {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl BeAdj {
    pub const TAG: &'static str = "be-adj";
}