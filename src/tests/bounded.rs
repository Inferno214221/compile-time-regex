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
        assert_eq!(BoundedExpr::all_ranges(" "), vec![0..0; 0]);
    }

    #[test]
    fn range_of_all_matches_overlapping() {
        assert_eq!(BoundedExpr::all_ranges_overlap("aa"), vec![0..2]);
        assert_eq!(BoundedExpr::all_ranges_overlap("dcba"), vec![0..4, 1..4, 2..4]);
        assert_eq!(BoundedExpr::all_ranges_overlap("cba abc"), vec![0..3, 1..3]);
        assert_eq!(BoundedExpr::all_ranges_overlap("aa ba ca"), vec![0..2, 3..5, 6..8]);
        assert_eq!(BoundedExpr::all_ranges_overlap("abaca"), vec![0..5, 1..5, 2..5, 3..5]);
        assert_eq!(BoundedExpr::all_ranges_overlap(" "), vec![0..0; 0]);
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
        assert_eq!(LazyBoundedExpr::all_ranges(" "), vec![0..0; 0]);
    }

    #[test]
    fn range_of_all_matches_overlapping() {
        assert_eq!(LazyBoundedExpr::all_ranges_overlap("aa"), vec![0..2]);
        assert_eq!(LazyBoundedExpr::all_ranges_overlap("dcba"), vec![0..4, 1..4, 2..4]);
        assert_eq!(LazyBoundedExpr::all_ranges_overlap("cba abc"), vec![0..3, 1..3]);
        assert_eq!(LazyBoundedExpr::all_ranges_overlap("aa ba ca"), vec![0..2, 3..5, 6..8]);
        assert_eq!(LazyBoundedExpr::all_ranges_overlap("abaca"), vec![0..3, 1..3, 2..5, 3..5]);
        assert_eq!(LazyBoundedExpr::all_ranges_overlap(" "), vec![0..0; 0]);
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
}