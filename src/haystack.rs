use std::{iter::Peekable, str::Bytes};

#[derive(Debug, Clone)]
pub struct Haystack<'a> {
    iter: Peekable<Bytes<'a>>,
    start: bool,
}

impl<'a> Haystack<'a> {
    pub fn new(value: &'a str) -> Haystack<'a> {
        Haystack {
            iter: value.bytes().peekable(),
            start: true,
        }
    }

    pub fn byte(&mut self) -> Option<u8> {
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
        self.byte().is_none()
    }
}