use std::ops::Range;

use crate::haystack::{HaystackItem, HaystackIter, HaystackSlice};

// FIXME: This documentation is stale.

/// A type used to reference the haystack when matching of capturing against a
/// [`Regex`](crate::expr::Regex), in addition to tracking progression.
///
/// It is rare that users will have to interact with this trait, appart from Trait bounds. All
/// public methods will take an `impl IntoHaystack<'a, H>` as an argument.
///
/// Because of the progression tracking, a `Haystack` can't be matched against multiple times
/// without [`reset`](Self::reset)ting it first, or it will continue where the first pattern
/// finished.
///
/// The `Haystack` type is accompanied by a helper trait, [`HaystackItem`], representing an item
/// that can be matched against a [`Regex`](crate::expr::Regex).
pub trait Haystack<'a>: HaystackIter<'a> {
    fn is_start(&self) -> bool {
        self.current_index() == 0
    }

    fn is_end(&self) -> bool {
        self.item().is_none()
    }

    fn item(&self) -> Option<Self::Item> {
        self.current_item()
    }

    fn index(&self) -> usize {
        self.current_index()
    }

    // Progression is only completed by elements which explicitly check the byte and succeed.
    fn progress(&mut self) {
        self.next();
    }

    fn slice(&self, cap: Range<usize>) -> Self::Slice {
        self.slice_with(cap)
    }

    fn reset(&mut self) {
        self.go_to(0);
    }

    fn rollback(&mut self, state: usize) -> &mut Self {
        self.go_to(state);
        self
    }
}

impl<'a, T: HaystackIter<'a>> Haystack<'a> for T {}

pub trait HaystackWith<'a, I: HaystackItem>: Haystack<'a, Slice: HaystackSlice<'a, Item = I>> {}

impl<'a, I, T> HaystackWith<'a, I> for T
where
    I: HaystackItem,
    T: Haystack<'a, Slice: HaystackSlice<'a, Item = I>>
{}

/// A trait that is responsible for converting a slice into a stateful [`Haystack`], of type `H`.
/// The primary intent of this trait is to allow users to avoid creating their own `Haystack`,
/// instead passing a slice to methods on [`Regex`](crate::Regex).
pub trait IntoHaystack<'a, H: Haystack<'a>> {
    fn into_haystack(self) -> H;
}

// Avoid a blanket implementation here so that users don't have to specify types.
// impl<'a, I: HaystackItem, H: Haystack<'a, I>> IntoHaystack<'a, I, H> for H::Slice {
//     fn into_haystack(self) -> H {
//         <H as HaystackIter>::from_slice(self)
//     }
// }