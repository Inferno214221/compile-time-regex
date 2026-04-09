use std::{fmt::Debug, ops::Range};

mod haystack_item {
    pub trait Sealed {}
}

use haystack_item::Sealed;

/// A trait that represents an individual item that can be matched against a
/// [`Regex`](crate::expr::Regex). The primary (and only) two implementors are [`char`] and [`u8`].
///
/// # Sealed
///
/// This trait is sealed, preventing implementations because the `regex!` macro can't produce
/// `Regex` types that match against any `HaystackItem` other than the default. If you need to match
/// against another item type and want to use this crate, you may as well fork it so that you don't
/// have to write manual `Matcher` expressions.
pub trait HaystackItem: Debug + Default + Copy + Eq + Ord + Sealed {
    /// Creates a `Vec` of this item from the provided `&str`, used to convert string literals from
    /// parsed regular expressions into individual `HaystackItem`s that can be matched in a
    /// haystack.
    fn vec_from_str(value: &str) -> Vec<Self>;
}

/// A trait representing a slice of the underlying haystack for various
/// [`Haystack`](crate::haystack::Haystack) types.
///
/// The implementor of this trait is usually but not always, the only implementor of
/// [`IntoHaystack`](crate::haystack::IntoHaystack) for a haystack type.
///
/// It should be noted that this trait is often implemented of a reference to the type in question,
/// e.g. `&str` or `&[u8]` rather than `str` or `[u8]` themselves, so that the implementing type can
/// be cloned as required.
pub trait HaystackSlice<'a>: Debug + Clone + Sized {
    /// The `HaystackItem` contained within this slice.
    type Item: HaystackItem;


    /// Slices the underlying slice with the provided (half-open) `range`, used for retrieving
    /// values of capture groups.
    fn slice_with(&self, range: Range<usize>) -> Self;
}

impl Sealed for char {}

impl HaystackItem for char {
    fn vec_from_str(value: &str) -> Vec<Self> {
        value.chars().collect()
    }
}

impl<'a> HaystackSlice<'a> for &'a str {
    type Item = char;

    fn slice_with(&self, range: Range<usize>) -> Self {
        &self[range]
    }
}

impl Sealed for u8 {}

impl HaystackItem for u8 {
    fn vec_from_str(s: &str) -> Vec<Self> {
        s.as_bytes().to_vec()
    }
}

impl<'a> HaystackSlice<'a> for &'a [u8] {
    type Item = u8;

    fn slice_with(&self, range: Range<usize>) -> Self {
        &self[range]
    }
}