use std::ops::Range;

use ecow::{EcoString, EcoVec};

use crate::haystack::{ByteStack, HaystackIter, IntoHaystack, OwnedHaystackable, StrStack};

impl OwnedHaystackable<char> for EcoString {
    type Hay<'a> = StrStack<'a>;

    fn replace_range<'a>(
        &mut self,
        range: Range<usize>,
        with: <Self::Hay<'a> as HaystackIter<'a>>::Slice
    ) where Self: 'a {
        let tail = EcoString::from(self.split_at(range.end).1);
        self.truncate(range.start);
        self.push_str(with);
        self.push_str(&tail);
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

impl OwnedHaystackable<u8> for EcoVec<u8> {
    type Hay<'a> = ByteStack<'a>;

    fn replace_range<'a>(
        &mut self,
        range: Range<usize>,
        with: <Self::Hay<'a> as HaystackIter<'a>>::Slice
    ) where Self: 'a {
        if range.len() == with.len() {
            let mut_self = self.make_mut();
            for (src, target) in range.enumerate() {
                mut_self[target] = with[src];
            }
        } else {
            let tail = EcoVec::from(self.split_at(range.end).1);
            self.truncate(range.start);
            self.extend_from_slice(with);
            self.extend_from_slice(&tail);
        }
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