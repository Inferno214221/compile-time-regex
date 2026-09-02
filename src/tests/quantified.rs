use super::*;

mod greedy {
    use super::*;

    regex! {
        pub QuantifiedExpr = r"a[a-z]*"
    }

    #[test]
    fn is_match() {
        assert!(QuantifiedExpr::is_match("aaa"));
        assert!(QuantifiedExpr::is_match("abc"));
        assert!(!QuantifiedExpr::is_match(" abc"));
        assert!(!QuantifiedExpr::is_match("abc "));
        assert!(!QuantifiedExpr::is_match("abc def"));
        assert!(!QuantifiedExpr::is_match(" "));
    }

    #[test]
    fn contains_match() {
        assert!(QuantifiedExpr::contains_match("aaa"));
        assert!(QuantifiedExpr::contains_match("abc"));
        assert!(QuantifiedExpr::contains_match(" abc"));
        assert!(QuantifiedExpr::contains_match("abc "));
        assert!(QuantifiedExpr::contains_match("abc def"));
        assert!(!QuantifiedExpr::contains_match(" "));
    }

    #[test]
    fn count_matches_non_overlapping() {
        assert_eq!(QuantifiedExpr::count_matches("abc", false), 1);
        assert_eq!(QuantifiedExpr::count_matches("abc aad", false), 2);
        assert_eq!(QuantifiedExpr::count_matches("abc def a abcd", false), 3);
        assert_eq!(QuantifiedExpr::count_matches("12abc34", false), 1);
        assert_eq!(QuantifiedExpr::count_matches("abcadef", false), 1);
        assert_eq!(QuantifiedExpr::count_matches(" ", false), 0);
    }

    #[test]
    fn count_matches_overlapping() {
        assert_eq!(QuantifiedExpr::count_matches("abc", true), 1);
        assert_eq!(QuantifiedExpr::count_matches("abc aad", true), 3);
        assert_eq!(QuantifiedExpr::count_matches("abc def a abcd", true), 3);
        assert_eq!(QuantifiedExpr::count_matches("12abc34", true), 1);
        assert_eq!(QuantifiedExpr::count_matches("abcadef", true), 2);
        assert_eq!(QuantifiedExpr::count_matches(" ", true), 0);
    }

    #[test]
    fn range_of_match() {
        assert_eq!(QuantifiedExpr::range_of_match("aaa"), Some(0..3));
        assert_eq!(QuantifiedExpr::range_of_match("abc"), Some(0..3));
        assert_eq!(QuantifiedExpr::range_of_match(" abc"), Some(1..4));
        assert_eq!(QuantifiedExpr::range_of_match("abc "), Some(0..3));
        assert_eq!(QuantifiedExpr::range_of_match("abc def"), Some(0..3));
        assert_eq!(QuantifiedExpr::range_of_match(" "), None);
    }

    #[test]
    fn range_of_all_matches_non_overlapping() {
        assert_eq!(QuantifiedExpr::all_ranges("abc"), vec![0..3]);
        assert_eq!(QuantifiedExpr::all_ranges("abc aad"), vec![0..3, 4..7]);
        assert_eq!(QuantifiedExpr::all_ranges("abc def a abcd"), vec![0..3, 8..9, 10..14]);
        assert_eq!(QuantifiedExpr::all_ranges("12abc34"), vec![2..5]);
        assert_eq!(QuantifiedExpr::all_ranges("abcadef"), vec![0..7]);
        assert_eq!(QuantifiedExpr::all_ranges(" "), vec![0..1; 0]);
    }

    #[test]
    fn range_of_all_matches_overlapping() {
        assert_eq!(QuantifiedExpr::all_ranges_overlap("abc"), vec![0..3]);
        assert_eq!(QuantifiedExpr::all_ranges_overlap("abc aad"), vec![0..3, 4..7, 5..7]);
        assert_eq!(QuantifiedExpr::all_ranges_overlap("abc def a abcd"), vec![0..3, 8..9, 10..14]);
        assert_eq!(QuantifiedExpr::all_ranges_overlap("12abc34"), vec![2..5]);
        assert_eq!(QuantifiedExpr::all_ranges_overlap("abcadef"), vec![0..7, 3..7]);
        assert_eq!(QuantifiedExpr::all_ranges_overlap(" "), vec![0..1; 0]);
    }

