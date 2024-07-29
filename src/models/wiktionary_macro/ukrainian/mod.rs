pub mod uk_ipa;
pub mod uk_adj;
pub mod uk_adecl;
pub mod uk_ndecl;
pub mod uk_noun;
pub mod uk_verb;
pub mod uk_conj;

pub use {
    uk_adecl::UkADecl,
    uk_adj::UkAdj,
    uk_conj::UkConj,
    uk_ipa::UkIpa,
    uk_ndecl::UkNDecl,
    uk_noun::UkNoun,
    uk_verb::UkVerb
};