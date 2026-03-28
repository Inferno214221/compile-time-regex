use crate::haystack::{HaystackItem, HaystackWith, IntoHaystack};
use super::Regex;

/// A trait that is automatically implemented for 'anonymous' regular expression types. There is
/// only one difference between this and [`Regex`]: all functions take self as the first parameter,
/// removing the need to name the type itself.
///
/// An `AnonRegex` can be created by invoking `regex!()` without an type identifier or visibility.
/// The result is an instance of an unnamable type implementing `AnonRegex`.
pub trait AnonRegex<I: HaystackItem, const N: usize>: Regex<I, N> {
    /// See [`Regex::is_match`].
    fn is_match<'a, H: HaystackWith<'a, I>>(&self, hay: impl IntoHaystack<'a, H>) -> bool {
        <Self as Regex<I, N>>::is_match(hay)
    }

    /// See [`Regex::contains_match`].
    fn contains_match<'a, H: HaystackWith<'a, I>>(&self, hay: impl IntoHaystack<'a, H>) -> bool {
        <Self as Regex<I, N>>::contains_match(hay)
    }

    /// See [`Regex::slice_matching`].
    fn slice_matching<'a, H: HaystackWith<'a, I>>(
        &self,
        hay: impl IntoHaystack<'a, H>
    ) -> Option<H::Slice> {
        <Self as Regex<I, N>>::slice_matching(hay)
    }

    /// See [`Regex::slice_all_matching`].
    fn slice_all_matching<'a, H: HaystackWith<'a, I>>(
        &self,
        hay: impl IntoHaystack<'a, H>,
        overlapping: bool
    ) -> Vec<H::Slice> {
        <Self as Regex<I, N>>::slice_all_matching(hay, overlapping)
    }

    /// See [`Regex::do_capture`].
    fn do_capture<'a, H: HaystackWith<'a, I>>(
        &self,
        hay: impl IntoHaystack<'a, H>
    ) -> Option<Self::Capture<'a, H>> {
        <Self as Regex<I, N>>::do_capture(hay)
    }

    /// See [`Regex::find_capture`].
    fn find_capture<'a, H: HaystackWith<'a, I>>(
        &self,
        hay: impl IntoHaystack<'a, H>
    ) -> Option<Self::Capture<'a, H>> {
        <Self as Regex<I, N>>::find_capture(hay)
    }

    /// See [`Regex::find_all_captures`].
    fn find_all_captures<'a, H: HaystackWith<'a, I>>(
        &self,
        hay: impl IntoHaystack<'a, H>,
        overlapping: bool
    ) -> Vec<Self::Capture<'a, H>> {
        <Self as Regex<I, N>>::find_all_captures(hay, overlapping)
    }
}