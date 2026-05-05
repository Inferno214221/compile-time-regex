use std::fmt::Debug;

use crate::{expr::IndexedCaptures, haystack::{HaystackItem, HaystackOf}};

// New strategy: use Iterators. Lazy can wrap quantifiers which implement a Quantifier trait or
// similar. Lazy's iterator method calls all_matches_lazy or similar and does nothing else.

pub trait Matcher<I: HaystackItem>: Debug + Default {
    type AllMatches<'a, H: HaystackOf<'a, I>>: Iterator<Item = usize>;
    type AllCaptures<'a, H: HaystackOf<'a, I>>: Iterator<Item = (usize, IndexedCaptures)>;

    /// Checks if the start of the haystack contains a match for this [`Matcher`]. If this method
    /// successfully matches the start of the haystack, `hay` is progressed so that `hay.item()`
    /// hasn't been matched yet. On a fail, the state of hay is undefined.
    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool;

    // It would be nice to use a custom Iterator here rather than a Vec, but reversing an arbitrary
    // match is not easy, so we just progress through linearly and store them all.
    // This could cause issues with huge haystacks, but: all regexes need to be compiled at compile
    // time and are hence controlled by the author. If their pattern will be operating on huge
    // haystacks and need backtracking, that's up to them.

    /// Produces a Vec of all valid haystack states produced as the result of a valid match at the
    /// start of `hay`, used to implement backtracking. The Vec is produced in reverse priority
    /// order, so the last match has the highest priority. After calling all_matches, the state of
    /// `hay` itself is undefined.
    ///
    /// # Required
    /// This method needs to be implemented by all [`Matcher`]s that can match more than one string
    /// of characters from a haystack.
    fn all_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> Self::AllMatches<'a, H>;

    /// Checks if the start of the haystack contains a match for this Matcher, writing any groups
    /// to `caps`. Similar to [`matches`], this method progresses `hay` and `caps` on a success. On
    /// a fail, they have undefined states.
    ///
    /// # Required
    /// This method needs to be implemented for capturing groups or any type that holds other
    /// [`Matcher`]s, so that it can redirect to the relevant `capture` methods.
    fn captures<'a, H: HaystackOf<'a, I>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        let _ = caps;
        Self::matches(hay)
    }

    /// Produces a Vec of all valid captures (and accompanying haystack states) present at the start
    /// of `hay`. Used to implement backtracking for capturing methods. As with
    /// [`all_matches`](Matcher::all_matches), the resulting Vec is produced in reverse priority
    /// order. After calling all_captures, the state of `hay` and `caps` are undefined.
    ///
    /// # Required
    /// This method needs to be implemented for any type that also implements
    /// [`captures`](Matcher::captures) and [`all_matches`](Matcher::all_matches).
    fn all_captures<'a, H: HaystackOf<'a, I>>(
        hay: &mut H,
        caps: &mut IndexedCaptures
    ) -> Self::AllCaptures<'a, H>;
}

pub trait LazyMatcher<I: HaystackItem>: Matcher<I> {
    type LazyAllMatches<'a, H: HaystackOf<'a, I>>: Iterator<Item = usize>;
    type LazyAllCaptures<'a, H: HaystackOf<'a, I>>: Iterator<Item = (usize, IndexedCaptures)>;

    fn lazy_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool;

    fn lazy_all_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> Self::LazyAllMatches<'a, H>;

    fn lazy_captures<'a, H: HaystackOf<'a, I>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        let _ = caps;
        Self::matches(hay)
    }

    fn lazy_all_captures<'a, H: HaystackOf<'a, I>>(
        hay: &mut H,
        caps: &mut IndexedCaptures
    ) -> Self::LazyAllCaptures<'a, H>;
}