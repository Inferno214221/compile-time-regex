use std::iter;
use std::ops::Range;

use ct_regex_internal::expr::Capture;

use super::*;

regex! {
    pub StartExpr = r"^"
}

regex! {
    pub EndExpr = r"$"
}

regex! {
    pub EmptyExpr = r""
}

fn cap_to_parts<'a, R, const N: usize>(cap: R::Capture<'a, &'a str>) -> (Range<usize>, &'a str)
where
    R: Regex<char, N>,
{
    (cap.whole_match_range(), cap.whole_match())
}

fn quote_capture<'a, R, const N: usize>(cap: R::Capture<'a, &'a str>) -> String
where
    R: Regex<char, N>,
{
    format!("'{}'", cap.whole_match())
}

fn create_counter() -> impl FnMut() -> String {
    let mut count = 0;
    move || {
        count += 1;
        count.to_string()
    }
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
    fn do_capture<R, const N: usize>(hay: &str) -> Option<(Range<usize>, &str)>
    where
        R: Regex<char, N>,
    {
        R::do_capture(hay).map(cap_to_parts::<R, _>)
    }

    assert_eq!(do_capture::<StartExpr, _>(""), Some((0..0, "")));
    assert_eq!(do_capture::<StartExpr, _>("a"), None);
    assert_eq!(do_capture::<StartExpr, _>("abc def"), None);

    assert_eq!(do_capture::<EndExpr, _>(""), Some((0..0, "")));
    assert_eq!(do_capture::<EndExpr, _>("a"), None);
    assert_eq!(do_capture::<EndExpr, _>("abc def"), None);

    assert_eq!(do_capture::<EmptyExpr, _>(""), Some((0..0, "")));
    assert_eq!(do_capture::<EmptyExpr, _>("a"), None);
    assert_eq!(do_capture::<EmptyExpr, _>("abc def"), None);
}

#[test]
fn find_capture() {
    fn find_capture<R, const N: usize>(hay: &str) -> Option<(Range<usize>, &str)>
    where
        R: Regex<char, N>,
    {
        R::find_capture(hay).map(cap_to_parts::<R, _>)
    }

    assert_eq!(find_capture::<StartExpr, _>(""), Some((0..0, "")));
    assert_eq!(find_capture::<StartExpr, _>("a"), Some((0..0, "")));
    assert_eq!(find_capture::<StartExpr, _>("abc def"), Some((0..0, "")));

    assert_eq!(find_capture::<EndExpr, _>(""), Some((0..0, "")));
    assert_eq!(find_capture::<EndExpr, _>("a"), Some((1..1, "")));
    assert_eq!(find_capture::<EndExpr, _>("abc def"), Some((7..7, "")));

    assert_eq!(find_capture::<EmptyExpr, _>(""), Some((0..0, "")));
    assert_eq!(find_capture::<EmptyExpr, _>("a"), Some((0..0, "")));
    assert_eq!(find_capture::<EmptyExpr, _>("abc def"), Some((0..0, "")));
}

#[allow(clippy::single_range_in_vec_init)]
#[test]
fn find_all_captures_non_overlapping() {
    fn collect_all_captures<R, const N: usize>(hay: &str) -> Vec<(Range<usize>, &str)>
    where
        R: Regex<char, N>,
    {
        R::find_all_captures(hay, false).map(cap_to_parts::<R, _>).collect::<Vec<_>>()
    }

    fn zip_literal(vec: Vec<Range<usize>>) -> Vec<(Range<usize>, &'static str)> {
        vec.into_iter().zip(
            iter::repeat("")
        ).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_captures::<StartExpr, _>(""), zip_literal(vec![0..0]));
    assert_eq!(collect_all_captures::<StartExpr, _>("a"), zip_literal(vec![0..0]));
    assert_eq!(collect_all_captures::<StartExpr, _>("abc def"), zip_literal(vec![0..0]));

    assert_eq!(collect_all_captures::<EndExpr, _>(""), zip_literal(vec![0..0]));
    assert_eq!(collect_all_captures::<EndExpr, _>("a"), zip_literal(vec![1..1]));
    assert_eq!(collect_all_captures::<EndExpr, _>("abc def"), zip_literal(vec![7..7]));

    assert_eq!(collect_all_captures::<EmptyExpr, _>(""), zip_literal(vec![0..0]));
    assert_eq!(collect_all_captures::<EmptyExpr, _>("a"), zip_literal(vec![0..0, 1..1]));
    assert_eq!(
        collect_all_captures::<EmptyExpr, _>("abc def"),
        zip_literal(vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7])
    );
}

