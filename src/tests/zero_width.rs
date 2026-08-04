use super::*;

mod start {
    use super::*;

    regex! {
        pub StartExpr = r"^"
    }

    #[test]
    fn is_match() {
        assert!(StartExpr::is_match(""));
        assert!(!StartExpr::is_match("a"));
        assert!(!StartExpr::is_match("abc def"));
    }

    #[test]
    fn contains_match() {
        assert!(StartExpr::contains_match(""));
        assert!(StartExpr::contains_match("a"));
        assert!(StartExpr::contains_match("abc def"));
    }

    #[test]
    fn count_matches_non_overlapping() {
        assert_eq!(StartExpr::count_matches("", false), 1);
        assert_eq!(StartExpr::count_matches("a", false), 1);
        assert_eq!(StartExpr::count_matches("abc def", false), 1);
    }

    #[test]
    fn count_matches_overlapping() {
        assert_eq!(StartExpr::count_matches("", true), 1);
        assert_eq!(StartExpr::count_matches("a", true), 1);
        assert_eq!(StartExpr::count_matches("abc def", true), 1);
    }

    #[test]
    fn range_of_match() {
        assert_eq!(StartExpr::range_of_match(""), Some(0..0));
        assert_eq!(StartExpr::range_of_match("a"), Some(0..0));
        assert_eq!(StartExpr::range_of_match("abc def"), Some(0..0));
    }

    #[test]
    fn range_of_all_matches_non_overlapping() {
        assert_eq!(StartExpr::all_ranges(""), vec![0..0]);
        assert_eq!(StartExpr::all_ranges("a"), vec![0..0]);
        assert_eq!(StartExpr::all_ranges("abc def"), vec![0..0]);
    }

    #[test]
    fn range_of_all_matches_overlapping() {
        assert_eq!(StartExpr::all_ranges_overlap(""), vec![0..0]);
        assert_eq!(StartExpr::all_ranges_overlap("a"), vec![0..0]);
        assert_eq!(StartExpr::all_ranges_overlap("abc def"), vec![0..0]);
    }

    #[test]
    fn slice_match() {
        assert_eq!(StartExpr::slice_match(""), Some(""));
        assert_eq!(StartExpr::slice_match("a"), Some(""));
        assert_eq!(StartExpr::slice_match("abc def"), Some(""));
    }

    #[test]
    fn slice_all_matches_non_overlapping() {
        assert_eq!(StartExpr::all_slices(""), vec![""]);
        assert_eq!(StartExpr::all_slices("a"), vec![""]);
        assert_eq!(StartExpr::all_slices("abc def"), vec![""]);
    }


    #[test]
    fn slice_all_matches_overlapping() {
        assert_eq!(StartExpr::all_slices_overlap(""), vec![""]);
        assert_eq!(StartExpr::all_slices_overlap("a"), vec![""]);
        assert_eq!(StartExpr::all_slices_overlap("abc def"), vec![""]);
    }

    #[test]
    fn do_capture() {
        assert_eq!(StartExpr::whole_capture(""), Some((0..0, "")));
        assert_eq!(StartExpr::whole_capture("a"), None);
        assert_eq!(StartExpr::whole_capture("abc def"), None);
    }

    #[test]
    fn find_capture() {
        assert_eq!(StartExpr::first_capture(""), Some((0..0, "")));
        assert_eq!(StartExpr::first_capture("a"), Some((0..0, "")));
        assert_eq!(StartExpr::first_capture("abc def"), Some((0..0, "")));
    }

    #[allow(clippy::single_range_in_vec_init)]
    #[test]
    fn find_all_captures_non_overlapping() {
        assert_eq!(StartExpr::all_captures(""), zip_literal(vec![0..0], ""));
        assert_eq!(StartExpr::all_captures("a"), zip_literal(vec![0..0], ""));
        assert_eq!(StartExpr::all_captures("abc def"), zip_literal(vec![0..0], ""));
    }

