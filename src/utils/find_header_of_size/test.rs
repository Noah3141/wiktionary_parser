use super::*;

#[test]
fn finds_headers() {

    let text = "
==Foo==
adyq8tminxn7os4v

===Bar===
asdasd
asds

===Biz===

====Baz====

asdasd
    ";

    let foo = find_header_of_size(2, text).unwrap();
    assert!(text[foo..].starts_with("\n==Foo=="));
    let bar = find_header_of_size(3, text).unwrap();
    assert!(text[bar..].starts_with("\n===Bar==="));

    let after_bar = &text[bar + 1..];
    let biz = find_header_of_size(3, after_bar).unwrap();
    assert!(after_bar[biz..].starts_with("\n===Biz==="));


}

#[test]
fn complex_byte_boundaries() {
    let text = "===Etymology 1===
{{root|uk|ine-pro|*bʰeh₂-|id=speak}}
{{inh+|uk|sla-pro|*bajьka}}.

====Noun====
{{uk-noun|ба́йка&lt;*&gt;|adj=ба́єчний}}

# [[fable]], [[fairy-tale]]

=====Declension=====
{{uk-ndecl|ба́йка&lt;*&gt;}}

=====Derive====";

    assert!(
        text[
            find_header_of_size(4, text).unwrap()..
        ].starts_with("\n====Noun")
    )
}