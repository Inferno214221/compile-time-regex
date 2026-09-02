use std::iter;
use std::ops::Range;

use ct_regex_internal::expr::{Capture, Regex};

fn create_counter() -> impl FnMut() -> String {
    let mut count = 0;
    move || {
        count += 1;
        count.to_string()
    }
}

pub(crate) fn zip_literal(
    vec: Vec<Range<usize>>,
    lit: &'static str,
) -> Vec<(Range<usize>, &'static str)> {
    vec.into_iter().zip(iter::repeat(lit)).collect::<Vec<_>>()
}

pub(crate) trait RegexTestExt<const N: usize>: Regex<char, N> {
    fn cap_to_parts<'a>(cap: Self::Capture<'a, &'a str>) -> (Range<usize>, &'a str) {
        (cap.whole_match_range(), cap.whole_match())
    }

    fn quote_capture<'a>(cap: Self::Capture<'a, &'a str>) -> String {
        format!("'{}'", cap.whole_match())
    }

    fn slice_first_byte<'a>(cap: Self::Capture<'a, &'a str>) -> String {
        cap.whole_match()[1..].into()
    }

    fn all_ranges(hay: &str) -> Vec<Range<usize>> {
        Self::range_of_all_matches(hay, false).collect::<Vec<_>>()
    }

    fn all_ranges_overlap(hay: &str) -> Vec<Range<usize>> {
        Self::range_of_all_matches(hay, true).collect::<Vec<_>>()
    }

    fn all_slices(hay: &str) -> Vec<&str> {
        Self::slice_all_matches(hay, false).collect::<Vec<_>>()
    }

    fn all_slices_overlap(hay: &str) -> Vec<&str> {
        Self::slice_all_matches(hay, true).collect::<Vec<_>>()
    }

    fn whole_capture(hay: &str) -> Option<(Range<usize>, &str)> {
        Self::do_capture(hay).map(Self::cap_to_parts)
    }

    fn first_capture(hay: &str) -> Option<(Range<usize>, &str)> {
        Self::find_capture(hay).map(Self::cap_to_parts)
    }

    fn all_captures(hay: &str) -> Vec<(Range<usize>, &str)> {
        Self::find_all_captures(hay, false)
            .map(Self::cap_to_parts)
            .collect::<Vec<_>>()
    }

    fn all_captures_overlap(hay: &str) -> Vec<(Range<usize>, &str)> {
        Self::find_all_captures(hay, true)
            .map(Self::cap_to_parts)
            .collect::<Vec<_>>()
    }

    fn first_replaced(hay: &str, with: &str) -> (bool, String) {
        let mut hay = String::from(hay);
        (Self::replace(&mut hay, with), hay)
    }

    fn all_replaced(hay: &str, with: &str) -> (usize, String) {
        let mut hay = String::from(hay);
        (Self::replace_all(&mut hay, with), hay)
    }

    fn all_replaced_using(hay: &str) -> (usize, String) {
        let mut hay = String::from(hay);
        (Self::replace_all_using(&mut hay, create_counter()), hay)
    }

    fn replaced_using_iter(hay: &str) -> (usize, String) {
        let iter = (1..=2).map(|n| n.to_string());
        let mut hay = String::from(hay);
        (Self::replace_using_iter(&mut hay, iter), hay)
    }

    fn capture_replaced_quoted(hay: &str) -> (bool, String) {
        let mut hay = String::from(hay);
        (Self::replace_captured(&mut hay, Self::quote_capture), hay)
    }

    fn all_captures_replaced_quoted(hay: &str) -> (usize, String) {
        let mut hay = String::from(hay);
        (Self::replace_all_captured(&mut hay, Self::quote_capture), hay)
    }

    fn capture_replaced_sliced(hay: &str) -> (bool, String) {
        let mut hay = String::from(hay);
        (Self::replace_captured(&mut hay, Self::slice_first_byte), hay)
    }

    fn all_captures_replaced_sliced(hay: &str) -> (usize, String) {
        let mut hay = String::from(hay);
        (Self::replace_all_captured(&mut hay, Self::slice_first_byte), hay)
    }
}

impl<R: Regex<char, N>, const N: usize> RegexTestExt<N> for R {}