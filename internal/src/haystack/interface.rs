use std::fmt::Debug;
use std::ops::Range;

use crate::haystack::HaystackItem;

/// A trait representing a slice of the underlying haystack for various [`Haystack`] types.
///
/// The implementer of this trait is usually but not always, the only implementer of
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

    fn as_bytes(&self) -> &[u8];
}

/// A trait used to interface the haystack types use when matching of capturing against a
/// [`Regex`](crate::expr::Regex), including tracking progression and slicing captures.
///
/// It is rare that users will have to interact with this trait, apart from Trait bounds. All public
/// methods will take an `impl IntoHaystack<'a, H>` as an argument.
///
/// `Haystack` is accompanied by another trait, [`HaystackItem`], representing items that can be
/// matched against a [`Regex`](crate::expr::Regex).
///
/// `Haystack`s are stateful and therefore can't be matched against multiple times without being
/// [`reset`](Self::reset) first, or they will continue where the first pattern finished. They store
/// their state as a `usize`, which can be obtained via [`index`](Self::index) and restored via
/// [`rollback`](Self::rollback). Additionally, `Haystack`s are cheap to clone, relying on shallow
/// clones or reference counting.
///
/// # Implementing
///
/// `Haystack` can be implemented for other types to allow searching, matching and capturing within
/// other string and byte slice-like types.
///
/// For unicode-based haystacks like [`&str`](str), the implementing type needs to be able to deal
/// with the contained variable width code points.
///
/// This trait requires that implementers also implement
/// [`Iterator<Item = Self::Slice::Item>`](Iterator). When [`Iterator::next`] is called, on a
/// `Haystack` it should return the same value that previous calls to [`item`](Self::item) have,
/// before progressing the index to the next item. When the last item has been returned by `next`,
/// the iterators should return None. Any future calls should avoid incrementing the index.
///
/// Additionally, `Haystack`s should be cheap to clone and able to produce and restore an index
/// representing the current position.
///
/// Although possible, there is no point implementing a `Haystack` that shares a `Slice` with
/// another `Haystack`.
pub trait Haystack<'a>: Debug + Clone + Iterator<Item = <Self::Slice as HaystackSlice<'a>>::Item> {
    /// The `HaystackSlice` returned by this type when slicing the underlying haystack. This type is
    /// usually also contained within the implementer used to create an instance via
    /// [`IntoHaystack`].
    type Slice: HaystackSlice<'a>;

    /// Returns the item currently being matched in the haystack. Repeatedly calling this method
    /// should return the same item, until progressed with [`Iterator::next`].
    fn item(&self) -> Option<Self::Item>;

    /// Returns the item last matched in the haystack without making any changes.
    fn prev_item(&self) -> Option<Self::Item>;

    /// Returns the index of the current item in the original haystack. The returned value should be
    /// valid to pass to [`Self::go_to`] without causing a panic.
    fn index(&self) -> usize;

    // Progression is only completed by elements which explicitly check the byte and succeed.
    fn progress(&mut self) {
        self.next();
    }

    /// Returns the underlying slice, as it was when this `Haystack` was created - representing
    /// the entire haystack being matched against.
    fn inner_slice(&self) -> Self::Slice;

    fn slice_with(&self, range: Range<usize>) -> Self::Slice {
        self.inner_slice().slice_with(range)
    }

    /// Returns the remaining contents of this haystack, as a `Slice`. For slice based haystacks,
    /// this is can be implemented as `&self.inner[self.index..]`.
    fn remainder_as_slice(&self) -> Self::Slice;

    /// Restores the `index` of the haystack to the provided one. This should only be called with
    /// indexes obtained by calling [`index`](Self::index) on this `Haystack`.
    fn go_to(&mut self, index: usize);

    fn rollback(&mut self, state: usize) -> &mut Self {
        self.go_to(state);
        self
    }

    fn skip(&mut self, count: usize) {
        self.go_to(self.index() + count);
    }

    fn reset(&mut self) {
        self.go_to(0);
    }

    fn is_start(&self) -> bool {
        self.index() == 0
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
        match self.item() {
            Some(n) if n.is_newline() => !self.prev_item().is_some_and(HaystackItem::is_return),
            Some(r) if r.is_return() => true,
            Some(_) => false,
            None => true,
        }
    }
}

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
    /// Creates a new [`Haystack`] from self. The result should be initialized at index 0.
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
//         <H as Haystack>::from_slice(self)
//     }
// }

/// A trait representing an owned, mutable type that can be converted into a [`Haystack`] as
/// required. This allows for [`Regex`](crate::expr::Regex) methods that replace matches or captures
/// from the original `Haystack`.
///
/// It is also used as the return type of the closures take by a couple of `Regex` replace methods.
pub trait OwnedHaystackable<I: HaystackItem> {
    type Hay<'a>: HaystackOf<'a, I> where Self: 'a;

    /// Replaces the substring at the position indicated by `range` with the `replacement`
    /// [`HaystackSlice`].
    fn replace_range<'a>(
        &mut self,
        range: Range<usize>,
        replacement: <Self::Hay<'a> as Haystack<'a>>::Slice
    ) where Self: 'a;

    /// Creates a temporary [`Haystack`] out of the underlying slice. This should usually be done by
    /// borrowing (or cloning if reference counted) and calling [`IntoHaystack::into_haystack`].
    fn as_haystack<'a>(&'a self) -> Self::Hay<'a>;

    /// Borrows the underlying [`HaystackSlice`] without creating a haystack. Used for slicing
    /// substrings. Note that `HaystackSlice` is inherently borrowed and probably be implemented for
    /// a reference.
    fn as_slice<'a>(&'a self) -> <Self::Hay<'a> as Haystack<'a>>::Slice;

    /// Returns the length of the underlying slice.
    fn len(&self) -> usize;

    /// Returns true if the underlying slice is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
