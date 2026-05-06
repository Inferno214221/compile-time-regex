use std::{fmt::{self, Debug}, ops::Range};

use crate::haystack::{HaystackIter, HaystackSlice, IntoHaystack, OwnedHaystackable};

/// A haystack type for matching against the [`u8`]s in a [`&[u8]`](slice). This type provides very
/// straightforward indexing and iteration over the contained slice.
#[derive(Clone, PartialEq, Eq)]
pub struct ByteStack<'a> {
    inner: &'a [u8],
    index: usize,
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

    fn prev_item(&self) -> Option<Self::Item> {
        self.inner.get(self.index.checked_sub(1)?).copied()
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

impl<'a> HaystackSlice<'a> for &'a [u8] {
    type Item = u8;

    fn slice_with(&self, range: Range<usize>) -> Self {
        &self[range]
    }
}

impl<'a> IntoHaystack<'a, ByteStack<'a>> for &'a [u8] {
    fn into_haystack(self) -> ByteStack<'a> {
        ByteStack {
            inner: self,
            index: 0,
        }
    }
}

impl<'a> IntoHaystack<'a, ByteStack<'a>> for &'a Vec<u8> {
    fn into_haystack(self) -> ByteStack<'a> {
        ByteStack {
            inner: self,
            index: 0,
        }
    }
}

impl OwnedHaystackable<u8> for Vec<u8> {
    type Hay<'a> = ByteStack<'a>;

    fn replace_range<'a>(
        &mut self,
        range: Range<usize>,
        with: <Self::Hay<'a> as HaystackIter<'a>>::Slice
    ) {
        self.splice(range, with.iter().cloned());
    }

    fn as_haystack<'a>(&'a self) -> Self::Hay<'a> {
        self.into_haystack()
    }

    fn as_slice<'a>(&'a self) -> <Self::Hay<'a> as HaystackIter<'a>>::Slice {
        self
    }

    fn len(&self) -> usize {
        self.len()
    }
}