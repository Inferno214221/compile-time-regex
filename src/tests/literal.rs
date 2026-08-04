use super::*;

regex! {
    pub LiteralExpr = r"eae"
}

#[test]
fn is_match() {
    assert!(LiteralExpr::is_match("eae"));
    assert!(!LiteralExpr::is_match("aeae"));
    assert!(!LiteralExpr::is_match("eaea"));
    assert!(!LiteralExpr::is_match("eae eae"));
    assert!(!LiteralExpr::is_match("ene"));
    assert!(!LiteralExpr::is_match("ea"));
}

#[test]
fn contains_match() {
    assert!(LiteralExpr::contains_match("eae"));
    assert!(LiteralExpr::contains_match("aeae"));
    assert!(LiteralExpr::contains_match("eaea"));
    assert!(LiteralExpr::contains_match("eae eae"));
    assert!(!LiteralExpr::contains_match("ene"));
    assert!(!LiteralExpr::contains_match("ea"));
}

#[test]
fn count_matches_non_overlapping() {
    assert_eq!(LiteralExpr::count_matches("eae", false), 1);
    assert_eq!(LiteralExpr::count_matches("eae eae", false), 2);
    assert_eq!(LiteralExpr::count_matches("eae eae eae eae", false), 4);
    assert_eq!(LiteralExpr::count_matches("eaeae", false), 1);
    assert_eq!(LiteralExpr::count_matches("eaeaeaeaeae", false), 3);
    assert_eq!(LiteralExpr::count_matches("ea", false), 0);
}

#[test]
fn count_matches_overlapping() {
    assert_eq!(LiteralExpr::count_matches("eae", true), 1);
    assert_eq!(LiteralExpr::count_matches("eae eae", true), 2);
    assert_eq!(LiteralExpr::count_matches("eae eae eae eae", true), 4);
    assert_eq!(LiteralExpr::count_matches("eaeae", true), 2);
    assert_eq!(LiteralExpr::count_matches("eaeaeaeaeae", true), 5);
    assert_eq!(LiteralExpr::count_matches("ea", true), 0);
}

#[test]
fn range_of_match() {
    assert_eq!(LiteralExpr::range_of_match("eae"), Some(0..3));
    assert_eq!(LiteralExpr::range_of_match("aeae"), Some(1..4));
    assert_eq!(LiteralExpr::range_of_match("eaea"), Some(0..3));
    assert_eq!(LiteralExpr::range_of_match("eae eae"), Some(0..3));
    assert_eq!(LiteralExpr::range_of_match("ene"), None);
    assert_eq!(LiteralExpr::range_of_match("ea"), None);
}

#[test]
fn range_of_all_matches_non_overlapping() {
    assert_eq!(LiteralExpr::all_ranges("eae"), vec![0..3]);
    assert_eq!(LiteralExpr::all_ranges("eae eae"), vec![0..3, 4..7]);
    assert_eq!(LiteralExpr::all_ranges("eae eae eae eae"), vec![0..3, 4..7, 8..11, 12..15]);
    assert_eq!(LiteralExpr::all_ranges("eaeae"), vec![0..3]);
    assert_eq!(LiteralExpr::all_ranges("eaeaeaeaeae"), vec![0..3, 4..7, 8..11]);
    assert_eq!(LiteralExpr::all_ranges("ea"), vec![0..1; 0]);
}

#[test]
fn range_of_all_matches_overlapping() {
    assert_eq!(LiteralExpr::all_ranges_overlap("eae"), vec![0..3]);
    assert_eq!(LiteralExpr::all_ranges_overlap("eae eae"), vec![0..3, 4..7]);
    assert_eq!(LiteralExpr::all_ranges_overlap("eae eae eae eae"), vec![0..3, 4..7, 8..11, 12..15]);
    assert_eq!(LiteralExpr::all_ranges_overlap("eaeae"), vec![0..3, 2..5]);
    assert_eq!(LiteralExpr::all_ranges_overlap("eaeaeaeaeae"), vec![0..3, 2..5, 4..7, 6..9, 8..11]);
    assert_eq!(LiteralExpr::all_ranges_overlap("ea"), vec![0..1; 0]);
}

#[test]
fn slice_match() {
    assert_eq!(LiteralExpr::slice_match("eae"), Some("eae"));
    assert_eq!(LiteralExpr::slice_match("aeae"), Some("eae"));
    assert_eq!(LiteralExpr::slice_match("eaea"), Some("eae"));
    assert_eq!(LiteralExpr::slice_match("eae eae"), Some("eae"));
    assert_eq!(LiteralExpr::slice_match("ene"), None);
    assert_eq!(LiteralExpr::slice_match("ea"), None);
}

