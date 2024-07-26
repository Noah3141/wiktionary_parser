
use serde::{Deserialize, Serialize};

use crate::models::section_header::SectionHeader;
use crate::models::language::Language;

/// # Head Macro 
/// `head`
/// ## Denotes
/// [1]: language
/// [2]: 
#[derive(Debug, Serialize, Deserialize)]
pub struct Head {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

impl Head {
    pub const TAG: &'static str = "head";
}