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
        while A::matches(hay) {
            count += 1;

            if count == M && count >= N {
                return true;
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
        while A::captures(hay, caps) {
            count += 1;

            if count == M && count >= N {
                return true;
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

#[derive(Default)]
pub struct QuantifierThen<I: HaystackItem, Q: Matcher<I>, T: Matcher<I>>(
    pub PhantomData<I>,
    pub PhantomData<Q>,
    pub PhantomData<T>,
);

impl<I: HaystackItem, Q: Matcher<I>, T: Matcher<I>> Matcher<I> for QuantifierThen<I, Q, T> {
    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        let mut rollback = hay.clone();
        if Then::<I, Q, T>::matches(hay) {
            true
        } else {
            // Try all valid match points for Q in reverse order (greedy).
            let match_points = Q::all_matches(&mut rollback);

            for point in match_points.into_iter().rev() {
                // Overwrite the provided haystack with the progressed version.
                hay.rollback(point);
                if T::matches(hay) {
                    return true;
                }
            }
            false
        }
    }

    fn all_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> Vec<usize> {
        Then::<I, Q, T>::all_matches(hay)
    }

    fn captures<'a, H: HaystackOf<'a, I>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        let mut rollback = (hay.clone(), caps.clone());
        if Then::<I, Q, T>::captures(hay, caps) {
            true
        } else {
            // Try all valid match points for Q in reverse order (greedy).
            let match_points = Q::all_captures(&mut rollback.0, &mut rollback.1);

            for (point_state, mut point_caps) in match_points.into_iter().rev() {
                // Overwrite the provided haystack with the progressed version.
                hay.rollback(point_state);
                if T::captures(hay, &mut point_caps) {
                    // Overwrite captures with the progressed version.
                    *caps = point_caps;
                    return true;
                }
            }
            false
        }
    }

    fn all_captures<'a, H: HaystackOf<'a, I>>(
        hay: &mut H,
        caps: &mut IndexedCaptures
    ) -> Vec<(usize, IndexedCaptures)> {
        Then::<I, Q, T>::all_captures(hay, caps)
    }
}

impl<I: HaystackItem, Q: Matcher<I>, T: Matcher<I>> Debug for QuantifierThen<I, Q, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", Then::<I, Q, T>::default())
    }
}