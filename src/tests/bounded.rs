use super::*;

mod greedy {
    use super::*;

    regex! {
        pub BoundedExpr = r"[a-z]+a"
    }

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
    }

    #[test]
    fn count_matches_non_overlapping() {
        assert_eq!(BoundedExpr::count_matches("aa", false), 1);
        assert_eq!(BoundedExpr::count_matches("dcba", false), 1);
        assert_eq!(BoundedExpr::count_matches("cba abc", false), 1);
        assert_eq!(BoundedExpr::count_matches("aa ba ca", false), 3);
        assert_eq!(BoundedExpr::count_matches("abaca", false), 1);
        assert_eq!(BoundedExpr::count_matches(" ", false), 0);
    }

    #[test]
    fn count_matches_overlapping() {
        assert_eq!(BoundedExpr::count_matches("aa", true), 1);
        assert_eq!(BoundedExpr::count_matches("dcba", true), 3);
        assert_eq!(BoundedExpr::count_matches("cba abc", true), 2);
        assert_eq!(BoundedExpr::count_matches("aa ba ca", true), 3);
        assert_eq!(BoundedExpr::count_matches("abaca", true), 4);
        assert_eq!(BoundedExpr::count_matches(" ", true), 0);
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
    }

    #[test]
    fn range_of_all_matches_non_overlapping() {
        assert_eq!(BoundedExpr::all_ranges("aa"), vec![0..2]);
        assert_eq!(BoundedExpr::all_ranges("dcba"), vec![0..4]);
        assert_eq!(BoundedExpr::all_ranges("cba abc"), vec![0..3]);
        assert_eq!(BoundedExpr::all_ranges("aa ba ca"), vec![0..2, 3..5, 6..8]);
        assert_eq!(BoundedExpr::all_ranges("abaca"), vec![0..5]);
        assert_eq!(BoundedExpr::all_ranges(" "), vec![]);
    }

    #[test]
    fn range_of_all_matches_overlapping() {
        assert_eq!(BoundedExpr::all_ranges_overlap("aa"), vec![0..2]);
        assert_eq!(BoundedExpr::all_ranges_overlap("dcba"), vec![0..4, 1..4, 2..4]);
        assert_eq!(BoundedExpr::all_ranges_overlap("cba abc"), vec![0..3, 1..3]);
        assert_eq!(BoundedExpr::all_ranges_overlap("aa ba ca"), vec![0..2, 3..5, 6..8]);
        assert_eq!(BoundedExpr::all_ranges_overlap("abaca"), vec![0..5, 1..5, 2..5, 3..5]);
        assert_eq!(BoundedExpr::all_ranges_overlap(" "), vec![]);
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
    }

    #[test]
    fn slice_all_matches_non_overlapping() {
        assert_eq!(BoundedExpr::all_slices("aa"), vec!["aa"]);
        assert_eq!(BoundedExpr::all_slices("dcba"), vec!["dcba"]);
        assert_eq!(BoundedExpr::all_slices("cba abc"), vec!["cba"]);
        assert_eq!(BoundedExpr::all_slices("aa ba ca"), vec!["aa", "ba", "ca"]);
        assert_eq!(BoundedExpr::all_slices("abaca"), vec!["abaca"]);
        assert_eq!(BoundedExpr::all_slices(" "), vec![""; 0]);
    }

    #[test]
    fn slice_all_matches_overlapping() {
        assert_eq!(BoundedExpr::all_slices_overlap("aa"), vec!["aa"]);
        assert_eq!(BoundedExpr::all_slices_overlap("dcba"), vec!["dcba", "cba", "ba"]);
        assert_eq!(BoundedExpr::all_slices_overlap("cba abc"), vec!["cba", "ba"]);
        assert_eq!(BoundedExpr::all_slices_overlap("aa ba ca"), vec!["aa", "ba", "ca"]);
        assert_eq!(BoundedExpr::all_slices_overlap("abaca"), vec!["abaca", "baca", "aca", "ca"]);
        assert_eq!(BoundedExpr::all_slices_overlap(" "), vec![""; 0]);
    }

    #[test]
    fn do_capture() {
        assert_eq!(BoundedExpr::whole_capture("aa"), Some((0..2, "aa")));
        assert_eq!(BoundedExpr::whole_capture("dcba"), Some((0..4, "dcba")));
        assert_eq!(BoundedExpr::whole_capture("baca"), Some((0..4, "baca")));
        assert_eq!(BoundedExpr::whole_capture(" ba"), None);
        assert_eq!(BoundedExpr::whole_capture("ba "), None);
        assert_eq!(BoundedExpr::whole_capture("cba abc"), None);
        assert_eq!(BoundedExpr::whole_capture("a"), None);
        assert_eq!(BoundedExpr::whole_capture(" "), None);
    }

    #[test]
    fn find_capture() {
        assert_eq!(BoundedExpr::first_capture("aa"), Some((0..2, "aa")));
        assert_eq!(BoundedExpr::first_capture("dcba"), Some((0..4, "dcba")));
        assert_eq!(BoundedExpr::first_capture("baca"), Some((0..4, "baca")));
        assert_eq!(BoundedExpr::first_capture(" ba"), Some((1..3, "ba")));
        assert_eq!(BoundedExpr::first_capture("ba "), Some((0..2, "ba")));
        assert_eq!(BoundedExpr::first_capture("cba abc"), Some((0..3, "cba")));
        assert_eq!(BoundedExpr::first_capture("a"), None);
        assert_eq!(BoundedExpr::first_capture(" "), None);
    }

    #[test]
    fn find_all_captures_non_overlapping() {
        assert_eq!(BoundedExpr::all_captures("aa"), vec![(0..2, "aa")]);
        assert_eq!(BoundedExpr::all_captures("dcba"), vec![(0..4, "dcba")]);
        assert_eq!(BoundedExpr::all_captures("cba abc"), vec![(0..3, "cba")]);
        assert_eq!(BoundedExpr::all_captures("aa ba ca"), vec![(0..2, "aa"), (3..5, "ba"), (6..8, "ca")]);
        assert_eq!(BoundedExpr::all_captures("abaca"), vec![(0..5, "abaca")]);
        assert_eq!(BoundedExpr::all_captures(" "), vec![]);
    }

    #[test]
    fn find_all_captures_overlapping() {
        assert_eq!(BoundedExpr::all_captures_overlap("aa"), vec![(0..2, "aa")]);
        assert_eq!(BoundedExpr::all_captures_overlap("dcba"), vec![(0..4, "dcba"), (1..4, "cba"), (2..4, "ba")]);
        assert_eq!(BoundedExpr::all_captures_overlap("cba abc"), vec![(0..3, "cba"), (1..3, "ba")]);
        assert_eq!(BoundedExpr::all_captures_overlap("aa ba ca"), vec![(0..2, "aa"), (3..5, "ba"), (6..8, "ca")]);
        assert_eq!(
            BoundedExpr::all_captures_overlap("abaca"),
            vec![(0..5, "abaca"), (1..5, "baca"), (2..5, "aca"), (3..5, "ca")]
        );
        assert_eq!(BoundedExpr::all_captures_overlap(" "), vec![]);
    }

    #[test]
    fn replace() {
        assert_eq!(BoundedExpr::first_replaced("aa", "r"), (true, "r".into()));
        assert_eq!(BoundedExpr::first_replaced("dcba", "r"), (true, "r".into()));
        assert_eq!(BoundedExpr::first_replaced("baca", "r"), (true, "r".into()));
        assert_eq!(BoundedExpr::first_replaced(" ba", "r"), (true, " r".into()));
        assert_eq!(BoundedExpr::first_replaced("ba ", "r"), (true, "r ".into()));
        assert_eq!(BoundedExpr::first_replaced("cba abc", "r"), (true, "r abc".into()));
        assert_eq!(BoundedExpr::first_replaced("a", "r"), (false, "a".into()));
        assert_eq!(BoundedExpr::first_replaced(" ", "r"), (false, " ".into()));
    }

    #[test]
    fn replace_all() {
        assert_eq!(BoundedExpr::all_replaced("aa", "r"), (1, "r".into()));
        assert_eq!(BoundedExpr::all_replaced("dcba", "r"), (1, "r".into()));
        assert_eq!(BoundedExpr::all_replaced("cba abc", "r"), (1, "r abc".into()));
        assert_eq!(BoundedExpr::all_replaced("aa ba ca", "r"), (3, "r r r".into()));
        assert_eq!(BoundedExpr::all_replaced("abaca", "r"), (1, "r".into()));
        assert_eq!(BoundedExpr::all_replaced(" ", "r"), (0, " ".into()));
    }

    #[test]
    fn replace_all_using() {
        assert_eq!(BoundedExpr::all_replaced_using("aa"), (1, "1".into()));
        assert_eq!(BoundedExpr::all_replaced_using("dcba"), (1, "1".into()));
        assert_eq!(BoundedExpr::all_replaced_using("cba abc"), (1, "1 abc".into()));
        assert_eq!(BoundedExpr::all_replaced_using("aa ba ca"), (3, "1 2 3".into()));
        assert_eq!(BoundedExpr::all_replaced_using("abaca"), (1, "1".into()));
        assert_eq!(BoundedExpr::all_replaced_using(" "), (0, " ".into()));
    }

    #[test]
    fn replace_using_iter() {
        assert_eq!(BoundedExpr::replaced_using_iter("aa"), (1, "1".into()));
        assert_eq!(BoundedExpr::replaced_using_iter("dcba"), (1, "1".into()));
        assert_eq!(BoundedExpr::replaced_using_iter("cba abc"), (1, "1 abc".into()));
        assert_eq!(BoundedExpr::replaced_using_iter("aa ba ca"), (2, "1 2 ca".into()));
        assert_eq!(BoundedExpr::replaced_using_iter("abaca"), (1, "1".into()));
        assert_eq!(BoundedExpr::replaced_using_iter(" "), (0, " ".into()));
    }

    #[test]
    fn replace_captured() {
        assert_eq!(BoundedExpr::capture_replaced_sliced("aa"), (true, "a".into()));
        assert_eq!(BoundedExpr::capture_replaced_sliced("dcba"), (true, "cba".into()));
        assert_eq!(BoundedExpr::capture_replaced_sliced("baca"), (true, "aca".into()));
        assert_eq!(BoundedExpr::capture_replaced_sliced(" ba"), (true, " a".into()));
        assert_eq!(BoundedExpr::capture_replaced_sliced("ba "), (true, "a ".into()));
        assert_eq!(BoundedExpr::capture_replaced_sliced("cba abc"), (true, "ba abc".into()));
        assert_eq!(BoundedExpr::capture_replaced_sliced("a"), (false, "a".into()));
        assert_eq!(BoundedExpr::capture_replaced_sliced(" "), (false, " ".into()));
    }

    #[test]
    fn replace_all_captured() {
        assert_eq!(BoundedExpr::all_captures_replaced_sliced("aa"), (1, "a".into()));
        assert_eq!(BoundedExpr::all_captures_replaced_sliced("dcba"), (1, "cba".into()));
        assert_eq!(BoundedExpr::all_captures_replaced_sliced("cba abc"), (1, "ba abc".into()));
        assert_eq!(BoundedExpr::all_captures_replaced_sliced("aa ba ca"), (3, "a a a".into()));
        assert_eq!(BoundedExpr::all_captures_replaced_sliced("abaca"), (1, "baca".into()));
        assert_eq!(BoundedExpr::all_captures_replaced_sliced(" "), (0, " ".into()));
    }
}

