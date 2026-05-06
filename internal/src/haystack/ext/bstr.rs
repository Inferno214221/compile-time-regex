use std::ops::Range;

use bstr::{BStr, BString};

use crate::haystack::{HaystackIter, HaystackSlice, IntoHaystack, OwnedHaystackable};

impl<'a> HaystackSlice<'a> for &'a BStr {
    type Item = u8;

    fn slice_with(&self, range: Range<usize>) -> Self {
        &self[range]
    }
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

// BString implements Deref<Target = Vec<u8>>, so it will implicitly go to the wrong haystack type.
// This does raise the question of how different the types really are and what benefit there is to
// restricting everything to BStr when conversions are cheap. Answer: Why not? Its pretty easy to
// implement.
impl<'a> IntoHaystack<'a, BStrStack<'a>> for &'a BString {
    fn into_haystack(self) -> BStrStack<'a> {
        BStrStack {
            inner: BStr::new(self),
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
        &self.inner[self.index..]
    }

    fn go_to(&mut self, index: usize) {
        self.index = index;
    }
}

impl OwnedHaystackable<u8> for BString {
    type Hay<'a> = BStrStack<'a>;

    fn replace_range<'a>(
        &mut self,
        range: Range<usize>,
        with: <Self::Hay<'a> as HaystackIter<'a>>::Slice
    ) where Self: 'a {
        self.splice(range, with.iter().copied());
    }

    fn as_haystack<'a>(&'a self) -> Self::Hay<'a> {
        self.into_haystack()
    }

    fn as_slice<'a>(&'a self) -> <Self::Hay<'a> as HaystackIter<'a>>::Slice {
        BStr::new(self)
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }
}