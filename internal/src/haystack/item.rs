use std::fmt::Debug;

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

    fn is_newline(self) -> bool;

    fn is_return(self) -> bool;
}

impl Sealed for char {}

impl HaystackItem for char {
    fn vec_from_str(value: &str) -> Vec<Self> {
        value.chars().collect()
    }

    fn is_newline(self) -> bool {
        self == '\n'
    }

    fn is_return(self) -> bool {
        self == '\r'
    }
}

impl Sealed for u8 {}

impl HaystackItem for u8 {
    fn vec_from_str(s: &str) -> Vec<Self> {
        s.as_bytes().to_vec()
    }

    fn is_newline(self) -> bool {
        self == b'\n'
    }

    fn is_return(self) -> bool {
        self == b'\r'
    }
}