    #[test]
    fn slice_match() {
        assert_eq!(QuantifiedExpr::slice_match("aaa"), Some("aaa"));
        assert_eq!(QuantifiedExpr::slice_match("abc"), Some("abc"));
        assert_eq!(QuantifiedExpr::slice_match(" abc"), Some("abc"));
        assert_eq!(QuantifiedExpr::slice_match("abc "), Some("abc"));
        assert_eq!(QuantifiedExpr::slice_match("abc def"), Some("abc"));
        assert_eq!(QuantifiedExpr::slice_match(" "), None);
    }

    #[test]
    fn slice_all_matches_non_overlapping() {
        assert_eq!(QuantifiedExpr::all_slices("abc"), vec!["abc"]);
        assert_eq!(QuantifiedExpr::all_slices("abc aad"), vec!["abc", "aad"]);
        assert_eq!(QuantifiedExpr::all_slices("abc def a abcd"), vec!["abc", "a", "abcd"]);
        assert_eq!(QuantifiedExpr::all_slices("12abc34"), vec!["abc"]);
        assert_eq!(QuantifiedExpr::all_slices("abcadef"), vec!["abcadef"]);
        assert_eq!(QuantifiedExpr::all_slices(" "), vec![""; 0]);
    }

    #[test]
    fn slice_all_matches_overlapping() {
        assert_eq!(QuantifiedExpr::all_slices_overlap("abc"), vec!["abc"]);
        assert_eq!(QuantifiedExpr::all_slices_overlap("abc aad"), vec!["abc", "aad", "ad"]);
        assert_eq!(QuantifiedExpr::all_slices_overlap("abc def a abcd"), vec!["abc", "a", "abcd"]);
        assert_eq!(QuantifiedExpr::all_slices_overlap("12abc34"), vec!["abc"]);
        assert_eq!(QuantifiedExpr::all_slices_overlap("abcadef"), vec!["abcadef", "adef"]);
        assert_eq!(QuantifiedExpr::all_slices_overlap(" "), vec![""; 0]);
    }

    #[test]
    fn do_capture() {
        assert_eq!(QuantifiedExpr::whole_capture("aaa"), Some((0..3, "aaa")));
        assert_eq!(QuantifiedExpr::whole_capture("abc"), Some((0..3, "abc")));
        assert_eq!(QuantifiedExpr::whole_capture(" abc"), None);
        assert_eq!(QuantifiedExpr::whole_capture("abc "), None);
        assert_eq!(QuantifiedExpr::whole_capture("abc def"), None);
        assert_eq!(QuantifiedExpr::whole_capture(" "), None);
    }

    #[test]
    fn find_capture() {
        assert_eq!(QuantifiedExpr::first_capture("aaa"), Some((0..3, "aaa")));
        assert_eq!(QuantifiedExpr::first_capture("abc"), Some((0..3, "abc")));
        assert_eq!(QuantifiedExpr::first_capture(" abc"), Some((1..4, "abc")));
        assert_eq!(QuantifiedExpr::first_capture("abc "), Some((0..3, "abc")));
        assert_eq!(QuantifiedExpr::first_capture("abc def"), Some((0..3, "abc")));
        assert_eq!(QuantifiedExpr::first_capture(" "), None);
    }

    #[test]
    fn find_all_captures_non_overlapping() {
        assert_eq!(QuantifiedExpr::all_captures("abc"), vec![(0..3, "abc")]);
        assert_eq!(QuantifiedExpr::all_captures("abc aad"), vec![(0..3, "abc"), (4..7, "aad")]);
        assert_eq!(QuantifiedExpr::all_captures("abc def a abcd"), vec![(0..3, "abc"), (8..9, "a"), (10..14, "abcd")]);
        assert_eq!(QuantifiedExpr::all_captures("12abc34"), vec![(2..5, "abc")]);
        assert_eq!(QuantifiedExpr::all_captures("abcadef"), vec![(0..7, "abcadef")]);
        assert_eq!(QuantifiedExpr::all_captures(" "), vec![(0..1, ""); 0]);
    }