    #[allow(clippy::single_range_in_vec_init)]
    #[test]
    fn find_all_captures_overlapping() {
        assert_eq!(StartExpr::all_captures_overlap(""), zip_literal(vec![0..0], ""));
        assert_eq!(StartExpr::all_captures_overlap("a"), zip_literal(vec![0..0], ""));
        assert_eq!(StartExpr::all_captures_overlap("abc def"), zip_literal(vec![0..0], ""));
    }

    #[test]
    fn replace() {
        assert_eq!(StartExpr::first_replaced("", "r"), (true, "r".into()));
        assert_eq!(StartExpr::first_replaced("a", "r"), (true, "ra".into()));
        assert_eq!(StartExpr::first_replaced("abc def", "r"), (true, "rabc def".into()));
    }

    #[test]
    fn replace_all() {
        assert_eq!(StartExpr::all_replaced("", "r"), (1, "r".into()));
        assert_eq!(StartExpr::all_replaced("a", "r"), (1, "ra".into()));
        assert_eq!(StartExpr::all_replaced("abc def", "r"), (1, "rabc def".into()));
    }

    #[test]
    fn replace_all_using() {
        assert_eq!(StartExpr::all_replaced_using(""), (1, "1".into()));
        assert_eq!(StartExpr::all_replaced_using("a"), (1, "1a".into()));
        assert_eq!(StartExpr::all_replaced_using("abc def"), (1, "1abc def".into()));
    }

    #[test]
    fn replace_using_iter() {
        assert_eq!(StartExpr::replaced_using_iter(""), (1, "1".into()));
        assert_eq!(StartExpr::replaced_using_iter("a"), (1, "1a".into()));
        assert_eq!(StartExpr::replaced_using_iter("abc def"), (1, "1abc def".into()));
    }

    #[test]
    fn replace_captured() {
        assert_eq!(StartExpr::capture_replaced_quoted(""), (true, "''".into()));
        assert_eq!(StartExpr::capture_replaced_quoted("a"), (true, "''a".into()));
        assert_eq!(StartExpr::capture_replaced_quoted("abc def"), (true, "''abc def".into()));
    }

    #[test]
    fn replace_all_captured() {
        assert_eq!(StartExpr::all_captures_replaced_quoted(""), (1, "''".into()));
        assert_eq!(StartExpr::all_captures_replaced_quoted("a"), (1, "''a".into()));
        assert_eq!(StartExpr::all_captures_replaced_quoted("abc def"), (1, "''abc def".into()));
    }
}

mod end {
    use super::*;

    regex! {
        pub EndExpr = r"$"
    }

    #[test]
    fn is_match() {
        assert!(EndExpr::is_match(""));
        assert!(!EndExpr::is_match("a"));
        assert!(!EndExpr::is_match("abc def"));
    }

    #[test]
    fn contains_match() {
        assert!(EndExpr::contains_match(""));
        assert!(EndExpr::contains_match("a"));
        assert!(EndExpr::contains_match("abc def"));
    }

    #[test]
    fn count_matches_non_overlapping() {
        assert_eq!(EndExpr::count_matches("", false), 1);
        assert_eq!(EndExpr::count_matches("a", false), 1);
        assert_eq!(EndExpr::count_matches("abc def", false), 1);
    }

    #[test]
    fn count_matches_overlapping() {
        assert_eq!(EndExpr::count_matches("", true), 1);
        assert_eq!(EndExpr::count_matches("a", true), 1);
        assert_eq!(EndExpr::count_matches("abc def", true), 1);
    }

    #[test]
    fn range_of_match() {
        assert_eq!(EndExpr::range_of_match(""), Some(0..0));
        assert_eq!(EndExpr::range_of_match("a"), Some(1..1));
        assert_eq!(EndExpr::range_of_match("abc def"), Some(7..7));
    }

