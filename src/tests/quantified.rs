use std::ops::Range;

use super::*;

regex! {
    pub QuantifiedExpr = r"a[a-z]*"
}

fn cap_to_parts<'a>(cap: QuantifiedExprCapture<'a, &'a str>) -> (Range<usize>, &'a str) {
    (cap.whole_match_range(), cap.whole_match())
}

fn create_counter() -> impl FnMut() -> String {
    let mut count = 0;
    move || {
        count += 1;
        count.to_string()
    }
}

fn slice_first_byte<'a>(cap: QuantifiedExprCapture<'a, &'a str>) -> String {
    cap.whole_match()[1..].into()
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
    assert_eq!(QuantifiedExpr::count_matches("abc ade", false), 2);
    assert_eq!(QuantifiedExpr::count_matches("abc def a abcd", false), 3);
    assert_eq!(QuantifiedExpr::count_matches("12abc34", false), 1);
    assert_eq!(QuantifiedExpr::count_matches("abcadef", false), 1);
    assert_eq!(QuantifiedExpr::count_matches(" ", false), 0);
}

#[test]
fn count_matches_overlapping() {
    assert_eq!(QuantifiedExpr::count_matches("abc", true), 1);
    assert_eq!(QuantifiedExpr::count_matches("abc ade", true), 2);
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
    fn collect_all_ranges(hay: &str) -> Vec<Range<usize>> {
        QuantifiedExpr::range_of_all_matches(hay, false).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_ranges("abc"), vec![0..3]);
    assert_eq!(collect_all_ranges("abc ade"), vec![0..3, 4..7]);
    assert_eq!(collect_all_ranges("abc def a abcd"), vec![0..3, 8..9, 10..14]);
    assert_eq!(collect_all_ranges("12abc34"), vec![2..5]);
    assert_eq!(collect_all_ranges("abcadef"), vec![0..7]);
    assert_eq!(collect_all_ranges(" "), vec![0..1; 0]);
}

#[test]
fn range_of_all_matches_overlapping() {
    fn collect_all_ranges(hay: &str) -> Vec<Range<usize>> {
        QuantifiedExpr::range_of_all_matches(hay, true).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_ranges("abc"), vec![0..3]);
    assert_eq!(collect_all_ranges("abc ade"), vec![0..3, 4..7]);
    assert_eq!(collect_all_ranges("abc def a abcd"), vec![0..3, 8..9, 10..14]);
    assert_eq!(collect_all_ranges("12abc34"), vec![2..5]);
    assert_eq!(collect_all_ranges("abcadef"), vec![0..7, 3..7]);
    assert_eq!(collect_all_ranges(" "), vec![0..1; 0]);
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
    fn collect_all_slices(hay: &str) -> Vec<&str> {
        QuantifiedExpr::slice_all_matches(hay, false).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_slices("abc"), vec!["abc"]);
    assert_eq!(collect_all_slices("abc ade"), vec!["abc", "ade"]);
    assert_eq!(collect_all_slices("abc def a abcd"), vec!["abc", "a", "abcd"]);
    assert_eq!(collect_all_slices("12abc34"), vec!["abc"]);
    assert_eq!(collect_all_slices("abcadef"), vec!["abcadef"]);
    assert_eq!(collect_all_slices(" "), vec![""; 0]);
}

#[test]
fn slice_all_matches_overlapping() {
    fn collect_all_slices(hay: &str) -> Vec<&str> {
        QuantifiedExpr::slice_all_matches(hay, true).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_slices("abc"), vec!["abc"]);
    assert_eq!(collect_all_slices("abc ade"), vec!["abc", "ade"]);
    assert_eq!(collect_all_slices("abc def a abcd"), vec!["abc", "a", "abcd"]);
    assert_eq!(collect_all_slices("12abc34"), vec!["abc"]);
    assert_eq!(collect_all_slices("abcadef"), vec!["abcadef", "adef"]);
    assert_eq!(collect_all_slices(" "), vec![""; 0]);
}

#[test]
fn do_capture() {
    fn do_capture(hay: &str) -> Option<(Range<usize>, &str)> {
        QuantifiedExpr::do_capture(hay).map(cap_to_parts)
    }

    assert_eq!(do_capture("aaa"), Some((0..3, "aaa")));
    assert_eq!(do_capture("abc"), Some((0..3, "abc")));

    assert_eq!(do_capture(" abc"), None);
    assert_eq!(do_capture("abc "), None);
    assert_eq!(do_capture("abc def"), None);
    assert_eq!(do_capture(" "), None);
}

#[test]
fn find_capture() {
    fn find_capture(hay: &str) -> Option<(Range<usize>, &str)> {
        QuantifiedExpr::find_capture(hay).map(cap_to_parts)
    }

    assert_eq!(find_capture("aaa"), Some((0..3, "aaa")));
    assert_eq!(find_capture("abc"), Some((0..3, "abc")));
    assert_eq!(find_capture(" abc"), Some((1..4, "abc")));
    assert_eq!(find_capture("abc "), Some((0..3, "abc")));
    assert_eq!(find_capture("abc def"), Some((0..3, "abc")));

    assert_eq!(find_capture(" "), None);
}

