use std::ops::ControlFlow;

use crate::{anchor::Anchor, haystack::{Haystack, HaystackItem}, sealed::Sealed};

#[derive(Debug, Default, Clone, Copy)]
pub struct ZeroWidthBoundary;

impl Sealed for ZeroWidthBoundary {}

impl Anchor for ZeroWidthBoundary {
    fn assert<'a, H: Haystack<'a>>(hay: &H) -> ControlFlow<(), bool> {
        ControlFlow::Continue(H::Item::is_utf8_boundary(hay.item()))
    }

    fn assert_fixed<'a, H: Haystack<'a>>(hay: &H) -> bool {
        H::Item::is_utf8_boundary(hay.item())
    }
}