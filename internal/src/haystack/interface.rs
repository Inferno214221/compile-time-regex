use std::{fmt::Debug, ops::Range};

use crate::haystack::HaystackItem;

/// The main underlying trait for [`Haystack`] types, `HaystackIter` should be implemented on new
/// types that understand slicing and iterating over a haystack that can be sliced into instances of
/// `Self::Slice`.
///
/// For unicode-based haystacks like [`&str`](str), the implementing type needs to be able to deal
/// with the contained variable width code points.
///
/// This trait requires that implementors also implement
/// [`Iterator<Item = Self::Slice::Item>`](Iterator). When [`Iterator::next`] is called, on a
/// `HaystackIter` it should return the same value that previous calls to
/// [`current_item`](Self::current_item) have, before progressing the index to the next item. When
/// the last item has been returned by `next`, the iterators should return None. Any future calls
/// should avoid incrementing the index.
///
/// Additionally, `HaystackIter`s should be cheap to clone and able to produce and restore an index
/// representing the current position.
///
/// Although possible, there is no point implementing a `HaystackIter` that shares a `Slice` with
/// another `HaystackIter`.
pub trait HaystackIter<'a>: Debug + Clone
    + Iterator<Item = <Self::Slice as HaystackSlice<'a>>::Item>
{
    /// The `HaystackSlice` returned by this type when slicing the underlying haystack. This type is
    /// usually also contained within the implementor used to create an instance via
    /// [`IntoHaystack`].
    type Slice: HaystackSlice<'a>;

    /// Returns the item currently being matched in the haystack. Repeatedly calling this method
    /// should return the same item, until progressed with [`Iterator::next`].
    fn current_item(&self) -> Option<Self::Item>;

    fn prev_item(&self) -> Option<Self::Item>;

    /// Returns the index of the current item in the original haystack. The returned value should be
    /// valid to pass to [`Self::go_to`] without causing a panic.
    fn current_index(&self) -> usize;

    /// Returns the underlying slice, as it was when this `HaystackIter` was created - representing
    /// the entire haystack being matched against.
    fn whole_slice(&self) -> Self::Slice;

    /// Returns the remaining contents of this haystack, as a `Slice`. For slice based haystacks,
    /// this is can be implemented as `&self.inner[self.index..]`.
    fn remainder_as_slice(&self) -> Self::Slice;

    /// Restores the `index` of the haystack to the provided one. This should only be called with
    /// indexes obtained by calling [`current_index`](Self::current_index) on this `HaystackIter`.
    fn go_to(&mut self, index: usize);
}

/// A trait representing a slice of the underlying haystack for various [`Haystack`] types.
///
/// The implementor of this trait is usually but not always, the only implementor of
/// [`IntoHaystack`] for a haystack type.
///
/// It should be noted that this trait is often implemented of a reference to the type in question,
/// e.g. `&str` or `&[u8]` rather than `str` or `[u8]` themselves, so that the implementing type can
/// be cloned as required.
pub trait HaystackSlice<'a>: Debug + Clone + Sized + ToOwned {
    /// The `HaystackItem` contained within this slice.
    type Item: HaystackItem;

    /// Slices the underlying slice with the provided (half-open) `range`, used for retrieving
    /// values of capture groups.
    fn slice_with(&self, range: Range<usize>) -> Self;
}

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

    fn inner_slice(&self) -> Self::Slice {
        self.whole_slice()
    }

    fn slice_with(&self, range: Range<usize>) -> Self::Slice {
        self.inner_slice().slice_with(range)
    }

    fn reset(&mut self) {
        self.go_to(0);
    }

    fn rollback(&mut self, state: usize) -> &mut Self {
        self.go_to(state);
        self
    }

    fn is_start(&self) -> bool {
        self.current_index() == 0
    }

    fn is_end(&self) -> bool {
        self.item().is_none()
    }

    fn is_line_start(&self) -> bool {
        self.prev_item().is_none_or(HaystackItem::is_newline)
    }

    fn is_line_end(&self) -> bool {
        self.item().is_none_or(HaystackItem::is_newline)
    }

    fn is_crlf_start(&self) -> bool {
        match self.prev_item() {
            Some(n) if n.is_newline() => true,
            Some(r) if r.is_return() => !self.item().is_some_and(HaystackItem::is_newline),
            Some(_) => false,
            None => true,
        }
    }

    fn is_crlf_end(&self) -> bool {
        // TODO: Clarify semantics surrounding "\r?(EndCRLF)"
        match self.item() {
            Some(n) if n.is_newline() => !self.prev_item().is_some_and(HaystackItem::is_return),
            Some(r) if r.is_return() => true,
            Some(_) => false,
            None => true,
        }
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
    T: Haystack<'a, Slice<>: HaystackSlice<'a, Item = I>>
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

impl<'a, H: Haystack<'a>> IntoHaystack<'a, H> for H {
    fn into_haystack(self) -> H {
        self
    }
}

// Avoid a blanket implementation here so that users don't have to specify types.
// impl<'a, I: HaystackItem, H: Haystack<'a, I>> IntoHaystack<'a, I, H> for H::Slice {
//     fn into_haystack(self) -> H {
//         <H as HaystackIter>::from_slice(self)
//     }
// }

#[allow(clippy::len_without_is_empty)]
pub trait OwnedHaystackable<I: HaystackItem> {
    type Hay<'a>: HaystackOf<'a, I> where Self: 'a;

    fn replace_range<'a>(
        &mut self,
        range: Range<usize>,
        with: <Self::Hay<'a> as HaystackIter<'a>>::Slice
    ) where Self: 'a;

    fn as_haystack<'a>(&'a self) -> Self::Hay<'a>;

    fn as_slice<'a>(&'a self) -> <Self::Hay<'a> as HaystackIter<'a>>::Slice;

    fn len(&self) -> usize;
}
