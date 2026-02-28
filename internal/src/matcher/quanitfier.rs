use std::marker::PhantomData;

use crate::{general::IndexedCaptures, haystack::{Haystack, HaystackItem}, matcher::{Matcher, Then}};

#[derive(Debug, Default)]
pub struct QuantifierN<I: HaystackItem, A: Matcher<I>, const N: usize>(
    pub PhantomData<I>,
    pub PhantomData<A>,
);

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Matcher<I> for QuantifierN<I, A, N> {
    fn matches(hay: &mut Haystack<I>) -> bool {
        let mut count = 0;
        while A::matches(hay) {
            count += 1;
        }
        count == N
    }
    
    fn capture(hay: &mut Haystack<I>, caps: &mut IndexedCaptures) -> bool {
        let mut count = 0;
        while A::capture(hay, caps) {
            count += 1;
        }
        count == N
    }
}

#[derive(Debug, Default)]
pub struct QuantifierNOrMore<I: HaystackItem, A: Matcher<I>, const N: usize>(
    pub PhantomData<I>,
    pub PhantomData<A>,
);

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Matcher<I> for QuantifierNOrMore<I, A, N> {
    fn matches(hay: &mut Haystack<I>) -> bool {
        let mut count = 0;
        while A::matches(hay) {
            count += 1;
        }
        count >= N
    }
    
    fn all_matches<'a>(hay: &mut Haystack<'a, I>) -> Vec<Haystack<'a, I>> {
        let mut matches = vec![];
        let mut count = 0;

        // Include zero-match position when N=0
        if N == 0 {
            matches.push(hay.clone());
        }

        while A::matches(hay) {
            count += 1;
            if count >= N {
                matches.push(hay.clone());
            }
        }
        matches
    }

    fn capture(hay: &mut Haystack<I>, caps: &mut IndexedCaptures) -> bool {
        let mut count = 0;
        while A::capture(hay, caps) {
            count += 1;
        }
        count >= N
    }

    // TODO: Has implications for all_captures
}

#[derive(Debug, Default)]
pub struct QuantifierNToM<I: HaystackItem, A: Matcher<I>, const N: usize, const M: usize>(
    pub PhantomData<I>,
    pub PhantomData<A>,
);

impl<I: HaystackItem, A: Matcher<I>, const N: usize, const M: usize> Matcher<I> for QuantifierNToM<I, A, N, M> {
    fn matches(hay: &mut Haystack<I>) -> bool {
        let mut count = 0;
        while A::matches(hay) {
            count += 1;

            if count == M && count >= N {
                return true;
            }
        }
        N <= count && count <= M
    }
    
    fn all_matches<'a>(hay: &mut Haystack<'a, I>) -> Vec<Haystack<'a, I>> {
        let mut matches = Vec::new();
        let mut count = 0;
        
        // Include zero-match position when N=0
        if N == 0 {
            matches.push(hay.clone());
        }
        
        while A::matches(hay) {
            count += 1;
            if N <= count && count <= M {
                matches.push(hay.clone());
                
                if count == M {
                    return matches;
                }
            }
        }
        matches
    }

    fn capture(hay: &mut Haystack<I>, caps: &mut IndexedCaptures) -> bool {
        let mut count = 0;
        while A::capture(hay, caps) {
            count += 1;

            if count == M && count >= N {
                return true;
            }
        }
        N <= count && count <= M
    }

    // TODO: Has implications for all_captures
}

#[derive(Debug, Default)]
pub struct QuantifierThen<I: HaystackItem, Q: Matcher<I>, T: Matcher<I>>(
    pub PhantomData<I>,
    pub PhantomData<Q>,
    pub PhantomData<T>,
);

impl<I: HaystackItem, Q: Matcher<I>, T: Matcher<I>> Matcher<I> for QuantifierThen<I, Q, T> {
    fn matches(hay: &mut Haystack<I>) -> bool {
        let mut rollback = hay.clone();
        if Then::<I, Q, T>::matches(hay) {
            true
        } else {
            // Try all valid match points for Q in reverse order (greedy).
            let match_points = Q::all_matches(&mut rollback);

            for mut point in match_points.into_iter().rev() {
                if T::matches(&mut point) {
                    // Overwrite the provided haystack with the progressed version.
                    *hay = point;
                    return true;
                }
            }
            false
        }
    }

    fn all_matches<'a>(hay: &mut Haystack<'a, I>) -> Vec<Haystack<'a, I>> {
        Then::<I, Q, T>::all_matches(hay)
    }

    fn capture(_hay: &mut Haystack<I>, _caps: &mut IndexedCaptures) -> bool {
        todo!("Needs a Q::all_captures");
        // let mut rollback = (hay.clone(), caps.clone());
        // if Then::<I, Q, T>::matches(hay) {
        //     true
        // } else {
        //     // Try all valid match points for Q in reverse order (greedy).
        //     let match_points = Q::all_matches(&mut rollback);

        //     for mut point in match_points.into_iter().rev() {
        //         if T::matches(&mut point) {
        //             // Overwrite the provided haystack with the progressed version.
        //             *hay = point;
        //             return true;
        //         }
        //     }
        //     false
        // }
    }
}