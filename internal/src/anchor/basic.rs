use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::ControlFlow;

use crate::haystack::{Haystack, HaystackSlice};
use crate::sealed::Sealed;

pub trait Anchor: Sealed + Debug + Default + Clone + Copy {
    /// Asserts that each anchor in this set could possibly succeed with the given haystack state.
    /// In the presence of a start anchor, the haystack's position at the start doesn't need to be
    /// checked again.
    ///
    /// The return value represents two things:
    ///
    /// - The outer [`ControlFlow`] represents whether the haystack has reached a point of
    ///   no-return. If the value is [`ControlFlow::Break`], no more matches will succeed; no
    ///   further attempts should be made to match against the haystack.
    ///   For functions that return an option, it should be possible to try the return value with
    ///   `.continue_value()?`.
    ///
    /// - If the value of the outer type is [`ControlFlow::Continue`], the inner [`bool`] represents
    ///   whether the current position should be checked for a match. A value of `false` indicates
    ///   that the haystack should be progressed before checking again.
    fn assert<'a, H: Haystack<'a>>(hay: &H) -> ControlFlow<(), bool>;

    /// Asserts that each anchor in this set could possibly succeed with the given haystack state.
    /// In the presence of a start anchor, the haystack's position at the start doesn't need to be
    /// checked again. This variant provide no information about whether the search should continue
    /// and should be called by searches that intend to match the entirety of the provided haystack.
    fn assert_fixed<'a, H: Haystack<'a>>(hay: &H) -> bool;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Start;

impl Sealed for Start {}

impl Anchor for Start {
    fn assert<'a, H: Haystack<'a>>(hay: &H) -> ControlFlow<(), bool> {
        if hay.is_start() {
            ControlFlow::Continue(true)
        } else {
            ControlFlow::Break(())
        }
    }

    fn assert_fixed<'a, H: Haystack<'a>>(_hay: &H) -> bool {
        true
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct MinLen<const N: usize>;

impl<const N: usize> Sealed for MinLen<N> {}

impl<const N: usize> Anchor for MinLen<N> {
    fn assert<'a, H: Haystack<'a>>(hay: &H) -> ControlFlow<(), bool> {
        if hay.remainder_as_slice().as_bytes().len() >= N {
            ControlFlow::Continue(true)
        } else {
            ControlFlow::Break(())
        }
    }

    fn assert_fixed<'a, H: Haystack<'a>>(hay: &H) -> bool {
        hay.remainder_as_slice().as_bytes().len() >= N
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct MaxLen<const N: usize>;

impl<const N: usize> Sealed for MaxLen<N> {}

impl<const N: usize> Anchor for MaxLen<N> {
    fn assert<'a, H: Haystack<'a>>(_hay: &H) -> ControlFlow<(), bool> {
        ControlFlow::Continue(true)
    }

    fn assert_fixed<'a, H: Haystack<'a>>(hay: &H) -> bool {
        hay.remainder_as_slice().as_bytes().len() <= N
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EndAndMaxLen<const N: usize>;

impl<const N: usize> Sealed for EndAndMaxLen<N> {}

impl<const N: usize> Anchor for EndAndMaxLen<N> {
    fn assert<'a, H: Haystack<'a>>(hay: &H) -> ControlFlow<(), bool> {
        if hay.remainder_as_slice().as_bytes().len() <= N {
            ControlFlow::Continue(true)
        } else {
            ControlFlow::Continue(false)
        }
    }

    fn assert_fixed<'a, H: Haystack<'a>>(hay: &H) -> bool {
        hay.remainder_as_slice().as_bytes().len() <= N
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct AnchorNone;

impl Sealed for AnchorNone {}

impl Anchor for AnchorNone {
    fn assert<'a, H: Haystack<'a>>(_hay: &H) -> ControlFlow<(), bool> {
        ControlFlow::Continue(true)
    }

    fn assert_fixed<'a, H: Haystack<'a>>(_hay: &H) -> bool {
        true
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct AnchorPair<A: Anchor, B: Anchor>(pub PhantomData<(A, B)>);

impl<A: Anchor, B: Anchor> Sealed for AnchorPair<A, B> {}

impl<A: Anchor, B: Anchor> Anchor for AnchorPair<A, B> {
    fn assert<'a, H: Haystack<'a>>(hay: &H) -> ControlFlow<(), bool> {
        ControlFlow::Continue(
            A::assert(hay)? && B::assert(hay)?
        )
    }

    fn assert_fixed<'a, H: Haystack<'a>>(hay: &H) -> bool {
        A::assert_fixed(hay) && B::assert_fixed(hay)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct AnchorSet<A: Anchor, B: Anchor, C: Anchor>(pub PhantomData<(A, B, C)>);

impl<A: Anchor, B: Anchor, C: Anchor> Sealed for AnchorSet<A, B, C> {}

impl<A: Anchor, B: Anchor, C: Anchor> Anchor for AnchorSet<A, B, C> {
    fn assert<'a, H: Haystack<'a>>(hay: &H) -> ControlFlow<(), bool> {
        ControlFlow::Continue(
            A::assert(hay)? && B::assert(hay)? && C::assert(hay)?
        )
    }

    fn assert_fixed<'a, H: Haystack<'a>>(hay: &H) -> bool {
        A::assert_fixed(hay) && B::assert_fixed(hay) && C::assert_fixed(hay)
    }
}
