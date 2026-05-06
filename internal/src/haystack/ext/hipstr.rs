use std::fmt::{self, Debug};
use std::ops::Range;

use hipstr::Backend;
use hipstr::bytes::HipByt;
use hipstr::string::HipStr;

use crate::haystack::{
    HaystackIter, HaystackSlice, IntoHaystack, OwnedHaystackable, first_char, first_char_and_width
};

impl<'a, B: Backend> HaystackSlice<'a> for HipStr<'a, B> {
    type Item = char;

    fn slice_with(&self, range: Range<usize>) -> Self {
        self.slice(range)
    }
}

/// A haystack type for matching against the [`char`]s in a [`HipStr<'a, B>`](hipstr::HipStr).
pub struct HipStrStack<'a, B: Backend> {
    inner: HipStr<'a, B>,
    index: usize,
}

impl<'a, B: Backend> IntoHaystack<'a, HipStrStack<'a, B>> for HipStr<'a, B> {
    fn into_haystack(self) -> HipStrStack<'a, B> {
        HipStrStack {
            inner: self,
            index: 0,
        }
    }
}

impl<'a, B: Backend> Iterator for HipStrStack<'a, B> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let (width, first) = first_char_and_width(&self.remainder_as_slice());
        // The width won't exceed the remaining slice, so it can't overflow then length.
        self.index += width;
        first
    }
}

impl<'a, B: Backend> HaystackIter<'a> for HipStrStack<'a, B> {
    type Slice = HipStr<'a, B>;

    fn current_item(&self) -> Option<Self::Item> {
        first_char(&self.remainder_as_slice())
    }

    fn prev_item(&self) -> Option<Self::Item> {
        let prev_index = self.inner.floor_char_boundary(self.index.checked_sub(1)?);
        first_char(&self.inner[prev_index..])
    }

    fn current_index(&self) -> usize {
        self.index
    }

    fn whole_slice(&self) -> Self::Slice {
        self.inner.clone()
    }

    fn remainder_as_slice(&self) -> Self::Slice {
        self.inner.slice(self.index..)
    }

    fn go_to(&mut self, index: usize) {
        self.index = index;
    }
}

impl<'s, B: Backend> OwnedHaystackable<char> for HipStr<'s, B> {
    type Hay<'a> = HipStrStack<'a, B> where Self: 'a;

    fn replace_range<'a>(
        &mut self,
        range: Range<usize>,
        with: <Self::Hay<'a> as HaystackIter<'a>>::Slice
    ) where Self: 'a {
        self.mutate().replace_range(range, &with);
    }

    fn as_haystack<'a>(&'a self) -> Self::Hay<'a> {
        self.clone().into_haystack()
    }

    fn as_slice<'a>(&'a self) -> <Self::Hay<'a> as HaystackIter<'a>>::Slice {
        self.clone()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

// Implemented to relax B: Debug bound.
impl<'a, B: Backend> Debug for HipStrStack<'a, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HipStrStack")
            .field("inner", &self.inner)
            .field("index", &self.index)
            .finish()
    }
}

impl<'a, B: Backend> Clone for HipStrStack<'a, B> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            index: self.index,
        }
    }
}

impl<'a, B: Backend> HaystackSlice<'a> for HipByt<'a, B> {
    type Item = u8;

    fn slice_with(&self, range: Range<usize>) -> Self {
        self.slice(range)
    }
}

/// A haystack type for matching against the [`u8`]s in a [`HipByt<'a, B>`](hipstr::HipByt).
pub struct HipBytStack<'a, B: Backend> {
    inner: HipByt<'a, B>,
    index: usize,
}

impl<'a, B: Backend> IntoHaystack<'a, HipBytStack<'a, B>> for HipByt<'a, B> {
    fn into_haystack(self) -> HipBytStack<'a, B> {
        HipBytStack {
            inner: self,
            index: 0,
        }
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
        self.inner.clone()
    }

    fn remainder_as_slice(&self) -> Self::Slice {
        self.inner.slice(self.index..)
    }

    fn go_to(&mut self, index: usize) {
        self.index = index;
    }
}

impl<'a, B: Backend> Debug for HipBytStack<'a, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HipBytStack")
            .field("inner", &self.inner)
            .field("index", &self.index)
            .finish()
    }
}

impl<'a, B: Backend> Clone for HipBytStack<'a, B> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            index: self.index,
        }
    }
}

impl<'s, B: Backend> OwnedHaystackable<u8> for HipByt<'s, B> {
    type Hay<'a> = HipBytStack<'a, B> where Self: 'a;

    fn replace_range<'a>(
        &mut self,
        range: Range<usize>,
        with: <Self::Hay<'a> as HaystackIter<'a>>::Slice
    ) where Self: 'a {
        self.mutate().splice(range, with.iter().copied());
    }

    fn as_haystack<'a>(&'a self) -> Self::Hay<'a> {
        self.clone().into_haystack()
    }

    fn as_slice<'a>(&'a self) -> <Self::Hay<'a> as HaystackIter<'a>>::Slice {
        self.clone()
    }

    fn len(&self) -> usize {
        self.len()
    }
}