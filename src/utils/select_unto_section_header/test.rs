use super::*;
use super::super::select_unto_language_header::select_unto_language_header;

#[test]
fn real_example() {
    let text = String::from("===================================================
<page>
    <title>газе</title>
    <ns>0</ns>
    <id>5171528</id>
    <revision>
      <id>80322024</id>
      <parentid>71908908</parentid>
      <timestamp>2024-06-18T05:31:45Z</timestamp>
      <contributor>
        <username>WingerBot</username>
        <id>2024159</id>
      </contributor>
      <minor />
      \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0
      <origin>80322024</origin>
      <model>wikitext</model>
      <format>text/x-wiki</format>
      <text bytes=\"372\" sha1=\"adyq8rtw0ihpjcwdrf6ptminxn7os4v\" xml:space=\"preserve\">{{also|ГАЗе}}
==Macedonian==

===Pronunciation===
* {{mk-IPA}}

===Noun===
{{mk-noun|n|газиња|dim=газенце}}

# {{diminutive of|mk|газ}}

====Declension====
{{mk-decl-noun-n-е-иња|газ}}

==Russian==

===Pronunciation===
* {{ru-IPA|га́зе|pos=pre}}

===Noun===
{{head|ru|noun form|head=га́зе|g=m-in}}

# {{inflection of|ru|газ||pre|s}}</text>
      <sha1>adyq8rtw0ihpjcwdrf6ptminxn7os4v</sha1>
    </revision>
  </page>
===================================================
      ");
    let result = select_unto_section_header(&text, "===Pronunciation===").unwrap();
    assert!(result.contains("* {{mk-IPA}}"));
    assert!(!result.contains("Noun"));
}


#[test]
fn real_example_2() {
    let text = String::from("{{also|зѫба}}
==Belarusian==

===Pronunciation===
* {{be-IPA|зу́ба}}

===Noun===
{{head|be|noun form|head=зу́ба|g=m-in}}

# {{inflection of|be|зуб||gen|s}}

==Russian==

===Pronunciation===
* {{ru-IPA|зу́ба}}

===Noun===
{{head|ru|noun form|head=зу́ба|g=m-in}}

# {{inflection of|ru|зу́б||gen|s}}

==Serbo-Croatian==

===Pronunciation 1===
{{rfc-pron-n|sh|Pronunciation 1}}
{{rfp|sh}}

====Noun====
{{sh-noun-form}}

# {{inflection of|sh|зу̑б||gen|s}}

===Pronunciation 2===
* {{IPA|sh|/zǔːbaː/}}

====Noun====
{{sh-noun-form|head=зу́ба̄}}

# {{inflection of|sh|зу̑б||gen|p}}

==Ukrainian==

===Pronunciation===
* {{uk-IPA|зу́ба}}

===Noun===
{{head|uk|noun form|head=зу́ба|g=m-in}}

# {{inflection of|uk|зуб||gen|s}}</text>
      <sha1>svsgpjh2rect1yyvy5p0nb3e59cduj5</sha1>
    </revision>
  </page>");
    let russian = select_unto_language_header(&text, "==Russian==").unwrap();
    let russian_pronunciation = select_unto_section_header(russian, "Pronunciation").unwrap();
    
    assert!(russian_pronunciation.contains("* {{ru-IPA|зу́ба}}"));
    assert!(!russian_pronunciation.contains("{{head|ru|noun form|head=зу́ба|g=m-in}}"));
    
    let ukrainian = select_unto_language_header(&text, "==Ukrainian==").unwrap();
    let ukrainian_pronunciation = select_unto_section_header(ukrainian, "Noun").unwrap();

    assert!(ukrainian_pronunciation.contains("{{head|uk|noun form|head=зу́ба|g=m-in}}"));
    assert!(!ukrainian_pronunciation.contains("* {{uk-IPA|зу́ба}}"));
    assert!(!ukrainian_pronunciation.contains("{{head|be|noun form|head=зу́ба|g=m-in}}"));
    
}