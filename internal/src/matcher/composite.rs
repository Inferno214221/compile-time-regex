use std::marker::PhantomData;

use crate::{general::IndexedCaptures, haystack::{Haystack, HaystackItem}, matcher::Matcher};

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

    // /(a*|b*)c/ should prefer aa, a, bb, b -> vec![b, bb, a, aa]
    fn all_matches<'a>(hay: &mut Haystack<'a, I>) -> Vec<Haystack<'a, I>> {
        let mut fork = hay.clone();
        // We match B first because the output needs to be reversed for greedy matching.
        // TODO: Consider implications for lazy matching.
        let mut vec = B::all_matches(hay);
        vec.append(&mut A::all_matches(&mut fork));
        vec
    }

    fn captures(hay: &mut Haystack<I>, caps: &mut IndexedCaptures) -> bool {
        let rollback = (hay.clone(), caps.clone());
        if A::captures(hay, caps) {
            true
        } else {
            (*hay, *caps) = rollback;
            B::captures(hay, caps)
        }
    }

    fn all_captures<'a>(
        hay: &mut Haystack<'a, I>,
        caps: &mut IndexedCaptures
    ) -> Vec<(Haystack<'a, I>, IndexedCaptures)> {
        let mut fork = (hay.clone(), caps.clone());
        // We match B first because the output needs to be reversed for greedy matching.
        let mut vec = B::all_captures(hay, caps);
        vec.append(&mut A::all_captures(&mut fork.0, &mut fork.1));
        vec
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

    fn all_matches<'a>(hay: &mut Haystack<'a, I>) -> Vec<Haystack<'a, I>> {
        A::all_matches(hay).into_iter().flat_map(|mut m| {
            B::all_matches(&mut m)
        }).collect()
    }

    fn captures(hay: &mut Haystack<I>, caps: &mut IndexedCaptures) -> bool {
        if A::captures(hay, caps) {
            B::captures(hay, caps)
        } else {
            false
        }
    }

    fn all_captures<'a>(
        hay: &mut Haystack<'a, I>,
        caps: &mut IndexedCaptures
    ) -> Vec<(Haystack<'a, I>, IndexedCaptures)> {
        A::all_captures(hay, caps).into_iter().flat_map(|mut m| {
            B::all_captures(&mut m.0, &mut m.1)
        }).collect()
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

            fn all_matches<'a>(hay: &mut Haystack<'a, Z>) -> Vec<Haystack<'a, Z>> {
                $combiner::<Z, $($pair<Z, $a, $b>),+>::all_matches(hay)
            }

            fn captures(hay: &mut Haystack<Z>, caps: &mut IndexedCaptures) -> bool {
                $combiner::<Z, $($pair<Z, $a, $b>),+>::captures(hay, caps)
            }

            fn all_captures<'a>(
                hay: &mut Haystack<'a, Z>,
                caps: &mut IndexedCaptures
            ) -> Vec<(Haystack<'a, Z>, IndexedCaptures)> {
                $combiner::<Z, $($pair<Z, $a, $b>),+>::all_captures(hay, caps)
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