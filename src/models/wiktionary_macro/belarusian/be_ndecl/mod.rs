use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;


/// # Belarusian Noun Declension Table
/// ##
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BeNDecl {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

pub mod lemma;
pub mod entry;
pub mod expand;

impl BeNDecl {
    pub const TAG: &'static str = "be-ndecl";
}

/// CSS selectors for forms
pub mod class_selectors {
    pub const ALL: [&str; 12] = [LOC_S,LOC_P,INS_S,INS_P,ACC_S,ACC_P,DAT_S,DAT_P,GEN_S,GEN_P,NOM_S,NOM_P];
    pub const LOC_S: &'static str = ".loc\\|s-form-of";
    pub const LOC_P: &'static str = ".loc\\|p-form-of";
    pub const INS_S: &'static str = ".ins\\|s-form-of";
    pub const INS_P: &'static str = ".ins\\|p-form-of";
    pub const ACC_S: &'static str = ".acc\\|s-form-of";
    pub const ACC_P: &'static str = ".acc\\|p-form-of";
    pub const DAT_S: &'static str = ".dat\\|s-form-of";
    pub const DAT_P: &'static str = ".dat\\|p-form-of";
    pub const GEN_S: &'static str = ".gen\\|s-form-of";
    pub const GEN_P: &'static str = ".gen\\|p-form-of";
    pub const NOM_S: &'static str = ".nom\\|s-form-of";
    pub const NOM_P: &'static str = ".nom\\|p-form-of";
    pub const VOC_S: &'static str = ".voc\\|s-form-of";

}