#[test]
fn find_all_captures_non_overlapping() {
    fn collect_all_captures(hay: &str) -> Vec<(Range<usize>, &str)> {
        QuantifiedExpr::find_all_captures(hay, false).map(cap_to_parts).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_captures("abc"), vec![(0..3, "abc")]);
    assert_eq!(collect_all_captures("abc ade"), vec![(0..3, "abc"), (4..7, "ade")]);
    assert_eq!(collect_all_captures("abc def a abcd"), vec![(0..3, "abc"), (8..9, "a"), (10..14, "abcd")]);
    assert_eq!(collect_all_captures("12abc34"), vec![(2..5, "abc")]);
    assert_eq!(collect_all_captures("abcadef"), vec![(0..7, "abcadef")]);
    assert_eq!(collect_all_captures(" "), vec![(0..1, ""); 0]);
}

#[test]
fn find_all_captures_overlapping() {
    fn collect_all_captures(hay: &str) -> Vec<(Range<usize>, &str)> {
        QuantifiedExpr::find_all_captures(hay, true).map(cap_to_parts).collect::<Vec<_>>()
    }

    assert_eq!(collect_all_captures("abc"), vec![(0..3, "abc")]);
    assert_eq!(collect_all_captures("abc ade"), vec![(0..3, "abc"), (4..7, "ade")]);
    assert_eq!(collect_all_captures("abc def a abcd"), vec![(0..3, "abc"), (8..9, "a"), (10..14, "abcd")]);
    assert_eq!(collect_all_captures("12abc34"), vec![(2..5, "abc")]);
    assert_eq!(collect_all_captures("abcadef"), vec![(0..7, "abcadef"), (3..7, "adef")]);
    assert_eq!(collect_all_captures(" "), vec![(0..1, ""); 0]);
}

#[test]
fn replace() {
    fn replace(hay: &str, success: bool) -> String {
        let mut hay = String::from(hay);
        assert_eq!(QuantifiedExpr::replace(&mut hay, "new"), success);
        hay
    }

    assert_eq!(replace("aaa", true), "new");
    assert_eq!(replace("abc", true), "new");
    assert_eq!(replace(" abc", true), " new");
    assert_eq!(replace("abc ", true), "new ");
    assert_eq!(replace("abc def", true), "new def");

    assert_eq!(replace(" ", false), " ");
}

#[test]
fn replace_all() {
    fn replace_all(hay: &str, count: usize) -> String {
        let mut hay = String::from(hay);
        assert_eq!(QuantifiedExpr::replace_all(&mut hay, "new"), count);
        hay
    }

    assert_eq!(replace_all("abc", 1), "new");
    assert_eq!(replace_all("abc ade", 2), "new new");
    assert_eq!(replace_all("abc def a abcd", 3), "new def new new");
    assert_eq!(replace_all("12abc34", 1), "12new34");
    assert_eq!(replace_all("abcadef", 1), "new");
    assert_eq!(replace_all(" ", 0), " ");
}

#[test]
fn replace_all_using() {
    fn replace_all_using(hay: &str, count: usize) -> String {
        let mut hay = String::from(hay);
        assert_eq!(QuantifiedExpr::replace_all_using(&mut hay, create_counter()), count);
        hay
    }

    assert_eq!(replace_all_using("abc", 1), "1");
    assert_eq!(replace_all_using("abc ade", 2), "1 2");
    assert_eq!(replace_all_using("abc def a abcd", 3), "1 def 2 3");
    assert_eq!(replace_all_using("12abc34", 1), "12134");
    assert_eq!(replace_all_using("abcadef", 1), "1");
    assert_eq!(replace_all_using(" ", 0), " ");
}

#[test]
fn replace_using_iter() {
    fn replace_using_iter(hay: &str, count: usize) -> String {
        let iter = (1..=2).map(|n| n.to_string());

        let mut hay = String::from(hay);
        assert_eq!(QuantifiedExpr::replace_using_iter(&mut hay, iter), count);
        hay
    }

    assert_eq!(replace_using_iter("abc", 1), "1");
    assert_eq!(replace_using_iter("abc ade", 2), "1 2");
    assert_eq!(replace_using_iter("abc def a abcd", 2), "1 def 2 abcd");
    assert_eq!(replace_using_iter("12abc34", 1), "12134");
    assert_eq!(replace_using_iter("abcadef", 1), "1");
    assert_eq!(replace_using_iter(" ", 0), " ");
}

#[test]
fn replace_captured() {
    fn replace_captured(hay: &str, success: bool) -> String {
        let mut hay = String::from(hay);
        assert_eq!(QuantifiedExpr::replace_captured(&mut hay, slice_first_byte), success);
        hay
    }

    assert_eq!(replace_captured("aaa", true), "aa");
    assert_eq!(replace_captured("abc", true), "bc");
    assert_eq!(replace_captured(" abc", true), " bc");
    assert_eq!(replace_captured("abc ", true), "bc ");
    assert_eq!(replace_captured("abc def", true), "bc def");

    assert_eq!(replace_captured(" ", false), " ");
}

#[test]
fn replace_all_captured() {
    fn replace_all_captured(hay: &str, count: usize) -> String {
        let mut hay = String::from(hay);
        assert_eq!(QuantifiedExpr::replace_all_captured(&mut hay, slice_first_byte), count);
        hay
    }

    assert_eq!(replace_all_captured("abc", 1), "bc");
    assert_eq!(replace_all_captured("abc ade", 2), "bc de");
    assert_eq!(replace_all_captured("abc def a abcd", 3), "bc def  bcd");
    assert_eq!(replace_all_captured("12abc34", 1), "12bc34");
    assert_eq!(replace_all_captured("abcadef", 1), "bcadef");
    assert_eq!(replace_all_captured(" ", 0), " ");
}