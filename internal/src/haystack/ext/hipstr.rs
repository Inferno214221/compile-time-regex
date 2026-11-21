use std::ops::Range;

use hipstr::Backend;
use hipstr::bytes::HipByt;
use hipstr::string::HipStr;

use crate::haystack::{ByteStack, Haystack, IntoHaystack, OwnedHaystackable, StrStack};

impl<'s, B: Backend> OwnedHaystackable<char> for HipStr<'s, B> {
    type Hay<'a> = StrStack<'a> where Self: 'a;

    fn replace_range<'a>(
        &mut self,
        range: Range<usize>,
        with: &'a <Self::Hay<'a> as Haystack<'a>>::Slice
    ) where Self: 'a {
        self.mutate().replace_range(range, with);
    }

    fn as_haystack<'a>(&'a self) -> Self::Hay<'a> {
        self.as_str().into_haystack()
    }

    fn as_slice<'a>(&'a self) -> &'a <Self::Hay<'a> as Haystack<'a>>::Slice {
        self.as_str()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<'s, B: Backend> OwnedHaystackable<u8> for HipByt<'s, B> {
    type Hay<'a> = ByteStack<'a> where Self: 'a;

    fn replace_range<'a>(
        &mut self,
        range: Range<usize>,
        with: &'a <Self::Hay<'a> as Haystack<'a>>::Slice
    ) where Self: 'a {
        self.mutate().splice(range, with.iter().copied());
    }

    fn as_haystack<'a>(&'a self) -> Self::Hay<'a> {
        self.as_slice().into_haystack()
    }

    fn as_slice<'a>(&'a self) -> &'a <Self::Hay<'a> as Haystack<'a>>::Slice {
        self.as_slice()
    }

    fn len(&self) -> usize {
        self.len()
    }
}