#[allow(clippy::single_range_in_vec_init)]
#[test]
fn find_all_captures_overlapping() {
    fn collect_all_captures<R, const N: usize>(hay: &str) -> Vec<(Range<usize>, &str)>
    where
        R: Regex<char, N>,
    {
        R::find_all_captures(hay, true).map(cap_to_parts::<R, _>).collect::<Vec<_>>()
    }

    fn zip_literal(vec: Vec<Range<usize>>) -> Vec<(Range<usize>, &'static str)> {
        vec.into_iter().zip(
            iter::repeat("")
        ).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_captures::<StartExpr, _>(""), zip_literal(vec![0..0]));
    assert_eq!(collect_all_captures::<StartExpr, _>("a"), zip_literal(vec![0..0]));
    assert_eq!(collect_all_captures::<StartExpr, _>("abc def"), zip_literal(vec![0..0]));

    assert_eq!(collect_all_captures::<EndExpr, _>(""), zip_literal(vec![0..0]));
    assert_eq!(collect_all_captures::<EndExpr, _>("a"), zip_literal(vec![1..1]));
    assert_eq!(collect_all_captures::<EndExpr, _>("abc def"), zip_literal(vec![7..7]));

    assert_eq!(collect_all_captures::<EmptyExpr, _>(""), zip_literal(vec![0..0]));
    assert_eq!(collect_all_captures::<EmptyExpr, _>("a"), zip_literal(vec![0..0, 1..1]));
    assert_eq!(
        collect_all_captures::<EmptyExpr, _>("abc def"),
        zip_literal(vec![0..0, 1..1, 2..2, 3..3, 4..4, 5..5, 6..6, 7..7])
    );
}

#[test]
fn replace() {
    fn replace<R: Regex<char, N>, const N: usize>(hay: &str, success: bool) -> String {
        let mut hay = String::from(hay);
        assert_eq!(R::replace(&mut hay, "r"), success);
        hay
    }

    assert_eq!(replace::<StartExpr, _>("", true), "r");
    assert_eq!(replace::<StartExpr, _>("a", true), "ra");
    assert_eq!(replace::<StartExpr, _>("abc def", true), "rabc def");

    assert_eq!(replace::<EndExpr, _>("", true), "r");
    assert_eq!(replace::<EndExpr, _>("a", true), "ar");
    assert_eq!(replace::<EndExpr, _>("abc def", true), "abc defr");

    assert_eq!(replace::<EmptyExpr, _>("", true), "r");
    assert_eq!(replace::<EmptyExpr, _>("a", true), "ra");
    assert_eq!(replace::<EmptyExpr, _>("abc def", true), "rabc def");
}

#[test]
fn replace_all() {
    fn replace_all<R: Regex<char, N>, const N: usize>(hay: &str, count: usize) -> String {
        let mut hay = String::from(hay);
        assert_eq!(R::replace_all(&mut hay, "r"), count);
        hay
    }

    assert_eq!(replace_all::<StartExpr, _>("", 1), "r");
    assert_eq!(replace_all::<StartExpr, _>("a", 1), "ra");
    assert_eq!(replace_all::<StartExpr, _>("abc def", 1), "rabc def");

    assert_eq!(replace_all::<EndExpr, _>("", 1), "r");
    assert_eq!(replace_all::<EndExpr, _>("a", 1), "ar");
    assert_eq!(replace_all::<EndExpr, _>("abc def", 1), "abc defr");

    assert_eq!(replace_all::<EmptyExpr, _>("", 1), "r");
    assert_eq!(replace_all::<EmptyExpr, _>("a", 2), "rar");
    assert_eq!(replace_all::<EmptyExpr, _>("abc def", 8), "rarbrcr rdrerfr");
}

