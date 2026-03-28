use std::ops::Range;

use crate::haystack::{ByteIter, HaystackItem, HaystackIter, HaystackSlice, StrIter};

// TODO: Create a HatstackState(usize) for even cheaper clones.

/// A type used to reference the haystack when matching of capturing against a
/// [`Regex`](crate::expr::Regex), in addition to tracking progression.
///
/// It is rare that users will have to interact with this trait, appart from Trait bounds. All
/// public methods will take an `impl Into<Haystack<'a, I>>` as an argument.
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
}

impl<'a, T: HaystackIter<'a>> Haystack<'a> for T {}

pub trait HaystackWith<'a, I: HaystackItem>: Haystack<'a, Slice: HaystackSlice<'a, Item = I>> {}

impl<'a, I, T> HaystackWith<'a, I> for T
where
    I: HaystackItem,
    T: Haystack<'a, Slice: HaystackSlice<'a, Item = I>>
{}

pub trait IntoHaystack<'a, H: Haystack<'a>> {
    fn into_haystack(self) -> H;
}

// Avoid a blanket implementation here so that users don't have to specify types.
// impl<'a, I: HaystackItem, H: Haystack<'a, I>> IntoHaystack<'a, I, H> for H::Slice {
//     fn into_haystack(self) -> H {
//         <H as HaystackIter>::from_slice(self)
//     }
// }

impl<'a> IntoHaystack<'a, ByteIter<'a>> for &'a [u8] {
    fn into_haystack(self) -> ByteIter<'a> {
        ByteIter::from_slice(self)
    }
}

impl<'a> IntoHaystack<'a, StrIter<'a>> for &'a str {
    fn into_haystack(self) -> StrIter<'a> {
        StrIter::from_slice(self)
    }
}