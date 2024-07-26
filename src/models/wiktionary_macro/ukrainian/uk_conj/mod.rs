use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

///
/// # Ukrainian Conjugation Table 
/// ## 
/// 
#[derive(Serialize, Deserialize)]
pub struct UkConj {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl UkConj {
    pub const TAG: &'static str = "uk-conj";
}
