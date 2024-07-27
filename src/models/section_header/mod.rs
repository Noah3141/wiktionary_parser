use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum SectionHeader {
    Pronunciation,

    //
    Noun, // -> ====Declension====
    Verb, // -> ====Conjugation====
    Adjective,
    Participle,
    Adverb,
    Predicative,
    Numeral,
    //
    ProperNoun,
    PrepositionalPhrase,
    Phrase,
    Proverb,
    Preposition,
    Idiom,
    Conjunction,
    Combining,
    Suffix,
    Interfix,
    Prefix,
    Determiner,
    Pronoun,
    Particle,
    Interjection,
    AlternativeForms,
    Etymology,
    References,
}


impl From<&str> for SectionHeader {
    fn from(value: &str) -> Self {
        match value.trim() {
            "===Pronunciation===" => SectionHeader::Pronunciation,
            "===Noun===" => SectionHeader::Noun,
            "===Verb===" => SectionHeader::Verb,
            "===Adjective===" => SectionHeader::Adjective,
            "===Participle===" => SectionHeader::Participle,
            "===Adverb===" => SectionHeader::Adverb,
            "===Predicative===" => SectionHeader::Predicative,
            "===Numeral===" => SectionHeader::Numeral,
            "===Proper noun===" => SectionHeader::ProperNoun,
            "===Prepositional phrase===" => SectionHeader::PrepositionalPhrase,
            "===Phrase===" => SectionHeader::Phrase,
            "===Proverb===" => SectionHeader::Proverb,
            "===Preposition===" => SectionHeader::Preposition,
            "===Idiom===" => SectionHeader::Idiom,
            "===Conjunction===" => SectionHeader::Conjunction,
            "===Combining===" => SectionHeader::Combining,
            "===Suffix===" => SectionHeader::Suffix,
            "===Interfix===" => SectionHeader::Interfix,
            "===Prefix===" => SectionHeader::Prefix,
            "===Determiner===" => SectionHeader::Determiner,
            "===Pronoun===" => SectionHeader::Pronoun,
            "===Particle===" => SectionHeader::Particle,
            "===Interjection===" => SectionHeader::Interjection,
            "===Alternative forms===" => SectionHeader::AlternativeForms,
            "===Etymology===" => SectionHeader::Etymology,
            "===References===" => SectionHeader::References,
            unknown_header => panic!("Unimplemented header encountered! {unknown_header}")
        } 
    }
}