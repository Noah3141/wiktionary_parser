use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

///
/// # Ukrainian Verb 
/// ## 
/// 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UkVerb {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl UkVerb {
    pub const TAG: &'static str = "uk-verb";
}
