use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

///
/// # Belarusian IPA
/// ## Generates Belarusian IPA of word
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct BeIpa {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl BeIpa {
    pub const TAG: &'static str = "be-IPA";
}