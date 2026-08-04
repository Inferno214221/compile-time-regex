use std::ops::Range;

use super::*;

regex! {
    pub BoundedExpr = r"[a-z]+a"
}

regex! {
    pub LazyBoundedExpr = r"[a-z]+?a"
}

// TODO: split into greedy and lazy modules
// TODO: move methods to tests/util

#[test]
fn is_match() {
    assert!(BoundedExpr::is_match("aa"));
    assert!(BoundedExpr::is_match("dcba"));
    assert!(BoundedExpr::is_match("baca"));
    assert!(!BoundedExpr::is_match(" ba"));
    assert!(!BoundedExpr::is_match("ba "));
    assert!(!BoundedExpr::is_match("cba abc"));
    assert!(!BoundedExpr::is_match("a"));
    assert!(!BoundedExpr::is_match(" "));

    assert!(LazyBoundedExpr::is_match("aa"));
    assert!(LazyBoundedExpr::is_match("dcba"));
    assert!(LazyBoundedExpr::is_match("baca"));
    assert!(!LazyBoundedExpr::is_match(" ba"));
    assert!(!LazyBoundedExpr::is_match("ba "));
    assert!(!LazyBoundedExpr::is_match("cba abc"));
    assert!(!LazyBoundedExpr::is_match("a"));
    assert!(!LazyBoundedExpr::is_match(" "));
}

#[test]
fn contains_match() {
    assert!(BoundedExpr::contains_match("aa"));
    assert!(BoundedExpr::contains_match("dcba"));
    assert!(BoundedExpr::contains_match("baca"));
    assert!(BoundedExpr::contains_match(" ba"));
    assert!(BoundedExpr::contains_match("ba "));
    assert!(BoundedExpr::contains_match("cba abc"));
    assert!(!BoundedExpr::contains_match("a"));
    assert!(!BoundedExpr::contains_match(" "));

    assert!(LazyBoundedExpr::contains_match("aa"));
    assert!(LazyBoundedExpr::contains_match("dcba"));
    assert!(LazyBoundedExpr::contains_match("baca"));
    assert!(LazyBoundedExpr::contains_match(" ba"));
    assert!(LazyBoundedExpr::contains_match("ba "));
    assert!(LazyBoundedExpr::contains_match("cba abc"));
    assert!(!LazyBoundedExpr::contains_match("a"));
    assert!(!LazyBoundedExpr::contains_match(" "));
}

#[test]
fn count_matches_non_overlapping() {
    assert_eq!(BoundedExpr::count_matches("aa", false), 1);
    assert_eq!(BoundedExpr::count_matches("dcba", false), 1);
    assert_eq!(BoundedExpr::count_matches("cba abc", false), 1);
    assert_eq!(BoundedExpr::count_matches("aa ba ca", false), 3);
    assert_eq!(BoundedExpr::count_matches("abaca", false), 1);
    assert_eq!(BoundedExpr::count_matches(" ", false), 0);

    assert_eq!(LazyBoundedExpr::count_matches("aa", false), 1);
    assert_eq!(LazyBoundedExpr::count_matches("dcba", false), 1);
    assert_eq!(LazyBoundedExpr::count_matches("cba abc", false), 1);
    assert_eq!(LazyBoundedExpr::count_matches("aa ba ca", false), 3);
    assert_eq!(LazyBoundedExpr::count_matches("abaca", false), 2);
    assert_eq!(LazyBoundedExpr::count_matches(" ", false), 0);
}

#[test]
fn count_matches_overlapping() {
    assert_eq!(BoundedExpr::count_matches("aa", true), 1);
    assert_eq!(BoundedExpr::count_matches("dcba", true), 3);
    assert_eq!(BoundedExpr::count_matches("cba abc", true), 2);
    assert_eq!(BoundedExpr::count_matches("aa ba ca", true), 3);
    assert_eq!(BoundedExpr::count_matches("abaca", true), 4);
    assert_eq!(BoundedExpr::count_matches(" ", true), 0);

    assert_eq!(LazyBoundedExpr::count_matches("aa", true), 1);
    assert_eq!(LazyBoundedExpr::count_matches("dcba", true), 3);
    assert_eq!(LazyBoundedExpr::count_matches("cba abc", true), 2);
    assert_eq!(LazyBoundedExpr::count_matches("aa ba ca", true), 3);
    assert_eq!(LazyBoundedExpr::count_matches("abaca", true), 4);
    assert_eq!(LazyBoundedExpr::count_matches(" ", true), 0);
}

