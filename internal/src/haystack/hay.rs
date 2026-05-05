use std::ops::Range;

use crate::haystack::{HaystackItem, HaystackIter, HaystackSlice, StrStack};

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
pub trait MutIntoHaystack<'a, I: HaystackItem> {
    type Hay<'b>: HaystackOf<'b, I> where Self: 'b;

    fn replace_range(&mut self, range: Range<usize>, with: &str);

    fn as_haystack<'b>(&'b self) -> Self::Hay<'b>;

    fn len(&self) -> usize;
}

impl<'a> MutIntoHaystack<'a, char> for String {
    type Hay<'b> = StrStack<'b>;

    fn replace_range(&mut self, range: Range<usize>, with: &str) {
        self.replace_range(range, with);
    }

    fn as_haystack<'b>(&'b self) -> Self::Hay<'b> {
        self.into_haystack()
    }

    fn len(&self) -> usize {
        String::len(self)
    }
}