    #[test]
    fn find_all_captures_overlapping() {
        assert_eq!(QuantifiedExpr::all_captures_overlap("abc"), vec![(0..3, "abc")]);
        assert_eq!(QuantifiedExpr::all_captures_overlap("abc aad"), vec![(0..3, "abc"), (4..7, "aad"), (5..7, "ad")]);
        assert_eq!(QuantifiedExpr::all_captures_overlap("abc def a abcd"), vec![(0..3, "abc"), (8..9, "a"), (10..14, "abcd")]);
        assert_eq!(QuantifiedExpr::all_captures_overlap("12abc34"), vec![(2..5, "abc")]);
        assert_eq!(QuantifiedExpr::all_captures_overlap("abcadef"), vec![(0..7, "abcadef"), (3..7, "adef")]);
        assert_eq!(QuantifiedExpr::all_captures_overlap(" "), vec![(0..1, ""); 0]);
    }

    #[test]
    fn replace() {
        assert_eq!(QuantifiedExpr::first_replaced("aaa", "new"), (true, "new".into()));
        assert_eq!(QuantifiedExpr::first_replaced("abc", "new"), (true, "new".into()));
        assert_eq!(QuantifiedExpr::first_replaced(" abc", "new"), (true, " new".into()));
        assert_eq!(QuantifiedExpr::first_replaced("abc ", "new"), (true, "new ".into()));
        assert_eq!(QuantifiedExpr::first_replaced("abc def", "new"), (true, "new def".into()));
        assert_eq!(QuantifiedExpr::first_replaced(" ", "new"), (false, " ".into()));
    }

    #[test]
    fn replace_all() {
        assert_eq!(QuantifiedExpr::all_replaced("abc", "new"), (1, "new".into()));
        assert_eq!(QuantifiedExpr::all_replaced("abc aad", "new"), (2, "new new".into()));
        assert_eq!(QuantifiedExpr::all_replaced("abc def a abcd", "new"), (3, "new def new new".into()));
        assert_eq!(QuantifiedExpr::all_replaced("12abc34", "new"), (1, "12new34".into()));
        assert_eq!(QuantifiedExpr::all_replaced("abcadef", "new"), (1, "new".into()));
        assert_eq!(QuantifiedExpr::all_replaced(" ", "new"), (0, " ".into()));
    }

    #[test]
    fn replace_all_using() {
        assert_eq!(QuantifiedExpr::all_replaced_using("abc"), (1, "1".into()));
        assert_eq!(QuantifiedExpr::all_replaced_using("abc aad"), (2, "1 2".into()));
        assert_eq!(QuantifiedExpr::all_replaced_using("abc def a abcd"), (3, "1 def 2 3".into()));
        assert_eq!(QuantifiedExpr::all_replaced_using("12abc34"), (1, "12134".into()));
        assert_eq!(QuantifiedExpr::all_replaced_using("abcadef"), (1, "1".into()));
        assert_eq!(QuantifiedExpr::all_replaced_using(" "), (0, " ".into()));
    }

    #[test]
    fn replace_using_iter() {
        assert_eq!(QuantifiedExpr::replaced_using_iter("abc"), (1, "1".into()));
        assert_eq!(QuantifiedExpr::replaced_using_iter("abc aad"), (2, "1 2".into()));
        assert_eq!(QuantifiedExpr::replaced_using_iter("abc def a abcd"), (2, "1 def 2 abcd".into()));
        assert_eq!(QuantifiedExpr::replaced_using_iter("12abc34"), (1, "12134".into()));
        assert_eq!(QuantifiedExpr::replaced_using_iter("abcadef"), (1, "1".into()));
        assert_eq!(QuantifiedExpr::replaced_using_iter(" "), (0, " ".into()));
    }

