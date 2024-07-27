

use super::*;


#[test]
fn detected_arbitrary_length_delimiters() {
    let text = "

==Foo==

===Bar===

s
as

===Biz===

asdass

===Beetle===

===Cow===

====Baz====

asd

==Bock==

asdasd

====

    ";


    let language_sections = split_at_delimiters_bracketed_by_n_equals(text, 2);
    assert!(language_sections.len() == 3);

    let language_sections = split_at_delimiters_bracketed_by_n_equals(&language_sections[1], 3);
    assert!(language_sections.len() == 5)

}