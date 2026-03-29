use std::{fmt::{self, Debug}, ops::Range};

use hipstr::{Backend, bytes::HipByt, string::HipStr};

use crate::haystack::{HaystackIter, HaystackSlice, IntoHaystack, get_first_char};

fn get_item<I>((_, item): (usize, I)) -> I { item }

impl<'a, B: Backend> HaystackSlice<'a> for HipStr<'a, B> {
    type Item = char;
}

/// A haystack type for matching against the [`char`]s in a [`HipStr<'a, B>`](hipstr::HipStr).
pub struct HipStrStack<'a, B: Backend> {
    inner: HipStr<'a, B>,
    index: usize,
}

impl<'a, B: Backend> IntoHaystack<'a, HipStrStack<'a, B>> for HipStr<'a, B> {
    fn into_haystack(self) -> HipStrStack<'a, B> {
        HipStrStack::from_slice(self)
    }
}

impl<'a, B: Backend> Iterator for HipStrStack<'a, B> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let (width, first) = get_first_char(&self.remainder_as_slice());
        // The width won't exceed the remaining slice, so it can't overflow then length.
        self.index += width;
        first
    }
}

impl<'a, B: Backend> HaystackIter<'a> for HipStrStack<'a, B> {
    type Slice = HipStr<'a, B>;

    fn from_slice(inner: Self::Slice) -> Self {
        HipStrStack {
            inner,
            index: 0,
        }
    }

    fn current_item(&self) -> Option<Self::Item> {
        get_item(get_first_char(&self.remainder_as_slice()))
    }

    fn current_index(&self) -> usize {
        self.index
    }

    fn as_slice(&self) -> Self::Slice {
        self.inner.clone()
    }

    fn remainder_as_slice(&self) -> Self::Slice {
        self.inner.slice(self.index..)
    }

    fn slice_with(&self, range: Range<usize>) -> Self::Slice {
        self.inner.slice(range)
    }

    fn go_to(&mut self, index: usize) {
        self.index = index;
    }
}

impl<'a, B: Backend> Debug for HipStrStack<'a, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HipStrStack").field("inner", &self.inner).field("index", &self.index).finish()
    }
}

impl<'a, B: Backend> Clone for HipStrStack<'a, B> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone(), index: self.index.clone() }
    }
}

impl<'a, B: Backend> HaystackSlice<'a> for HipByt<'a, B> {
    type Item = u8;
}

/// A haystack type for matching against the [`u8`]s in a [`HipByt<'a, B>`](hipstr::HipByt).
pub struct HipBytStack<'a, B: Backend> {
    inner: HipByt<'a, B>,
    index: usize,
}

impl<'a, B: Backend> IntoHaystack<'a, HipBytStack<'a, B>> for HipByt<'a, B> {
    fn into_haystack(self) -> HipBytStack<'a, B> {
        HipBytStack::from_slice(self)
    }
}

impl<'a, B: Backend> Iterator for HipBytStack<'a, B> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let byte = self.inner.get(self.index).copied();

        if byte.is_some() {
            self.index += 1;
        }

        byte
    }
}

impl<'a, B: Backend> HaystackIter<'a> for HipBytStack<'a, B> {
    type Slice = HipByt<'a, B>;

    fn from_slice(inner: Self::Slice) -> Self {
        HipBytStack {
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
        self.inner.clone()
    }

    fn remainder_as_slice(&self) -> Self::Slice {
        self.inner.slice(self.index..)
    }

    fn slice_with(&self, range: Range<usize>) -> Self::Slice {
        self.inner.slice(range)
    }

    fn go_to(&mut self, index: usize) {
        self.index = index;
    }
}

impl<'a, B: Backend> Debug for HipBytStack<'a, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HipBytStack").field("inner", &self.inner).field("index", &self.index).finish()
    }
}

impl<'a, B: Backend> Clone for HipBytStack<'a, B> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone(), index: self.index.clone() }
    }
}