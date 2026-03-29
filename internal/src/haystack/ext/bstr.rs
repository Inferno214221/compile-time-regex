use std::ops::Range;

use bstr::BStr;

use crate::haystack::{HaystackIter, HaystackSlice, IntoHaystack};

impl<'a> HaystackSlice<'a> for &'a BStr {
    type Item = u8;
}

/// A haystack type for matching against the [`u8`]s in a [`&'a BStr`](bstr::BStr). This type is a
/// very basic example of how the haystack traits can be implemented outside of the crate itself.
#[derive(Debug, Clone)]
pub struct BStrStack<'a> {
    inner: &'a BStr,
    index: usize,
}

impl<'a> IntoHaystack<'a, BStrStack<'a>> for &'a BStr {
    fn into_haystack(self) -> BStrStack<'a> {
        BStrStack {
            inner: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for BStrStack<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let byte = self.inner.get(self.index).copied();

        if byte.is_some() {
            self.index += 1;
        }

        byte
    }
}

impl<'a> HaystackIter<'a> for BStrStack<'a> {
    type Slice = &'a BStr;

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
        &self.inner[self.index..]
    }

    fn slice_with(&self, range: Range<usize>) -> Self::Slice {
        &self.inner[range]
    }

    fn go_to(&mut self, index: usize) {
        self.index = index;
    }
}