    #[test]
    fn replace_captured() {
        assert_eq!(QuantifiedExpr::capture_replaced_sliced("aaa"), (true, "aa".into()));
        assert_eq!(QuantifiedExpr::capture_replaced_sliced("abc"), (true, "bc".into()));
        assert_eq!(QuantifiedExpr::capture_replaced_sliced(" abc"), (true, " bc".into()));
        assert_eq!(QuantifiedExpr::capture_replaced_sliced("abc "), (true, "bc ".into()));
        assert_eq!(QuantifiedExpr::capture_replaced_sliced("abc def"), (true, "bc def".into()));
        assert_eq!(QuantifiedExpr::capture_replaced_sliced(" "), (false, " ".into()));
    }

    #[test]
    fn replace_all_captured() {
        assert_eq!(QuantifiedExpr::all_captures_replaced_sliced("abc"), (1, "bc".into()));
        assert_eq!(QuantifiedExpr::all_captures_replaced_sliced("abc aad"), (2, "bc ad".into()));
        assert_eq!(QuantifiedExpr::all_captures_replaced_sliced("abc def a abcd"), (3, "bc def  bcd".into()));
        assert_eq!(QuantifiedExpr::all_captures_replaced_sliced("12abc34"), (1, "12bc34".into()));
        assert_eq!(QuantifiedExpr::all_captures_replaced_sliced("abcadef"), (1, "bcadef".into()));
        assert_eq!(QuantifiedExpr::all_captures_replaced_sliced(" "), (0, " ".into()));
    }
}

mod lazy {
    use super::*;

    regex! {
        pub LazyQuantifiedExpr = r"a[a-z]+?"
    }

    #[test]
    fn is_match() {
        assert!(LazyQuantifiedExpr::is_match("aaa"));
        assert!(LazyQuantifiedExpr::is_match("abc"));
        assert!(!LazyQuantifiedExpr::is_match(" abc"));
        assert!(!LazyQuantifiedExpr::is_match("abc "));
        assert!(!LazyQuantifiedExpr::is_match("abc def"));
        assert!(!LazyQuantifiedExpr::is_match(" "));
    }

    #[test]
    fn contains_match() {
        assert!(LazyQuantifiedExpr::contains_match("aaa"));
        assert!(LazyQuantifiedExpr::contains_match("abc"));
        assert!(LazyQuantifiedExpr::contains_match(" abc"));
        assert!(LazyQuantifiedExpr::contains_match("abc "));
        assert!(LazyQuantifiedExpr::contains_match("abc def"));
        assert!(!LazyQuantifiedExpr::contains_match(" "));
    }

    #[test]
    fn count_matches_non_overlapping() {
        assert_eq!(LazyQuantifiedExpr::count_matches("abc", false), 1);
        assert_eq!(LazyQuantifiedExpr::count_matches("abc aad", false), 2);
        assert_eq!(LazyQuantifiedExpr::count_matches("abc def a abcd", false), 2);
        assert_eq!(LazyQuantifiedExpr::count_matches("12abc34", false), 1);
        assert_eq!(LazyQuantifiedExpr::count_matches("abcadef", false), 2);
        assert_eq!(LazyQuantifiedExpr::count_matches(" ", false), 0);
    }

    #[test]
    fn count_matches_overlapping() {
        assert_eq!(LazyQuantifiedExpr::count_matches("abc", true), 1);
        assert_eq!(LazyQuantifiedExpr::count_matches("abc aad", true), 3);
        assert_eq!(LazyQuantifiedExpr::count_matches("abc def a abcd", true), 2);
        assert_eq!(LazyQuantifiedExpr::count_matches("12abc34", true), 1);
        assert_eq!(LazyQuantifiedExpr::count_matches("abcadef", true), 2);
        assert_eq!(LazyQuantifiedExpr::count_matches(" ", true), 0);
    }

