use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

/// 
/// # Ukrainain Conjugation Table 
/// ## Expands into the conjugation forms of a Russian verb
/// 
/// 1)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UkADecl {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

pub mod lemma;
pub mod expand;
pub mod entry;

impl UkADecl {
    pub const TAG: &'static str = "uk-adecl";
}


/// CSS selectors for forms
pub mod class_selectors {
    pub const ALL: [&str; 30] = [LOC_M,LOC_F,LOC_N,LOC_P,INSTR_M,INSTR_F,INSTR_N,INSTR_P,AN_ACC_M,INAN_ACC_M,ACC_F,ACC_N,IN_ACC_P,INAN_ACC_P,DAT_M,DAT_F,DAT_N,DAT_P,GEN_M,GEN_F,GEN_N,GEN_P,NOM_M,NOM_F,NOM_N,NOM_P,SHORT_M,SHORT_F,SHORT_N,SHORT_P,];
    //
    pub const LOC_M: &'static str = ".loc\\|m\\/\\/n\\|s-form-of";
    pub const LOC_F: &'static str = ".loc\\|f\\|s-form-of";
    pub const LOC_N: &'static str = ".loc\\|m\\/\\/n\\|s-form-of";
    pub const LOC_P: &'static str = ".loc\\|p-form-of";
    //
    pub const INSTR_M: &'static str = ".ins\\|m\\/\\/n\\|s-form-of";
    pub const INSTR_F: &'static str = ".ins\\|f\\|s-form-of";
    pub const INSTR_N: &'static str = ".ins\\|m\\/\\/n\\|s-form-of";
    pub const INSTR_P: &'static str = ".ins\\|p-form-of";
    //
    pub const AN_ACC_M: &'static str = ".an\\|acc\\|m\\|s-form-of";
    pub const INAN_ACC_M: &'static str = ".in\\|acc\\|m\\|s-form-of";
    pub const ACC_F: &'static str = ".acc\\|f\\|s-form-of";
    pub const ACC_N: &'static str = ".acc\\|n\\|s-form-of";
    pub const IN_ACC_P: &'static str = ".in\\|acc\\|p-form-of";
    pub const INAN_ACC_P: &'static str = ".an\\|acc\\|p-form-of";
    //
    pub const DAT_M: &'static str = ".dat\\|m\\/\\/n\\|s-form-of";
    pub const DAT_F: &'static str = ".dat\\|f\\|s-form-of";
    pub const DAT_N: &'static str = ".dat\\|m\\/\\/n\\|s-form-of";
    pub const DAT_P: &'static str = ".dat\\|p-form-of";
    //
    pub const GEN_M: &'static str = ".gen\\|m\\/\\/n\\|s-form-of";
    pub const GEN_F: &'static str = ".gen\\|f\\|s-form-of";
    pub const GEN_N: &'static str = ".gen\\|m\\/\\/n\\|s-form-of";
    pub const GEN_P: &'static str = ".gen\\|p-form-of";
    //
    pub const NOM_M: &'static str = ".nom\\|m\\|s-form-of";
    pub const NOM_F: &'static str = ".nom\\|f\\|s-form-of";
    pub const NOM_N: &'static str = ".nom\\|n\\|s-form-of";
    pub const NOM_P: &'static str = ".nom\\|p-form-of";
    //
    pub const SHORT_M: &'static str = ".short\\|m\\|s-form-of";
    pub const SHORT_F: &'static str = ".short\\|f\\|s-form-of";
    pub const SHORT_N: &'static str = ".short\\|n\\|s-form-of";
    pub const SHORT_P: &'static str = ".short\\|p-form-of";
    
}