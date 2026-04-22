use std::{fmt::{self, Debug}, marker::PhantomData, vec};

use crate::{expr::IndexedCaptures, haystack::{HaystackItem, HaystackOf}, matcher::{Matcher, Then}};

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

    fn captures<'a, H: HaystackOf<'a, I>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        let mut count = 0;
        while A::captures(hay, caps) {
            count += 1;
        }
        count == N
    }
}

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Debug for QuantifierN<I, A, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{{{N}}}", A::default())
    }
}

#[derive(Default)]
pub struct QuantifierNOrMore<I: HaystackItem, A: Matcher<I>, const N: usize>(
    pub PhantomData<I>,
    pub PhantomData<A>,
);

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Matcher<I> for QuantifierNOrMore<I, A, N> {
    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        let mut count = 0;
        while A::matches(hay) {
            count += 1;
        }
        count >= N
    }

    fn all_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> Vec<usize> {
        let mut matches = vec![];
        let mut count = 0;

        // Include zero-match position when N=0
        if N == 0 {
            matches.push(hay.index());
        }

        while A::matches(hay) {
            count += 1;
            if count >= N {
                matches.push(hay.index());
            }
        }
        matches
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
    ) -> Vec<(usize, IndexedCaptures)> {
        let mut captures = vec![];
        let mut count = 0;

        // Include zero-match position when N=0
        if N == 0 {
            captures.push((hay.index(), caps.clone()));
        }

        while A::captures(hay, caps) {
            count += 1;
            if count >= N {
                captures.push((hay.index(), caps.clone()));
            }
        }
        captures
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

    fn all_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> Vec<usize> {
        let mut matches = vec![];
        let mut count = 0;

        // Include zero-match position when N=0
        if N == 0 {
            matches.push(hay.index());
        }

        while A::matches(hay) {
            count += 1;
            if N <= count && count <= M {
                matches.push(hay.index());

                if count == M {
                    return matches;
                }
            }
        }
        matches
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
    ) -> Vec<(usize, IndexedCaptures)> {
        let mut captures = vec![];
        let mut count = 0;

        // Include zero-match position when N=0
        if N == 0 {
            captures.push((hay.index(), caps.clone()));
        }

        while A::captures(hay, caps) {
            count += 1;
            if N <= count && count <= M {
                captures.push((hay.index(), caps.clone()));

                if count == M {
                    return captures;
                }
            }
        }
        captures
    }
}

impl<I: HaystackItem, A: Matcher<I>, const N: usize, const M: usize> Debug for QuantifierNToM<I, A, N, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{{{N},{M}}}", A::default())
    }
}