#[test]
fn slice_all_matches_non_overlapping() {
    assert_eq!(LiteralExpr::all_slices("eae"), vec!["eae"; 1]);
    assert_eq!(LiteralExpr::all_slices("eae eae"), vec!["eae"; 2]);
    assert_eq!(LiteralExpr::all_slices("eae eae eae eae"), vec!["eae"; 4]);
    assert_eq!(LiteralExpr::all_slices("eaeae"), vec!["eae"; 1]);
    assert_eq!(LiteralExpr::all_slices("eaeaeaeaeae"), vec!["eae"; 3]);
    assert_eq!(LiteralExpr::all_slices("ea"), vec!["eae"; 0]);
}

#[test]
fn slice_all_matches_overlapping() {
    assert_eq!(LiteralExpr::all_slices_overlap("eae"), vec!["eae"; 1]);
    assert_eq!(LiteralExpr::all_slices_overlap("eae eae"), vec!["eae"; 2]);
    assert_eq!(LiteralExpr::all_slices_overlap("eae eae eae eae"), vec!["eae"; 4]);
    assert_eq!(LiteralExpr::all_slices_overlap("eaeae"), vec!["eae"; 2]);
    assert_eq!(LiteralExpr::all_slices_overlap("eaeaeaeaeae"), vec!["eae"; 5]);
    assert_eq!(LiteralExpr::all_slices_overlap("ea"), vec!["eae"; 0]);
}

#[test]
fn do_capture() {
    assert_eq!(LiteralExpr::whole_capture("eae"), Some((0..3, "eae")));
    assert_eq!(LiteralExpr::whole_capture("eae eae"), None);
    assert_eq!(LiteralExpr::whole_capture("aeae"), None);
    assert_eq!(LiteralExpr::whole_capture("eaea"), None);
    assert_eq!(LiteralExpr::whole_capture("ene"), None);
    assert_eq!(LiteralExpr::whole_capture("ea"), None);
}

#[test]
fn find_capture() {
    assert_eq!(LiteralExpr::first_capture("eae"), Some((0..3, "eae")));
    assert_eq!(LiteralExpr::first_capture("aeae"), Some((1..4, "eae")));
    assert_eq!(LiteralExpr::first_capture("eaea"), Some((0..3, "eae")));
    assert_eq!(LiteralExpr::first_capture("eae eae"), Some((0..3, "eae")));
    assert_eq!(LiteralExpr::first_capture("ene"), None);
    assert_eq!(LiteralExpr::first_capture("ea"), None);
}

#[allow(clippy::single_range_in_vec_init)]
#[test]
fn find_all_captures_non_overlapping() {
    assert_eq!(LiteralExpr::all_captures("eae"), zip_literal(vec![0..3], "eae"));
    assert_eq!(LiteralExpr::all_captures("eae eae"), zip_literal(vec![0..3, 4..7], "eae"));
    assert_eq!(LiteralExpr::all_captures("eae eae eae eae"), zip_literal(vec![0..3, 4..7, 8..11, 12..15], "eae"));
    assert_eq!(LiteralExpr::all_captures("eaeae"), zip_literal(vec![0..3], "eae"));
    assert_eq!(LiteralExpr::all_captures("eaeaeaeaeae"), zip_literal(vec![0..3, 4..7, 8..11], "eae"));
    assert_eq!(LiteralExpr::all_captures("ea"), vec![]);
}

#[allow(clippy::single_range_in_vec_init)]
#[test]
fn find_all_captures_overlapping() {
    assert_eq!(LiteralExpr::all_captures_overlap("eae"), zip_literal(vec![0..3], "eae"));
    assert_eq!(LiteralExpr::all_captures_overlap("eae eae"), zip_literal(vec![0..3, 4..7], "eae"));
    assert_eq!(LiteralExpr::all_captures_overlap("eae eae eae eae"), zip_literal(vec![0..3, 4..7, 8..11, 12..15], "eae"));
    assert_eq!(LiteralExpr::all_captures_overlap("eaeae"), zip_literal(vec![0..3, 2..5], "eae"));
    assert_eq!(LiteralExpr::all_captures_overlap("eaeaeaeaeae"), zip_literal(vec![0..3, 2..5, 4..7, 6..9, 8..11], "eae"));
    assert_eq!(LiteralExpr::all_captures_overlap("ea"), vec![]);
}