    #[test]
    fn range_of_all_matches_non_overlapping() {
        assert_eq!(EndExpr::all_ranges(""), vec![0..0]);
        assert_eq!(EndExpr::all_ranges("a"), vec![1..1]);
        assert_eq!(EndExpr::all_ranges("abc def"), vec![7..7]);
    }

    #[test]
    fn range_of_all_matches_overlapping() {
        assert_eq!(EndExpr::all_ranges_overlap(""), vec![0..0]);
        assert_eq!(EndExpr::all_ranges_overlap("a"), vec![1..1]);
        assert_eq!(EndExpr::all_ranges_overlap("abc def"), vec![7..7]);
    }

    #[test]
    fn slice_match() {
        assert_eq!(EndExpr::slice_match(""), Some(""));
        assert_eq!(EndExpr::slice_match("a"), Some(""));
        assert_eq!(EndExpr::slice_match("abc def"), Some(""));
    }

    #[test]
    fn slice_all_matches_non_overlapping() {
        assert_eq!(EndExpr::all_slices(""), vec![""]);
        assert_eq!(EndExpr::all_slices("a"), vec![""]);
        assert_eq!(EndExpr::all_slices("abc def"), vec![""]);
    }


    #[test]
    fn slice_all_matches_overlapping() {
        assert_eq!(EndExpr::all_slices_overlap(""), vec![""]);
        assert_eq!(EndExpr::all_slices_overlap("a"), vec![""]);
        assert_eq!(EndExpr::all_slices_overlap("abc def"), vec![""]);
    }

    #[test]
    fn do_capture() {
        assert_eq!(EndExpr::whole_capture(""), Some((0..0, "")));
        assert_eq!(EndExpr::whole_capture("a"), None);
        assert_eq!(EndExpr::whole_capture("abc def"), None);
    }

    #[test]
    fn find_capture() {
        assert_eq!(EndExpr::first_capture(""), Some((0..0, "")));
        assert_eq!(EndExpr::first_capture("a"), Some((1..1, "")));
        assert_eq!(EndExpr::first_capture("abc def"), Some((7..7, "")));
    }

    #[allow(clippy::single_range_in_vec_init)]
    #[test]
    fn find_all_captures_non_overlapping() {
        assert_eq!(EndExpr::all_captures(""), zip_literal(vec![0..0], ""));
        assert_eq!(EndExpr::all_captures("a"), zip_literal(vec![1..1], ""));
        assert_eq!(EndExpr::all_captures("abc def"), zip_literal(vec![7..7], ""));
    }

    #[allow(clippy::single_range_in_vec_init)]
    #[test]
    fn find_all_captures_overlapping() {
        assert_eq!(EndExpr::all_captures_overlap(""), zip_literal(vec![0..0], ""));
        assert_eq!(EndExpr::all_captures_overlap("a"), zip_literal(vec![1..1], ""));
        assert_eq!(EndExpr::all_captures_overlap("abc def"), zip_literal(vec![7..7], ""));
    }

    #[test]
    fn replace() {
        assert_eq!(EndExpr::first_replaced("", "r"), (true, "r".into()));
        assert_eq!(EndExpr::first_replaced("a", "r"), (true, "ar".into()));
        assert_eq!(EndExpr::first_replaced("abc def", "r"), (true, "abc defr".into()));
    }

    #[test]
    fn replace_all() {
        assert_eq!(EndExpr::all_replaced("", "r"), (1, "r".into()));
        assert_eq!(EndExpr::all_replaced("a", "r"), (1, "ar".into()));
        assert_eq!(EndExpr::all_replaced("abc def", "r"), (1, "abc defr".into()));
    }

    #[test]
    fn replace_all_using() {
        assert_eq!(EndExpr::all_replaced_using(""), (1, "1".into()));
        assert_eq!(EndExpr::all_replaced_using("a"), (1, "a1".into()));
        assert_eq!(EndExpr::all_replaced_using("abc def"), (1, "abc def1".into()));
    }

