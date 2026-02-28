use std::fmt::Debug;
    
use crate::{haystack::{ByteIter, HaystackIter, StrIter}, hir::{CastClass, WriteMatcher}};

pub trait HaystackItem: Copy + WriteMatcher + CastClass {
    type Iter<'a>: HaystackIter<'a, Item = Self> + Clone;

    type Slice<'a>: Debug + Copy;

    fn iter_from_str<'a>(value: &'a str) -> Self::Iter<'a>;

    fn iter_from_slice<'a>(value: Self::Slice<'a>) -> Self::Iter<'a>;

    fn vec_from_str(value: &str) -> Vec<Self>;
}

impl HaystackItem for u8 {
    type Iter<'a> = ByteIter<'a>;

    type Slice<'a> = &'a [u8];
    
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

    type Slice<'a> = &'a str;
    
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