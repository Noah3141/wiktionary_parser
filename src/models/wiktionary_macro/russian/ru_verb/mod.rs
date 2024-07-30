use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

///
/// # Russian Verb
/// ## The main macro for a Russian verb
#[derive(Debug, Serialize, Deserialize)]
pub struct RuVerb {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl RuVerb {
    pub const TAG: &'static str = "ru-verb";
}