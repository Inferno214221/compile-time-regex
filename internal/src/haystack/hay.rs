use std::ops::Range;

use crate::{haystack::{ByteIter, HaystackIter, StrIter}, hir::{CastClass, WriteMatcher}};

#[derive(Debug, Clone)]
pub struct Haystack<'a, I: HaystackItem> {
    inner: I::Iter<'a>,
}

// TODO: Make Haystack track progress and then record it for groups?

impl<'a> From<&'a str> for Haystack<'a, char> {
    fn from(value: &'a str) -> Self {
        Haystack {
            inner: char::iter_from_slice(value),
        }
    }
}

impl<'a> From<&'a [u8]> for Haystack<'a, u8> {
    fn from(value: &'a [u8]) -> Self {
        Haystack {
            inner: u8::iter_from_slice(value),
        }
    }
}

impl<'a, I: HaystackItem> Haystack<'a, I> {
    pub fn item(&mut self) -> Option<I> {
        self.inner.current_item()
    }

    pub fn index(&mut self) -> usize {
        self.inner.current_index()
    }

    // Progression is only completed by elements which explicitly check the byte and succeed.
    pub fn progress(&mut self) {
        self.inner.next();
    }

    pub fn is_start(&mut self) -> bool {
        self.inner.is_start()
    }

    pub fn is_end(&mut self) -> bool {
        self.item().is_none()
    }

    pub fn slice(&'a self, range: Range<usize>) -> <I::Iter<'a> as HaystackIter<'a>>::Slice<'a> {
        self.inner.slice_with(range)
    }
}

pub trait HaystackItem: Copy + WriteMatcher + CastClass {
    type Iter<'a>: HaystackIter<'a, Item = Self> + Clone;

    type Slice<'a>: Copy;

    fn iter_from_str<'a>(value: &'a str) -> Self::Iter<'a>;

    fn iter_from_slice<'a>(value: Self::Slice<'a>) -> Self::Iter<'a>;

    fn vec_from_str(value: &str) -> Vec<Self>;
}

impl HaystackItem for u8 {
    type Iter<'a> = ByteIter<'a>;

    type Slice<'a> = <Self::Iter<'a> as HaystackIter<'a>>::Slice<'a>;
    
    fn iter_from_str<'a>(value: &'a str) -> Self::Iter<'a> {
        Self::iter_from_slice(value.as_bytes())
    }

    fn iter_from_slice<'a>(value: Self::Slice<'a>) -> Self::Iter<'a> {
        ByteIter::from(value)
    }

    fn vec_from_str(s: &str) -> Vec<Self> {
        s.as_bytes().to_vec()
    }
}

impl HaystackItem for char {
    type Iter<'a> = StrIter<'a>;

    type Slice<'a> = <Self::Iter<'a> as HaystackIter<'a>>::Slice<'a>;
    
    fn iter_from_str<'a>(value: &'a str) -> Self::Iter<'a> {
        StrIter::from(value)
    }

    fn iter_from_slice<'a>(value: Self::Slice<'a>) -> Self::Iter<'a> {
        Self::iter_from_str(value)
    }

    fn vec_from_str(value: &str) -> Vec<Self> {
        value.chars().collect()
    }
}