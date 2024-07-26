use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

///
/// # Affix Macro
/// ## This template shows the parts (morphemes) that make up a word, for use in etymology sections.
/// 
#[derive(Serialize, Deserialize)]
pub struct Affix {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl Affix {
    pub const TAG: &'static str = "af";
}