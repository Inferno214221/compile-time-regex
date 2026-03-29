use std::fmt::{self, Debug};

use crate::{expr::IndexedCaptures, haystack::{HaystackItem, HaystackOf}};

pub trait Matcher<I: HaystackItem>: Debug + Default {
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
    fn all_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> Vec<usize> {
        if Self::matches(hay) {
            vec![hay.index()]
        } else {
            vec![]
        }
    }

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
    ) -> Vec<(usize, IndexedCaptures)> {
        if Self::captures(hay, caps) {
            vec![(hay.index(), caps.clone())]
        } else {
            vec![]
        }
    }
}

#[derive(Default)]
pub struct Byte<const N: u8>;

impl<const N: u8> Matcher<u8> for Byte<N> {
    fn matches<'a, H: HaystackOf<'a, u8>>(hay: &mut H) -> bool {
        if hay.item() == Some(N) {
            hay.progress();
            true
        } else {
            false
        }
    }
}

impl<const N: u8> Debug for Byte<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{N:#04x}")
    }
}

#[derive(Default)]
pub struct ByteRange<const A: u8, const B: u8>;

impl<const A: u8, const B: u8> Matcher<u8> for ByteRange<A, B> {
    fn matches<'a, H: HaystackOf<'a, u8>>(hay: &mut H) -> bool {
        if let Some(byte) = hay.item() && A <= byte && byte <= B {
            hay.progress();
            true
        } else {
            false
        }
    }
}

impl<const A: u8, const B: u8> Debug for ByteRange<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{A:#04x}-{B:#04x}]")
    }
}

#[derive(Default)]
pub struct Scalar<const N: char>;

impl<const N: char> Matcher<char> for Scalar<N> {
    fn matches<'a, H: HaystackOf<'a, char>>(hay: &mut H) -> bool {
        if hay.item() == Some(N) {
            hay.progress();
            true
        } else {
            false
        }
    }
}

impl<const N: char> Debug for Scalar<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", N.escape_debug())
    }
}

#[derive(Default)]
pub struct ScalarRange<const A: char, const B: char>;

impl<const A: char, const B: char> Matcher<char> for ScalarRange<A, B> {
    fn matches<'a, H: HaystackOf<'a, char>>(hay: &mut H) -> bool {
        if let Some(scalar) = hay.item() && A <= scalar && scalar <= B {
            hay.progress();
            true
        } else {
            false
        }
    }
}

impl<const A: char, const B: char> Debug for ScalarRange<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}-{}]", A.escape_debug(), B.escape_debug())
    }
}

#[derive(Default)]
pub struct Always;

impl<I: HaystackItem> Matcher<I> for Always {
    fn matches<'a, H: HaystackOf<'a, I>>(_hay: &mut H) -> bool {
        true
    }
}

impl Debug for Always {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "()")
    }
}

#[derive(Default)]
pub struct Beginning;

impl<I: HaystackItem> Matcher<I> for Beginning {
    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        hay.is_start()
    }
}

impl Debug for Beginning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "^")
    }
}

#[derive(Default)]
pub struct End;

impl<I: HaystackItem> Matcher<I> for End {
    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        hay.is_end()
    }
}

impl Debug for End {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "$")
    }
}