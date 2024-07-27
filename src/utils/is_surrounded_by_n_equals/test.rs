use super::*;

#[test]
fn true_positive() {
    assert!(is_surrounded_by_n_equals("=====GKLjsl\0fkj фдлывао;sldkfjad osfijao fijasdpfoadfj=====", 5))
}

#[test]
fn true_positive_despite_newline() {
    assert!(is_surrounded_by_n_equals("\n\n======afaasdfak======  ", 6))
}

#[test]
fn true_negative_due_to_only_double() {
    assert!(!is_surrounded_by_n_equals("==False==", 3))
}

#[test]
fn true_negative_due_to_many() {
    assert!(!is_surrounded_by_n_equals("====False====", 1))
}

#[test]
fn true_negative_due_to_gaps() {
    assert!(!is_surrounded_by_n_equals("= =False= =", 2))
}

#[test]
fn true_negative_due_to_gaps_despite_matching_chunks() {
    assert!(!is_surrounded_by_n_equals("= =False= =", 1))
}

#[test]
fn true_negative_due_to_newline() {
    assert!(!is_surrounded_by_n_equals("==False\n==", 2))
}

#[test]
fn true_negative_due_to_empty_inside() {
    assert!(!is_surrounded_by_n_equals("====", 2));
    assert!(!is_surrounded_by_n_equals("====", 3));
    assert!(!is_surrounded_by_n_equals("===== =====", 5));
}