#[test]
fn replace_all_using() {
    fn replace_all_using<R: Regex<char, N>, const N: usize>(hay: &str, count: usize) -> String {
        let mut hay = String::from(hay);
        assert_eq!(R::replace_all_using(&mut hay, create_counter()), count);
        hay
    }

    assert_eq!(replace_all_using::<StartExpr, _>("", 1), "1");
    assert_eq!(replace_all_using::<StartExpr, _>("a", 1), "1a");
    assert_eq!(replace_all_using::<StartExpr, _>("abc def", 1), "1abc def");

    assert_eq!(replace_all_using::<EndExpr, _>("", 1), "1");
    assert_eq!(replace_all_using::<EndExpr, _>("a", 1), "a1");
    assert_eq!(replace_all_using::<EndExpr, _>("abc def", 1), "abc def1");

    assert_eq!(replace_all_using::<EmptyExpr, _>("", 1), "1");
    assert_eq!(replace_all_using::<EmptyExpr, _>("a", 2), "1a2");
    assert_eq!(replace_all_using::<EmptyExpr, _>("abc def", 8), "1a2b3c4 5d6e7f8");
}

#[test]
fn replace_using_iter() {
    fn replace_using_iter<R: Regex<char, N>, const N: usize>(hay: &str, count: usize) -> String {
        let iter = (1..=2).map(|n| n.to_string());
        let mut hay = String::from(hay);
        assert_eq!(R::replace_using_iter(&mut hay, iter), count);
        hay
    }

    assert_eq!(replace_using_iter::<StartExpr, _>("", 1), "1");
    assert_eq!(replace_using_iter::<StartExpr, _>("a", 1), "1a");
    assert_eq!(replace_using_iter::<StartExpr, _>("abc def", 1), "1abc def");

    assert_eq!(replace_using_iter::<EndExpr, _>("", 1), "1");
    assert_eq!(replace_using_iter::<EndExpr, _>("a", 1), "a1");
    assert_eq!(replace_using_iter::<EndExpr, _>("abc def", 1), "abc def1");

    assert_eq!(replace_using_iter::<EmptyExpr, _>("", 1), "1");
    assert_eq!(replace_using_iter::<EmptyExpr, _>("a", 2), "1a2");
    assert_eq!(replace_using_iter::<EmptyExpr, _>("abc def", 2), "1a2bc def");
}

#[test]
fn replace_captured() {
    fn replace_captured<R: Regex<char, N>, const N: usize>(hay: &str, success: bool) -> String {
        let mut hay = String::from(hay);
        assert_eq!(R::replace_captured(&mut hay, quote_capture::<R, _>), success);
        hay
    }

    assert_eq!(replace_captured::<StartExpr, _>("", true), "''");
    assert_eq!(replace_captured::<StartExpr, _>("a", true), "''a");
    assert_eq!(replace_captured::<StartExpr, _>("abc def", true), "''abc def");

    assert_eq!(replace_captured::<EndExpr, _>("", true), "''");
    assert_eq!(replace_captured::<EndExpr, _>("a", true), "a''");
    assert_eq!(replace_captured::<EndExpr, _>("abc def", true), "abc def''");

    assert_eq!(replace_captured::<EmptyExpr, _>("", true), "''");
    assert_eq!(replace_captured::<EmptyExpr, _>("a", true), "''a");
    assert_eq!(replace_captured::<EmptyExpr, _>("abc def", true), "''abc def");
}

#[test]
fn replace_all_captured() {
    fn replace_all_captured<R: Regex<char, N>, const N: usize>(hay: &str, count: usize) -> String {
        let mut hay = String::from(hay);
        assert_eq!(R::replace_all_captured(&mut hay, quote_capture::<R, _>), count);
        hay
    }

    assert_eq!(replace_all_captured::<StartExpr, _>("", 1), "''");
    assert_eq!(replace_all_captured::<StartExpr, _>("a", 1), "''a");
    assert_eq!(replace_all_captured::<StartExpr, _>("abc def", 1), "''abc def");

    assert_eq!(replace_all_captured::<EndExpr, _>("", 1), "''");
    assert_eq!(replace_all_captured::<EndExpr, _>("a", 1), "a''");
    assert_eq!(replace_all_captured::<EndExpr, _>("abc def", 1), "abc def''");

    assert_eq!(replace_all_captured::<EmptyExpr, _>("", 1), "''");
    assert_eq!(replace_all_captured::<EmptyExpr, _>("a", 2), "''a''");
    assert_eq!(replace_all_captured::<EmptyExpr, _>("abc def", 8), "''a''b''c'' ''d''e''f''");
}