use serde::{Deserialize, Serialize};

use crate::models::section_header::SectionHeader;
use crate::models::language::Language;

/// 
/// # Related Terms Macro
/// ## Links the provided term to related terms
#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl Reference {
    pub const TAG_INITIAL: &'static str = "R:"; // 1nth colon section is language param, 2nd is Source (only one of a handful of recurring sources, like "Websters")
}