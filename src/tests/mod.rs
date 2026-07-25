#![allow(clippy::module_inception)]
use ct_regex::*;

mod literal_expression {
    use std::{iter, ops::Range};

    use super::*;

    regex! {
        pub LiteralExpr = r"eae"
    }

    fn cap_to_parts<'a>(cap: LiteralExprCapture<'a, &'a str>) -> (Range<usize>, &'a str) {
        (cap.whole_match_range(), cap.whole_match())
    }

    fn create_counter() -> impl FnMut() -> String {
        let mut count = 0;
        move || {
            count += 1;
            count.to_string()
        }
    }

    fn slice_first_byte<'a>(cap: LiteralExprCapture<'a, &'a str>) -> String {
        cap.whole_match()[1..].into()
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


    #[test]
    fn slice_all_matches_non_overlapping() {
        fn collect_all_slices(hay: &str) -> Vec<&str> {
            LiteralExpr::slice_all_matches(hay, false).collect::<Vec<_>>()
        }

        assert_eq!(collect_all_slices("eae"), vec!["eae"; 1]);
        assert_eq!(collect_all_slices("eae eae"), vec!["eae"; 2]);
        assert_eq!(collect_all_slices("eae eae eae eae"), vec!["eae"; 4]);
        assert_eq!(collect_all_slices("eaeae"), vec!["eae"; 1]);
        assert_eq!(collect_all_slices("eaeaeaeaeae"), vec!["eae"; 3]);
        assert_eq!(collect_all_slices("ea"), vec!["eae"; 0]);
    }

    #[test]
    fn slice_all_matches_overlapping() {
        fn collect_all_slices(hay: &str) -> Vec<&str> {
            LiteralExpr::slice_all_matches(hay, true).collect::<Vec<_>>()
        }

        assert_eq!(collect_all_slices("eae"), vec!["eae"; 1]);
        assert_eq!(collect_all_slices("eae eae"), vec!["eae"; 2]);
        assert_eq!(collect_all_slices("eae eae eae eae"), vec!["eae"; 4]);
        assert_eq!(collect_all_slices("eaeae"), vec!["eae"; 2]);
        assert_eq!(collect_all_slices("eaeaeaeaeae"), vec!["eae"; 5]);
        assert_eq!(collect_all_slices("ea"), vec!["eae"; 0]);
    }

    #[test]
    fn do_capture() {
        fn do_capture(hay: &str) -> Option<(Range<usize>, &str)> {
            LiteralExpr::do_capture(hay).map(cap_to_parts)
        }

        assert_eq!(do_capture("eae"), Some((0..3, "eae")));

        assert_eq!(do_capture("ene"), None);
        assert_eq!(do_capture("ea"), None);
        assert_eq!(do_capture("aeae"), None);
        assert_eq!(do_capture("eaea"), None);
    }

    #[test]
    fn find_capture() {
        fn find_capture(hay: &str) -> Option<(Range<usize>, &str)> {
            LiteralExpr::find_capture(hay).map(cap_to_parts)
        }

        assert_eq!(find_capture("eae"), Some((0..3, "eae")));
        assert_eq!(find_capture("aeae"), Some((1..4, "eae")));
        assert_eq!(find_capture("eaea"), Some((0..3, "eae")));

        assert_eq!(find_capture("ene"), None);
        assert_eq!(find_capture("ea"), None);
    }

    #[allow(clippy::single_range_in_vec_init)]
    #[test]
    fn find_all_captures_non_overlapping() {
        fn collect_all_captures(hay: &str) -> Vec<(Range<usize>, &str)> {
            LiteralExpr::find_all_captures(hay, false).map(cap_to_parts).collect::<Vec<_>>()
        }

        fn zip_literal(vec: Vec<Range<usize>>) -> Vec<(Range<usize>, &'static str)> {
            vec.into_iter().zip(
                iter::repeat("eae")
            ).collect::<Vec<_>>()
        }

        assert_eq!(collect_all_captures("eae"), zip_literal(vec![0..3]));
        assert_eq!(collect_all_captures("eae eae"), zip_literal(vec![0..3, 4..7]));
        assert_eq!(collect_all_captures("eae eae eae eae"), zip_literal(vec![0..3, 4..7, 8..11, 12..15]));
        assert_eq!(collect_all_captures("eaeae"), zip_literal(vec![0..3]));
        assert_eq!(collect_all_captures("eaeaeaeaeae"), zip_literal(vec![0..3, 4..7, 8..11]));
        assert_eq!(collect_all_captures("ea"), vec![]);
    }

    #[allow(clippy::single_range_in_vec_init)]
    #[test]
    fn find_all_captures_overlapping() {
        fn collect_all_captures(hay: &str) -> Vec<(Range<usize>, &str)> {
            LiteralExpr::find_all_captures(hay, true).map(cap_to_parts).collect::<Vec<_>>()
        }

        fn zip_literal(vec: Vec<Range<usize>>) -> Vec<(Range<usize>, &'static str)> {
            vec.into_iter().zip(
                iter::repeat("eae")
            ).collect::<Vec<_>>()
        }

        assert_eq!(collect_all_captures("eae"), zip_literal(vec![0..3]));
        assert_eq!(collect_all_captures("eae eae"), zip_literal(vec![0..3, 4..7]));
        assert_eq!(collect_all_captures("eae eae eae eae"), zip_literal(vec![0..3, 4..7, 8..11, 12..15]));
        assert_eq!(collect_all_captures("eaeae"), zip_literal(vec![0..3, 2..5]));
        assert_eq!(collect_all_captures("eaeaeaeaeae"), zip_literal(vec![0..3, 2..5, 4..7, 6..9, 8..11]));
        assert_eq!(collect_all_captures("ea"), vec![]);
    }



    #[test]
    fn replace() {
        fn replace(hay: &str, success: bool) -> String {
            let mut hay = String::from(hay);
            assert_eq!(LiteralExpr::replace(&mut hay, "new"), success);
            hay
        }

        assert_eq!(replace("eae", true), "new");
        assert_eq!(replace("aeae", true), "anew");
        assert_eq!(replace("eaea", true), "newa");

        assert_eq!(replace("ene", false), "ene");
        assert_eq!(replace("ea", false), "ea");
    }

    #[test]
    fn replace_all() {
        fn replace_all(hay: &str, count: usize) -> String {
            let mut hay = String::from(hay);
            assert_eq!(LiteralExpr::replace_all(&mut hay, "new"), count);
            hay
        }

        assert_eq!(replace_all("eae", 1), "new");
        assert_eq!(replace_all("eae eae", 2), "new new");
        assert_eq!(replace_all("eae eae eae eae", 4), "new new new new");
        assert_eq!(replace_all("eaeae", 1), "newae");
        assert_eq!(replace_all("eaeaeaeaeae", 3), "newanewanew");
        assert_eq!(replace_all("ea", 0), "ea");
    }

    #[test]
    fn replace_all_using() {
        fn replace_all_using(hay: &str, count: usize) -> String {
            let mut hay = String::from(hay);
            assert_eq!(LiteralExpr::replace_all_using(&mut hay, create_counter()), count);
            hay
        }

        assert_eq!(replace_all_using("eae", 1), "1");
        assert_eq!(replace_all_using("eae eae", 2), "1 2");
        assert_eq!(replace_all_using("eae eae eae eae", 4), "1 2 3 4");
        assert_eq!(replace_all_using("eaeae", 1), "1ae");
        assert_eq!(replace_all_using("eaeaeaeaeae", 3), "1a2a3");
        assert_eq!(replace_all_using("ea", 0), "ea");
    }

    #[test]
    fn replace_using_iter() {
        fn replace_using_iter(hay: &str, count: usize) -> String {
            let iter = (1..=2).map(|n| n.to_string());

            let mut hay = String::from(hay);
            assert_eq!(LiteralExpr::replace_using_iter(&mut hay, iter), count);
            hay
        }

        assert_eq!(replace_using_iter("eae", 1), "1");
        assert_eq!(replace_using_iter("eae eae", 2), "1 2");
        assert_eq!(replace_using_iter("eae eae eae eae", 2), "1 2 eae eae");
        assert_eq!(replace_using_iter("eaeae", 1), "1ae");
        assert_eq!(replace_using_iter("eaeaeaeaeae", 2), "1a2aeae");
        assert_eq!(replace_using_iter("ea", 0), "ea");
    }

    #[test]
    fn replace_captured() {
        fn replace_captured(hay: &str, success: bool) -> String {
            let mut hay = String::from(hay);
            // This is two hard to express as a parameter
            assert_eq!(LiteralExpr::replace_captured(&mut hay, slice_first_byte), success);
            hay
        }

        assert_eq!(replace_captured("eae", true), "ae");
        assert_eq!(replace_captured("aeae", true), "aae");
        assert_eq!(replace_captured("eaea", true), "aea");

        assert_eq!(replace_captured("ene", false), "ene");
        assert_eq!(replace_captured("ea", false), "ea");
    }

    #[test]
    fn replace_all_captured() {
        fn replace_all_captured(hay: &str, count: usize) -> String {
            let mut hay = String::from(hay);
            assert_eq!(LiteralExpr::replace_all_captured(&mut hay, slice_first_byte), count);
            hay
        }

        assert_eq!(replace_all_captured("eae", 1), "ae");
        assert_eq!(replace_all_captured("eae eae", 2), "ae ae");
        assert_eq!(replace_all_captured("eae eae eae eae", 4), "ae ae ae ae");
        assert_eq!(replace_all_captured("eaeae", 1), "aeae");
        assert_eq!(replace_all_captured("eaeaeaeaeae", 3), "aeaaeaae");
        assert_eq!(replace_all_captured("ea", 0), "ea");
    }
}

// TODO: test flags