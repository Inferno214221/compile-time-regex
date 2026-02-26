use std::marker::PhantomData;

use crate::{haystack::{Haystack, HaystackItem}, matcher::{Matcher, Then}};

#[derive(Debug, Default)]
pub struct QuantifierN<I: HaystackItem, A: Matcher<I>, const N: usize>(
    pub PhantomData<I>,
    pub PhantomData<A>,
);

// TODO: Is this ever used?
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
    
    fn all_matches<'a>(hay: &mut Haystack<'a, I>) -> Vec<Haystack<'a, I>> {
        let mut vec = Vec::new();
        let mut matches = 0;
        while A::matches(hay) {
            matches += 1;
            if matches >= N {
                vec.push(hay.clone());
            }
        }
        vec
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

            if matches == M && matches >= N {
                return true;
            }
        }
        N <= matches && matches <= M
    }
    
    fn all_matches<'a>(hay: &mut Haystack<'a, I>) -> Vec<Haystack<'a, I>> {
        let mut vec = Vec::new();
        let mut matches = 0;
        while A::matches(hay) {
            matches += 1;
            if N <= matches && matches <= M {
                vec.push(hay.clone());
                
                if matches == M {
                    return vec;
                }
            }
        }
        vec
    }
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
}