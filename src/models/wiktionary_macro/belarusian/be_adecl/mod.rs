
use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

/// # Belarusian Adjective Declension Macro
/// ##
#[derive(Debug, Serialize, Deserialize)]
pub struct BeADecl {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl BeADecl {
    pub const TAG: &'static str = "be-adecl";
}