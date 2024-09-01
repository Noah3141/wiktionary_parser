use serde::{Deserialize, Serialize};

use crate::models::section_header::SectionHeader;
use crate::models::language::Language;

/// 
/// # Related Terms Macro
/// ## Links the provided term to related terms
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelatedTerms {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl RelatedTerms {
    pub const TAG: &'static str = "l";
}