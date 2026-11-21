use super::*;

mod start {
    use super::*;
    use util::RegexTestScalar;

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
        assert_eq!(StartExpr::all_captures(""), StartExpr::zip_literal(vec![0..0], ""));
        assert_eq!(StartExpr::all_captures("a"), StartExpr::zip_literal(vec![0..0], ""));
        assert_eq!(StartExpr::all_captures("abc def"), StartExpr::zip_literal(vec![0..0], ""));
    }

    #[allow(clippy::single_range_in_vec_init)]
    #[test]
    fn find_all_captures_overlapping() {
        assert_eq!(StartExpr::all_captures_overlap(""), StartExpr::zip_literal(vec![0..0], ""));
        assert_eq!(StartExpr::all_captures_overlap("a"), StartExpr::zip_literal(vec![0..0], ""));
        assert_eq!(StartExpr::all_captures_overlap("abc def"), StartExpr::zip_literal(vec![0..0], ""));
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
    use util::RegexTestScalar;

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
        assert_eq!(EndExpr::all_captures(""), EndExpr::zip_literal(vec![0..0], ""));
        assert_eq!(EndExpr::all_captures("a"), EndExpr::zip_literal(vec![1..1], ""));
        assert_eq!(EndExpr::all_captures("abc def"), EndExpr::zip_literal(vec![7..7], ""));
    }

    #[allow(clippy::single_range_in_vec_init)]
    #[test]
    fn find_all_captures_overlapping() {
        assert_eq!(EndExpr::all_captures_overlap(""), EndExpr::zip_literal(vec![0..0], ""));
        assert_eq!(EndExpr::all_captures_overlap("a"), EndExpr::zip_literal(vec![1..1], ""));
        assert_eq!(EndExpr::all_captures_overlap("abc def"), EndExpr::zip_literal(vec![7..7], ""));
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

mod empty_scalar {
    use super::*;
    use util::RegexTestScalar;

    regex! {
        pub EmptyExpr = r""
    }

    #[test]
    fn is_match() {
        assert!(EmptyExpr::is_match(""));
        assert!(!EmptyExpr::is_match("a"));
        assert!(!EmptyExpr::is_match("abc def"));
        assert!(!EmptyExpr::is_match("😀🧑‍🔬"));
    }

    #[test]
    fn contains_match() {
        assert!(EmptyExpr::contains_match(""));
        assert!(EmptyExpr::contains_match("a"));
        assert!(EmptyExpr::contains_match("abc def"));
        assert!(EmptyExpr::contains_match("😀🧑‍🔬"));
    }

    #[test]
    fn count_matches_non_overlapping() {
        assert_eq!(EmptyExpr::count_matches("", false), 1);
        assert_eq!(EmptyExpr::count_matches("a", false), 2);
        assert_eq!(EmptyExpr::count_matches("abc def", false), 8);
        assert_eq!(EmptyExpr::count_matches("😀🧑‍🔬", false), 5);
    }

    #[test]
    fn count_matches_overlapping() {
        assert_eq!(EmptyExpr::count_matches("", true), 1);
        assert_eq!(EmptyExpr::count_matches("a", true), 2);
        assert_eq!(EmptyExpr::count_matches("abc def", true), 8);
        assert_eq!(EmptyExpr::count_matches("😀🧑‍🔬", true), 5);
    }

    #[test]
    fn range_of_match() {
        assert_eq!(EmptyExpr::range_of_match(""), Some(0..0));
        assert_eq!(EmptyExpr::range_of_match("a"), Some(0..0));
        assert_eq!(EmptyExpr::range_of_match("abc def"), Some(0..0));
        assert_eq!(EmptyExpr::range_of_match("😀🧑‍🔬"), Some(0..0));
    }

    #[test]
    fn range_of_all_matches_non_overlapping() {
        assert_eq!(EmptyExpr::all_ranges(""), vec![0..0]);
        assert_eq!(EmptyExpr::all_ranges("a"), vec![0..0, 1..1]);
        assert_eq!(
            EmptyExpr::all_ranges("abc def"),
            vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7]
        );
        assert_eq!(
            EmptyExpr::all_ranges("😀🧑‍🔬"),
            vec![0..0, 4..4, 8..8, 11..11, 15..15]
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
        assert_eq!(
            EmptyExpr::all_ranges_overlap("😀🧑‍🔬"),
            vec![0..0, 4..4, 8..8, 11..11, 15..15]
        );
    }

    #[test]
    fn slice_match() {
        assert_eq!(EmptyExpr::slice_match("".as_bytes()), Some("".as_bytes()));
        assert_eq!(EmptyExpr::slice_match("a".as_bytes()), Some("".as_bytes()));
        assert_eq!(EmptyExpr::slice_match("abc def".as_bytes()), Some("".as_bytes()));
        assert_eq!(EmptyExpr::slice_match("😀🧑‍🔬".as_bytes()), Some("".as_bytes()));
    }

    #[test]
    fn slice_all_matches_non_overlapping() {
        assert_eq!(EmptyExpr::all_slices(""), vec![""]);
        assert_eq!(EmptyExpr::all_slices("a"), vec![""; 2]);
        assert_eq!(EmptyExpr::all_slices("abc def"), vec![""; 8]);
        assert_eq!(EmptyExpr::all_slices("😀🧑‍🔬"), vec![""; 5]);
    }


    #[test]
    fn slice_all_matches_overlapping() {
        assert_eq!(EmptyExpr::all_slices_overlap(""), vec![""]);
        assert_eq!(EmptyExpr::all_slices_overlap("a"), vec![""; 2]);
        assert_eq!(EmptyExpr::all_slices_overlap("abc def"), vec![""; 8]);
        assert_eq!(EmptyExpr::all_slices_overlap("😀🧑‍🔬"), vec![""; 5]);
    }

    #[test]
    fn do_capture() {
        assert_eq!(EmptyExpr::whole_capture(""), Some((0..0, "")));
        assert_eq!(EmptyExpr::whole_capture("a"), None);
        assert_eq!(EmptyExpr::whole_capture("abc def"), None);
        assert_eq!(EmptyExpr::whole_capture("😀🧑‍🔬"), None);
    }

    #[test]
    fn find_capture() {
        assert_eq!(EmptyExpr::first_capture(""), Some((0..0, "")));
        assert_eq!(EmptyExpr::first_capture("a"), Some((0..0, "")));
        assert_eq!(EmptyExpr::first_capture("abc def"), Some((0..0, "")));
        assert_eq!(EmptyExpr::first_capture("😀🧑‍🔬"), Some((0..0, "")));
    }

    #[allow(clippy::single_range_in_vec_init)]
    #[test]
    fn find_all_captures_non_overlapping() {
        assert_eq!(EmptyExpr::all_captures(""), EmptyExpr::zip_literal(vec![0..0], ""));
        assert_eq!(EmptyExpr::all_captures("a"), EmptyExpr::zip_literal(vec![0..0, 1..1], ""));
        assert_eq!(
            EmptyExpr::all_captures("abc def"),
            EmptyExpr::zip_literal(vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7], "")
        );
        assert_eq!(
            EmptyExpr::all_captures("😀🧑‍🔬"),
            EmptyExpr::zip_literal(vec![0..0, 4..4, 8..8, 11..11, 15..15], "")
        );
    }

    #[allow(clippy::single_range_in_vec_init)]
    #[test]
    fn find_all_captures_overlapping() {
        assert_eq!(EmptyExpr::all_captures_overlap(""), EmptyExpr::zip_literal(vec![0..0], ""));
        assert_eq!(EmptyExpr::all_captures_overlap("a"), EmptyExpr::zip_literal(vec![0..0, 1..1], ""));
        assert_eq!(
            EmptyExpr::all_captures_overlap("abc def"),
            EmptyExpr::zip_literal(vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7], "")
        );
        assert_eq!(
            EmptyExpr::all_captures_overlap("😀🧑‍🔬"),
            EmptyExpr::zip_literal(vec![0..0, 4..4, 8..8, 11..11, 15..15], "")
        );
    }

    #[test]
    fn replace() {
        assert_eq!(EmptyExpr::first_replaced("", "r"), (true, "r".into()));
        assert_eq!(EmptyExpr::first_replaced("a", "r"), (true, "ra".into()));
        assert_eq!(EmptyExpr::first_replaced("abc def", "r"), (true, "rabc def".into()));
        assert_eq!(EmptyExpr::first_replaced("abc def", "r"), (true, "rabc def".into()));
        assert_eq!(EmptyExpr::first_replaced("😀🧑‍🔬", "r"), (true, "r😀🧑‍🔬".into()));
    }

    #[test]
    fn replace_all() {
        assert_eq!(EmptyExpr::all_replaced("", "r"), (1, "r".into()));
        assert_eq!(EmptyExpr::all_replaced("a", "r"), (2, "rar".into()));
        assert_eq!(EmptyExpr::all_replaced("abc def", "r"), (8, "rarbrcr rdrerfr".into()));
        assert_eq!(EmptyExpr::all_replaced("😀🧑‍🔬", "r"), (5, "r😀r🧑r\u{200d}r🔬r".into()));
    }

    #[test]
    fn replace_all_using() {
        assert_eq!(EmptyExpr::all_replaced_using(""), (1, "1".into()));
        assert_eq!(EmptyExpr::all_replaced_using("a"), (2, "1a2".into()));
        assert_eq!(EmptyExpr::all_replaced_using("abc def"), (8, "1a2b3c4 5d6e7f8".into()));
        assert_eq!(EmptyExpr::all_replaced_using("😀🧑‍🔬"), (5, "1😀2🧑3\u{200d}4🔬5".into()));
    }

    #[test]
    fn replace_using_iter() {
        assert_eq!(EmptyExpr::replaced_using_iter(""), (1, "1".into()));
        assert_eq!(EmptyExpr::replaced_using_iter("a"), (2, "1a2".into()));
        assert_eq!(EmptyExpr::replaced_using_iter("abc def"), (2, "1a2bc def".into()));
        assert_eq!(EmptyExpr::replaced_using_iter("😀🧑‍🔬"), (2, "1😀2🧑‍🔬".into()));
    }

    #[test]
    fn replace_captured() {
        assert_eq!(EmptyExpr::capture_replaced_quoted(""), (true, "''".into()));
        assert_eq!(EmptyExpr::capture_replaced_quoted("a"), (true, "''a".into()));
        assert_eq!(EmptyExpr::capture_replaced_quoted("abc def"), (true, "''abc def".into()));
        assert_eq!(EmptyExpr::capture_replaced_quoted("😀🧑‍🔬"), (true, "''😀🧑‍🔬".into()));
    }

    #[test]
    fn replace_all_captured() {
        assert_eq!(EmptyExpr::all_captures_replaced_quoted(""), (1, "''".into()));
        assert_eq!(EmptyExpr::all_captures_replaced_quoted("a"), (2, "''a''".into()));
        assert_eq!(EmptyExpr::all_captures_replaced_quoted("abc def"), (8, "''a''b''c'' ''d''e''f''".into()));
        assert_eq!(EmptyExpr::all_captures_replaced_quoted("😀🧑‍🔬"), (5, "''😀''🧑''\u{200d}''🔬''".into()));
    }
}

