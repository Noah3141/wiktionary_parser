use crate::models::language::Language;
use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};

///
///  Polish Conjugation of an Imperfective Aspect verb
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlConjAi {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

pub mod entry;
pub mod expand;
pub mod lemma;

impl PlConjAi {
    pub const TAG: &'static str = "pl-conj-ai";
}