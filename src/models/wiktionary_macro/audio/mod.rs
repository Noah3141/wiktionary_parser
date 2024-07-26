
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

use crate::models::section_header::SectionHeader;

/// # Audio Macro 
/// ## 
#[derive(Debug, Serialize, Deserialize)]
pub struct Audio {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl Audio {
    pub const TAG: &'static str = "audio";
}