mod lazy {
    use super::*;

    regex! {
        pub LazyBoundedExpr = r"[a-z]+?a"
    }

    #[test]
    fn is_match() {
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
        assert_eq!(LazyBoundedExpr::count_matches("aa", false), 1);
        assert_eq!(LazyBoundedExpr::count_matches("dcba", false), 1);
        assert_eq!(LazyBoundedExpr::count_matches("cba abc", false), 1);
        assert_eq!(LazyBoundedExpr::count_matches("aa ba ca", false), 3);
        assert_eq!(LazyBoundedExpr::count_matches("abaca", false), 2);
        assert_eq!(LazyBoundedExpr::count_matches(" ", false), 0);
    }

    #[test]
    fn count_matches_overlapping() {
        assert_eq!(LazyBoundedExpr::count_matches("aa", true), 1);
        assert_eq!(LazyBoundedExpr::count_matches("dcba", true), 3);
        assert_eq!(LazyBoundedExpr::count_matches("cba abc", true), 2);
        assert_eq!(LazyBoundedExpr::count_matches("aa ba ca", true), 3);
        assert_eq!(LazyBoundedExpr::count_matches("abaca", true), 4);
        assert_eq!(LazyBoundedExpr::count_matches(" ", true), 0);
    }

    #[test]
    fn range_of_match() {
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
        assert_eq!(LazyBoundedExpr::all_ranges("aa"), vec![0..2]);
        assert_eq!(LazyBoundedExpr::all_ranges("dcba"), vec![0..4]);
        assert_eq!(LazyBoundedExpr::all_ranges("cba abc"), vec![0..3]);
        assert_eq!(LazyBoundedExpr::all_ranges("aa ba ca"), vec![0..2, 3..5, 6..8]);
        assert_eq!(LazyBoundedExpr::all_ranges("abaca"), vec![0..3, 3..5]);
        assert_eq!(LazyBoundedExpr::all_ranges(" "), vec![]);
    }

