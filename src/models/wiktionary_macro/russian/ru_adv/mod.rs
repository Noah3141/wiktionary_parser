use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

/// # Russian Adverb
/// ## The adverb
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuAdv {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

pub mod lemma;
pub mod entry;
// pub mod expand;

impl RuAdv {
    pub const TAG: &'static str = "ru-adv";
}