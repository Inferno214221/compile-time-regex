use std::marker::PhantomData;
use std::ops::Range;

use arcstr::{ArcStr, Substr};

use crate::haystack::{
    Haystack, HaystackSlice, IntoHaystack, first_char, first_char_and_width,
};

impl<'a> HaystackSlice<'a> for Substr {
    type Item = char;

    fn slice_with(&self, range: Range<usize>) -> Self {
        self.substr(range)
    }

    fn as_bytes(&self) -> &[u8] {
        Substr::as_str(self).as_bytes()
    }
}

/// A haystack type for matching against the [`char`]s in an [`ArcStr`]. Although [`IntoHaystack`]
/// is implemented for `ArcStr`, the associated `Slice` type for this `Haystack` is `Substr`.
#[derive(Debug, Clone, Hash)]
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
        let (width, first) = first_char_and_width(&self.inner);
        // The width won't exceed the remaining slice, so it can't overflow then length.
        self.index += width;
        first
    }
}

impl<'a> Haystack<'a> for ArcStrStack<'a> {
    type Slice = Substr;

    fn item(&self) -> Option<Self::Item> {
        first_char(&self.inner[self.index..])
    }

    fn prev_item(&self) -> Option<Self::Item> {
        let prev_index = self.inner.floor_char_boundary(self.index.checked_sub(1)?);
        first_char(&self.inner[prev_index..])
    }

    fn index(&self) -> usize {
        self.index
    }

    fn inner_slice(&self) -> Self::Slice {
        Substr::full(self.inner.clone())
    }

    fn remainder_as_slice(&self) -> Self::Slice {
        self.inner.substr(self.index..)
    }

    fn go_to(&mut self, index: usize) {
        self.index = index;
    }
}
