use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

/// # Ukrainian Adverb
/// ## The adverb
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UkAdv {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl UkAdv {
    pub const TAG: &'static str = "uk-adv";
}