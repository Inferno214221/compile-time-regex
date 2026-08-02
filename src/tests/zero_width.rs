use std::iter;
use std::ops::Range;

use super::*;

regex! {
    pub StartExpr = r"^"
}

fn start_cap_to_parts<'a>(cap: StartExprCapture<'a, &'a str>) -> (Range<usize>, &'a str) {
    (cap.whole_match_range(), cap.whole_match())
}

regex! {
    pub EndExpr = r"$"
}

fn end_cap_to_parts<'a>(cap: EndExprCapture<'a, &'a str>) -> (Range<usize>, &'a str) {
    (cap.whole_match_range(), cap.whole_match())
}

regex! {
    pub EmptyExpr = r""
}

fn empty_cap_to_parts<'a>(cap: EmptyExprCapture<'a, &'a str>) -> (Range<usize>, &'a str) {
    (cap.whole_match_range(), cap.whole_match())
}

#[test]
fn is_match() {
    assert!(StartExpr::is_match(""));
    assert!(!StartExpr::is_match("a"));
    assert!(!StartExpr::is_match("abc def"));

    assert!(EndExpr::is_match(""));
    assert!(!EndExpr::is_match("a"));
    assert!(!EndExpr::is_match("abc def"));

    assert!(EmptyExpr::is_match(""));
    assert!(!EmptyExpr::is_match("a"));
    assert!(!EmptyExpr::is_match("abc def"));
}

#[test]
fn contains_match() {
    assert!(StartExpr::contains_match(""));
    assert!(StartExpr::contains_match("a"));
    assert!(StartExpr::contains_match("abc def"));

    assert!(EndExpr::contains_match(""));
    assert!(EndExpr::contains_match("a"));
    assert!(EndExpr::contains_match("abc def"));

    assert!(EmptyExpr::contains_match(""));
    assert!(EmptyExpr::contains_match("a"));
    assert!(EmptyExpr::contains_match("abc def"));
}

#[test]
fn count_matches_non_overlapping() {
    assert_eq!(StartExpr::count_matches("", false), 1);
    assert_eq!(StartExpr::count_matches("a", false), 1);
    assert_eq!(StartExpr::count_matches("abc def", false), 1);

    assert_eq!(EndExpr::count_matches("", false), 1);
    assert_eq!(EndExpr::count_matches("a", false), 1);
    assert_eq!(EndExpr::count_matches("abc def", false), 1);

    assert_eq!(EmptyExpr::count_matches("", false), 1);
    assert_eq!(EmptyExpr::count_matches("a", false), 2);
    assert_eq!(EmptyExpr::count_matches("abc def", false), 8);
}

#[test]
fn count_matches_overlapping() {
    assert_eq!(StartExpr::count_matches("", true), 1);
    assert_eq!(StartExpr::count_matches("a", true), 1);
    assert_eq!(StartExpr::count_matches("abc def", true), 1);

    assert_eq!(EndExpr::count_matches("", true), 1);
    assert_eq!(EndExpr::count_matches("a", true), 1);
    assert_eq!(EndExpr::count_matches("abc def", true), 1);

    assert_eq!(EmptyExpr::count_matches("", true), 1);
    assert_eq!(EmptyExpr::count_matches("a", true), 2);
    assert_eq!(EmptyExpr::count_matches("abc def", true), 8);
}

#[test]
fn range_of_match() {
    assert_eq!(StartExpr::range_of_match(""), Some(0..0));
    assert_eq!(StartExpr::range_of_match("a"), Some(0..0));
    assert_eq!(StartExpr::range_of_match("abc def"), Some(0..0));

    assert_eq!(EndExpr::range_of_match(""), Some(0..0));
    assert_eq!(EndExpr::range_of_match("a"), Some(1..1));
    assert_eq!(EndExpr::range_of_match("abc def"), Some(7..7));

    assert_eq!(EmptyExpr::range_of_match(""), Some(0..0));
    assert_eq!(EmptyExpr::range_of_match("a"), Some(0..0));
    assert_eq!(EmptyExpr::range_of_match("abc def"), Some(0..0));
}