    #[test]
    fn range_of_match() {
        assert_eq!(LazyQuantifiedExpr::range_of_match("aaa"), Some(0..2));
        assert_eq!(LazyQuantifiedExpr::range_of_match("abc"), Some(0..2));
        assert_eq!(LazyQuantifiedExpr::range_of_match(" abc"), Some(1..3));
        assert_eq!(LazyQuantifiedExpr::range_of_match("abc "), Some(0..2));
        assert_eq!(LazyQuantifiedExpr::range_of_match("abc def"), Some(0..2));
        assert_eq!(LazyQuantifiedExpr::range_of_match(" "), None);
    }

    #[test]
    fn range_of_all_matches_non_overlapping() {
        assert_eq!(LazyQuantifiedExpr::all_ranges("abc"), vec![0..2]);
        assert_eq!(LazyQuantifiedExpr::all_ranges("abc aad"), vec![0..2, 4..6]);
        assert_eq!(LazyQuantifiedExpr::all_ranges("abc def a abcd"), vec![0..2, 10..12]);
        assert_eq!(LazyQuantifiedExpr::all_ranges("12abc34"), vec![2..4]);
        assert_eq!(LazyQuantifiedExpr::all_ranges("abcadef"), vec![0..2, 3..5]);
        assert_eq!(LazyQuantifiedExpr::all_ranges(" "), vec![0..1; 0]);
    }

    #[test]
    fn range_of_all_matches_overlapping() {
        assert_eq!(LazyQuantifiedExpr::all_ranges_overlap("abc"), vec![0..2]);
        assert_eq!(LazyQuantifiedExpr::all_ranges_overlap("abc aad"), vec![0..2, 4..6, 5..7]);
        assert_eq!(LazyQuantifiedExpr::all_ranges_overlap("abc def a abcd"), vec![0..2, 10..12]);
        assert_eq!(LazyQuantifiedExpr::all_ranges_overlap("12abc34"), vec![2..4]);
        assert_eq!(LazyQuantifiedExpr::all_ranges_overlap("abcadef"), vec![0..2, 3..5]);
        assert_eq!(LazyQuantifiedExpr::all_ranges_overlap(" "), vec![0..1; 0]);
    }

    #[test]
    fn slice_match() {
        assert_eq!(LazyQuantifiedExpr::slice_match("aaa"), Some("aa"));
        assert_eq!(LazyQuantifiedExpr::slice_match("abc"), Some("ab"));
        assert_eq!(LazyQuantifiedExpr::slice_match(" abc"), Some("ab"));
        assert_eq!(LazyQuantifiedExpr::slice_match("abc "), Some("ab"));
        assert_eq!(LazyQuantifiedExpr::slice_match("abc def"), Some("ab"));
        assert_eq!(LazyQuantifiedExpr::slice_match(" "), None);
    }

    #[test]
    fn slice_all_matches_non_overlapping() {
        assert_eq!(LazyQuantifiedExpr::all_slices("abc"), vec!["ab"]);
        assert_eq!(LazyQuantifiedExpr::all_slices("abc aad"), vec!["ab", "aa"]);
        assert_eq!(LazyQuantifiedExpr::all_slices("abc def a abcd"), vec!["ab", "ab"]);
        assert_eq!(LazyQuantifiedExpr::all_slices("12abc34"), vec!["ab"]);
        assert_eq!(LazyQuantifiedExpr::all_slices("abcadef"), vec!["ab", "ad"]);
        assert_eq!(LazyQuantifiedExpr::all_slices(" "), vec![""; 0]);
    }

    #[test]
    fn slice_all_matches_overlapping() {
        assert_eq!(LazyQuantifiedExpr::all_slices_overlap("abc"), vec!["ab"]);
        assert_eq!(LazyQuantifiedExpr::all_slices_overlap("abc aad"), vec!["ab", "aa", "ad"]);
        assert_eq!(LazyQuantifiedExpr::all_slices_overlap("abc def a abcd"), vec!["ab", "ab"]);
        assert_eq!(LazyQuantifiedExpr::all_slices_overlap("12abc34"), vec!["ab"]);
        assert_eq!(LazyQuantifiedExpr::all_slices_overlap("abcadef"), vec!["ab", "ad"]);
        assert_eq!(LazyQuantifiedExpr::all_slices_overlap(" "), vec![""; 0]);
    }

