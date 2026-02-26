use crate::haystack::{Haystack, HaystackItem};

pub trait Matcher<I: HaystackItem> {
    fn matches(hay: &mut Haystack<I>) -> bool;

    // It would be nice to use a customer Iterator here rather than a Vec, but reversing an
    // arbitrary match is not easy, so we just progress through linearly and store them all.
    // This could cause issues with huge haystacks, but: all regexes need to be compiled at compile
    // time and are hence controlled by the author. If their pattern will be operating on huge
    // haystacks and need backtracking, that's up to them.
    fn all_matches<'a>(hay: &mut Haystack<'a, I>) -> Vec<Haystack<'a, I>> {
        if Self::matches(hay) {
            vec![hay.clone()]
        } else {
            vec![]
        }
    }
}

#[derive(Debug, Default)]
pub struct Byte<const N: u8>;

impl<const N: u8> Matcher<u8> for Byte<N> {
    fn matches(hay: &mut Haystack<u8>) -> bool {
        if hay.item() == Some(N) {
            hay.progress();
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
pub struct ByteRange<const A: u8, const B: u8>;

impl<const A: u8, const B: u8> Matcher<u8> for ByteRange<A, B> {
    fn matches(hay: &mut Haystack<u8>) -> bool {
        if let Some(byte) = hay.item() && A <= byte && byte <= B {
            hay.progress();
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
pub struct Scalar<const N: char>;

impl<const N: char> Matcher<char> for Scalar<N> {
    fn matches(hay: &mut Haystack<char>) -> bool {
        if hay.item() == Some(N) {
            hay.progress();
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
pub struct ScalarRange<const A: char, const B: char>;

impl<const A: char, const B: char> Matcher<char> for ScalarRange<A, B> {
    fn matches(hay: &mut Haystack<char>) -> bool {
        if let Some(scalar) = hay.item() && A <= scalar && scalar <= B {
            hay.progress();
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
pub struct Always;

impl<I: HaystackItem> Matcher<I> for Always {
    fn matches(_: &mut Haystack<I>) -> bool {
        true
    }
}

#[derive(Debug, Default)]
pub struct Beginning;

impl<I: HaystackItem> Matcher<I> for Beginning {
    fn matches(hay: &mut Haystack<I>) -> bool {
        hay.is_start()
    }
}

#[derive(Debug, Default)]
pub struct End;

impl<I: HaystackItem> Matcher<I> for End {
    fn matches(hay: &mut Haystack<I>) -> bool {
        hay.is_end()
    }
}