    #[test]
    fn replace_using_iter() {
        assert_eq!(EndExpr::replaced_using_iter(""), (1, "1".into()));
        assert_eq!(EndExpr::replaced_using_iter("a"), (1, "a1".into()));
        assert_eq!(EndExpr::replaced_using_iter("abc def"), (1, "abc def1".into()));
    }

    #[test]
    fn replace_captured() {
        assert_eq!(EndExpr::capture_replaced_quoted(""), (true, "''".into()));
        assert_eq!(EndExpr::capture_replaced_quoted("a"), (true, "a''".into()));
        assert_eq!(EndExpr::capture_replaced_quoted("abc def"), (true, "abc def''".into()));
    }

    #[test]
    fn replace_all_captured() {
        assert_eq!(EndExpr::all_captures_replaced_quoted(""), (1, "''".into()));
        assert_eq!(EndExpr::all_captures_replaced_quoted("a"), (1, "a''".into()));
        assert_eq!(EndExpr::all_captures_replaced_quoted("abc def"), (1, "abc def''".into()));
    }
}

mod empty {
    use super::*;

    regex! {
        pub EmptyExpr = r""
    }

    #[test]
    fn is_match() {
        assert!(EmptyExpr::is_match(""));
        assert!(!EmptyExpr::is_match("a"));
        assert!(!EmptyExpr::is_match("abc def"));
    }

    #[test]
    fn contains_match() {
        assert!(EmptyExpr::contains_match(""));
        assert!(EmptyExpr::contains_match("a"));
        assert!(EmptyExpr::contains_match("abc def"));
    }

    #[test]
    fn count_matches_non_overlapping() {
        assert_eq!(EmptyExpr::count_matches("", false), 1);
        assert_eq!(EmptyExpr::count_matches("a", false), 2);
        assert_eq!(EmptyExpr::count_matches("abc def", false), 8);
    }

    #[test]
    fn count_matches_overlapping() {
        assert_eq!(EmptyExpr::count_matches("", true), 1);
        assert_eq!(EmptyExpr::count_matches("a", true), 2);
        assert_eq!(EmptyExpr::count_matches("abc def", true), 8);
    }

    #[test]
    fn range_of_match() {
        assert_eq!(EmptyExpr::range_of_match(""), Some(0..0));
        assert_eq!(EmptyExpr::range_of_match("a"), Some(0..0));
        assert_eq!(EmptyExpr::range_of_match("abc def"), Some(0..0));
    }