    #[test]
    fn do_capture() {
        assert_eq!(LazyQuantifiedExpr::whole_capture("aaa"), Some((0..3, "aaa")));
        assert_eq!(LazyQuantifiedExpr::whole_capture("abc"), Some((0..3, "abc")));
        assert_eq!(LazyQuantifiedExpr::whole_capture(" abc"), None);
        assert_eq!(LazyQuantifiedExpr::whole_capture("abc "), None);
        assert_eq!(LazyQuantifiedExpr::whole_capture("abc def"), None);
        assert_eq!(LazyQuantifiedExpr::whole_capture(" "), None);
    }

    #[test]
    fn find_capture() {
        assert_eq!(LazyQuantifiedExpr::first_capture("aaa"), Some((0..2, "aa")));
        assert_eq!(LazyQuantifiedExpr::first_capture("abc"), Some((0..2, "ab")));
        assert_eq!(LazyQuantifiedExpr::first_capture(" abc"), Some((1..3, "ab")));
        assert_eq!(LazyQuantifiedExpr::first_capture("abc "), Some((0..2, "ab")));
        assert_eq!(LazyQuantifiedExpr::first_capture("abc def"), Some((0..2, "ab")));
        assert_eq!(LazyQuantifiedExpr::first_capture(" "), None);
    }

    #[test]
    fn find_all_captures_non_overlapping() {
        assert_eq!(LazyQuantifiedExpr::all_captures("abc"), vec![(0..2, "ab")]);
        assert_eq!(LazyQuantifiedExpr::all_captures("abc aad"), vec![(0..2, "ab"), (4..6, "aa")]);
        assert_eq!(LazyQuantifiedExpr::all_captures("abc def a abcd"), vec![(0..2, "ab"), (10..12, "ab")]);
        assert_eq!(LazyQuantifiedExpr::all_captures("12abc34"), vec![(2..4, "ab")]);
        assert_eq!(LazyQuantifiedExpr::all_captures("abcadef"), vec![(0..2, "ab"), (3..5, "ad")]);
        assert_eq!(LazyQuantifiedExpr::all_captures(" "), vec![(0..1, ""); 0]);
    }

    #[test]
    fn find_all_captures_overlapping() {
        assert_eq!(LazyQuantifiedExpr::all_captures_overlap("abc"), vec![(0..2, "ab")]);
        assert_eq!(LazyQuantifiedExpr::all_captures_overlap("abc aad"), vec![(0..2, "ab"), (4..6, "aa"), (5..7, "ad")]);
        assert_eq!(LazyQuantifiedExpr::all_captures_overlap("abc def a abcd"), vec![(0..2, "ab"), (10..12, "ab")]);
        assert_eq!(LazyQuantifiedExpr::all_captures_overlap("12abc34"), vec![(2..4, "ab")]);
        assert_eq!(LazyQuantifiedExpr::all_captures_overlap("abcadef"), vec![(0..2, "ab"), (3..5, "ad")]);
        assert_eq!(LazyQuantifiedExpr::all_captures_overlap(" "), vec![(0..1, ""); 0]);
    }

    #[test]
    fn replace() {
        assert_eq!(LazyQuantifiedExpr::first_replaced("aaa", "new"), (true, "newa".into()));
        assert_eq!(LazyQuantifiedExpr::first_replaced("abc", "new"), (true, "newc".into()));
        assert_eq!(LazyQuantifiedExpr::first_replaced(" abc", "new"), (true, " newc".into()));
        assert_eq!(LazyQuantifiedExpr::first_replaced("abc ", "new"), (true, "newc ".into()));
        assert_eq!(LazyQuantifiedExpr::first_replaced("abc def", "new"), (true, "newc def".into()));
        assert_eq!(LazyQuantifiedExpr::first_replaced(" ", "new"), (false, " ".into()));
    }

