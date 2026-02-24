use std::{iter::Peekable, str::{Bytes, Chars}};

#[derive(Debug, Clone)]
pub struct Haystack<'a, I: HaystackItem> {
    iter: Peekable<I::Iter<'a>>,
    start: bool,
}

// TODO: Make Haystack track progress and then record it for groups?

impl<'a, I: HaystackItem> Haystack<'a, I> {
    pub fn new(value: &'a str) -> Haystack<'a, I> {
        Haystack {
            iter: I::from_str(value).peekable(),
            start: true,
        }
    }

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

pub trait HaystackItem: Copy {
    type Iter<'a>: Iterator<Item = Self> + Clone;

    fn from_str<'a>(s: &'a str) -> Self::Iter<'a>;
}

impl HaystackItem for u8 {
    type Iter<'a> = Bytes<'a>;
    
    fn from_str<'a>(s: &'a str) -> Self::Iter<'a> {
        s.bytes()
    }
}

impl HaystackItem for char {
    type Iter<'a> = Chars<'a>;
    
    fn from_str<'a>(s: &'a str) -> Self::Iter<'a> {
        s.chars()
    }
}