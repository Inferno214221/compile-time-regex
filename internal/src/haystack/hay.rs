use std::{fmt::{self, Debug}, ops::Range};

use crate::haystack::{HaystackItem, HaystackIter};

// TODO: Create a HatstackState(usize) for even cheaper clones.

/// A type used to reference the haystack when matching of capturing against a
/// [`Regex`](crate::expr::Regex), in addition to tracking progression.
///
/// It is rare that users will have to interact with this trait, appart from Trait bounds. All
/// public methods will take an `impl Into<Haystack<'a, I>>` as an argument.
///
/// Because of the progression tracking, a `Haystack` can't be matched against multiple times
/// without [`reset`](Self::reset)ting it first, or it will continue where the first pattern
/// finished.
///
/// The `Haystack` type is accompanied by a helper trait, [`HaystackItem`], representing an item
/// that can be matched against a [`Regex`](crate::expr::Regex).
#[derive(Clone)]
pub struct Haystack<'a, I: HaystackItem> {
    inner: I::Iter<'a>,
}

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
    pub fn item(&self) -> Option<I> {
        self.inner.current_item()
    }

    pub fn index(&self) -> usize {
        self.inner.current_index()
    }

    // Progression is only completed by elements which explicitly check the byte and succeed.
    pub fn progress(&mut self) {
        self.inner.next();
    }

    pub fn is_start(&self) -> bool {
        self.inner.is_start()
    }

    pub fn is_end(&self) -> bool {
        self.item().is_none()
    }

    pub fn slice(&self, cap: Range<usize>) -> I::Slice<'a> {
        self.inner.slice_with(cap)
    }

    pub fn reset(&mut self) {
        self.inner.rollback(0);
    }
}

impl<'a, I: HaystackItem> Debug for Haystack<'a, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Haystack(\n{:?}\n)", self.inner)
    }
}