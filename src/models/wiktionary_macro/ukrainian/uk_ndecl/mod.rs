use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

///
/// # Ukrainian Noun Declension Table
/// ## Generates declension table for a Ukrainian noun
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct UkNDecl {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl UkNDecl {
    pub const TAG: &'static str = "uk-ndecl";
}
