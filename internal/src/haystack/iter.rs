use std::{fmt::{self, Debug}, ops::Range};

use crate::haystack::{HaystackSlice, IntoHaystack};

// TODO: Document cheap cloning requirement. Understand slicing and iterating...
pub trait HaystackIter<'a>: Iterator<Item = <Self::Slice as HaystackSlice<'a>>::Item> + Debug + Clone {
    type Slice: HaystackSlice<'a>;

    fn from_slice(value: Self::Slice) -> Self;

    fn current_item(&self) -> Option<Self::Item>;

    fn current_index(&self) -> usize;

    fn as_slice(&self) -> Self::Slice;

    fn remainder_as_slice(&self) -> Self::Slice;

    fn slice_with(&self, range: Range<usize>) -> Self::Slice;

    fn go_to(&mut self, index: usize);
}

pub fn get_first_char(value: &str) -> (usize, Option<char>) {
    let mut iter = value.char_indices();
    let first = iter.next();
    (iter.offset(), first.map(get_item))
}

fn get_item<I>((_, item): (usize, I)) -> I { item }

/// A haystack type for matching against the [`char`]s in a [`&'a str`](str). This type abstracts
/// over the variable width scalars contained, to allow indexing without panics.
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
        StrStack::from_slice(self)
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

    fn from_slice(inner: Self::Slice) -> Self {
        StrStack {
            inner,
            index: 0,
        }
    }

    fn current_item(&self) -> Option<Self::Item> {
        get_item(get_first_char(self.remainder_as_slice()))
    }

    fn current_index(&self) -> usize {
        self.index
    }

    fn as_slice(&self) -> Self::Slice {
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

/// A haystack type for matching against the [`u8`]s in a [`&'a [u8]`](slice). This type provides
/// very straightforward indexing and iteration over the contained slice.
#[derive(Clone)]
pub struct ByteStack<'a> {
    inner: &'a [u8],
    index: usize,
}

impl<'a> IntoHaystack<'a, ByteStack<'a>> for &'a [u8] {
    fn into_haystack(self) -> ByteStack<'a> {
        ByteStack::from_slice(self)
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

    fn from_slice(inner: Self::Slice) -> Self {
        ByteStack {
            inner,
            index: 0,
        }
    }

    fn current_item(&self) -> Option<Self::Item> {
        self.inner.get(self.index).copied()
    }

    fn current_index(&self) -> usize {
        self.index
    }

    fn as_slice(&self) -> Self::Slice {
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