use std::ops::Range;

use crate::haystack::{HaystackItem, HaystackIter, HaystackSlice};

/// A trait used to interface the haystack types use when matching of capturing against a
/// [`Regex`](crate::expr::Regex), including tracking progression and slicing captures.
///
/// It is rare that users will have to interact with this trait, appart from Trait bounds. All
/// public methods will take an `impl IntoHaystack<'a, H>` as an argument.
///
/// `Haystack` is accompanied by another trait, [`HaystackItem`], representing items that can be
/// matched against a [`Regex`](crate::expr::Regex).
///
/// `Haystack`s are stateful and therefore can't be matched against multiple times without being
/// [`reset`](Self::reset) first, or they will continue where the first pattern finished. They store
/// their state as a `usize`, which can be obtained via [`index`](Self::index) and restored via
/// [`rollback`](Self::rollback). Additionally, `Haystack`s are cheap to clone, relying on shallow
/// clones or reference counting.
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

/// This trait is exactly the same as [`Haystack`], except that it simplifies bounds by requiring
/// that `Item = I`.
///
/// It is also blanket-implemented for all types that implement `Haystack<Item = I>`.
pub trait HaystackOf<'a, I: HaystackItem>: Haystack<'a, Slice: HaystackSlice<'a, Item = I>> {}

impl<'a, I, T> HaystackOf<'a, I> for T
where
    I: HaystackItem,
    T: Haystack<'a, Slice: HaystackSlice<'a, Item = I>>
{}

/// A trait that is responsible for converting a slice into a stateful [`Haystack`], of type `H`.
/// The primary intent of this trait is to allow users to avoid creating their own `Haystack`,
/// instead passing a slice to methods on [`Regex`](crate::expr::Regex).
///
/// If creating a new `Haystack` type, this trait should be implemented manually so that all types
/// can be inferred properly.
pub trait IntoHaystack<'a, H: Haystack<'a>> {
    /// Creates a new [`Haystack`] from self.
    fn into_haystack(self) -> H;
}

// Avoid a blanket implementation here so that users don't have to specify types.
// impl<'a, I: HaystackItem, H: Haystack<'a, I>> IntoHaystack<'a, I, H> for H::Slice {
//     fn into_haystack(self) -> H {
//         <H as HaystackIter>::from_slice(self)
//     }
// }