mod empty_byte {
    use super::*;
    use util::RegexTestByte;

    regex! {
        pub EmptyExpr = r""
    }

    #[test]
    fn is_match() {
        assert!(EmptyExpr::is_match("".as_bytes()));
        assert!(!EmptyExpr::is_match("a".as_bytes()));
        assert!(!EmptyExpr::is_match("abc def".as_bytes()));
        assert!(!EmptyExpr::is_match("😀🧑‍🔬".as_bytes()));
    }

    #[test]
    fn contains_match() {
        assert!(EmptyExpr::contains_match("".as_bytes()));
        assert!(EmptyExpr::contains_match("a".as_bytes()));
        assert!(EmptyExpr::contains_match("abc def".as_bytes()));
        assert!(EmptyExpr::contains_match("😀🧑‍🔬".as_bytes()));
    }

    #[test]
    fn count_matches_non_overlapping() {
        assert_eq!(EmptyExpr::count_matches("".as_bytes(), false), 1);
        assert_eq!(EmptyExpr::count_matches("a".as_bytes(), false), 2);
        assert_eq!(EmptyExpr::count_matches("abc def".as_bytes(), false), 8);
        assert_eq!(EmptyExpr::count_matches("😀🧑‍🔬".as_bytes(), false), 5);
    }

