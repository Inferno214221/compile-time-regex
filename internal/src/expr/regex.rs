use std::fmt::Debug;
use std::ops::Range;

use super::{CaptureFromRanges, IndexedCaptures};
use crate::expr::{FindAllCaptures, RangeOfAllMatches, SliceAllMatches};
use crate::haystack::{
    HaystackItem, HaystackIter, HaystackOf, HaystackSlice, IntoHaystack, OwnedHaystackable,
};
use crate::matcher::Matcher;

/// A trait that is automatically implemented for types produced by the `regex!` macro. Various
/// function are included that test this pattern against a provided
/// [`Haystack`](crate::haystack::Haystack).
///
/// Most methods will take an [`IntoHaystack`] or [`OwnedHaystackable`] parameter to save the
/// user from creating their own `Haystack`. This allows values with types like `&str` and
/// `&mut String` to be passed to these methods.
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
    /// This type is a macro generated combination of ZSTs responsible for doing all of the heavy
    /// lifting involved in actually matching or capturing against a `Haystack`. For realistic
    /// expressions, this type will be very long an unpleasant to type, hence implementing it as an
    /// associated type.
    type Pattern: Matcher<I>;

    /// A macro generated type holding all `N` capture groups in this expression, producing ranges
    /// or slices of the haystack with aliases for named groups. The generated type also understands
    /// which groups will always exist in a match and which are optional.
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

    /// Returns the number of matches present in the haystack provided, optionally including
    /// `overlapping` matches.
    ///
    /// If using this in conjunction with the actual matches themselves, you might be better of
    /// collecting the other output and checking the length.
    fn count_matches<'a, H: HaystackOf<'a, I>>(
        hay: impl IntoHaystack<'a, H>,
        overlapping: bool,
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

    /// Returns the range that matches this Regex first. This is the range variant of
    /// [`contains_match`](Self::contains_match). For the actual substring itself, see
    /// [`slice_match`](Self::slice_match).
    ///
    /// This function runs through the Regex first and then the haystack. This means that result is
    /// the one that matches the Regex first, not necessarily the first match in the haystack. In
    /// many cases, this makes no difference.
    ///
    /// Note that there is no range equivalent of [`is_match`](Self::is_match), because any match
    /// has to be the entire haystack.
    fn range_of_match<'a, H: HaystackOf<'a, I>>(
        hay: impl IntoHaystack<'a, H>,
    ) -> Option<Range<usize>> {
        let mut hay = hay.into_haystack();
        range_of_match::<Self, _, _>(&mut hay)
    }

    /// Returns an iterator over the ranges of all substrings in the provided haystack that match
    /// this Regex, optionally `overlapping`. For the actual substrins themself, see
    /// [`slice_all_matches`](Self::slice_all_matches).
    ///
    /// Note that each match is still made greedily. Even with `overlapping = true`, if two possible
    /// matches start at the same index in the haystack, only the first to match the regex will be
    /// included.
    fn range_of_all_matches<'a, H: HaystackOf<'a, I>>(
        hay: impl IntoHaystack<'a, H>,
        overlapping: bool,
    ) -> RangeOfAllMatches<'a, I, H, Self::Pattern> {
        RangeOfAllMatches::new(hay.into_haystack(), overlapping)
    }

    /// Returns the slice that matches this Regex first. This is the slicing variant of
    /// [`range_of_match`](Self::range_of_match).
    ///
    /// This function runs through the Regex first and then the haystack. This means that result is
    /// the one that matches the Regex first, not necessarily the first match in the haystack. In
    /// many cases, this makes no difference.
    ///
    /// Note that there is no slicing equivalent of [`is_match`](Self::is_match), because any match
    /// has to be the entire haystack.
    fn slice_match<'a, H: HaystackOf<'a, I>>(hay: impl IntoHaystack<'a, H>) -> Option<H::Slice> {
        let mut hay = hay.into_haystack();
        let range = range_of_match::<Self, _, _>(&mut hay)?;
        Some(hay.slice_with(range))
    }

    /// Returns an iterator over all slices of the provided haystack that match this Regex,
    /// optionally `overlapping`.
    ///
    /// Note that each match is still made greedily. Even with `overlapping = true`, if two possible
    /// matches start at the same index in the haystack, only the first to match the regex will be
    /// included.
    ///
    /// The returned iterator doesn't implement [`ExactSizeIterator`] because it is lazy, if you
    /// need to know how many matches are included, [`collect`](Iterator::collect) it first.
    fn slice_all_matches<'a, H: HaystackOf<'a, I>>(
        hay: impl IntoHaystack<'a, H>,
        overlapping: bool,
    ) -> SliceAllMatches<'a, I, H, Self::Pattern> {
        SliceAllMatches {
            inner: RangeOfAllMatches::new(hay.into_haystack(), overlapping),
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
        hay: impl IntoHaystack<'a, H>,
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
        hay: impl IntoHaystack<'a, H>,
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

    /// Returns an iterator over [`Self::Capture`]s representing every full match of this Regex in
    /// the provided haystack, similar to [`slice_all_matches`](Self::slice_all_matches). This can
    /// optionally include `overlapping` matches.
    ///
    /// Note that each match is still made greedily. Even with `overlapping = true`, if two possible
    /// matches start at the same index in the haystack, only the first to match the regex will be
    /// included.
    fn find_all_captures<'a, H: HaystackOf<'a, I>>(
        hay: impl IntoHaystack<'a, H>,
        overlapping: bool,
    ) -> FindAllCaptures<'a, Self, I, H, N> {
        FindAllCaptures::new(hay.into_haystack(), overlapping)
    }

    /// Replaces the first match of this Regex in the provided haystack with the provided slice. The
    /// slice type required is the one associated with the provided haystack. The return value is a
    /// boolean indicating whether a match was found and replaced.
    ///
    /// This function runs through the Regex first and then the haystack. This means that replaced
    /// substring is the one that matches the Regex first, not necessarily the first match in the
    /// haystack. In many cases, this makes no difference.
    fn replace<'a, M: OwnedHaystackable<I>>(
        hay_mut: &mut M,
        with: <M::Hay<'a> as HaystackIter<'a>>::Slice
    ) -> bool {
        let Some(range) = ({
            let mut hay = hay_mut.as_haystack();
            range_of_match::<Self, _, _>(&mut hay)
        }) else {
            return false;
        };
        hay_mut.replace_range(range, with);
        true
    }

    /// Replaces every matching substring in the provided haystack with a copy of the provided
    /// slice. The slice type required is the one associated with the provided haystack. The return
    /// value is an integer representing the number of matches and replacements that occured.
    fn replace_all<'a, M: OwnedHaystackable<I>>(
        hay_mut: &mut M,
        with: <M::Hay<'a> as HaystackIter<'a>>::Slice
    ) -> usize {
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
            hay_mut.replace_range(range, with.clone());
            delta.add_diff(hay_mut.len(), initial_len);
        }

        count
    }

    /// Replaces every matching substring in the provided haystack with the return value of the
    /// provided function. The return type of this functions needs to match the provided haystack.
    /// The return value is an integer representing the number of matches and replacements that
    /// occured.
    ///
    /// Because of the use of [`FnMut`] for the parameter, this can be used to replace all matches
    /// using an iterator by passing in `|| iter.next().unwrap_or_default()`.
    fn replace_all_using<M: OwnedHaystackable<I>>(
        hay_mut: &mut M,
        mut using: impl FnMut() -> M,
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
            hay_mut.replace_range(range, using().as_slice());
            delta.add_diff(hay_mut.len(), initial_len);
        }

        count
    }

    // The closure returns M because it can't continue to reference the source, given that we need
    // to overwrite it.

    /// Replaces the first captured substring in the provided haystack with a computed value. The
    /// return value is a boolean indicating whether a match was found and replaced.
    ///
    /// The provided function is used to create a replacement value when given the capture. The
    /// replacement value shares a type with the provided haystack. Its simplified signature would
    /// be `F: FnOnce(Self::Capture<'_, <M::Hay>::Slice>) -> M`. Because of limitations with higher
    /// ranked trait bounds surrounding closure, it may be necessary to implement this as function
    /// with lifetime annotations like so:
    /// ```ignore
    /// regex!(PhoneNum = r"(0|(?<country_code>\+[0-9]+))(?<number>[0-9]{9})");
    ///
    /// fn remove_country_code<'a>(value: PhoneNumCapture<'a, &'a str>) -> String {
    ///     format!("0{}", value.number())
    /// }
    ///
    /// fn main() {
    ///     let mut hay = String::from("+1234567890");
    ///     PhoneNum::replace_captured(hay, remove_country_code);
    ///     assert_eq!(hay, "0234567890");
    /// }
    /// ```
    ///
    /// This function runs through the Regex first and then the haystack. This means that replaced
    /// substring is the one that matches the Regex first, not necessarily the first match in the
    /// haystack. In many cases, this makes no difference.
    fn replace_captured<M, F>(hay_mut: &mut M, replacer: F) -> bool
    where
        M: OwnedHaystackable<I>,
        F: for<'a> FnOnce(Self::Capture<'a, <M::Hay<'a> as HaystackIter<'a>>::Slice>) -> M,
    {
        let (range, replacement) = {
            let Some(caps) = Self::find_capture(hay_mut.as_haystack()) else {
                return false;
            };
            let first = caps.whole_match_range().clone();

            (first, replacer(caps))
        };
        hay_mut.replace_range(range, replacement.as_slice());
        true
    }

    /// Replaces all captured substring in the provided haystack with a computed value. The return
    /// value is an integer indicating the number of matches found and replaced.
    ///
    /// The provided function is used to create a replacement value when given a capture. The
    /// replacement value shares a type with the provided haystack. Its simplified signature would
    /// be `F: FnOnce(Self::Capture<'_, <M::Hay>::Slice>) -> M`. Because of limitations with higher
    /// ranked trait bounds surrounding closure, it may be necessary to implement this as function
    /// with lifetime annotations as mentioned in the documentation for
    /// [`replace_captured`](Self::replace_captured).
    fn replace_all_captured<M, F>(hay_mut: &mut M, mut replacer: F) -> usize
    where
        M: OwnedHaystackable<I>,
        F: for<'a> FnMut(Self::Capture<'a, <M::Hay<'a> as HaystackIter<'a>>::Slice>) -> M,
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
            hay_mut.replace_range(range, replacement.as_slice());
            delta.add_diff(hay_mut.len(), initial_len);
        }

        count
    }
}

fn range_of_match<'a, R: Regex<I, N> + ?Sized, I: HaystackItem, const N: usize>(
    hay: &mut impl HaystackOf<'a, I>,
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