#[test]
fn range_of_match() {
    assert_eq!(BoundedExpr::range_of_match("aa"), Some(0..2));
    assert_eq!(BoundedExpr::range_of_match("dcba"), Some(0..4));
    assert_eq!(BoundedExpr::range_of_match("baca"), Some(0..4));
    assert_eq!(BoundedExpr::range_of_match(" ba"), Some(1..3));
    assert_eq!(BoundedExpr::range_of_match("ba "), Some(0..2));
    assert_eq!(BoundedExpr::range_of_match("cba abc"), Some(0..3));
    assert_eq!(BoundedExpr::range_of_match("a"), None);
    assert_eq!(BoundedExpr::range_of_match(" "), None);

    assert_eq!(LazyBoundedExpr::range_of_match("aa"), Some(0..2));
    assert_eq!(LazyBoundedExpr::range_of_match("dcba"), Some(0..4));
    assert_eq!(LazyBoundedExpr::range_of_match("baca"), Some(0..2));
    assert_eq!(LazyBoundedExpr::range_of_match(" ba"), Some(1..3));
    assert_eq!(LazyBoundedExpr::range_of_match("ba "), Some(0..2));
    assert_eq!(LazyBoundedExpr::range_of_match("cba abc"), Some(0..3));
    assert_eq!(LazyBoundedExpr::range_of_match("a"), None);
    assert_eq!(LazyBoundedExpr::range_of_match(" "), None);
}

#[test]
fn range_of_all_matches_non_overlapping() {
    fn collect_all_ranges<R: Regex<char, N>, const N: usize>(hay: &str) -> Vec<Range<usize>> {
        R::range_of_all_matches(hay, false).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_ranges::<BoundedExpr, _>("aa"), vec![0..2]);
    assert_eq!(collect_all_ranges::<BoundedExpr, _>("dcba"), vec![0..4]);
    assert_eq!(collect_all_ranges::<BoundedExpr, _>("cba abc"), vec![0..3]);
    assert_eq!(collect_all_ranges::<BoundedExpr, _>("aa ba ca"), vec![0..2, 3..5, 6..8]);
    assert_eq!(collect_all_ranges::<BoundedExpr, _>("abaca"), vec![0..5]);
    assert_eq!(collect_all_ranges::<BoundedExpr, _>(" "), vec![0..0; 0]);

    assert_eq!(collect_all_ranges::<LazyBoundedExpr, _>("aa"), vec![0..2]);
    assert_eq!(collect_all_ranges::<LazyBoundedExpr, _>("dcba"), vec![0..4]);
    assert_eq!(collect_all_ranges::<LazyBoundedExpr, _>("cba abc"), vec![0..3]);
    assert_eq!(collect_all_ranges::<LazyBoundedExpr, _>("aa ba ca"), vec![0..2, 3..5, 6..8]);
    assert_eq!(collect_all_ranges::<LazyBoundedExpr, _>("abaca"), vec![0..3, 3..5]);
    assert_eq!(collect_all_ranges::<LazyBoundedExpr, _>(" "), vec![0..0; 0]);
}

#[test]
fn range_of_all_matches_overlapping() {
    fn collect_all_ranges<R: Regex<char, N>, const N: usize>(hay: &str) -> Vec<Range<usize>> {
        R::range_of_all_matches(hay, true).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_ranges::<BoundedExpr, _>("aa"), vec![0..2]);
    assert_eq!(collect_all_ranges::<BoundedExpr, _>("dcba"), vec![0..4, 1..4, 2..4]);
    assert_eq!(collect_all_ranges::<BoundedExpr, _>("cba abc"), vec![0..3, 1..3]);
    assert_eq!(collect_all_ranges::<BoundedExpr, _>("aa ba ca"), vec![0..2, 3..5, 6..8]);
    assert_eq!(collect_all_ranges::<BoundedExpr, _>("abaca"), vec![0..5, 1..5, 2..5, 3..5]);
    assert_eq!(collect_all_ranges::<BoundedExpr, _>(" "), vec![0..0; 0]);

    assert_eq!(collect_all_ranges::<LazyBoundedExpr, _>("aa"), vec![0..2]);
    assert_eq!(collect_all_ranges::<LazyBoundedExpr, _>("dcba"), vec![0..4, 1..4, 2..4]);
    assert_eq!(collect_all_ranges::<LazyBoundedExpr, _>("cba abc"), vec![0..3, 1..3]);
    assert_eq!(collect_all_ranges::<LazyBoundedExpr, _>("aa ba ca"), vec![0..2, 3..5, 6..8]);
    assert_eq!(collect_all_ranges::<LazyBoundedExpr, _>("abaca"), vec![0..3, 1..3, 2..5, 3..5]);
    assert_eq!(collect_all_ranges::<LazyBoundedExpr, _>(" "), vec![0..0; 0]);
}

