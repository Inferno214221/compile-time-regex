use std::{fmt::Debug, ops::Range};

use crate::{expr::{FindAllCaptures, RangeOfAllMatches, SliceAllMatches}, haystack::{HaystackItem, HaystackIter, HaystackOf, HaystackSlice, IntoHaystack, MutIntoHaystack}, matcher::Matcher};
use super::{CaptureFromRanges, IndexedCaptures};

/// A trait that is automatically implemented for types produced by the `regex!` macro. Various
/// function are included that test this pattern against a provided
/// [`Haystack`](crate::haystack::Haystack).
///
/// Altough rarely encountered, this trait's generic parameter, `I` refers to the item that can be
/// matched individually from the provided `Haystack`. This is used so that the same expression can
/// be used to match various haystack types, including `&str` (`I = char`) and `&[u8]` (`I = u8`).
/// Implementations for both of these slice/item pairs will be implemented by the macro.
///
/// # Function Coverage
///
#[doc = include_str!("coverage.md")]
pub trait Regex<I: HaystackItem, const N: usize>: Debug {
    type Pattern: Matcher<I>;

    type Capture<'a, S: HaystackSlice<'a>>: CaptureFromRanges<'a, S, N> where I: 'a;

    /// Returns `true` if this Regex matches the **entire** haystack provided. This should probably
    /// be the default _matching_ function to use.
    ///
    /// A similar behavior can be achieved by using start and end anchors in an expression and then
    /// calling [`contains_match`](Self::contains_match). This function should be prefered however,
    /// because it fails fast if the first character doesn't match.
    ///
    /// To check if this Regex matches and perform capturing, use [`do_capture`](Self::do_capture)
    /// instead.
    fn is_match<'a, H: HaystackOf<'a, I>>(hay: impl IntoHaystack<'a, H>) -> bool {
        let mut hay = hay.into_haystack();

        Self::Pattern::all_matches(&mut hay)
            .any(|state| hay.rollback(state).is_end())
    }

    /// Returns `true` if this Regex matches any substring of the haystack provided. To retrieve the
    /// actual substring itself, use [`slice_match`](Self::slice_match) or
    /// [`find_capture`](Self::find_capture).
    ///
    /// Anchors can be used as a part of this Regex to perform more complex behaviors, but if you're
    /// just wrapping an expression with `^` and `$`, see [`is_match`](Self::is_match) instead.
    fn contains_match<'a, H: HaystackOf<'a, I>>(hay: impl IntoHaystack<'a, H>) -> bool {
        let mut hay = hay.into_haystack();

        while hay.item().is_some() {
            let start = hay.index();

            if Self::Pattern::all_matches(&mut hay).next().is_some() {
                return true;
            }

            hay.rollback(start).progress();
        }
        false
    }

    fn count_matches<'a, H: HaystackOf<'a, I>>(
        hay: impl IntoHaystack<'a, H>,
        overlapping: bool
    ) -> usize {
        let mut hay = hay.into_haystack();
        let mut count = 0;

        while hay.item().is_some() {
            let start = hay.index();

            if let Some(state_fork) = Self::Pattern::all_matches(&mut hay).next() {
                count += 1;

                if overlapping {
                    hay.rollback(start).progress();
                } else {
                    hay.rollback(state_fork);

                    debug_assert_ne!(start, state_fork)
                }
            } else {
                hay.rollback(start).progress();
            }
        }

        count
    }

    fn range_of_match<'a, H: HaystackOf<'a, I>>(
        hay: impl IntoHaystack<'a, H>
    ) -> Option<Range<usize>> {
        let mut hay = hay.into_haystack();
        range_of_match::<Self, _, _>(&mut hay)
    }

