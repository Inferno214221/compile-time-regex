use std::{fmt::{self, Debug}, marker::PhantomData};

use crate::{expr::IndexedCaptures, haystack::{Haystack, HaystackItem, HaystackWith}, matcher::Matcher};

#[derive(Default)]
pub struct Or<I: HaystackItem, A: Matcher<I>, B: Matcher<I>>(
    pub PhantomData<I>,
    pub PhantomData<A>,
    pub PhantomData<B>,
);

impl<I: HaystackItem, A: Matcher<I>, B: Matcher<I>> Matcher<I> for Or<I, A, B> {
    fn matches<'a, H: HaystackWith<'a, I>>(hay: &mut H) -> bool {
        let rollback = hay.clone();
        if A::matches(hay) {
            true
        } else {
            *hay = rollback;
            B::matches(hay)
        }
    }

    // /(a*|b*)c/ should prefer aa, a, bb, b -> vec![b, bb, a, aa]
    fn all_matches<'a, H: HaystackWith<'a, I>>(hay: &mut H) -> Vec<H> {
        let mut fork = hay.clone();
        // We match B first because the output needs to be reversed for greedy matching.
        // TODO: Consider implications for lazy matching.
        let mut vec = B::all_matches(hay);
        vec.append(&mut A::all_matches(&mut fork));
        vec
    }

    fn captures<'a, H: HaystackWith<'a, I>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        let rollback = (hay.clone(), caps.clone());
        if A::captures(hay, caps) {
            true
        } else {
            (*hay, *caps) = rollback;
            B::captures(hay, caps)
        }
    }

    fn all_captures<'a, H: HaystackWith<'a, I>>(
        hay: &mut H,
        caps: &mut IndexedCaptures
    ) -> Vec<(H, IndexedCaptures)> {
        let (mut hay_fork, mut caps_fork) = (hay.clone(), caps.clone());
        // We match B first because the output needs to be reversed for greedy matching.
        let mut vec = B::all_captures(hay, caps);
        vec.append(&mut A::all_captures(&mut hay_fork, &mut caps_fork));
        vec
    }
}

impl<I: HaystackItem, A: Matcher<I>, B: Matcher<I>> Debug for Or<I, A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}|{:?}", A::default(), B::default())
    }
}

#[derive(Default)]
pub struct Then<I: HaystackItem, A: Matcher<I>, B: Matcher<I>>(
    pub PhantomData<I>,
    pub PhantomData<A>,
    pub PhantomData<B>,
);

impl<I: HaystackItem, A: Matcher<I>, B: Matcher<I>> Matcher<I> for Then<I, A, B> {
    fn matches<'a, H: HaystackWith<'a, I>>(hay: &mut H) -> bool {
        if let Some(fork) = Self::all_matches(hay).pop() {
            *hay = fork;
            true
        } else {
            false
        }
    }

    fn all_matches<'a, H: HaystackWith<'a, I>>(hay: &mut H) -> Vec<H> {
        A::all_matches(hay).into_iter().flat_map(|mut h| {
            B::all_matches(&mut h)
        }).collect()
    }

    fn captures<'a, H: HaystackWith<'a, I>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        if let Some(fork) = Self::all_captures(hay, caps).pop() {
            (*hay, *caps) = fork;
            true
        } else {
            false
        }
    }

    fn all_captures<'a, H: HaystackWith<'a, I>>(
        hay: &mut H,
        caps: &mut IndexedCaptures
    ) -> Vec<(H, IndexedCaptures)> {
        A::all_captures(hay, caps).into_iter().flat_map(|(mut hay_fork, mut caps_fork)| {
            B::all_captures(&mut hay_fork, &mut caps_fork)
        }).collect()
    }
}

impl<I: HaystackItem, A: Matcher<I>, B: Matcher<I>> Debug for Then<I, A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{:?}", A::default(), B::default())
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
        #[derive(Default)]
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
            fn matches<'a, Y: HaystackWith<'a, Z>>(hay: &mut Y) -> bool {
                $combiner::<Z, $($pair<Z, $a, $b>),+>::matches(hay)
            }

            fn all_matches<'a, Y: HaystackWith<'a, Z>>(hay: &mut Y) -> Vec<Y> {
                $combiner::<Z, $($pair<Z, $a, $b>),+>::all_matches(hay)
            }

            fn captures<'a, Y: HaystackWith<'a, Z>>(hay: &mut Y, caps: &mut IndexedCaptures) -> bool {
                $combiner::<Z, $($pair<Z, $a, $b>),+>::captures(hay, caps)
            }

            fn all_captures<'a, Y: HaystackWith<'a, Z>>(
                hay: &mut Y,
                caps: &mut IndexedCaptures
            ) -> Vec<(Y, IndexedCaptures)> {
                $combiner::<Z, $($pair<Z, $a, $b>),+>::all_captures(hay, caps)
            }
        }

        impl<Z: HaystackItem, $($a: Matcher<Z>, $b: Matcher<Z>),+> Debug for $name<Z, $($a, $b),+> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", $combiner::<Z, $($pair<Z, $a, $b>),+>::default())
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