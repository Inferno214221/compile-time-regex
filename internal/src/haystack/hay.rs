use crate::{general::Capture, haystack::{HaystackItem, HaystackIter}};

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

    pub fn slice(&'a self, cap: Capture) -> <I::Iter<'a> as HaystackIter<'a>>::Slice<'a> {
        self.inner.slice_with(cap.0)
    }
}