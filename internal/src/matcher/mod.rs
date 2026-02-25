#[cfg(test)]
mod test;

use std::marker::PhantomData;

use crate::haystack::{Haystack, HaystackItem};

pub trait Matcher<I: HaystackItem> {
    fn matches(hay: &mut Haystack<I>) -> bool;
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
pub struct Or<I: HaystackItem, A: Matcher<I>, B: Matcher<I>>(
    pub PhantomData<I>,
    pub PhantomData<A>,
    pub PhantomData<B>,
);

impl<I: HaystackItem, A: Matcher<I>, B: Matcher<I>> Matcher<I> for Or<I, A, B> {
    fn matches(hay: &mut Haystack<I>) -> bool {
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
pub struct Then<I: HaystackItem, A: Matcher<I>, B: Matcher<I>>(
    pub PhantomData<I>,
    pub PhantomData<A>,
    pub PhantomData<B>,
);

impl<I: HaystackItem, A: Matcher<I>, B: Matcher<I>> Matcher<I> for Then<I, A, B> {
    fn matches(hay: &mut Haystack<I>) -> bool {
        if A::matches(hay) {
            B::matches(hay)
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
pub struct QuantifierN<I: HaystackItem, A: Matcher<I>, const N: usize>(
    pub PhantomData<I>,
    pub PhantomData<A>,
);

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Matcher<I> for QuantifierN<I, A, N> {
    fn matches(hay: &mut Haystack<I>) -> bool {
        let mut matches = 0;
        while A::matches(hay) {
            matches += 1;
        }
        matches == N
    }
}

#[derive(Debug, Default)]
pub struct QuantifierNOrMore<I: HaystackItem, A: Matcher<I>, const N: usize>(
    pub PhantomData<I>,
    pub PhantomData<A>,
);

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Matcher<I> for QuantifierNOrMore<I, A, N> {
    fn matches(hay: &mut Haystack<I>) -> bool {
        let mut matches = 0;
        while A::matches(hay) {
            matches += 1;
        }
        matches >= N
    }
}

#[derive(Debug, Default)]
pub struct QuantifierNToM<I: HaystackItem, A: Matcher<I>, const N: usize, const M: usize>(
    pub PhantomData<I>,
    pub PhantomData<A>,
);

impl<I: HaystackItem, A: Matcher<I>, const N: usize, const M: usize> Matcher<I> for QuantifierNToM<I, A, N, M> {
    fn matches(hay: &mut Haystack<I>) -> bool {
        let mut matches = 0;
        while A::matches(hay) {
            matches += 1;
        }
        N <= matches && matches <= M
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

#[derive(Debug, Default)]
pub struct Always;

impl<I: HaystackItem> Matcher<I> for Always {
    fn matches(_: &mut Haystack<I>) -> bool {
        true
    }
}

/// Macro to generate chunked Or types (Or4, Or8, Or16, etc.)
///
/// Each generated type takes N matchers and combines them pairwise using Or,
/// then delegates to the combiner type (which has N/2 parameters).
///
/// Usage: `define_or_n!(Or4, Or, [A B] [C D]);`
/// - First arg: name of the new type
/// - Second arg: the combiner type (Or for Or4, Or4 for Or8, etc.)
/// - Remaining args: pairs of type parameter names in brackets
macro_rules! define_paired_n {
    ($pair:ident, $name:ident, $combiner:ident, $([$a:ident $b:ident])+) => {
        #[derive(Debug, Default)]
        pub struct $name<
            Z: HaystackItem,
            $($a: Matcher<Z>, $b: Matcher<Z>),+
        >(
            pub PhantomData<Z>,
            $(pub PhantomData<$a>, pub PhantomData<$b>),+
        );

        impl<
            Z: HaystackItem,
            $($a: Matcher<Z>, $b: Matcher<Z>),+
        > Matcher<Z> for $name<Z, $($a, $b),+> {
            fn matches(hay: &mut Haystack<Z>) -> bool {
                $combiner::<Z, $($pair<Z, $a, $b>),+>::matches(hay)
            }
        }
    };
}

define_paired_n!(Or, Or4, Or, [A B] [C D]);
define_paired_n!(Or, Or8, Or4, [A B] [C D] [E F] [G H]);
define_paired_n!(Or, Or16, Or8, [A B] [C D] [E F] [G H] [I J] [K L] [M N] [O P]);

define_paired_n!(Then, Then4, Then, [A B] [C D]);
define_paired_n!(Then, Then8, Then4, [A B] [C D] [E F] [G H]);
define_paired_n!(Then, Then16, Then8, [A B] [C D] [E F] [G H] [I J] [K L] [M N] [O P]);