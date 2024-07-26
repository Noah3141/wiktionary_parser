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