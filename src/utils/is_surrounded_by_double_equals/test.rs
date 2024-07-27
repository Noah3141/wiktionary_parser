use super::*;

#[test]
fn true_positive() {
    assert!(is_surrounded_by_double_equals("==GKLjsl\0fkj фдлывао;sldkfjad osfijao fijasdpfoadfj=="))
}


#[test]
fn true_positive_despite_newline() {
    assert!(is_surrounded_by_double_equals("\n\n==afaasdfak==  "))
}

#[test]
fn true_negative_due_to_triple() {
    assert!(!is_surrounded_by_double_equals("===False==="))
}

#[test]
fn true_negative_due_to_many() {
    assert!(!is_surrounded_by_double_equals("==False====="))
}

#[test]
fn true_negative_due_to_gaps() {
    assert!(!is_surrounded_by_double_equals("= =False= ="))
}

#[test]
fn true_negative_due_to_newline() {
    assert!(!is_surrounded_by_double_equals("==False\n=="))
}

#[test]
fn true_negative_due_to_empty_inside() {
    assert!(!is_surrounded_by_double_equals("===="))
}

