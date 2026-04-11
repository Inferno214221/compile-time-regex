use std::ops::Range;

use crate::haystack::{HaystackItem, HaystackIter, MutIntoHaystack, HaystackOf, IntoHaystack};
use super::Regex;

/// A trait that is automatically implemented for 'anonymous' regular expression types. There is
/// only one difference between this and [`Regex`]: all functions take self as the first parameter,
/// removing the need to name the type itself.
///
/// An `AnonRegex` can be created by invoking `regex!()` without a type identifier or visibility.
/// The result is an instance of an unnamable type implementing `AnonRegex`.
pub trait AnonRegex<I: HaystackItem, const N: usize>: Regex<I, N> {
    /// See [`Regex::is_match`].
    fn is_match<'a, H: HaystackOf<'a, I>>(&self, hay: impl IntoHaystack<'a, H>) -> bool {
        <Self as Regex<I, N>>::is_match(hay)
    }

    /// See [`Regex::contains_match`].
    fn contains_match<'a, H: HaystackOf<'a, I>>(&self, hay: impl IntoHaystack<'a, H>) -> bool {
        <Self as Regex<I, N>>::contains_match(hay)
    }

    /// See [`Regex::count_matches`].
    fn count_matches<'a, H: HaystackOf<'a, I>>(
        &self,
        hay: impl IntoHaystack<'a, H>,
        overlapping: bool
    ) -> usize {
        <Self as Regex<I, N>>::count_matches(hay, overlapping)
    }

    /// See [`Regex::range_of_match`].
    fn range_of_match<'a, H: HaystackOf<'a, I>>(
        &self,
        hay: impl IntoHaystack<'a, H>
    ) -> Option<Range<usize>> {
        <Self as Regex<I, N>>::range_of_match(hay)
    }

    /// See [`Regex::range_of_all_matches`].
    fn range_of_all_matches<'a, H: HaystackOf<'a, I>>(
        &self,
        hay: impl IntoHaystack<'a, H>,
        overlapping: bool
    ) -> Vec<Range<usize>> {
        <Self as Regex<I, N>>::range_of_all_matches(hay, overlapping)
    }

    /// See [`Regex::slice_match`].
    fn slice_match<'a, H: HaystackOf<'a, I>>(
        &self,
        hay: impl IntoHaystack<'a, H>
    ) -> Option<H::Slice> {
        <Self as Regex<I, N>>::slice_match(hay)
    }

    /// See [`Regex::slice_all_matches`].
    fn slice_all_matches<'a, H: HaystackOf<'a, I>>(
        &self,
        hay: impl IntoHaystack<'a, H>,
        overlapping: bool
    ) -> Vec<H::Slice> {
        <Self as Regex<I, N>>::slice_all_matches(hay, overlapping)
    }

    /// See [`Regex::do_capture`].
    fn do_capture<'a, H: HaystackOf<'a, I>>(
        &self,
        hay: impl IntoHaystack<'a, H>
    ) -> Option<Self::Capture<'a, H::Slice>> {
        <Self as Regex<I, N>>::do_capture(hay)
    }

    /// See [`Regex::find_capture`].
    fn find_capture<'a, H: HaystackOf<'a, I>>(
        &self,
        hay: impl IntoHaystack<'a, H>
    ) -> Option<Self::Capture<'a, H::Slice>> {
        <Self as Regex<I, N>>::find_capture(hay)
    }

    /// See [`Regex::find_all_captures`].
    fn find_all_captures<'a, H: HaystackOf<'a, I>>(
        &self,
        hay: impl IntoHaystack<'a, H>,
        overlapping: bool
    ) -> Vec<Self::Capture<'a, H::Slice>> {
        <Self as Regex<I, N>>::find_all_captures(hay, overlapping)
    }

    /// See [`Regex::replace`].
    fn replace<'a, M: MutIntoHaystack<'a, I>>(&self, hay_mut: &'a mut M, with: &str) -> bool {
        <Self as Regex<I, N>>::replace(hay_mut, with)
    }

    /// See [`Regex::replace_all`].
    fn replace_all<'a, M: MutIntoHaystack<'a, I>>(&self, hay_mut: &'a mut M, with: &str) -> usize {
        <Self as Regex<I, N>>::replace_all(hay_mut, with)
    }

    /// See [`Regex::replace_all_using`].
    fn replace_all_using<'a, M: MutIntoHaystack<'a, I>>(
        &self,
        hay_mut: &'a mut M,
        using: impl FnMut() -> String
    ) -> usize {
        <Self as Regex<I, N>>::replace_all_using(hay_mut, using)
    }

    /// See [`Regex::replace_captured`].
    fn replace_captured<'a, M, F>(&self, hay_mut: &'a mut M, replacer: F) -> bool
    where
        I: 'a,
        M: MutIntoHaystack<'a, I>,
        F: for<'b> FnOnce(Self::Capture<'b, <M::Hay<'b> as HaystackIter<'b>>::Slice>) -> String
    {
        <Self as Regex<I, N>>::replace_captured::<M, F>(hay_mut, replacer)
    }

    /// See [`Regex::replace_all_captured`].
    fn replace_all_captured<'a, M, F>(&self, hay_mut: &'a mut M, replacer: F) -> usize
    where
        I: 'a,
        M: MutIntoHaystack<'a, I>,
        F: for<'b> FnMut(Self::Capture<'b, <M::Hay<'b> as HaystackIter<'b>>::Slice>) -> String
    {
        <Self as Regex<I, N>>::replace_all_captured::<M, F>(hay_mut, replacer)
    }
}