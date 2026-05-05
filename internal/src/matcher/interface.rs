use std::fmt::Debug;

use crate::{expr::IndexedCaptures, haystack::{HaystackItem, HaystackOf}};

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

// I'm just going to write this down cause I seem to keep forgetting. For alternations, either
// branch may match a longer sequence in the haystack, but the first branch should be the priority.
// This would complicate lazy semantics beyond just reversing the order of return values if it
// wasn't for the simple fact that *alternations can't be lazy*. Laziness only applies to
// quantifiers, even if that quantifiers is a 0-1 ("??" in the expression). Because there is always
// a type between Lazy and Or in the type expression, it doesn't matter.
// - Or<Lazy<..>, Lazy<..>> is valid and has clear semantics. No need for any special logic.
// - Lazy<QuantifierNOrMore<Or<..>, N>> is also valid and matches Or eagerly during every
//   repetition, rolling back if necessary.
// - Lazy<Or<..>> is where things would get complicated, but there is no way to express it in an
//   expression.

pub trait LazyMatcher<I: HaystackItem>: Matcher<I> {
    type LazyAllMatches<'a, H: HaystackOf<'a, I>>: Iterator<Item = usize>;
    type LazyAllCaptures<'a, H: HaystackOf<'a, I>>: Iterator<Item = (usize, IndexedCaptures)>;

    /// Functions exactly the same as [`Matcher::matches`], except that the haystack's index is left
    /// at the first location where the match is considered successful.
    fn lazy_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool;

    /// Functions exactly the same as [`Matcher::all_matches`], except that the indices are produced
    /// in the opposite order. The first value returned by the iterator is the one first encountered
    /// in the haystack, not the one that produces the longest match.
    fn lazy_all_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> Self::LazyAllMatches<'a, H>;

    /// Functions exactly the same as [`Matcher::captures`], except that the haystack's index and
    /// captures represent the first location where the match is considered successful.
    fn lazy_captures<'a, H: HaystackOf<'a, I>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool;

    /// Functions exactly the same as [`Matcher::all_captures`], except that the indices and
    /// captures are produced in the opposite order. The first value returned by the iterator is the
    /// one first encountered in the haystack, not the one that produces the longest match.
    fn lazy_all_captures<'a, H: HaystackOf<'a, I>>(
        hay: &mut H,
        caps: &mut IndexedCaptures
    ) -> Self::LazyAllCaptures<'a, H>;
}