#[test]
fn replace() {
    assert_eq!(LiteralExpr::first_replaced("eae", "new"), (true, "new".into()));
    assert_eq!(LiteralExpr::first_replaced("aeae", "new"), (true, "anew".into()));
    assert_eq!(LiteralExpr::first_replaced("eaea", "new"), (true, "newa".into()));
    assert_eq!(LiteralExpr::first_replaced("eae eae", "new"), (true, "new eae".into()));
    assert_eq!(LiteralExpr::first_replaced("ene", "new"), (false, "ene".into()));
    assert_eq!(LiteralExpr::first_replaced("ea", "new"), (false, "ea".into()));
}

#[test]
fn replace_all() {
    assert_eq!(LiteralExpr::all_replaced("eae", "new"), (1, "new".into()));
    assert_eq!(LiteralExpr::all_replaced("eae eae", "new"), (2, "new new".into()));
    assert_eq!(LiteralExpr::all_replaced("eae eae eae eae", "new"), (4, "new new new new".into()));
    assert_eq!(LiteralExpr::all_replaced("eaeae", "new"), (1, "newae".into()));
    assert_eq!(LiteralExpr::all_replaced("eaeaeaeaeae", "new"), (3, "newanewanew".into()));
    assert_eq!(LiteralExpr::all_replaced("ea", "new"), (0, "ea".into()));
}

#[test]
fn replace_all_using() {
    assert_eq!(LiteralExpr::all_replaced_using("eae"), (1, "1".into()));
    assert_eq!(LiteralExpr::all_replaced_using("eae eae"), (2, "1 2".into()));
    assert_eq!(LiteralExpr::all_replaced_using("eae eae eae eae"), (4, "1 2 3 4".into()));
    assert_eq!(LiteralExpr::all_replaced_using("eaeae"), (1, "1ae".into()));
    assert_eq!(LiteralExpr::all_replaced_using("eaeaeaeaeae"), (3, "1a2a3".into()));
    assert_eq!(LiteralExpr::all_replaced_using("ea"), (0, "ea".into()));
}

#[test]
fn replace_using_iter() {
    assert_eq!(LiteralExpr::replaced_using_iter("eae"), (1, "1".into()));
    assert_eq!(LiteralExpr::replaced_using_iter("eae eae"), (2, "1 2".into()));
    assert_eq!(LiteralExpr::replaced_using_iter("eae eae eae eae"), (2, "1 2 eae eae".into()));
    assert_eq!(LiteralExpr::replaced_using_iter("eaeae"), (1, "1ae".into()));
    assert_eq!(LiteralExpr::replaced_using_iter("eaeaeaeaeae"), (2, "1a2aeae".into()));
    assert_eq!(LiteralExpr::replaced_using_iter("ea"), (0, "ea".into()));
}

#[test]
fn replace_captured() {
    assert_eq!(LiteralExpr::capture_replaced_sliced("eae"), (true, "ae".into()));
    assert_eq!(LiteralExpr::capture_replaced_sliced("aeae"), (true, "aae".into()));
    assert_eq!(LiteralExpr::capture_replaced_sliced("eaea"), (true, "aea".into()));
    assert_eq!(LiteralExpr::capture_replaced_sliced("eae eae"), (true, "ae eae".into()));

    assert_eq!(LiteralExpr::capture_replaced_sliced("ene"), (false, "ene".into()));
    assert_eq!(LiteralExpr::capture_replaced_sliced("ea"), (false, "ea".into()));
}

#[test]
fn replace_all_captured() {
    assert_eq!(LiteralExpr::all_captures_replaced_sliced("eae"), (1, "ae".into()));
    assert_eq!(LiteralExpr::all_captures_replaced_sliced("eae eae"), (2, "ae ae".into()));
    assert_eq!(LiteralExpr::all_captures_replaced_sliced("eae eae eae eae"), (4, "ae ae ae ae".into()));
    assert_eq!(LiteralExpr::all_captures_replaced_sliced("eaeae"), (1, "aeae".into()));
    assert_eq!(LiteralExpr::all_captures_replaced_sliced("eaeaeaeaeae"), (3, "aeaaeaae".into()));
    assert_eq!(LiteralExpr::all_captures_replaced_sliced("ea"), (0, "ea".into()));
}