    fn range_of_all_matches<'a, H: HaystackOf<'a, I>>(
        hay: impl IntoHaystack<'a, H>,
        overlapping: bool
    ) -> RangeOfAllMatches<'a, I, H, Self::Pattern> {
        RangeOfAllMatches::new(hay.into_haystack(), overlapping)
    }

    /// Returns the slice that matches this Regex first. This is the slicing variant of
    /// [`contains_match`](Self::contains_match).
    ///
    /// This function runs through the Regex first and then the haystack. This has a slight semantic
    /// difference in some scenarios.
    ///
    /// Note that there is no slicing equivalent of [`is_match`](Self::is_match), because any match
    /// has to be the entire haystack.
    fn slice_match<'a, H: HaystackOf<'a, I>>(
        hay: impl IntoHaystack<'a, H>
    ) -> Option<H::Slice> {
        let mut hay = hay.into_haystack();
        let range = range_of_match::<Self, _, _>(&mut hay)?;
        Some(hay.slice_with(range))
    }

    /// Returns all slices of the provided haystack that match this Regex, optionally `overlapping`.
    ///
    /// Note that each match is still made greedily. Even with `overlapping = true`, if two possible
    /// matches start at the same index in the haystack, only the first to match the regex will be
    /// included.
    ///
    /// This is the only match function that returns more than one result.
    fn slice_all_matches<'a, H: HaystackOf<'a, I>>(
        hay: impl IntoHaystack<'a, H>,
        overlapping: bool
    ) -> SliceAllMatches<'a, I, H, Self::Pattern> {
        SliceAllMatches {
            inner: RangeOfAllMatches::new(hay.into_haystack(), overlapping)
        }
    }

    /// Returns a [`Self::Capture`] representing the provided haystack matched against this Regex.
    /// This includes any named or numbered capturing groups in the expression. As with
    /// [`is_match`](Self::is_match), this function acts on the entire haystack, and needs to match
    /// every character from start to end.
    ///
    /// Provides the same result as [`find_capture`](Self::find_capture) with start and end anchors,
    /// although without needing to check any non-starting substring.
    fn do_capture<'a, H: HaystackOf<'a, I>>(
        hay: impl IntoHaystack<'a, H>
    ) -> Option<Self::Capture<'a, H::Slice>> {
        let mut hay = hay.into_haystack();
        let mut caps = IndexedCaptures::default();
        let start = hay.index();

        let all_captures = Self::Pattern::all_captures(&mut hay, &mut caps);

        for (state_fork, mut caps_fork) in all_captures {
            if hay.rollback(state_fork).is_end() {
                caps_fork.push(0, start..state_fork);

                return Some(
                    Self::Capture::from_ranges(caps_fork.into_array(), hay.whole_slice())
                        .expect("failed to convert captures despite matching correctly")
                );
            }
        }
        None
    }

    /// Returns the [`Self::Capture`] that matches this Regex first, similar to
    /// [`slice_match`](Self::slice_match) but with any named or numbered groups included.
    ///
    /// Anchors should be used for complex behavior, beyond unconditional start and end matches. See
    /// [`do_capture`](Self::do_capture) instead to capture a full haystack.
    fn find_capture<'a, H: HaystackOf<'a, I>>(
        hay: impl IntoHaystack<'a, H>
    ) -> Option<Self::Capture<'a, H::Slice>> {
        let mut hay = hay.into_haystack();

        while hay.item().is_some() {
            let start = hay.index();

            let mut caps = IndexedCaptures::default();

            let first = Self::Pattern::all_captures(&mut hay, &mut caps).next();

            hay.rollback(start);

            if let Some((state_fork, mut caps_fork)) = first {
                caps_fork.push(0, start..state_fork);

                return Some(
                    Self::Capture::from_ranges(caps_fork.into_array(), hay.inner_slice())
                        .expect("failed to convert captures despite matching correctly")
                );
            }
            hay.progress()
        }
        None
    }

    /// Returns a [`Self::Capture`] representing every full match of this Regex in the provided
    /// haystack, similar to [`slice_all_matches`](Self::slice_all_matches). This can optionally
    /// include `overlapping` matches.
    ///
    /// Note that each match is still made greedily. Even with `overlapping = true`, if two possible
    /// matches start at the same index in the haystack, only the first to match the regex will be
    /// included.
    fn find_all_captures<'a, H: HaystackOf<'a, I>>(
        hay: impl IntoHaystack<'a, H>,
        overlapping: bool
    ) -> FindAllCaptures<'a, Self, I, H, N> {
        FindAllCaptures::new(hay.into_haystack(), overlapping)
    }

    fn replace<'a, M: MutIntoHaystack<'a, I>>(hay_mut: &'a mut M, with: &str) -> bool {
        let Some(range) = ({
            let mut hay = hay_mut.as_haystack();
            range_of_match::<Self, _, _>(&mut hay)
        }) else {
            return false;
        };
        hay_mut.replace_range(range, with);
        true
    }

    fn replace_all<'a, M: MutIntoHaystack<'a, I>>(hay_mut: &'a mut M, with: &str) -> usize {
        // Avoids redirecting to replace_all_using to avoid unnessecary clones.
        let ranges = RangeOfAllMatches::<I, M::Hay<'_>, Self::Pattern>::new(
            hay_mut.as_haystack(),
            false
        ).collect::<Vec<_>>();

        let count = ranges.len();
        let mut delta = Delta::new();

        for mut range in ranges {
            delta.apply_to(&mut range);

            let initial_len = hay_mut.len();
            hay_mut.replace_range(range, with);
            delta.add_diff(hay_mut.len(), initial_len);
        }

        count
    }

    fn replace_all_using<'a, M: MutIntoHaystack<'a, I>>(
        hay_mut: &'a mut M,
        mut using: impl FnMut() -> String
    ) -> usize {
        let ranges = RangeOfAllMatches::<I, M::Hay<'_>, Self::Pattern>::new(
            hay_mut.as_haystack(),
            false
        ).collect::<Vec<_>>();

        let count = ranges.len();
        let mut delta = Delta::new();

        for mut range in ranges {
            delta.apply_to(&mut range);

            let initial_len = hay_mut.len();
            hay_mut.replace_range(range, &using());
            delta.add_diff(hay_mut.len(), initial_len);
        }

        count
    }

    fn replace_captured<'a, M, F>(hay_mut: &'a mut M, replacer: F) -> bool
    where
        I: 'a,
        M: MutIntoHaystack<'a, I>,
        F: for<'b> FnOnce(Self::Capture<'b, <M::Hay<'b> as HaystackIter<'b>>::Slice>) -> String
    {
        let (range, replacement) = {
            let Some(caps) = Self::find_capture(hay_mut.as_haystack()) else { return false; };
            let first = caps.whole_match_range().clone();

            (first, replacer(caps))
        };
        hay_mut.replace_range(range, &replacement);
        true
    }

    fn replace_all_captured<'a, M, F>(hay_mut: &'a mut M, mut replacer: F) -> usize
    where
        I: 'a,
        M: MutIntoHaystack<'a, I>,
        F: for<'b> FnMut(Self::Capture<'b, <M::Hay<'b> as HaystackIter<'b>>::Slice>) -> String
    {
        let replacements: Vec<_> = {
            let caps = Self::find_all_captures(hay_mut.as_haystack(), false);
            caps.into_iter()
                .map(|c| (c.whole_match_range().clone(), replacer(c)))
                .collect()
        };

        let count = replacements.len();
        let mut delta = Delta::new();

        for (mut range, replacement) in replacements {
            delta.apply_to(&mut range);

            let initial_len = hay_mut.len();
            hay_mut.replace_range(range, &replacement);
            delta.add_diff(hay_mut.len(), initial_len);
        }

        count
    }
}

fn range_of_match<'a, R: Regex<I, N> + ?Sized, I: HaystackItem, const N: usize>(
    hay: &mut impl HaystackOf<'a, I>
) -> Option<Range<usize>> {
    while hay.item().is_some() {
        let start = hay.index();

        if let Some(state_fork) = R::Pattern::all_matches(hay).next() {
            return Some(start..state_fork);
        }

        hay.rollback(start).progress()
    }
    None
}

struct Delta(isize);

impl Delta {
    fn new() -> Delta {
        Delta(0)
    }

    fn add_diff(&mut self, from: usize, to: usize) {
        self.0 = self.0.strict_add(
            from.checked_signed_diff(to)
                .expect("difference between usizes doesn't fit in an isize")
        )
    }

    fn apply_to(&self, range: &mut Range<usize>) {
        range.start = range.start.strict_add_signed(self.0);
        range.end = range.end.strict_add_signed(self.0);
    }
}