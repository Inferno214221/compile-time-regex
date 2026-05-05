use std::{fmt::{self, Debug}, iter::Chain, marker::PhantomData};

use crate::{expr::IndexedCaptures, haystack::{HaystackItem, HaystackOf}, matcher::Matcher};

#[derive(Default)]
pub struct Or<I: HaystackItem, A: Matcher<I>, B: Matcher<I>>(
    pub PhantomData<I>,
    pub PhantomData<A>,
    pub PhantomData<B>,
);

pub type AllMatchesOr<'a, I, H, A, B> = Chain<
    <A as Matcher<I>>::AllMatches<'a, H>,
    <B as Matcher<I>>::AllMatches<'a, H>
>;
pub type AllCapturesOr<'a, I, H, A, B> = Chain<
    <A as Matcher<I>>::AllCaptures<'a, H>,
    <B as Matcher<I>>::AllCaptures<'a, H>
>;

impl<I: HaystackItem, A: Matcher<I>, B: Matcher<I>> Matcher<I> for Or<I, A, B> {
    type AllMatches<'a, H: HaystackOf<'a, I>> = AllMatchesOr<'a, I, H, A, B>;
    type AllCaptures<'a, H: HaystackOf<'a, I>> = AllCapturesOr<'a, I, H, A, B>;

    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        let start = hay.index();

        if A::matches(hay) {
            true
        } else {
            hay.rollback(start);
            B::matches(hay)
        }
    }

    // /(a*|b*)c/ should prefer aa, a, bb, b -> vec![b, bb, a, aa]
    fn all_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> Self::AllMatches<'a, H> {
        let state_fork = hay.index();
        // There is no reversing anymore, yield elements in order of greediest to least greedy.
        let a_matches = A::all_matches(hay);
        hay.rollback(state_fork);
        a_matches.chain(B::all_matches(hay))
    }

    fn captures<'a, H: HaystackOf<'a, I>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        let (initial_state, initial_caps) = (hay.index(), caps.clone());
        if A::captures(hay, caps) {
            true
        } else {
            hay.rollback(initial_state);
            *caps = initial_caps;
            B::captures(hay, caps)
        }
    }

    fn all_captures<'a, H: HaystackOf<'a, I>>(
        hay: &mut H,
        caps: &mut IndexedCaptures
    ) -> Self::AllCaptures<'a, H> {
        let (state_fork, mut caps_fork) = (hay.index(), caps.clone());

        let a_captures = A::all_captures(hay, caps);
        hay.rollback(state_fork);
        a_captures.chain(B::all_captures(hay, &mut caps_fork))
    }
}

impl<I: HaystackItem, A: Matcher<I>, B: Matcher<I>> Debug for Or<I, A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}|{:?}", A::default(), B::default())
    }
}

pub struct AllMatchesThen<'a, I: HaystackItem, H: HaystackOf<'a, I>, A: Matcher<I>, B: Matcher<I>> {
    a_matches: A::AllMatches<'a, H>,
    b_matches: Option<B::AllMatches<'a, H>>,
    hay: H,
}

impl<'a, I, H, A, B> Iterator for AllMatchesThen<'a, I, H, A, B>
where
    I: HaystackItem,
    H: HaystackOf<'a, I>,
    A: Matcher<I>,
    B: Matcher<I>
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.b_matches.as_mut().and_then(Iterator::next) {
            Some(b) => Some(b),
            None => {
                self.hay.rollback(self.a_matches.next()?);
                self.b_matches = Some(B::all_matches(&mut self.hay));
                self.next()
            },
        }
    }
}

pub struct AllCapturesThen<'a, I, H, A, B>
where
    I: HaystackItem,
    H: HaystackOf<'a, I>,
    A: Matcher<I>,
    B: Matcher<I>
{
    a_captures: A::AllCaptures<'a, H>,
    b_captures: Option<B::AllCaptures<'a, H>>,
    hay: H,
}

