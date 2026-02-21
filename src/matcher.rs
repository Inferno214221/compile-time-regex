use std::marker::PhantomData;

use crate::haystack::Haystack;

pub trait Matcher {
    fn matches(hay: &mut Haystack) -> bool;
}

#[derive(Debug, Default)]
pub struct Byte<const N: u8>;

impl<const N: u8> Matcher for Byte<N> {
    fn matches(hay: &mut Haystack) -> bool {
        if hay.byte() == Some(N) {
            hay.progress();
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
pub struct ByteRange<const A: u8, const B: u8>;

impl<const A: u8, const B: u8> Matcher for ByteRange<A, B> {
    fn matches(hay: &mut Haystack) -> bool {
        if let Some(byte) = hay.byte() && A <= byte && byte <= B {
            hay.progress();
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
pub struct Or<A: Matcher, B: Matcher>(pub PhantomData<A>, pub PhantomData<B>);

impl<A: Matcher, B: Matcher> Matcher for Or<A, B> {
    fn matches(hay: &mut Haystack) -> bool {
        let rollback = hay.clone();
        if A::matches(hay) {
            true
        } else {
            *hay = rollback;
            B::matches(hay)
        }
    }
}

#[derive(Debug, Default)]
pub struct Then<A: Matcher, B: Matcher>(pub PhantomData<A>, pub PhantomData<B>);

impl<A: Matcher, B: Matcher> Matcher for Then<A, B> {
    fn matches(hay: &mut Haystack) -> bool {
        if A::matches(hay) {
            B::matches(hay)
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
pub struct QuantifierN<A: Matcher, const N: usize>(pub PhantomData<A>);

impl<A: Matcher, const N: usize> Matcher for QuantifierN<A, N> {
    fn matches(hay: &mut Haystack) -> bool {
        let mut matches = 0;
        while A::matches(hay) {
            matches += 1;
        }
        matches == N
    }
}

#[derive(Debug, Default)]
pub struct QuantifierNOrMore<A: Matcher, const N: usize>(pub PhantomData<A>);

impl<A: Matcher, const N: usize> Matcher for QuantifierNOrMore<A, N> {
    fn matches(hay: &mut Haystack) -> bool {
        let mut matches = 0;
        while A::matches(hay) {
            matches += 1;
        }
        matches >= N
    }
}

#[derive(Debug, Default)]
pub struct QuantifierNToM<A: Matcher, const N: usize, const M: usize>(pub PhantomData<A>);

impl<A: Matcher, const N: usize, const M: usize> Matcher for QuantifierNToM<A, N, M> {
    fn matches(hay: &mut Haystack) -> bool {
        let mut matches = 0;
        while A::matches(hay) {
            matches += 1;
        }
        N <= matches && matches <= M
    }
}

#[derive(Debug, Default)]
pub struct Beginning;

impl Matcher for Beginning {
    fn matches(hay: &mut Haystack) -> bool {
        hay.is_start()
    }
}

#[derive(Debug, Default)]
pub struct End;

impl Matcher for End {
    fn matches(hay: &mut Haystack) -> bool {
        hay.is_end()
    }
}

#[derive(Debug, Default)]
pub struct Always;

impl Matcher for Always {
    fn matches(_: &mut Haystack) -> bool {
        true
    }
}