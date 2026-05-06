use std::{fmt::{self, Debug}, ops::Range};

use crate::haystack::{HaystackIter, HaystackSlice, IntoHaystack, OwnedHaystackable};

/// A helper for getting the first `char` of a provided `&str`. Returns the width of the character
/// (possibly zero) and the character itself.
pub fn first_char_and_width(value: &str) -> (usize, Option<char>) {
    // Unfortunately, I don't think there is a stable way to get `char`s from a `str` without using
    // the `chars` or `char_indices` iterators. We can calculate the width easily but may as well
    // have it done for us.
    let mut iter = value.char_indices();
    let first = iter.next();
    (iter.offset(), first.map(|(_, c)| c))
}

pub fn first_char(value: &str) -> Option<char> {
    value.chars().next()
}

/// A haystack type for matching against the [`char`]s in a [`&str`](str). This type abstracts over
/// the variable width scalars contained, to allow indexing without panics.
///
/// To accomodate, calls to [`go_to`](Self::go_to) should only be made with an index previously
/// produced by this type for the specific haystack. Failure to do so, may cause a panic if indexing
/// on an invalid unicode boundary.
#[derive(Clone, PartialEq, Eq)]
pub struct StrStack<'a> {
    inner: &'a str,
    index: usize,
}

impl<'a> Iterator for StrStack<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let (width, first) = first_char_and_width(self.remainder_as_slice());
        // The width won't exceed the remaining slice, so it can't overflow then length.
        self.index += width;
        first
    }
}

impl<'a> HaystackIter<'a> for StrStack<'a> {
    type Slice = &'a str;

    fn current_item(&self) -> Option<Self::Item> {
        first_char(self.remainder_as_slice())
    }

    fn prev_item(&self) -> Option<Self::Item> {
        let prev_index = self.inner.floor_char_boundary(self.index.checked_sub(1)?);
        first_char(&self.inner[prev_index..])
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
            if index < self.index {
                len += debug.len();
            }
            debug.try_for_each(|debug_ch| write!(f, "{debug_ch}"))
        })?;

        write!(f, "\"\n ")?;
        (0..len).try_for_each(|_| write!(f, " "))?;
        write!(f, "^")
    }
}

impl<'a> HaystackSlice<'a> for &'a str {
    type Item = char;

    fn slice_with(&self, range: Range<usize>) -> Self {
        &self[range]
    }
}

impl<'a> IntoHaystack<'a, StrStack<'a>> for &'a str {
    fn into_haystack(self) -> StrStack<'a> {
        StrStack {
            inner: self,
            index: 0,
        }
    }
}

impl<'a> IntoHaystack<'a, StrStack<'a>> for &'a String {
    fn into_haystack(self) -> StrStack<'a> {
        StrStack {
            inner: self,
            index: 0,
        }
    }
}

impl<'a> OwnedHaystackable<'a, char> for String {
    type Hay<'b> = StrStack<'b>;

    fn replace_range<'b>(
        &mut self,
        range: Range<usize>,
        with: <Self::Hay<'b> as HaystackIter<'b>>::Slice
    ) {
        self.replace_range(range, with);
    }

    fn as_haystack<'b>(&'b self) -> Self::Hay<'b> {
        self.into_haystack()
    }

    fn as_slice<'b>(&'b self) -> <Self::Hay<'b> as HaystackIter<'b>>::Slice {
        self
    }

    fn len(&self) -> usize {
        self.len()
    }
}