#[test]
fn slice_match() {
    assert_eq!(BoundedExpr::slice_match("aa"), Some("aa"));
    assert_eq!(BoundedExpr::slice_match("dcba"), Some("dcba"));
    assert_eq!(BoundedExpr::slice_match("baca"), Some("baca"));
    assert_eq!(BoundedExpr::slice_match(" ba"), Some("ba"));
    assert_eq!(BoundedExpr::slice_match("ba "), Some("ba"));
    assert_eq!(BoundedExpr::slice_match("cba abc"), Some("cba"));
    assert_eq!(BoundedExpr::slice_match("a"), None);
    assert_eq!(BoundedExpr::slice_match(" "), None);

    assert_eq!(LazyBoundedExpr::slice_match("aa"), Some("aa"));
    assert_eq!(LazyBoundedExpr::slice_match("dcba"), Some("dcba"));
    assert_eq!(LazyBoundedExpr::slice_match("baca"), Some("ba"));
    assert_eq!(LazyBoundedExpr::slice_match(" ba"), Some("ba"));
    assert_eq!(LazyBoundedExpr::slice_match("ba "), Some("ba"));
    assert_eq!(LazyBoundedExpr::slice_match("cba abc"), Some("cba"));
    assert_eq!(LazyBoundedExpr::slice_match("a"), None);
    assert_eq!(LazyBoundedExpr::slice_match(" "), None);
}

#[test]
fn slice_all_matches_non_overlapping() {
    fn collect_all_slices<R: Regex<char, N>, const N: usize>(hay: &str) -> Vec<&str> {
        R::slice_all_matches(hay, false).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_slices::<BoundedExpr, _>("aa"), vec!["aa"]);
    assert_eq!(collect_all_slices::<BoundedExpr, _>("dcba"), vec!["dcba"]);
    assert_eq!(collect_all_slices::<BoundedExpr, _>("cba abc"), vec!["cba"]);
    assert_eq!(collect_all_slices::<BoundedExpr, _>("aa ba ca"), vec!["aa", "ba", "ca"]);
    assert_eq!(collect_all_slices::<BoundedExpr, _>("abaca"), vec!["abaca"]);
    assert_eq!(collect_all_slices::<BoundedExpr, _>(" "), vec![""; 0]);

    assert_eq!(collect_all_slices::<LazyBoundedExpr, _>("aa"), vec!["aa"]);
    assert_eq!(collect_all_slices::<LazyBoundedExpr, _>("dcba"), vec!["dcba"]);
    assert_eq!(collect_all_slices::<LazyBoundedExpr, _>("cba abc"), vec!["cba"]);
    assert_eq!(collect_all_slices::<LazyBoundedExpr, _>("aa ba ca"), vec!["aa", "ba", "ca"]);
    assert_eq!(collect_all_slices::<LazyBoundedExpr, _>("abaca"), vec!["aba", "ca"]);
    assert_eq!(collect_all_slices::<LazyBoundedExpr, _>(" "), vec![""; 0]);
}

#[test]
fn slice_all_matches_overlapping() {
    fn collect_all_slices<R: Regex<char, N>, const N: usize>(hay: &str) -> Vec<&str> {
        R::slice_all_matches(hay, true).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_slices::<BoundedExpr, _>("aa"), vec!["aa"]);
    assert_eq!(collect_all_slices::<BoundedExpr, _>("dcba"), vec!["dcba", "cba", "ba"]);
    assert_eq!(collect_all_slices::<BoundedExpr, _>("cba abc"), vec!["cba", "ba"]);
    assert_eq!(collect_all_slices::<BoundedExpr, _>("aa ba ca"), vec!["aa", "ba", "ca"]);
    assert_eq!(collect_all_slices::<BoundedExpr, _>("abaca"), vec!["abaca", "baca", "aca", "ca"]);
    assert_eq!(collect_all_slices::<BoundedExpr, _>(" "), vec![""; 0]);

    assert_eq!(collect_all_slices::<LazyBoundedExpr, _>("aa"), vec!["aa"]);
    assert_eq!(collect_all_slices::<LazyBoundedExpr, _>("dcba"), vec!["dcba", "cba", "ba"]);
    assert_eq!(collect_all_slices::<LazyBoundedExpr, _>("cba abc"), vec!["cba", "ba"]);
    assert_eq!(collect_all_slices::<LazyBoundedExpr, _>("aa ba ca"), vec!["aa", "ba", "ca"]);
    assert_eq!(collect_all_slices::<LazyBoundedExpr, _>("abaca"), vec!["aba", "ba", "aca", "ca"]);
    assert_eq!(collect_all_slices::<LazyBoundedExpr, _>(" "), vec![""; 0]);
}