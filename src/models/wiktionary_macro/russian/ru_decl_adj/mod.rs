use serde::{Deserialize, Serialize};
use crate::models::section_header::SectionHeader;
use crate::models::language::Language;

///
/// # Russian Adjective Declension Table
/// ## Provides the declension taable for a Russian adjective
#[derive(Debug, Serialize, Deserialize)]
pub struct RuDeclAdj {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl RuDeclAdj {
    pub const TAG: &'static str = "ru-decl-adj";
}