    #[test]
    fn range_of_all_matches_overlapping() {
        assert_eq!(LazyBoundedExpr::all_ranges_overlap("aa"), vec![0..2]);
        assert_eq!(LazyBoundedExpr::all_ranges_overlap("dcba"), vec![0..4, 1..4, 2..4]);
        assert_eq!(LazyBoundedExpr::all_ranges_overlap("cba abc"), vec![0..3, 1..3]);
        assert_eq!(LazyBoundedExpr::all_ranges_overlap("aa ba ca"), vec![0..2, 3..5, 6..8]);
        assert_eq!(LazyBoundedExpr::all_ranges_overlap("abaca"), vec![0..3, 1..3, 2..5, 3..5]);
        assert_eq!(LazyBoundedExpr::all_ranges_overlap(" "), vec![]);
    }

    #[test]
    fn slice_match() {
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
        assert_eq!(LazyBoundedExpr::all_slices("aa"), vec!["aa"]);
        assert_eq!(LazyBoundedExpr::all_slices("dcba"), vec!["dcba"]);
        assert_eq!(LazyBoundedExpr::all_slices("cba abc"), vec!["cba"]);
        assert_eq!(LazyBoundedExpr::all_slices("aa ba ca"), vec!["aa", "ba", "ca"]);
        assert_eq!(LazyBoundedExpr::all_slices("abaca"), vec!["aba", "ca"]);
        assert_eq!(LazyBoundedExpr::all_slices(" "), vec![""; 0]);
    }

