use crate::utils::select_unto_language_header::select_unto_language_header;
use super::*;


#[test]
fn general() {
    let page = "===================================================
<page>
    <title>чиновник</title>
    <ns>0</ns>
    <id>1699107</id>
    <revision>
      <id>79826638</id>
      <parentid>76088000</parentid>
      <timestamp>2024-06-02T16:36:15Z</timestamp>
      <contributor>
        <username>WingerBot</username>
        <id>2024159</id>
      </contributor>
      <minor />
      \0\0\0\0\0\0\0
        <origin>79826638</origin>
      <model>wikitext</model>
      <format>text/x-wiki</format>
      <text bytes=\"2637\" sha1=\"om5bwhxhzxn132w4po4sv2u6hobn7hg\" xml:space=\"preserve\">==Bulgarian==

===Noun===
{{bg-noun|чино́вник|m|f=чино́вница}}

# [[official]], [[functionary]]

====Declension====
{{bg-ndecl|чино́вник&lt;(h)&gt;}}

{{topics|bg|Occupations}}

==Macedonian==

===Pronunciation===
* {{mk-IPA}}

===Noun===
{{mk-noun|m|чиновници|f=чиновничка|adj=чиновнички|dim=чиновниче}}

# [[official]], [[functionary]], [[bureaucrat]]

====Declension====
{{mk-decl-noun-m||чиновниц|p=1}}

{{C|mk|Occupations}}

==Russian==

===Pronunciation===
* {{ru-IPA|чино́вник}}
* {{audio|ru|Ru-чиновник.ogg}}

===Noun===
{{ru-noun+|чино́вник|a=an|f=чино́вница|adj=чино́вничий}}

# [[official]], [[functionary]], [[bureaucrat]]
#: {{uxi|ru|[[высокопоста́вленный]] '''чино́вник'''|'''a''' high-ranking '''official'''}}

====Declension====
{{ru-noun-table|чино́вник|a=an}}

====Related terms====
* {{l|ru|чин|g=m}}
* {{l|ru|чино́вничество|g=n}}

====Descendants====
* {{desc|hy|չինովնիկ|bor=1}}
* {{desc|en|chinovnik|bor=1}}
* {{desc|hu|csinovnyik|bor=1}}
* {{desc|izh|cinovnikka|bor=1}}
* {{desc|lt|valdininkas|clq=1}}
* {{desc|ro|cinovnic|bor=1}}
* {{desc|sh|činovnik|bor=1}}

===Noun===
{{ru-noun+|чино́вник}}

# {{lb|ru|dated|religion}} book on episcopal missals

====Declension====
{{ru-noun-table|чино́вник}}

{{topics|ru|Occupations}}

==Serbo-Croatian==

===Etymology===
{{bor+|sh|ru|чино́вникъ}} (modern spelling {{m|ru|чино́вник}}).

===Pronunciation===
* {{sh-IPA|чино̀внӣк}}
* {{hyphenation|sh|чин|ов|ник}}

===Noun===
{{sh-noun|g=m|head=чино̀внӣк|f=чино̀вница}}

# [[clerk]]

====Declension====
{{sh-decl-noun
|чино̀внӣк|чиновни́ци
|чиновни́ка|чиновни́ка̄
|чиновнику|чиновницима
|чиновника|чиновнике
|чиновниче|чиновници
|чиновнику|чиновницима
|чиновником|чиновницима
}}

{{C|sh|Occupations}}

==Ukrainian==

===Pronunciation===
* {{uk-IPA|чино́вник}}
* {{audio|uk|Uk-чиновник.ogg}}

===Noun===
{{uk-noun|чино́вник&lt;pr&gt;|f=чино́вниця|adj=чино́вницький}}

# [[official]], [[functionary]], [[bureaucrat]]
#: {{syn|uk|урядо́вець}}

====Declension====
{{uk-ndecl|чино́вник&lt;pr&gt;}}

====Derived terms====
* {{l|uk|чино́вництво|g=n}}

===Further reading===
* {{R:uk:SUM-11}}
* {{R:uk:Horokh}}
* {{R:uk:Kyiv}}
* {{R:uk:Slovnyk}}

{{C|uk|Male people|Occupations}}</text>
      <sha1>om5bwhxhzxn132w4po4sv2u6hobn7hg</sha1>
    </revision>
  </page>
===================================================
";

    let russian = select_unto_language_header(page, "==Russian==").expect("selecting unto language");
    let sections = split_section(russian);

    let pronunciation_section = &sections[0];
    let noun_section_1 = &sections[1];
    let noun_section_2 = &sections[2];

    assert!(sections[0].0 == SectionHeader::Pronunciation);

    assert!(pronunciation_section.1.contains("* {{ru-IPA|чино́вник}}"));
    assert!(!pronunciation_section.1.contains("{{ru-noun+|чино́вник|a=an|f=чино́вница|adj=чино́вничий}}"));
    
    assert!(!noun_section_1.1.contains("* {{ru-IPA|чино́вник}}"));
    assert!(noun_section_1.1.contains("{{ru-noun+|чино́вник|a=an|f=чино́вница|adj=чино́вничий}}"));

    assert!(!noun_section_2.1.contains("{{ru-noun+|чино́вник|a=an|f=чино́вница|adj=чино́вничий}}"));

    assert!(noun_section_1.1.contains("====Related terms===="));
    assert!(!noun_section_2.1.contains("====Related terms===="));

}