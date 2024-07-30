use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;


/// # Belarusian Noun Declension Table
/// ##
#[derive(Debug, Serialize, Deserialize)]
pub struct BeNoun {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

pub mod lemma;
pub mod entry;
pub mod expand;

impl BeNoun {
    pub const TAG: &'static str = "be-noun";
}