    #[test]
    fn slice_all_matches_overlapping() {
        assert_eq!(LazyBoundedExpr::all_slices_overlap("aa"), vec!["aa"]);
        assert_eq!(LazyBoundedExpr::all_slices_overlap("dcba"), vec!["dcba", "cba", "ba"]);
        assert_eq!(LazyBoundedExpr::all_slices_overlap("cba abc"), vec!["cba", "ba"]);
        assert_eq!(LazyBoundedExpr::all_slices_overlap("aa ba ca"), vec!["aa", "ba", "ca"]);
        assert_eq!(LazyBoundedExpr::all_slices_overlap("abaca"), vec!["aba", "ba", "aca", "ca"]);
        assert_eq!(LazyBoundedExpr::all_slices_overlap(" "), vec![""; 0]);
    }

    #[test]
    fn do_capture() {
        assert_eq!(LazyBoundedExpr::whole_capture("aa"), Some((0..2, "aa")));
        assert_eq!(LazyBoundedExpr::whole_capture("dcba"), Some((0..4, "dcba")));
        assert_eq!(LazyBoundedExpr::whole_capture("baca"), Some((0..4, "baca")));
        assert_eq!(LazyBoundedExpr::whole_capture(" ba"), None);
        assert_eq!(LazyBoundedExpr::whole_capture("ba "), None);
        assert_eq!(LazyBoundedExpr::whole_capture("cba abc"), None);
        assert_eq!(LazyBoundedExpr::whole_capture("a"), None);
        assert_eq!(LazyBoundedExpr::whole_capture(" "), None);
    }

