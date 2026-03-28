use std::{fmt::{self, Debug}, ops::Range};

use crate::haystack::{HaystackSlice};

pub trait HaystackIter<'a>: Iterator<Item = <Self::Slice as HaystackSlice<'a>>::Item> + Debug + Clone
{
    type Slice: HaystackSlice<'a>;

    fn from_slice(value: Self::Slice) -> Self;

    fn current_item(&self) -> Option<Self::Item>;

    fn current_index(&self) -> usize;

    fn as_slice(&self) -> Self::Slice;

    fn remainder_as_slice(&self) -> Self::Slice;

    fn slice_with(&self, range: Range<usize>) -> Self::Slice;

    fn go_to(&mut self, index: usize);
}

fn get_item<I>((_, item): (usize, I)) -> I { item }

#[derive(Clone)]
pub struct StrIter<'a> {
    inner: &'a str,
    index: usize,
}

impl<'a> StrIter<'a> {
    fn get_first_char(&self) -> (usize, Option<char>) {
        let mut iter = self.remainder_as_slice().char_indices();
        let first = iter.next();
        (iter.offset(), first.map(get_item))
    }
}

impl<'a> From<&'a str> for StrIter<'a> {
    fn from(inner: &'a str) -> Self {
        StrIter {
            inner,
            index: 0,
        }
    }
}

impl<'a> Iterator for StrIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let (width, first) = self.get_first_char();
        // The width won't exceed the remaining slice, so it can't overflow then length.
        self.index += width;
        first
    }
}

impl<'a> HaystackIter<'a> for StrIter<'a> {
    type Slice = &'a str;

    fn from_slice(value: Self::Slice) -> Self {
        StrIter {
            inner: value,
            index: 0,
        }
    }

    fn current_item(&self) -> Option<Self::Item> {
        get_item(self.get_first_char())
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

impl<'a> Debug for StrIter<'a> {
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

#[derive(Clone)]
pub struct ByteIter<'a> {
    inner: &'a [u8],
    index: usize,
}

impl<'a> From<&'a [u8]> for ByteIter<'a> {
    fn from(inner: &'a [u8]) -> Self {
        ByteIter {
            inner,
            index: 0,
        }
    }
}

impl<'a> Iterator for ByteIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.inner.len() {
            None
        } else {
            let byte = self.inner[self.index];
            self.index += 1;
            Some(byte)
        }
    }
}

impl<'a> HaystackIter<'a> for ByteIter<'a> {
    type Slice = &'a [u8];

    fn from_slice(value: Self::Slice) -> Self {
        ByteIter {
            inner: value,
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

impl<'a> Debug for ByteIter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "b\"")?;

        self.inner.iter().try_for_each(|byte| write!(f, "{:02x}", byte))?;

        write!(f, "\"\n  ")?;
        (0..self.index).try_for_each(|_| write!(f, "  "))?;
        write!(f, "^")
    }
}