use std::fmt::Debug;

use crate::{haystack::{Haystack, HaystackItem, HaystackWith, IntoHaystack}, matcher::Matcher};
use super::{CaptureFromRanges, IndexedCaptures};

// TODO: Use iterator rather than Vec for return type.
// TODO: Provide a method that returns a range too, not just a slice.
// TODO: Switch to lazy rollback via iterators.

/// A trait that is automatically implemented for types produced by the `regex!()` macro. Various
/// function are included that test this pattern against a provided [`Haystack`].
///
/// Altough rarely encountered, this trait's generic parameter, `I` refers to the item that can be
/// matched individually from the provided `I::Slice`. This is used so that the same expression can
/// be used to match various haystack typesncluding `&str` (`I = char`) and `&[u8]` (`I = u8`).
/// Implementations for both of these slice/item pairs will be implemented by the macro.
pub trait Regex<I: HaystackItem, const N: usize>: Debug {
    type Pattern: Matcher<I>;

    type Capture<'a, H: Haystack<'a>>: CaptureFromRanges<'a, H, N> where I: 'a;

    /// Returns `true` if this Regex matches the **entire** haystack provided. This should probably
    /// be the default _matching_ function to use.
    ///
    /// A similar behavior can be achieved by using start and end anchors in an expression and then
    /// calling [`contains_match`](Self::contains_match). This function should be prefered however,
    /// because it fails fast if the first character doesn't match.
    ///
    /// To check if this Regex matches and perform capturing, use [`do_capture`](Self::do_capture)
    /// instead.
    fn is_match<'a, H: HaystackWith<'a, I>>(hay: impl IntoHaystack<'a, H>) -> bool {
        let mut hay = hay.into_haystack();

        Self::Pattern::all_matches(&mut hay).iter().any(H::is_end)
    }

    /// Returns `true` if this Regex matches any substring of the haystack provided. To retrieve the
    /// actual substring itself, use [`slice_matching`](Self::slice_matching) or
    /// [`find_capture`](Self::find_capture).
    ///
    /// Anchors can be used as a part of this Regex to perform more complex behaviors, but if you're
    /// just wrapping an expression with `^` and `$`, see [`is_match`](Self::is_match) instead.
    fn contains_match<'a, H: HaystackWith<'a, I>>(hay: impl IntoHaystack<'a, H>) -> bool {
        let mut hay = hay.into_haystack();

        while hay.item().is_some() {
            if Self::Pattern::all_matches(&mut hay.clone()).pop().is_some() {
                return true;
            }
            hay.progress()
        }
        false
    }

    /// Returns the slice that matches this Regex first. This is the slicing variant of
    /// [`contains_match`](Self::contains_match).
    ///
    /// This function runs through the Regex first and then the haystack. This has a slight semantic
    /// difference in some scenarios.
    ///
    /// Note that there is no slicing equivalent of [`is_match`](Self::is_match), because any match
    /// has to be the entire haystack.
    fn slice_matching<'a, H: HaystackWith<'a, I>>(hay: impl IntoHaystack<'a, H>) -> Option<H::Slice> {
        let mut hay = hay.into_haystack();

        while hay.item().is_some() {
            let start = hay.index();

            if let Some(fork) = Self::Pattern::all_matches(&mut hay.clone()).pop() {
                let cap = start..fork.index();
                return Some(hay.slice(cap));
            }
            hay.progress()
        }
        None
    }

    /// Returns all slices of the provided haystack that match this Regex, optionally `overlapping`.
    ///
    /// This is the only match function that returns more than one result.
    fn slice_all_matching<'a, H: HaystackWith<'a, I>>(
        hay: impl IntoHaystack<'a, H>,
        overlapping: bool
    ) -> Vec<H::Slice> {
        let mut hay = hay.into_haystack();

        let mut all_matches = vec![];

        while hay.item().is_some() {
            let rollback = hay.clone();

            if overlapping {
                if let Some(fork) = Self::Pattern::all_matches(&mut hay).pop() {
                    all_matches.push(rollback.index()..fork.index());
                }

                hay = rollback;
                hay.progress();
            } else {
                if let Some(fork) = Self::Pattern::all_matches(&mut hay).pop() {
                    all_matches.push(rollback.index()..fork.index());
                    hay = fork;

                    // This doesn't seem to make a difference...
                    debug_assert_ne!(rollback.index(), hay.index())
                    // if rollback.index() == hay.index() {
                    //     // We've already matched at this index.
                    //     hay.progress();
                    // }
                } else {
                    hay = rollback;
                    hay.progress();
                }
            }
        }

        all_matches.into_iter().map(|m| hay.slice(m)).collect()
    }

    /// Returns a [`Self::Capture`] representing the provided haystack matched against this Regex.
    /// This includes any named or numbered capturing groups in the expression. As with
    /// [`is_match`](Self::is_match), this function acts on the entire haystack, and needs to match
    /// every character from start to end.
    ///
    /// Provides the same result as [`find_capture`](Self::find_capture) with start and end anchors,
    /// although without needing to check any non-starting substring.
    fn do_capture<'a, H: HaystackWith<'a, I>>(hay: impl IntoHaystack<'a, H>) -> Option<Self::Capture<'a, H>> {
        let mut hay = hay.into_haystack();

        let mut caps = IndexedCaptures::default();

        let start = hay.index();

        Self::Pattern::all_captures(&mut hay, &mut caps)
            .into_iter()
            .rev()
            .filter(|(h, _)| h.is_end())
            .map(|(hay_fork, mut caps_fork)| {
                caps_fork.push(0, start..hay_fork.index());
                Self::Capture::from_ranges(caps_fork.into_array(), hay_fork)
                    .expect("failed to convert captures despite matching correctly")
            })
            .next()
    }

    /// Returns the [`Self::Capture`] that matches this Regex first, similar to
    /// [`slice_matching`](Self::slice_matching) but with any named or numbered groups included.
    ///
    /// Anchors should be used for complex behavior, beyond unconditional start and end matches. See
    /// [`do_capture`](Self::do_capture) instead to capture a full haystack.
    fn find_capture<'a, H: HaystackWith<'a, I>>(hay: impl IntoHaystack<'a, H>) -> Option<Self::Capture<'a, H>> {
        let mut hay = hay.into_haystack();

        let mut caps = IndexedCaptures::default();

        while hay.item().is_some() {
            let start = hay.index();

            let first = Self::Pattern::all_captures(&mut hay.clone(), &mut caps)
                .into_iter()
                .last();

            if let Some((hay_fork, mut caps_fork)) = first {
                caps_fork.push(0, start..hay_fork.index());
                return Some(
                    Self::Capture::from_ranges(caps_fork.into_array(), hay_fork)
                        .expect("failed to convert captures despite matching correctly")
                );
            }
            hay.progress()
        }
        None
    }

    /// Returns a [`Self::Capture`] representing every full match of this Regex in the provided
    /// haystack, similar to [`slice_all_matching`](Self::slice_all_matching). This can optionally
    /// include `overlapping` matches.
    fn find_all_captures<'a, H: HaystackWith<'a, I>>(
        hay: impl IntoHaystack<'a, H>,
        overlapping: bool
    ) -> Vec<Self::Capture<'a, H>> {
        todo!("find_all_matches equivalent ({:?}, {:?})", hay.into_haystack(), overlapping)
    }
}