    #[test]
    fn find_capture() {
        assert_eq!(LazyBoundedExpr::first_capture("aa"), Some((0..2, "aa")));
        assert_eq!(LazyBoundedExpr::first_capture("dcba"), Some((0..4, "dcba")));
        assert_eq!(LazyBoundedExpr::first_capture("baca"), Some((0..2, "ba")));
        assert_eq!(LazyBoundedExpr::first_capture(" ba"), Some((1..3, "ba")));
        assert_eq!(LazyBoundedExpr::first_capture("ba "), Some((0..2, "ba")));
        assert_eq!(LazyBoundedExpr::first_capture("cba abc"), Some((0..3, "cba")));
        assert_eq!(LazyBoundedExpr::first_capture("a"), None);
        assert_eq!(LazyBoundedExpr::first_capture(" "), None);
    }

    #[test]
    fn find_all_captures_non_overlapping() {
        assert_eq!(LazyBoundedExpr::all_captures("aa"), vec![(0..2, "aa")]);
        assert_eq!(LazyBoundedExpr::all_captures("dcba"), vec![(0..4, "dcba")]);
        assert_eq!(LazyBoundedExpr::all_captures("cba abc"), vec![(0..3, "cba")]);
        assert_eq!(LazyBoundedExpr::all_captures("aa ba ca"), vec![(0..2, "aa"), (3..5, "ba"), (6..8, "ca")]);
        assert_eq!(LazyBoundedExpr::all_captures("abaca"), vec![(0..3, "aba"), (3..5, "ca")]);
        assert_eq!(LazyBoundedExpr::all_captures(" "), vec![]);
    }

    #[test]
    fn find_all_captures_overlapping() {
        assert_eq!(LazyBoundedExpr::all_captures_overlap("aa"), vec![(0..2, "aa")]);
        assert_eq!(LazyBoundedExpr::all_captures_overlap("dcba"), vec![(0..4, "dcba"), (1..4, "cba"), (2..4, "ba")]);
        assert_eq!(LazyBoundedExpr::all_captures_overlap("cba abc"), vec![(0..3, "cba"), (1..3, "ba")]);
        assert_eq!(LazyBoundedExpr::all_captures_overlap("aa ba ca"), vec![(0..2, "aa"), (3..5, "ba"), (6..8, "ca")]);
        assert_eq!(
            LazyBoundedExpr::all_captures_overlap("abaca"),
            vec![(0..3, "aba"), (1..3, "ba"), (2..5, "aca"), (3..5, "ca")]
        );
        assert_eq!(LazyBoundedExpr::all_captures_overlap(" "), vec![]);
    }

    #[test]
    fn replace() {
        assert_eq!(LazyBoundedExpr::first_replaced("aa", "r"), (true, "r".into()));
        assert_eq!(LazyBoundedExpr::first_replaced("dcba", "r"), (true, "r".into()));
        assert_eq!(LazyBoundedExpr::first_replaced("baca", "r"), (true, "rca".into()));
        assert_eq!(LazyBoundedExpr::first_replaced(" ba", "r"), (true, " r".into()));
        assert_eq!(LazyBoundedExpr::first_replaced("ba ", "r"), (true, "r ".into()));
        assert_eq!(LazyBoundedExpr::first_replaced("cba abc", "r"), (true, "r abc".into()));
        assert_eq!(LazyBoundedExpr::first_replaced("a", "r"), (false, "a".into()));
        assert_eq!(LazyBoundedExpr::first_replaced(" ", "r"), (false, " ".into()));
    }