    #[test]
    fn range_of_all_matches_non_overlapping() {
        assert_eq!(EmptyExpr::all_ranges(""), vec![0..0]);
        assert_eq!(EmptyExpr::all_ranges("a"), vec![0..0, 1..1]);
        assert_eq!(
            EmptyExpr::all_ranges("abc def"),
            vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7]
        );
    }

    #[test]
    fn range_of_all_matches_overlapping() {
        assert_eq!(EmptyExpr::all_ranges_overlap(""), vec![0..0]);
        assert_eq!(EmptyExpr::all_ranges_overlap("a"), vec![0..0, 1..1]);
        assert_eq!(
            EmptyExpr::all_ranges_overlap("abc def"),
            vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7]
        );
    }

    #[test]
    fn slice_match() {
        assert_eq!(EmptyExpr::slice_match(""), Some(""));
        assert_eq!(EmptyExpr::slice_match("a"), Some(""));
        assert_eq!(EmptyExpr::slice_match("abc def"), Some(""));
    }

    #[test]
    fn slice_all_matches_non_overlapping() {
        assert_eq!(EmptyExpr::all_slices(""), vec![""]);
        assert_eq!(EmptyExpr::all_slices("a"), vec![""; 2]);
        assert_eq!(EmptyExpr::all_slices("abc def"), vec![""; 8]);
    }


    #[test]
    fn slice_all_matches_overlapping() {
        assert_eq!(EmptyExpr::all_slices_overlap(""), vec![""]);
        assert_eq!(EmptyExpr::all_slices_overlap("a"), vec![""; 2]);
        assert_eq!(EmptyExpr::all_slices_overlap("abc def"), vec![""; 8]);
    }

    #[test]
    fn do_capture() {
        assert_eq!(EmptyExpr::whole_capture(""), Some((0..0, "")));
        assert_eq!(EmptyExpr::whole_capture("a"), None);
        assert_eq!(EmptyExpr::whole_capture("abc def"), None);
    }

    #[test]
    fn find_capture() {
        assert_eq!(EmptyExpr::first_capture(""), Some((0..0, "")));
        assert_eq!(EmptyExpr::first_capture("a"), Some((0..0, "")));
        assert_eq!(EmptyExpr::first_capture("abc def"), Some((0..0, "")));
    }

    #[allow(clippy::single_range_in_vec_init)]
    #[test]
    fn find_all_captures_non_overlapping() {
        assert_eq!(EmptyExpr::all_captures(""), zip_literal(vec![0..0], ""));
        assert_eq!(EmptyExpr::all_captures("a"), zip_literal(vec![0..0, 1..1], ""));
        assert_eq!(
            EmptyExpr::all_captures("abc def"),
            zip_literal(vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7], "")
        );
    }

    #[allow(clippy::single_range_in_vec_init)]
    #[test]
    fn find_all_captures_overlapping() {
        assert_eq!(EmptyExpr::all_captures_overlap(""), zip_literal(vec![0..0], ""));
        assert_eq!(EmptyExpr::all_captures_overlap("a"), zip_literal(vec![0..0, 1..1], ""));
        assert_eq!(
            EmptyExpr::all_captures_overlap("abc def"),
            zip_literal(vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7], "")
        );
    }

    #[test]
    fn replace() {
        assert_eq!(EmptyExpr::first_replaced("", "r"), (true, "r".into()));
        assert_eq!(EmptyExpr::first_replaced("a", "r"), (true, "ra".into()));
        assert_eq!(EmptyExpr::first_replaced("abc def", "r"), (true, "rabc def".into()));
    }

    #[test]
    fn replace_all() {
        assert_eq!(EmptyExpr::all_replaced("", "r"), (1, "r".into()));
        assert_eq!(EmptyExpr::all_replaced("a", "r"), (2, "rar".into()));
        assert_eq!(EmptyExpr::all_replaced("abc def", "r"), (8, "rarbrcr rdrerfr".into()));
    }

    #[test]
    fn replace_all_using() {
        assert_eq!(EmptyExpr::all_replaced_using(""), (1, "1".into()));
        assert_eq!(EmptyExpr::all_replaced_using("a"), (2, "1a2".into()));
        assert_eq!(EmptyExpr::all_replaced_using("abc def"), (8, "1a2b3c4 5d6e7f8".into()));
    }

    #[test]
    fn replace_using_iter() {
        assert_eq!(EmptyExpr::replaced_using_iter(""), (1, "1".into()));
        assert_eq!(EmptyExpr::replaced_using_iter("a"), (2, "1a2".into()));
        assert_eq!(EmptyExpr::replaced_using_iter("abc def"), (2, "1a2bc def".into()));
    }

    #[test]
    fn replace_captured() {
        assert_eq!(EmptyExpr::capture_replaced_quoted(""), (true, "''".into()));
        assert_eq!(EmptyExpr::capture_replaced_quoted("a"), (true, "''a".into()));
        assert_eq!(EmptyExpr::capture_replaced_quoted("abc def"), (true, "''abc def".into()));
    }

    #[test]
    fn replace_all_captured() {
        assert_eq!(EmptyExpr::all_captures_replaced_quoted(""), (1, "''".into()));
        assert_eq!(EmptyExpr::all_captures_replaced_quoted("a"), (2, "''a''".into()));
        assert_eq!(EmptyExpr::all_captures_replaced_quoted("abc def"), (8, "''a''b''c'' ''d''e''f''".into()));
    }
}