    #[test]
    fn replace_all() {
        assert_eq!(LazyQuantifiedExpr::all_replaced("abc", "new"), (1, "newc".into()));
        assert_eq!(LazyQuantifiedExpr::all_replaced("abc aad", "new"), (2, "newc newd".into()));
        assert_eq!(LazyQuantifiedExpr::all_replaced("abc def a abcd", "new"), (2, "newc def a newcd".into()));
        assert_eq!(LazyQuantifiedExpr::all_replaced("12abc34", "new"), (1, "12newc34".into()));
        assert_eq!(LazyQuantifiedExpr::all_replaced("abcadef", "new"), (2, "newcnewef".into()));
        assert_eq!(LazyQuantifiedExpr::all_replaced(" ", "new"), (0, " ".into()));
    }

    #[test]
    fn replace_all_using() {
        assert_eq!(LazyQuantifiedExpr::all_replaced_using("abc"), (1, "1c".into()));
        assert_eq!(LazyQuantifiedExpr::all_replaced_using("abc aad"), (2, "1c 2d".into()));
        assert_eq!(LazyQuantifiedExpr::all_replaced_using("abc def a abcd"), (2, "1c def a 2cd".into()));
        assert_eq!(LazyQuantifiedExpr::all_replaced_using("12abc34"), (1, "121c34".into()));
        assert_eq!(LazyQuantifiedExpr::all_replaced_using("abcadef"), (2, "1c2ef".into()));
        assert_eq!(LazyQuantifiedExpr::all_replaced_using(" "), (0, " ".into()));
    }

    #[test]
    fn replace_using_iter() {
        assert_eq!(LazyQuantifiedExpr::replaced_using_iter("abc"), (1, "1c".into()));
        assert_eq!(LazyQuantifiedExpr::replaced_using_iter("abc aad"), (2, "1c 2d".into()));
        assert_eq!(LazyQuantifiedExpr::replaced_using_iter("abc def a abcd"), (2, "1c def a 2cd".into()));
        assert_eq!(LazyQuantifiedExpr::replaced_using_iter("12abc34"), (1, "121c34".into()));
        assert_eq!(LazyQuantifiedExpr::replaced_using_iter("abcadef"), (2, "1c2ef".into()));
        assert_eq!(LazyQuantifiedExpr::replaced_using_iter(" "), (0, " ".into()));
    }

    #[test]
    fn replace_captured() {
        assert_eq!(LazyQuantifiedExpr::capture_replaced_sliced("aaa"), (true, "aa".into()));
        assert_eq!(LazyQuantifiedExpr::capture_replaced_sliced("abc"), (true, "bc".into()));
        assert_eq!(LazyQuantifiedExpr::capture_replaced_sliced(" abc"), (true, " bc".into()));
        assert_eq!(LazyQuantifiedExpr::capture_replaced_sliced("abc "), (true, "bc ".into()));
        assert_eq!(LazyQuantifiedExpr::capture_replaced_sliced("abc def"), (true, "bc def".into()));
        assert_eq!(LazyQuantifiedExpr::capture_replaced_sliced(" "), (false, " ".into()));
    }

    #[test]
    fn replace_all_captured() {
        assert_eq!(LazyQuantifiedExpr::all_captures_replaced_sliced("abc"), (1, "bc".into()));
        assert_eq!(LazyQuantifiedExpr::all_captures_replaced_sliced("abc aad"), (2, "bc ad".into()));
        assert_eq!(LazyQuantifiedExpr::all_captures_replaced_sliced("abc def a abcd"), (2, "bc def a bcd".into()));
        assert_eq!(LazyQuantifiedExpr::all_captures_replaced_sliced("12abc34"), (1, "12bc34".into()));
        assert_eq!(LazyQuantifiedExpr::all_captures_replaced_sliced("abcadef"), (2, "bcdef".into()));
        assert_eq!(LazyQuantifiedExpr::all_captures_replaced_sliced(" "), (0, " ".into()));
    }
}