impl<'a, I, H, A, B> Iterator for AllCapturesThen<'a, I, H, A, B>
where
    I: HaystackItem,
    H: HaystackOf<'a, I>,
    A: Matcher<I>,
    B: Matcher<I>
{
    type Item = (usize, IndexedCaptures);

    fn next(&mut self) -> Option<Self::Item> {
        match self.b_captures.as_mut().and_then(Iterator::next) {
            Some(b) => Some(b),
            None => {
                let (state_fork, mut caps_fork) = self.a_captures.next()?;
                self.hay.rollback(state_fork);
                self.b_captures = Some(B::all_captures(&mut self.hay, &mut caps_fork));
                self.next()
            },
        }
    }
}

#[derive(Default)]
pub struct Then<I: HaystackItem, A: Matcher<I>, B: Matcher<I>>(
    pub PhantomData<I>,
    pub PhantomData<A>,
    pub PhantomData<B>,
);

impl<I: HaystackItem, A: Matcher<I>, B: Matcher<I>> Matcher<I> for Then<I, A, B> {
    type AllMatches<'a, H: HaystackOf<'a, I>> = AllMatchesThen<'a, I, H, A, B>;
    type AllCaptures<'a, H: HaystackOf<'a, I>> = AllCapturesThen<'a, I, H, A, B>;

    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        if let Some(state_fork) = Self::all_matches(hay).next() {
            hay.rollback(state_fork);
            true
        } else {
            false
        }
    }

    fn all_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> Self::AllMatches<'a, H> {
        AllMatchesThen {
            a_matches: A::all_matches(hay),
            b_matches: None,
            // The state of hay is unspecified because we're forking. Therefore, we just clone hay
            // to remove the need for (very) complicated lifetime bounds.
            hay: hay.clone()
        }
    }

    fn captures<'a, H: HaystackOf<'a, I>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        if let Some((state_fork, caps_fork)) = Self::all_captures(hay, caps).next() {
            hay.rollback(state_fork);
            *caps = caps_fork;
            true
        } else {
            false
        }
    }

    fn all_captures<'a, H: HaystackOf<'a, I>>(
        hay: &mut H,
        caps: &mut IndexedCaptures
    ) -> Self::AllCaptures<'a, H> {
        AllCapturesThen {
            a_captures: A::all_captures(hay, caps),
            b_captures: None,
            hay: hay.clone(),
        }
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
    ($pair:ident, $all_matches:ident, $name:ident, $combiner:ident, $([$a:ident $b:ident])+) => {
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
            type AllMatches<'a, Y: HaystackOf<'a, Z>> = <
                $combiner::<Z, $($pair<Z, $a, $b>),+> as Matcher<Z>
            >::AllMatches<'a, Y>;

            type AllCaptures<'a, Y: HaystackOf<'a, Z>> = <
                $combiner::<Z, $($pair<Z, $a, $b>),+> as Matcher<Z>
            >::AllCaptures<'a, Y>;

            fn matches<'a, Y: HaystackOf<'a, Z>>(hay: &mut Y) -> bool {
                $combiner::<Z, $($pair<Z, $a, $b>),+>::matches(hay)
            }

            fn all_matches<'a, Y: HaystackOf<'a, Z>>(hay: &mut Y) -> Self::AllMatches<'a, Y> {
                $combiner::<Z, $($pair<Z, $a, $b>),+>::all_matches(hay)
            }

            fn captures<'a, Y: HaystackOf<'a, Z>>(hay: &mut Y, caps: &mut IndexedCaptures) -> bool {
                $combiner::<Z, $($pair<Z, $a, $b>),+>::captures(hay, caps)
            }

            fn all_captures<'a, Y: HaystackOf<'a, Z>>(
                hay: &mut Y,
                caps: &mut IndexedCaptures
            ) -> Self::AllCaptures<'a, Y> {
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

define_paired_n!(Or, AllMatchesOr, Or4, Or, [A B] [C D]);
define_paired_n!(Or, AllMatchesOr, Or8, Or4, [A B] [C D] [E F] [G H]);
define_paired_n!(Or, AllMatchesOr, Or16, Or8, [A B] [C D] [E F] [G H] [I J] [K L] [M N] [O P]);

define_paired_n!(Then, AllMatchesThen, Then4, Then, [A B] [C D]);
define_paired_n!(Then, AllMatchesThen, Then8, Then4, [A B] [C D] [E F] [G H]);
define_paired_n!(Then, AllMatchesThen, Then16, Then8, [A B] [C D] [E F] [G H] [I J] [K L] [M N] [O P]);