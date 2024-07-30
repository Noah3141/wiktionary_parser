use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

/// # Russian Noun Declension Table
/// ## 
#[derive(Debug, Serialize, Deserialize)]
pub struct RuNounTable {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

pub mod lemma;
pub mod expand;
pub mod entry;

impl RuNounTable {
    pub const TAG: &'static str = "ru-noun-table";
}

/// CSS selectors for forms
pub mod class_selectors {
    pub const ALL: [&str; 12] = [PREP_S,PREP_P,INSTR_S,INSTR_P,ACC_S,ACC_P,DAT_S,DAT_P,GEN_S,GEN_P,NOM_S,NOM_P];
    pub const PREP_S: &'static str = ".pre\\|s-form-of";
    pub const PREP_P: &'static str = ".pre\\|p-form-of";
    pub const INSTR_S: &'static str = ".ins\\|s-form-of";
    pub const INSTR_P: &'static str = ".ins\\|p-form-of";
    pub const ACC_S: &'static str = ".acc\\|s-form-of";
    pub const ACC_P: &'static str = ".acc\\|p-form-of";
    pub const DAT_S: &'static str = ".dat\\|s-form-of";
    pub const DAT_P: &'static str = ".dat\\|p-form-of";
    pub const GEN_S: &'static str = ".gen\\|s-form-of";
    pub const GEN_P: &'static str = ".gen\\|p-form-of";
    pub const NOM_S: &'static str = ".nom\\|s-form-of";
    pub const NOM_P: &'static str = ".nom\\|p-form-of";
}

impl RuNounTable {
    pub fn is_old(&self) -> bool {
        self.macro_text.contains("|old=1")
    }
}