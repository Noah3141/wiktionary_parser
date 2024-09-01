use serde::{Deserialize, Serialize};
use crate::models::section_header::SectionHeader;
use crate::models::language::Language;

/// 
/// # Synonyms Macro
/// ## This template shows a line with synonyms. It is intended to be used below each definition, before any usage examples or quotes. 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Synonyms {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl Synonyms {
    pub const TAG: &'static str = "syn"; // 1nth colon section is language param, 2nd is Source (only one of a handful of recurring sources, like "Websters")
}