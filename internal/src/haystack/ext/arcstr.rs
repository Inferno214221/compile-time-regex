use std::{marker::PhantomData, ops::Range};

use arcstr::{ArcStr, Substr};

use crate::haystack::{HaystackIter, HaystackSlice, IntoHaystack, get_first_char};

fn get_item<I>((_, item): (usize, I)) -> I { item }

impl<'a> HaystackSlice<'a> for Substr {
    type Item = char;
}

/// A haystack type for matching against the [`char`]s in an [`ArcStr`](arcstr::ArcStr). Rather than
/// actual `ArcStr`s, this type internally stores [`Substr`](arcstr::Substr)s. Although
/// [`IntoHaystack`] is implemented for `ArcStr`, the associated `Slice` type for this `Haystack` is
/// `Substr`.
#[derive(Debug, Clone)]
pub struct ArcStrStack<'a> {
    inner: ArcStr,
    index: usize,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> IntoHaystack<'a, ArcStrStack<'a>> for ArcStr {
    fn into_haystack(self) -> ArcStrStack<'a> {
        ArcStrStack {
            inner: self,
            index: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a> Iterator for ArcStrStack<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let (width, first) = get_first_char(&self.inner);
        // The width won't exceed the remaining slice, so it can't overflow then length.
        self.index += width;
        first
    }
}

impl<'a> HaystackIter<'a> for ArcStrStack<'a> {
    type Slice = Substr;

    fn current_item(&self) -> Option<Self::Item> {
        get_item(get_first_char(&self.inner))
    }

    fn current_index(&self) -> usize {
        self.index
    }

    fn whole_slice(&self) -> Self::Slice {
        Substr::full(self.inner.clone())
    }

    fn remainder_as_slice(&self) -> Self::Slice {
        self.inner.substr(self.index..)
    }

    fn slice_with(&self, range: Range<usize>) -> Self::Slice {
        self.inner.substr(range)
    }

    fn go_to(&mut self, index: usize) {
        self.index = index;
    }
}