#[test]
fn range_of_all_matches_non_overlapping() {
    fn collect_all_ranges<R: Regex<char, N>, const N: usize>(hay: &str) -> Vec<Range<usize>> {
        R::range_of_all_matches(hay, false).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_ranges::<StartExpr, _>(""), vec![0..0]);
    assert_eq!(collect_all_ranges::<StartExpr, _>("a"), vec![0..0]);
    assert_eq!(collect_all_ranges::<StartExpr, _>("abc def"), vec![0..0]);

    assert_eq!(collect_all_ranges::<EndExpr, _>(""), vec![0..0]);
    assert_eq!(collect_all_ranges::<EndExpr, _>("a"), vec![1..1]);
    assert_eq!(collect_all_ranges::<EndExpr, _>("abc def"), vec![7..7]);

    assert_eq!(collect_all_ranges::<EmptyExpr, _>(""), vec![0..0]);
    assert_eq!(collect_all_ranges::<EmptyExpr, _>("a"), vec![0..0, 1..1]);
    assert_eq!(
        collect_all_ranges::<EmptyExpr, _>("abc def"),
        vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7]
    );
}

#[test]
fn range_of_all_matches_overlapping() {
    fn collect_all_ranges<R: Regex<char, N>, const N: usize>(hay: &str) -> Vec<Range<usize>> {
        R::range_of_all_matches(hay, true).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_ranges::<StartExpr, _>(""), vec![0..0]);
    assert_eq!(collect_all_ranges::<StartExpr, _>("a"), vec![0..0]);
    assert_eq!(collect_all_ranges::<StartExpr, _>("abc def"), vec![0..0]);

    assert_eq!(collect_all_ranges::<EndExpr, _>(""), vec![0..0]);
    assert_eq!(collect_all_ranges::<EndExpr, _>("a"), vec![1..1]);
    assert_eq!(collect_all_ranges::<EndExpr, _>("abc def"), vec![7..7]);

    assert_eq!(collect_all_ranges::<EmptyExpr, _>(""), vec![0..0]);
    assert_eq!(collect_all_ranges::<EmptyExpr, _>("a"), vec![0..0, 1..1]);
    assert_eq!(
        collect_all_ranges::<EmptyExpr, _>("abc def"),
        vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7]
    );
}

#[test]
fn slice_match() {
    assert_eq!(StartExpr::slice_match(""), Some(""));
    assert_eq!(StartExpr::slice_match("a"), Some(""));
    assert_eq!(StartExpr::slice_match("abc def"), Some(""));

    assert_eq!(EndExpr::slice_match(""), Some(""));
    assert_eq!(EndExpr::slice_match("a"), Some(""));
    assert_eq!(EndExpr::slice_match("abc def"), Some(""));

    assert_eq!(EmptyExpr::slice_match(""), Some(""));
    assert_eq!(EmptyExpr::slice_match("a"), Some(""));
    assert_eq!(EmptyExpr::slice_match("abc def"), Some(""));
}

#[test]
fn slice_all_matches_non_overlapping() {
    fn collect_all_ranges<R: Regex<char, N>, const N: usize>(hay: &str) -> Vec<&str> {
        R::slice_all_matches(hay, false).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_ranges::<StartExpr, _>(""), vec![""]);
    assert_eq!(collect_all_ranges::<StartExpr, _>("a"), vec![""]);
    assert_eq!(collect_all_ranges::<StartExpr, _>("abc def"), vec![""]);

    assert_eq!(collect_all_ranges::<EndExpr, _>(""), vec![""]);
    assert_eq!(collect_all_ranges::<EndExpr, _>("a"), vec![""]);
    assert_eq!(collect_all_ranges::<EndExpr, _>("abc def"), vec![""]);

    assert_eq!(collect_all_ranges::<EmptyExpr, _>(""), vec![""]);
    assert_eq!(collect_all_ranges::<EmptyExpr, _>("a"), vec![""; 2]);
    assert_eq!(collect_all_ranges::<EmptyExpr, _>("abc def"), vec![""; 8]);
}


