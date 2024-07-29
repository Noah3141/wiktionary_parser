use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

///
/// # Inflection of Macro
/// ## Denotes a 
/// 
#[derive(Serialize, Deserialize)]
pub struct InflectionOf {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
    
}

impl InflectionOf {
    pub const TAG1: &'static str = "infl of";
    pub const TAG2: &'static str = "inflection of";
}