    #[test]
    fn count_matches_overlapping() {
        assert_eq!(EmptyExpr::count_matches("".as_bytes(), true), 1);
        assert_eq!(EmptyExpr::count_matches("a".as_bytes(), true), 2);
        assert_eq!(EmptyExpr::count_matches("abc def".as_bytes(), true), 8);
        assert_eq!(EmptyExpr::count_matches("😀🧑‍🔬".as_bytes(), true), 5);
    }

    #[test]
    fn range_of_match() {
        assert_eq!(EmptyExpr::range_of_match("".as_bytes()), Some(0..0));
        assert_eq!(EmptyExpr::range_of_match("a".as_bytes()), Some(0..0));
        assert_eq!(EmptyExpr::range_of_match("abc def".as_bytes()), Some(0..0));
        assert_eq!(EmptyExpr::range_of_match("😀🧑‍🔬".as_bytes()), Some(0..0));
    }

    #[test]
    fn range_of_all_matches_non_overlapping() {
        assert_eq!(EmptyExpr::all_ranges(b""), vec![0..0]);
        assert_eq!(EmptyExpr::all_ranges(b"a"), vec![0..0, 1..1]);
        assert_eq!(
            EmptyExpr::all_ranges(b"abc def"),
            vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7]
        );
        assert_eq!(
            EmptyExpr::all_ranges("😀🧑‍🔬".as_bytes()),
            vec![0..0, 4..4, 8..8, 11..11, 15..15]
        );
    }

    #[test]
    fn range_of_all_matches_overlapping() {
        assert_eq!(EmptyExpr::all_ranges_overlap(b""), vec![0..0]);
        assert_eq!(EmptyExpr::all_ranges_overlap(b"a"), vec![0..0, 1..1]);
        assert_eq!(
            EmptyExpr::all_ranges_overlap(b"abc def"),
            vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7]
        );
        assert_eq!(
            EmptyExpr::all_ranges_overlap("😀🧑‍🔬".as_bytes()),
            vec![0..0, 4..4, 8..8, 11..11, 15..15]
        );
    }

    #[test]
    fn slice_match() {
        assert_eq!(EmptyExpr::slice_match("".as_bytes()), Some("".as_bytes()));
        assert_eq!(EmptyExpr::slice_match("a".as_bytes()), Some("".as_bytes()));
        assert_eq!(EmptyExpr::slice_match("abc def".as_bytes()), Some("".as_bytes()));
        assert_eq!(EmptyExpr::slice_match("😀🧑‍🔬".as_bytes()), Some("".as_bytes()));
    }

    #[test]
    fn slice_all_matches_non_overlapping() {
        assert_eq!(EmptyExpr::all_slices(b""), vec![b""]);
        assert_eq!(EmptyExpr::all_slices(b"a"), vec![b""; 2]);
        assert_eq!(EmptyExpr::all_slices(b"abc def"), vec![b""; 8]);
        assert_eq!(EmptyExpr::all_slices("😀🧑‍🔬".as_bytes()), vec![b""; 5]);
    }


    #[test]
    fn slice_all_matches_overlapping() {
        assert_eq!(EmptyExpr::all_slices_overlap(b""), vec![b""]);
        assert_eq!(EmptyExpr::all_slices_overlap(b"a"), vec![b""; 2]);
        assert_eq!(EmptyExpr::all_slices_overlap(b"abc def"), vec![b""; 8]);
        assert_eq!(EmptyExpr::all_slices_overlap("😀🧑‍🔬".as_bytes()), vec![b""; 5]);
    }

    #[test]
    fn do_capture() {
        assert_eq!(EmptyExpr::whole_capture(b""), Some((0..0, "".as_bytes())));
        assert_eq!(EmptyExpr::whole_capture(b"a"), None);
        assert_eq!(EmptyExpr::whole_capture(b"abc def"), None);
        assert_eq!(EmptyExpr::whole_capture("😀🧑‍🔬".as_bytes()), None);
    }

    #[test]
    fn find_capture() {
        assert_eq!(EmptyExpr::first_capture(b""), Some((0..0, "".as_bytes())));
        assert_eq!(EmptyExpr::first_capture(b"a"), Some((0..0, "".as_bytes())));
        assert_eq!(EmptyExpr::first_capture(b"abc def"), Some((0..0, "".as_bytes())));
        assert_eq!(EmptyExpr::first_capture("😀🧑‍🔬".as_bytes()), Some((0..0, "".as_bytes())));
    }

    #[allow(clippy::single_range_in_vec_init)]
    #[test]
    fn find_all_captures_non_overlapping() {
        assert_eq!(EmptyExpr::all_captures(b""), EmptyExpr::zip_literal(vec![0..0], b""));
        assert_eq!(EmptyExpr::all_captures(b"a"), EmptyExpr::zip_literal(vec![0..0, 1..1], b""));
        assert_eq!(
            EmptyExpr::all_captures(b"abc def"),
            EmptyExpr::zip_literal(vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7], b"")
        );
        assert_eq!(
            EmptyExpr::all_captures("😀🧑‍🔬".as_bytes()),
            EmptyExpr::zip_literal(vec![0..0, 4..4, 8..8, 11..11, 15..15], b"")
        );
    }

    #[allow(clippy::single_range_in_vec_init)]
    #[test]
    fn find_all_captures_overlapping() {
        assert_eq!(EmptyExpr::all_captures_overlap(b""), EmptyExpr::zip_literal(vec![0..0], b""));
        assert_eq!(EmptyExpr::all_captures_overlap(b"a"), EmptyExpr::zip_literal(vec![0..0, 1..1], b""));
        assert_eq!(
            EmptyExpr::all_captures_overlap(b"abc def"),
            EmptyExpr::zip_literal(vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7], b"")
        );
        assert_eq!(
            EmptyExpr::all_captures_overlap("😀🧑‍🔬".as_bytes()),
            EmptyExpr::zip_literal(vec![0..0, 4..4, 8..8, 11..11, 15..15], b"")
        );
    }

    #[test]
    fn replace() {
        assert_eq!(EmptyExpr::first_replaced(b"", b"r"), (true, "r".into()));
        assert_eq!(EmptyExpr::first_replaced(b"a", b"r"), (true, "ra".into()));
        assert_eq!(EmptyExpr::first_replaced(b"abc def", b"r"), (true, "rabc def".into()));
        assert_eq!(EmptyExpr::first_replaced(b"abc def", b"r"), (true, "rabc def".into()));
        assert_eq!(EmptyExpr::first_replaced("😀🧑‍🔬".as_bytes(), b"r"), (true, "r😀🧑‍🔬".into()));
    }

    #[test]
    fn replace_all() {
        assert_eq!(EmptyExpr::all_replaced(b"", b"r"), (1, "r".into()));
        assert_eq!(EmptyExpr::all_replaced(b"a", b"r"), (2, "rar".into()));
        assert_eq!(EmptyExpr::all_replaced(b"abc def", b"r"), (8, "rarbrcr rdrerfr".into()));
        assert_eq!(EmptyExpr::all_replaced("😀🧑‍🔬".as_bytes(), b"r"), (5, "r😀r🧑r\u{200d}r🔬r".into()));
    }

    #[test]
    fn replace_all_using() {
        assert_eq!(EmptyExpr::all_replaced_using(b""), (1, "1".into()));
        assert_eq!(EmptyExpr::all_replaced_using(b"a"), (2, "1a2".into()));
        assert_eq!(EmptyExpr::all_replaced_using(b"abc def"), (8, "1a2b3c4 5d6e7f8".into()));
        assert_eq!(EmptyExpr::all_replaced_using("😀🧑‍🔬".as_bytes()), (5, "1😀2🧑3\u{200d}4🔬5".into()));
    }

    #[test]
    fn replace_using_iter() {
        assert_eq!(EmptyExpr::replaced_using_iter(b""), (1, "1".into()));
        assert_eq!(EmptyExpr::replaced_using_iter(b"a"), (2, "1a2".into()));
        assert_eq!(EmptyExpr::replaced_using_iter(b"abc def"), (2, "1a2bc def".into()));
        assert_eq!(EmptyExpr::replaced_using_iter("😀🧑‍🔬".as_bytes()), (2, "1😀2🧑‍🔬".into()));
    }

    #[test]
    fn replace_captured() {
        assert_eq!(EmptyExpr::capture_replaced_quoted(b""), (true, "''".into()));
        assert_eq!(EmptyExpr::capture_replaced_quoted(b"a"), (true, "''a".into()));
        assert_eq!(EmptyExpr::capture_replaced_quoted(b"abc def"), (true, "''abc def".into()));
        assert_eq!(EmptyExpr::capture_replaced_quoted("😀🧑‍🔬".as_bytes()), (true, "''😀🧑‍🔬".into()));
    }

    #[test]
    fn replace_all_captured() {
        assert_eq!(EmptyExpr::all_captures_replaced_quoted(b""), (1, "''".into()));
        assert_eq!(EmptyExpr::all_captures_replaced_quoted(b"a"), (2, "''a''".into()));
        assert_eq!(EmptyExpr::all_captures_replaced_quoted(b"abc def"), (8, "''a''b''c'' ''d''e''f''".into()));
        assert_eq!(EmptyExpr::all_captures_replaced_quoted("😀🧑‍🔬".as_bytes()), (5, "''😀''🧑''\u{200d}''🔬''".into()));
    }
}