use crate::models::language::Language;
use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};

///
/// # Russian Conjugation Table
/// ## Expands into the conjugation forms of a Russian verb
///
/// 1)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuConj {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
}

pub mod entry;
pub mod expand;
pub mod lemma;

impl RuConj {
    pub const TAG: &'static str = "ru-conj";
}

/// CSS selectors for forms
pub mod class_selectors {
    pub const ALL: [&str; 25] = [
        INFINITIVE,
        PAST_ACTIVE_PARTICIPLE,
        PAST_PASSIVE_PARTICIPLE,
        PRESENT_PASSIVE_PARTICIPLE,
        PRESENT_ACTIVE_PARTICIPLE,
        PRESENT_ADV_PARTICIPLE,
        PAST_ADV_PARTICIPLE,
        Я_PRESENT_INDICATIVE,
        ТЫ_PRESENT_INDICATIVE,
        ОН_PRESENT_INDICATIVE,
        МЫ_PRESENT_INDICATIVE,
        ВЫ_PRESENT_INDICATIVE,
        ОНИ_PRESENT_INDICATIVE,
        Я_FUTURE_INDICATIVE,
        ТЫ_FUTURE_INDICATIVE,
        ОН_FUTURE_INDICATIVE,
        МЫ_FUTURE_INDICATIVE,
        ВЫ_FUTURE_INDICATIVE,
        ОНИ_FUTURE_INDICATIVE,
        ТЫ_IMPERATIVE,
        ВЫ_IMPERATIVE,
        MASC_PAST,
        PLUR_PAST,
        FEM_PAST,
        NEUT_PAST,
    ];

    pub const INFINITIVE: &'static str = ".inf-form-of";
    pub const PAST_ACTIVE_PARTICIPLE: &'static str = ".past\\|act\\|part-form-of";
    pub const PAST_PASSIVE_PARTICIPLE: &'static str = ".past\\|pass\\|part-form-of";
    pub const PRESENT_PASSIVE_PARTICIPLE: &'static str = ".pres\\|pass\\|part-form-of";
    pub const PRESENT_ACTIVE_PARTICIPLE: &'static str = ".pres\\|act\\|part-form-of";
    pub const PRESENT_ADV_PARTICIPLE: &'static str = ".pres\\|adv\\|part-form-of";
    pub const PAST_ADV_PARTICIPLE: &'static str = ".past\\|adv\\|part-form-of";
    //
    pub const Я_PRESENT_INDICATIVE: &'static str = ".\\31\\|s\\|pres\\|ind-form-of"; // These are CSS selector escaped. Initial digits have to be escaped w unicode code point
    pub const ТЫ_PRESENT_INDICATIVE: &'static str = ".\\32\\|s\\|pres\\|ind-form-of";
    pub const ОН_PRESENT_INDICATIVE: &'static str = ".\\33\\|s\\|pres\\|ind-form-of";
    pub const МЫ_PRESENT_INDICATIVE: &'static str = ".\\31\\|p\\|pres\\|ind-form-of";
    pub const ВЫ_PRESENT_INDICATIVE: &'static str = ".\\32\\|p\\|pres\\|ind-form-of";
    pub const ОНИ_PRESENT_INDICATIVE: &'static str = ".\\33\\|p\\|pres\\|ind-form-of";
    //
    pub const Я_FUTURE_INDICATIVE: &'static str = ".\\31\\|s\\|fut\\|ind-form-of";
    pub const ТЫ_FUTURE_INDICATIVE: &'static str = ".\\32\\|s\\|fut\\|ind-form-of";
    pub const ОН_FUTURE_INDICATIVE: &'static str = ".\\33\\|s\\|fut\\|ind-form-of";
    pub const МЫ_FUTURE_INDICATIVE: &'static str = ".\\31\\|p\\|fut\\|ind-form-of";
    pub const ВЫ_FUTURE_INDICATIVE: &'static str = ".\\32\\|p\\|fut\\|ind-form-of";
    pub const ОНИ_FUTURE_INDICATIVE: &'static str = ".\\33\\|p\\|fut\\|ind-form-of";
    //
    pub const ТЫ_IMPERATIVE: &'static str = ".\\32\\|s\\|imp-form-of";
    pub const ВЫ_IMPERATIVE: &'static str = ".\\32\\|p\\|imp-form-of";
    //
    pub const MASC_PAST: &'static str = ".m\\|s\\|past\\|ind-form-of";
    pub const FEM_PAST: &'static str = ".f\\|s\\|past\\|ind-form-of";
    pub const NEUT_PAST: &'static str = ".n\\|s\\|past\\|ind-form-of";
    pub const PLUR_PAST: &'static str = ".p\\|past\\|ind-form-of";
}

impl RuConj {
    pub fn is_old(&self) -> bool {
        self.macro_text.contains("|old=1")
    }

    pub fn is_impersonal(&self) -> bool {
        self.macro_text.contains("|impf-impers")
    }
}
