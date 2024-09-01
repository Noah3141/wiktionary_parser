use crate::models::section_header::SectionHeader;
use serde::{Deserialize, Serialize};
use crate::models::language::Language;

///
/// # Inflection of Macro
/// ## This template is used to create definition lines for inflected (non-lemma) forms
/// 
/// - |1= (required) The language code of the lemma, of which this term is an inflection. See Wiktionary:List of languages. The parameter |lang= is a deprecated synonym; please do not use. If this is used, all numbered parameters move down by one.
/// - |2= (required)  The lemma of which this term is an inflection. A link is created to the lemma, similar to links created using {{l}} and {{m}} (but normally displayed in upright boldface). To specify more than one lemma, separate them by commas. Inline modifiers can also be attached to individual lemmas to specify transliteration, qualifiers, etc. See the section below on multiple lemmas and inline modifiers.
/// - |3= or |alt= The alternative display form of the lemma. This works like the third parameter of {{l}} and {{m}}.
/// - |4= (required), |5= ... etc.  One or more grammar tags to show. These give the definition by describing the relevant grammatical properties of this inflected form. A grammar tag can potentially be any text, but certain tags such as nominative, feminine, first-person or subjunctive that are recognized internally will automatically be linked to the appropriate entry in Appendix:Glossary (or in some cases, to the relevant Wiktionary or Wikipedia entry). As an example, nominative is displayed as nominative, with an appropriate link. Certain tags are recognised as shortcuts and are equivalent to spelling out the tag. For example, 1 is equivalent to first-person; both will be displayed as first-person. Similarly, f is equivalent to feminine, and nom is equivalent to nominative. The full, up-to-date list of recognized tags and their shortcuts and display forms is specified below.
///     Multiple tags are normally separated by spaces, so that e.g. nom|f|s will be displayed as nominative feminine singular. However, when punctuation characters are used as tags, they will be displayed appropriately for that punctuation character. For example, nom|,|with|3|s|object will display as nominative, with third-person singular object (i.e. without a space preceding the comma). Among the punctuation characters recognized and handled correctly are comma, colon, parens, brackets, slash, and hyphen. The full list can be found below.
///     It is also possible to put // separators between one or more tags or shortcuts to create a list separated by slashes. For example, writing nom//acc will expand to nominative/accusative, and writing nom//acc//voc//dat will expand to nominative/accusative/vocative/dative.
///     The inflection tag ; is recognized specially and is used to separate two inflections of the same word. Sets of tags separated by a semicolon tag will be displayed on separate lines. See examples below.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InflectionOf {
    pub page_id: u64,
    pub page_title: String,
    pub language: Language,
    pub section: SectionHeader,
    pub macro_text: String,
    
}

pub mod lemma;

impl InflectionOf {
    pub const TAG1: &'static str = "infl of";
    pub const TAG2: &'static str = "inflection of";
}

  /// There is a long list of potential extra parameters (even positionally, for this tag). Some key ones are:
pub mod params {
    /// Used when a form is multiple different inflections of the same lemma: `{{inflection of|ru|путь||gen//dat//pre|s|;|nom//acc|p}}`
    pub const MULTI_INFLECTION_BOUND: &'static str = ";";
    pub const PERFECTIVE: &'static str = "pfv";
}