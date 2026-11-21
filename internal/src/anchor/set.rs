use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::ControlFlow;

use crate::anchor::Anchor;
use crate::haystack::{Haystack};
use crate::sealed::Sealed;

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
