use std::ops::Range;

use crate::haystack::{
    HaystackIter, HaystackSlice, IntoHaystack, OwnedHaystackable, first_char, first_char_and_width
};

/// A haystack type for matching against the [`char`]s in a [`&str`](str). This type abstracts over
/// the variable width scalars contained, to allow indexing without panics.
///
/// To accomodate, calls to [`go_to`](Self::go_to) should only be made with an index previously
/// produced by this type for the specific haystack. Failure to do so, may cause a panic if indexing
/// on an invalid unicode boundary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl OwnedHaystackable<char> for String {
    type Hay<'a> = StrStack<'a>;

    fn replace_range<'a>(
        &mut self,
        range: Range<usize>,
        with: <Self::Hay<'a> as HaystackIter<'a>>::Slice
    ) where Self: 'a {
        self.replace_range(range, with);
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