#[test]
fn slice_all_matches_overlapping() {
    fn collect_all_slices<R: Regex<char, N>, const N: usize>(hay: &str) -> Vec<&str> {
        R::slice_all_matches(hay, true).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_slices::<StartExpr, _>(""), vec![""]);
    assert_eq!(collect_all_slices::<StartExpr, _>("a"), vec![""]);
    assert_eq!(collect_all_slices::<StartExpr, _>("abc def"), vec![""]);

    assert_eq!(collect_all_slices::<EndExpr, _>(""), vec![""]);
    assert_eq!(collect_all_slices::<EndExpr, _>("a"), vec![""]);
    assert_eq!(collect_all_slices::<EndExpr, _>("abc def"), vec![""]);

    assert_eq!(collect_all_slices::<EmptyExpr, _>(""), vec![""]);
    assert_eq!(collect_all_slices::<EmptyExpr, _>("a"), vec![""; 2]);
    assert_eq!(collect_all_slices::<EmptyExpr, _>("abc def"), vec![""; 8]);
}

#[test]
fn do_capture() {
    fn do_capture<'a, R, F, const N: usize>(hay: &'a str, f: F) -> Option<(Range<usize>, &'a str)>
    where
        R: Regex<char, N>,
        F: FnOnce(<R as Regex<char, N>>::Capture<'a, &'a str>) -> (Range<usize>, &'a str),
    {
        R::do_capture(hay).map(f)
    }

    assert_eq!(do_capture::<StartExpr, _, _>("", start_cap_to_parts), Some((0..0, "")));
    assert_eq!(do_capture::<StartExpr, _, _>("a", start_cap_to_parts), None);
    assert_eq!(do_capture::<StartExpr, _, _>("abc def", start_cap_to_parts), None);

    assert_eq!(do_capture::<EndExpr, _, _>("", end_cap_to_parts), Some((0..0, "")));
    assert_eq!(do_capture::<EndExpr, _, _>("a", end_cap_to_parts), None);
    assert_eq!(do_capture::<EndExpr, _, _>("abc def", end_cap_to_parts), None);

    assert_eq!(do_capture::<EmptyExpr, _, _>("", empty_cap_to_parts), Some((0..0, "")));
    assert_eq!(do_capture::<EmptyExpr, _, _>("a", empty_cap_to_parts), None);
    assert_eq!(do_capture::<EmptyExpr, _, _>("abc def", empty_cap_to_parts), None);
}

#[test]
fn find_capture() {
    fn find_capture<'a, R, F, const N: usize>(hay: &'a str, f: F) -> Option<(Range<usize>, &'a str)>
    where
        R: Regex<char, N>,
        F: FnOnce(<R as Regex<char, N>>::Capture<'a, &'a str>) -> (Range<usize>, &'a str),
    {
        R::find_capture(hay).map(f)
    }

    assert_eq!(find_capture::<StartExpr, _, _>("", start_cap_to_parts), Some((0..0, "")));
    assert_eq!(find_capture::<StartExpr, _, _>("a", start_cap_to_parts), Some((0..0, "")));
    assert_eq!(find_capture::<StartExpr, _, _>("abc def", start_cap_to_parts), Some((0..0, "")));

    assert_eq!(find_capture::<EndExpr, _, _>("", end_cap_to_parts), Some((0..0, "")));
    assert_eq!(find_capture::<EndExpr, _, _>("a", end_cap_to_parts), Some((1..1, "")));
    assert_eq!(find_capture::<EndExpr, _, _>("abc def", end_cap_to_parts), Some((7..7, "")));

    assert_eq!(find_capture::<EmptyExpr, _, _>("", empty_cap_to_parts), Some((0..0, "")));
    assert_eq!(find_capture::<EmptyExpr, _, _>("a", empty_cap_to_parts), Some((0..0, "")));
    assert_eq!(find_capture::<EmptyExpr, _, _>("abc def", empty_cap_to_parts), Some((0..0, "")));
}

#[allow(clippy::single_range_in_vec_init)]
#[test]
fn find_all_captures_non_overlapping() {
    fn collect_all_captures<'a, R, F, const N: usize>(hay: &'a str, f: F) -> Vec<(Range<usize>, &'a str)>
    where
        R: Regex<char, N>,
        F: FnMut(<R as Regex<char, N>>::Capture<'a, &'a str>) -> (Range<usize>, &'a str),
    {
        R::find_all_captures(hay, false).map(f).collect::<Vec<_>>()
    }

    fn zip_literal(vec: Vec<Range<usize>>) -> Vec<(Range<usize>, &'static str)> {
        vec.into_iter().zip(
            iter::repeat("")
        ).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_captures::<StartExpr, _, _>("", start_cap_to_parts), zip_literal(vec![0..0]));
    assert_eq!(collect_all_captures::<StartExpr, _, _>("a", start_cap_to_parts), zip_literal(vec![0..0]));
    assert_eq!(collect_all_captures::<StartExpr, _, _>("abc def", start_cap_to_parts), zip_literal(vec![0..0]));

    assert_eq!(collect_all_captures::<EndExpr, _, _>("", end_cap_to_parts), zip_literal(vec![0..0]));
    assert_eq!(collect_all_captures::<EndExpr, _, _>("a", end_cap_to_parts), zip_literal(vec![1..1]));
    assert_eq!(collect_all_captures::<EndExpr, _, _>("abc def", end_cap_to_parts), zip_literal(vec![7..7]));

    assert_eq!(collect_all_captures::<EmptyExpr, _, _>("", empty_cap_to_parts), zip_literal(vec![0..0]));
    assert_eq!(collect_all_captures::<EmptyExpr, _, _>("a", empty_cap_to_parts), zip_literal(vec![0..0, 1..1]));
    assert_eq!(
        collect_all_captures::<EmptyExpr, _, _>("abc def", empty_cap_to_parts),
        zip_literal(vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7])
    );
}

#[allow(clippy::single_range_in_vec_init)]
#[test]
fn find_all_captures_overlapping() {
    fn collect_all_captures<'a, R, F, const N: usize>(hay: &'a str, f: F) -> Vec<(Range<usize>, &'a str)>
    where
        R: Regex<char, N>,
        F: FnMut(<R as Regex<char, N>>::Capture<'a, &'a str>) -> (Range<usize>, &'a str),
    {
        R::find_all_captures(hay, true).map(f).collect::<Vec<_>>()
    }

    fn zip_literal(vec: Vec<Range<usize>>) -> Vec<(Range<usize>, &'static str)> {
        vec.into_iter().zip(
            iter::repeat("")
        ).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_captures::<StartExpr, _, _>("", start_cap_to_parts), zip_literal(vec![0..0]));
    assert_eq!(collect_all_captures::<StartExpr, _, _>("a", start_cap_to_parts), zip_literal(vec![0..0]));
    assert_eq!(collect_all_captures::<StartExpr, _, _>("abc def", start_cap_to_parts), zip_literal(vec![0..0]));

    assert_eq!(collect_all_captures::<EndExpr, _, _>("", end_cap_to_parts), zip_literal(vec![0..0]));
    assert_eq!(collect_all_captures::<EndExpr, _, _>("a", end_cap_to_parts), zip_literal(vec![1..1]));
    assert_eq!(collect_all_captures::<EndExpr, _, _>("abc def", end_cap_to_parts), zip_literal(vec![7..7]));

    assert_eq!(collect_all_captures::<EmptyExpr, _, _>("", empty_cap_to_parts), zip_literal(vec![0..0]));
    assert_eq!(collect_all_captures::<EmptyExpr, _, _>("a", empty_cap_to_parts), zip_literal(vec![0..0, 1..1]));
    assert_eq!(
        collect_all_captures::<EmptyExpr, _, _>("abc def", empty_cap_to_parts),
        zip_literal(vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7])
    );
}

// TODO: Replace tests.