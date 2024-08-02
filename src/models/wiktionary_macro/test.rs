use core::panic;

use crate::models::{language::Language, section_header::SectionHeader, wiktionary_macro::{ipa::Ipa, RuIpa}};

use super::WiktionaryMacro;


#[test]
fn catches_ipa() {
    let page = "<page>
    <title>Испания</title>
    <ns>0</ns>
    <id>197902</id>
    <revision>
      <id>79516802</id>
      <parentid>76869725</parentid>
      <timestamp>2024-06-02T08:05:39Z</timestamp>
      <contributor>
        <username>WingerBot</username>
        <id>2024159</id>
      </contributor>
      <minor />
      <origin>79516802</origin>
      <model>wikitext</model>
      <format>text/x-wiki</format>
      <text bytes=\"1401\" sha1=\"3b2ox12x2eroqett7eu3vvkasidgo27\" xml:space=\"preserve\">{{also|Испанія|Іспанія}}
==Bulgarian==
{{wikipedia|lang=bg}}
[[File:Escudo de España ajunstado a la norma heráldica.svg|thumb|Испания]]

===Pronunciation===
* {{bg-IPA|Испа́ния}}
* {{audio|bg|LL-Q7918 (bul)-Kiril kovachev-Испания.wav}}

===Proper noun===
{{bg-proper noun|Испа́ния|f|adj=испа́нски}}

# {{tcl|bg|Spain|id=Q29}}

====Inflection====
{{bg-ndecl|Испа́ния&lt;(v)/n:sg&gt;}}

===See also===
* {{list:countries of Europe/bg}}

==Kazakh==
{{kk-regional|Испания|يسپانيا}}
{{wikipedia|lang=kk}}

===Pronunciation===
* {{IPA|kk|[ɪˈspanʲɪjə]}}

===Proper noun===
{{kk-proper noun}}

# {{tcl|kk|Spain|id=Q29}}

==Russian==
{{wikipedia|lang=ru}}

===Alternative forms===
* {{alter|ru|Испа́нія||ru-PRO}}
* {{alter|ru|Гишпа́ния|q=dated}}
* {{alter|ru|Гишпа́нія|q=dated||ru-PRO}}

===Pronunciation===
* {{IPA|ru|[ɪˈspanʲɪjɐ]}}
* {{audio|ru|Ru-Испания.ogg}}
* {{rhymes|ru|anʲɪja|s=4}}

===Proper noun===
{{ru-proper noun+|Испа́ния|adj=испа́нский}}

# {{tcl|ru|Spain|id=Q29}}

====Declension====
{{ru-noun-table|Испа́ния|n=sg}}

====Related terms====
* {{l|ru|испа́нец}}, {{l|ru|испа́нка}}
* {{l|ru|испа́нский}}

====Descendants====
* {{desc|hy|Իսպանիա|bor=1}}

===See also===
* {{list:countries of Europe/ru}}

{{cln|ru|exonyms}}</text>
      <sha1>3b2ox12x2eroqett7eu3vvkasidgo27</sha1>
    </revision>
  </page>";

    let macros = WiktionaryMacro::parse_from_xml(page, &Language::Russian).expect("its right there");
    println!("{:#?}", macros);

// "===Pronunciation===
// * {{IPA|ru|[ɪˈspanʲɪjɐ]}}
// * {{audio|ru|Ru-Испания.ogg}}
// * {{rhymes|ru|anʲɪja|s=4}}"

    let mut found: Option<Ipa> = None;
    for wiki_macro in macros {
        if let WiktionaryMacro::Ipa(m) = wiki_macro { found = Some(m) }
    }
        
    assert!(
        found.unwrap().macro_text == "{{IPA|ru|[ɪˈspanʲɪjɐ]}}".to_string()
    )

}
