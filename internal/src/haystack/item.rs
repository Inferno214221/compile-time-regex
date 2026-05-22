use std::fmt::Debug;

mod haystack_item {
    pub trait Sealed {}
}

use haystack_item::Sealed;

/// A trait that represents an individual item that can be matched against a
/// [`Regex`](crate::expr::Regex). The primary (and only) two implementers are [`char`] and [`u8`].
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
    fn collect_from_str(value: &str) -> Vec<Self>;

    fn collect_from_bytes(value: &[u8]) -> Vec<Self>;

    fn is_newline(self) -> bool;

    fn is_return(self) -> bool;
}

/// A helper for getting the first `char` of a provided `&str`. Returns the width of the character
/// (possibly zero) and the character itself.
pub fn first_char_and_width(value: &str) -> (usize, Option<char>) {
    // Unfortunately, I don't think there is a stable way to get `char`s from a `str` without using
    // the `chars` or `char_indices` iterators. We can calculate the width easily but may as well
    // have it done for us.
    let mut iter = value.char_indices();
    let first = iter.next();
    (iter.offset(), first.map(|(_, c)| c))
}

pub fn first_char(value: &str) -> Option<char> {
    value.chars().next()
}

impl Sealed for char {}

impl HaystackItem for char {
    fn collect_from_str(value: &str) -> Vec<Self> {
        value.chars().collect()
    }

    fn collect_from_bytes(value: &[u8]) -> Vec<Self> {
        Self::collect_from_str(
            str::from_utf8(value).expect("failed to convert bytes to valid unicode")
        )

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
    fn collect_from_str(value: &str) -> Vec<Self> {
        Self::collect_from_bytes(value.as_bytes())
    }


    fn collect_from_bytes(s: &[u8]) -> Vec<Self> {
        s.to_vec()
    }

    fn is_newline(self) -> bool {
        self == b'\n'
    }

    fn is_return(self) -> bool {
        self == b'\r'
    }
}