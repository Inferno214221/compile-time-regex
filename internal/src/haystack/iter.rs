use std::{fmt::{self, Debug}, ops::Range};

use crate::haystack::{HaystackSlice, IntoHaystack};

// TODO: Document cheap cloning requirement, usize state. Understand slicing and iterating, often
// dealing with variable width unicode characters...

/// The main underlying trait for [`Haystack`](crate::haystack::Haystack) types, `HaystackIter`
/// should be implemented on new types that understand slicing and iterating over a haystack that
/// can be sliced into instances of `Self::Slice`.
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

    /// Returns the index of the current item in the original haystack. The returned value should be
    /// valid to pass to [`Self::go_to`] without causing a panic.
    fn current_index(&self) -> usize;

    /// Returns the underlying slice, as it was when this `HaystackIter` was created - representing
    /// the entire haystack being matched against.
    fn whole_slice(&self) -> Self::Slice;

    /// Returns the remaining contents of this haystack, as a `Slice`. For slice based haystacks,
    /// this is can be implemented as `&self.inner[self.index..]`.
    fn remainder_as_slice(&self) -> Self::Slice;

    /// Slices the original haystack with the provided (half-open) `range`, used for retrieving
    /// values of capture groups.
    fn slice_with(&self, range: Range<usize>) -> Self::Slice;

    /// Restores the `index` of the haystack to the provided one. This should only be called with
    /// indexes obtained by calling [`current_index`](Self::current_index) on this `HaystackIter`.
    fn go_to(&mut self, index: usize);
}

pub fn get_first_char(value: &str) -> (usize, Option<char>) {
    let mut iter = value.char_indices();
    let first = iter.next();
    (iter.offset(), first.map(get_item))
}

fn get_item<I>((_, item): (usize, I)) -> I { item }

/// A haystack type for matching against the [`char`]s in a [`&str`](str). This type abstracts over
/// the variable width scalars contained, to allow indexing without panics.
///
/// To accomodate, calls to [`go_to`](Self::go_to) should only be made with an index previously
/// produced by this type for the specific haystack. Failure to do so, may cause a panic if indexing
/// on an invalid unicode boundary.
#[derive(Clone)]
pub struct StrStack<'a> {
    inner: &'a str,
    index: usize,
}

impl<'a> IntoHaystack<'a, StrStack<'a>> for &'a str {
    fn into_haystack(self) -> StrStack<'a> {
        StrStack {
            inner: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for StrStack<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let (width, first) = get_first_char(self.remainder_as_slice());
        // The width won't exceed the remaining slice, so it can't overflow then length.
        self.index += width;
        first
    }
}

impl<'a> HaystackIter<'a> for StrStack<'a> {
    type Slice = &'a str;

    fn current_item(&self) -> Option<Self::Item> {
        get_item(get_first_char(self.remainder_as_slice()))
    }

    fn current_index(&self) -> usize {
        self.index
    }

    fn whole_slice(&self) -> Self::Slice {
        self.inner
    }

    fn remainder_as_slice(&self) -> Self::Slice {
        &self.inner[self.index..]
    }

    fn slice_with(&self, range: Range<usize>) -> Self::Slice {
        &self.inner[range]
    }

    fn go_to(&mut self, index: usize) {
        self.index = index;
    }
}

impl<'a> Debug for StrStack<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut len = 0;
        write!(f, "\"")?;

        self.inner.char_indices().try_for_each(|(index, ch)| {
            let mut debug = ch.escape_debug();
            if index < self.index  {
                len += debug.len();
            }
            debug.try_for_each(|debug_ch| write!(f, "{debug_ch}"))
        })?;

        write!(f, "\"\n ")?;
        (0..len).try_for_each(|_| write!(f, " "))?;
        write!(f, "^")
    }
}

/// A haystack type for matching against the [`u8`]s in a [`&[u8]`](slice). This type provides very
/// straightforward indexing and iteration over the contained slice.
#[derive(Clone)]
pub struct ByteStack<'a> {
    inner: &'a [u8],
    index: usize,
}

impl<'a> IntoHaystack<'a, ByteStack<'a>> for &'a [u8] {
    fn into_haystack(self) -> ByteStack<'a> {
        ByteStack {
            inner: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for ByteStack<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let byte = self.inner.get(self.index).copied();

        if byte.is_some() {
            self.index += 1;
        }

        byte
    }
}

impl<'a> HaystackIter<'a> for ByteStack<'a> {
    type Slice = &'a [u8];

    fn current_item(&self) -> Option<Self::Item> {
        self.inner.get(self.index).copied()
    }

    fn current_index(&self) -> usize {
        self.index
    }

    fn whole_slice(&self) -> Self::Slice {
        self.inner
    }

    fn remainder_as_slice(&self) -> Self::Slice {
        // FIXME: Check for possible panics when slicing.
        &self.inner[self.index..]
    }

    fn slice_with(&self, range: Range<usize>) -> Self::Slice {
        &self.inner[range]
    }

    fn go_to(&mut self, index: usize) {
        self.index = index;
    }
}

impl<'a> Debug for ByteStack<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "b\"")?;

        self.inner.iter().try_for_each(|byte| write!(f, "{:02x}", byte))?;

        write!(f, "\"\n  ")?;
        (0..self.index).try_for_each(|_| write!(f, "  "))?;
        write!(f, "^")
    }
}