use std::{fmt::{self, Debug}, iter, marker::PhantomData, vec};

use crate::{expr::IndexedCaptures, haystack::{HaystackItem, HaystackOf}, matcher::{LazyMatcher, Matcher, impl_all_captures_single, impl_all_matches_single}};

#[derive(Default)]
pub struct QuantifierN<I: HaystackItem, A: Matcher<I>, const N: usize>(
    pub PhantomData<I>,
    pub PhantomData<A>,
);

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Matcher<I> for QuantifierN<I, A, N> {
    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        let mut count = 0;
        while A::matches(hay) {
            count += 1;
        }
        count == N
    }

    impl_all_matches_single!(I);

    fn captures<'a, H: HaystackOf<'a, I>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        let mut count = 0;
        while A::captures(hay, caps) {
            count += 1;
        }
        count == N
    }

    impl_all_captures_single!(I);
}

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Debug for QuantifierN<I, A, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{{{N}}}", A::default())
    }
}

pub type AllMatchesMultiple = iter::Rev<vec::IntoIter<usize>>;
pub type AllCapturesMultiple = iter::Rev<vec::IntoIter<(usize, IndexedCaptures)>>;

#[derive(Default)]
pub struct QuantifierNOrMore<I: HaystackItem, A: Matcher<I>, const N: usize>(
    pub PhantomData<I>,
    pub PhantomData<A>,
);

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Matcher<I> for QuantifierNOrMore<I, A, N> {
    type AllMatches<'a, H: HaystackOf<'a, I>> = AllMatchesMultiple;
    type AllCaptures<'a, H: HaystackOf<'a, I>> = AllCapturesMultiple;

    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        let mut count = 0;
        while A::matches(hay) {
            count += 1;
        }
        count >= N
    }

    // This **has to be** evaluated eagerly, so we use a vec.
    fn all_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> Self::AllMatches<'a, H> {
        Self::lazy_all_matches(hay)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
    }

    fn captures<'a, H: HaystackOf<'a, I>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        let mut count = 0;
        while A::captures(hay, caps) {
            count += 1;
        }
        count >= N
    }

    fn all_captures<'a, H: HaystackOf<'a, I>>(
        hay: &mut H,
        caps: &mut IndexedCaptures
    ) -> Self::AllCaptures<'a, H> {
        Self::lazy_all_captures(hay, caps)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
    }
}

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Debug for QuantifierNOrMore<I, A, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{{{N},}}", A::default())
    }
}

#[derive(Default)]
pub struct QuantifierNToM<I: HaystackItem, A: Matcher<I>, const N: usize, const M: usize>(
    pub PhantomData<I>,
    pub PhantomData<A>,
);

impl<I: HaystackItem, A: Matcher<I>, const N: usize, const M: usize> Matcher<I> for QuantifierNToM<I, A, N, M> {
    type AllMatches<'a, H: HaystackOf<'a, I>> = AllMatchesMultiple;
    type AllCaptures<'a, H: HaystackOf<'a, I>> = AllCapturesMultiple;

    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        let mut count = 0;
        loop {
            if count >= N && count == M {
                return true;
            }
            if A::matches(hay) {
                count += 1;
            } else {
                break;
            }
        }
        N <= count && count <= M
    }

    fn all_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> Self::AllMatches<'a, H> {
        Self::lazy_all_matches(hay)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
    }

    fn captures<'a, H: HaystackOf<'a, I>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        let mut count = 0;
        loop {
            if count >= N && count == M {
                return true;
            }
            if A::captures(hay, caps) {
                count += 1;
            } else {
                break;
            }
        }
        N <= count && count <= M
    }

    fn all_captures<'a, H: HaystackOf<'a, I>>(
        hay: &mut H,
        caps: &mut IndexedCaptures
    ) -> Self::AllCaptures<'a, H> {
        Self::lazy_all_captures(hay, caps)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
    }
}

impl<I: HaystackItem, A: Matcher<I>, const N: usize, const M: usize> Debug for QuantifierNToM<I, A, N, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{{{N},{M}}}", A::default())
    }
}