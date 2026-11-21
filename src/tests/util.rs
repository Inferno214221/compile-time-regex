use std::iter;
use std::ops::Range;

use ct_regex_internal::expr::{Capture, Regex};

pub(crate) trait RegexTestScalar<const N: usize>: Regex<char, N, true> {
    fn create_counter() -> impl FnMut() -> String {
        let mut count = 0;
        move || {
            count += 1;
            count.to_string()
        }
    }

    fn zip_literal(
        vec: Vec<Range<usize>>,
        lit: &'static str,
    ) -> Vec<(Range<usize>, &'static str)> {
        vec.into_iter().zip(iter::repeat(lit)).collect::<Vec<_>>()
    }

    fn cap_to_parts<'a>(cap: Self::Capture<'a, str>) -> (Range<usize>, &'a str) {
        (cap.whole_match_range(), cap.whole_match().into_inner())
    }

    fn quote_capture<'a>(cap: Self::Capture<'a, str>) -> String {
        format!("'{}'", cap.whole_match().into_inner())
    }

    fn slice_first_byte<'a>(cap: Self::Capture<'a, str>) -> String {
        cap.whole_match().into_inner()[1..].into()
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
        (Self::replace_all_using(&mut hay, Self::create_counter()), hay)
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

impl<R: Regex<char, N, true>, const N: usize> RegexTestScalar<N> for R {}

pub(crate) trait RegexTestByte<const N: usize>: Regex<u8, N, true> {
    fn create_counter() -> impl FnMut() -> Vec<u8> {
        let mut count = 0;
        move || {
            count += 1;
            count.to_string().into_bytes()
        }
    }

    fn zip_literal(
        vec: Vec<Range<usize>>,
        lit: &'static [u8],
    ) -> Vec<(Range<usize>, &'static [u8])> {
        vec.into_iter().zip(iter::repeat(lit)).collect::<Vec<_>>()
    }

    fn cap_to_parts<'a>(cap: Self::Capture<'a, [u8]>) -> (Range<usize>, &'a [u8]) {
        (cap.whole_match_range(), cap.whole_match().into_inner())
    }

    fn quote_capture<'a>(cap: Self::Capture<'a, [u8]>) -> Vec<u8> {
        let mut vec = cap.whole_match().into_inner().to_vec();
        vec.insert(0, b'\'');
        vec.push(b'\'');
        vec
    }

    // fn slice_first_byte<'a>(cap: Self::Capture<'a, [u8]>) -> Vec<u8> {
    //     cap.whole_match().into_inner()[1..].into()
    // }

    fn all_ranges(hay: &[u8]) -> Vec<Range<usize>> {
        Self::range_of_all_matches(hay, false).collect::<Vec<_>>()
    }

    fn all_ranges_overlap(hay: &[u8]) -> Vec<Range<usize>> {
        Self::range_of_all_matches(hay, true).collect::<Vec<_>>()
    }

    fn all_slices(hay: &[u8]) -> Vec<&[u8]> {
        Self::slice_all_matches(hay, false).collect::<Vec<_>>()
    }

    fn all_slices_overlap(hay: &[u8]) -> Vec<&[u8]> {
        Self::slice_all_matches(hay, true).collect::<Vec<_>>()
    }

    fn whole_capture(hay: &[u8]) -> Option<(Range<usize>, &[u8])> {
        Self::do_capture(hay).map(Self::cap_to_parts)
    }

    fn first_capture(hay: &[u8]) -> Option<(Range<usize>, &[u8])> {
        Self::find_capture(hay).map(Self::cap_to_parts)
    }

    fn all_captures(hay: &[u8]) -> Vec<(Range<usize>, &[u8])> {
        Self::find_all_captures(hay, false)
            .map(Self::cap_to_parts)
            .collect::<Vec<_>>()
    }

    fn all_captures_overlap(hay: &[u8]) -> Vec<(Range<usize>, &[u8])> {
        Self::find_all_captures(hay, true)
            .map(Self::cap_to_parts)
            .collect::<Vec<_>>()
    }

    fn first_replaced(hay: &[u8], with: &[u8]) -> (bool, Vec<u8>) {
        let mut hay = Vec::<u8>::from(hay);
        (Self::replace(&mut hay, with), hay)
    }

    fn all_replaced(hay: &[u8], with: &[u8]) -> (usize, Vec<u8>) {
        let mut hay = Vec::<u8>::from(hay);
        (Self::replace_all(&mut hay, with), hay)
    }

    fn all_replaced_using(hay: &[u8]) -> (usize, Vec<u8>) {
        let mut hay = Vec::<u8>::from(hay);
        (Self::replace_all_using(&mut hay, Self::create_counter()), hay)
    }

    fn replaced_using_iter(hay: &[u8]) -> (usize, Vec<u8>) {
        let iter = (1..=2).map(|n| n.to_string().into_bytes());
        let mut hay = Vec::<u8>::from(hay);
        (Self::replace_using_iter(&mut hay, iter), hay)
    }

    fn capture_replaced_quoted(hay: &[u8]) -> (bool, Vec<u8>) {
        let mut hay = Vec::<u8>::from(hay);
        (Self::replace_captured(&mut hay, Self::quote_capture), hay)
    }

    fn all_captures_replaced_quoted(hay: &[u8]) -> (usize, Vec<u8>) {
        let mut hay = Vec::<u8>::from(hay);
        (Self::replace_all_captured(&mut hay, Self::quote_capture), hay)
    }

//     fn capture_replaced_sliced(hay: &[u8]) -> (bool, Vec<u8>) {
//         let mut hay = Vec::<u8>::from(hay);
//         (Self::replace_captured(&mut hay, Self::slice_first_byte), hay)
//     }
//
//     fn all_captures_replaced_sliced(hay: &[u8]) -> (usize, Vec<u8>) {
//         let mut hay = Vec::<u8>::from(hay);
//         (Self::replace_all_captured(&mut hay, Self::slice_first_byte), hay)
//     }
}

impl<R: Regex<u8, N, true>, const N: usize> RegexTestByte<N> for R {}