    #[test]
    fn replace_all() {
        assert_eq!(LazyBoundedExpr::all_replaced("aa", "r"), (1, "r".into()));
        assert_eq!(LazyBoundedExpr::all_replaced("dcba", "r"), (1, "r".into()));
        assert_eq!(LazyBoundedExpr::all_replaced("cba abc", "r"), (1, "r abc".into()));
        assert_eq!(LazyBoundedExpr::all_replaced("aa ba ca", "r"), (3, "r r r".into()));
        assert_eq!(LazyBoundedExpr::all_replaced("abaca", "r"), (2, "rr".into()));
        assert_eq!(LazyBoundedExpr::all_replaced(" ", "r"), (0, " ".into()));
    }

    #[test]
    fn replace_all_using() {
        assert_eq!(LazyBoundedExpr::all_replaced_using("aa"), (1, "1".into()));
        assert_eq!(LazyBoundedExpr::all_replaced_using("dcba"), (1, "1".into()));
        assert_eq!(LazyBoundedExpr::all_replaced_using("cba abc"), (1, "1 abc".into()));
        assert_eq!(LazyBoundedExpr::all_replaced_using("aa ba ca"), (3, "1 2 3".into()));
        assert_eq!(LazyBoundedExpr::all_replaced_using("abaca"), (2, "12".into()));
        assert_eq!(LazyBoundedExpr::all_replaced_using(" "), (0, " ".into()));
    }

    #[test]
    fn replace_using_iter() {
        assert_eq!(LazyBoundedExpr::replaced_using_iter("aa"), (1, "1".into()));
        assert_eq!(LazyBoundedExpr::replaced_using_iter("dcba"), (1, "1".into()));
        assert_eq!(LazyBoundedExpr::replaced_using_iter("cba abc"), (1, "1 abc".into()));
        assert_eq!(LazyBoundedExpr::replaced_using_iter("aa ba ca"), (2, "1 2 ca".into()));
        assert_eq!(LazyBoundedExpr::replaced_using_iter("abaca"), (2, "12".into()));
        assert_eq!(LazyBoundedExpr::replaced_using_iter(" "), (0, " ".into()));
    }

    #[test]
    fn replace_captured() {
        assert_eq!(LazyBoundedExpr::capture_replaced_sliced("aa"), (true, "a".into()));
        assert_eq!(LazyBoundedExpr::capture_replaced_sliced("dcba"), (true, "cba".into()));
        assert_eq!(LazyBoundedExpr::capture_replaced_sliced("baca"), (true, "aca".into()));
        assert_eq!(LazyBoundedExpr::capture_replaced_sliced(" ba"), (true, " a".into()));
        assert_eq!(LazyBoundedExpr::capture_replaced_sliced("ba "), (true, "a ".into()));
        assert_eq!(LazyBoundedExpr::capture_replaced_sliced("cba abc"), (true, "ba abc".into()));
        assert_eq!(LazyBoundedExpr::capture_replaced_sliced("a"), (false, "a".into()));
        assert_eq!(LazyBoundedExpr::capture_replaced_sliced(" "), (false, " ".into()));
    }

    #[test]
    fn replace_all_captured() {
        assert_eq!(LazyBoundedExpr::all_captures_replaced_sliced("aa"), (1, "a".into()));
        assert_eq!(LazyBoundedExpr::all_captures_replaced_sliced("dcba"), (1, "cba".into()));
        assert_eq!(LazyBoundedExpr::all_captures_replaced_sliced("cba abc"), (1, "ba abc".into()));
        assert_eq!(LazyBoundedExpr::all_captures_replaced_sliced("aa ba ca"), (3, "a a a".into()));
        assert_eq!(LazyBoundedExpr::all_captures_replaced_sliced("abaca"), (2, "baa".into()));
        assert_eq!(LazyBoundedExpr::all_captures_replaced_sliced(" "), (0, " ".into()));
    }
}