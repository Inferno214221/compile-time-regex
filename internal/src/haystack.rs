use std::{iter::{Copied, Peekable}, slice::Iter, str::Chars};

use crate::hir::WriteTypeExpr;


#[derive(Debug, Clone)]
pub struct Haystack<'a, I: HaystackItem> {
    iter: Peekable<I::Iter<'a>>,
    start: bool,
}

// TODO: Make Haystack track progress and then record it for groups?

impl<'a> From<&'a str> for Haystack<'a, char> {
    fn from(value: &'a str) -> Self {
        Haystack {
            iter: value.chars().peekable(),
            start: true
        }
    }
}

impl<'a> From<&'a [u8]> for Haystack<'a, u8> {
    fn from(value: &'a [u8]) -> Self {
        Haystack {
            iter: value.iter().copied().peekable(),
            start: true
        }
    }
}

impl<'a, I: HaystackItem> Haystack<'a, I> {
    pub fn item(&mut self) -> Option<I> {
        self.iter.peek().copied()
    }

    // Progression is only completed by elements which explicitly check the byte and succeed.
    pub fn progress(&mut self) {
        self.iter.next();
        self.start = false;
    }

    pub fn is_start(&mut self) -> bool {
        self.start
    }

    pub fn is_end(&mut self) -> bool {
        // TODO: Check that there is no other way of getting a None
        self.item().is_none()
    }
}

pub trait HaystackItem: Copy + WriteTypeExpr {
    type Iter<'a>: Iterator<Item = Self> + Clone;

    fn from_str<'a>(s: &'a str) -> Self::Iter<'a>;
}

impl HaystackItem for u8 {
    type Iter<'a> = Copied<Iter<'a, u8>>;
    
    fn from_str<'a>(s: &'a str) -> Self::Iter<'a> {
        s.as_bytes().iter().copied()
    }
}

impl HaystackItem for char {
    type Iter<'a> = Chars<'a>;
    
    fn from_str<'a>(s: &'a str) -> Self::Iter<'a> {
        s.chars()
    }
}