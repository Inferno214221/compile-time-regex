#![allow(clippy::module_inception)]
use ct_regex::*;

mod literal_expression {
    use std::ops::Range;

use super::*;

    regex! {
        pub LiteralExpr = r"eae"
    }

    #[test]
    fn is_match() {
        assert!(LiteralExpr::is_match("eae"));

        assert!(!LiteralExpr::is_match("ene"));
        assert!(!LiteralExpr::is_match("ea"));
        assert!(!LiteralExpr::is_match("aeae"));
        assert!(!LiteralExpr::is_match("eaea"));
    }

    #[test]
    fn contains_match() {
        assert!(LiteralExpr::contains_match("eae"));
        assert!(LiteralExpr::contains_match("aeae"));
        assert!(LiteralExpr::contains_match("eaea"));

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

        assert_eq!(LiteralExpr::range_of_match("ene"), None);
        assert_eq!(LiteralExpr::range_of_match("ea"), None);
    }

    #[test]
    fn range_of_all_matches_non_overlapping() {
        fn collect_all_ranges(hay: &str) -> Vec<Range<usize>> {
            LiteralExpr::range_of_all_matches(hay, false).collect::<Vec<_>>()
        }

        assert_eq!(collect_all_ranges("eae"), vec![0..3]);
        assert_eq!(collect_all_ranges("eae eae"), vec![0..3, 4..7]);
        assert_eq!(collect_all_ranges("eae eae eae eae"), vec![0..3, 4..7, 8..11, 12..15]);
        assert_eq!(collect_all_ranges("eaeae"), vec![0..3]);
        assert_eq!(collect_all_ranges("eaeaeaeaeae"), vec![0..3, 4..7, 8..11]);
        assert_eq!(collect_all_ranges("ea"), vec![]);
    }

    #[test]
    fn range_of_all_matches_overlapping() {
        fn collect_all_ranges(hay: &str) -> Vec<Range<usize>> {
            LiteralExpr::range_of_all_matches(hay, true).collect::<Vec<_>>()
        }

        assert_eq!(collect_all_ranges("eae"), vec![0..3]);
        assert_eq!(collect_all_ranges("eae eae"), vec![0..3, 4..7]);
        assert_eq!(collect_all_ranges("eae eae eae eae"), vec![0..3, 4..7, 8..11, 12..15]);
        assert_eq!(collect_all_ranges("eaeae"), vec![0..3, 2..5]);
        assert_eq!(collect_all_ranges("eaeaeaeaeae"), vec![0..3, 2..5, 4..7, 6..9, 8..11]);
        assert_eq!(collect_all_ranges("ea"), vec![]);
    }

    #[test]
    fn slice_match() {
        assert_eq!(LiteralExpr::slice_match("eae"), Some("eae"));
        assert_eq!(LiteralExpr::slice_match("aeae"), Some("eae"));
        assert_eq!(LiteralExpr::slice_match("eaea"), Some("eae"));

        assert_eq!(LiteralExpr::slice_match("ene"), None);
        assert_eq!(LiteralExpr::slice_match("ea"), None);
    }
}

// TODO: test flags