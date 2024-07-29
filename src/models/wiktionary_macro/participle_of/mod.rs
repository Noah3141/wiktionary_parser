use serde::{Deserialize, Serialize};

use crate::models::section_header::SectionHeader;
use crate::models::language::Language;

/// 
/// # Participle of Macro
/// ## Links the provided term to related terms
/// 
/// Usually set below a paired "head" template
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipleOf {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl ParticipleOf {
    pub const TAG: &'static str = "participle of";
}