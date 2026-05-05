use std::fmt::{self, Debug};
use std::iter;
use std::marker::PhantomData;

use crate::expr::IndexedCaptures;
use crate::haystack::{HaystackItem, HaystackOf};
use crate::matcher::{
    AllCapturesSingle, AllMatchesSingle, LazyMatcher, Matcher, QuantifierNOrMore, QuantifierNToM,
};

#[derive(Default)]
pub struct Lazy<I: HaystackItem, Q: LazyMatcher<I>>(pub PhantomData<I>, pub PhantomData<Q>);

impl<I: HaystackItem, Q: LazyMatcher<I>> Matcher<I> for Lazy<I, Q> {
    type AllMatches<'a, H: HaystackOf<'a, I>> = Q::LazyAllMatches<'a, H>;
    type AllCaptures<'a, H: HaystackOf<'a, I>> = Q::LazyAllCaptures<'a, H>;

    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        Q::lazy_matches(hay)
    }

    fn all_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> Self::AllMatches<'a, H> {
        Q::lazy_all_matches(hay)
    }

    fn captures<'a, H: HaystackOf<'a, I>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        Q::lazy_captures(hay, caps)
    }

    fn all_captures<'a, H: HaystackOf<'a, I>>(
        hay: &mut H,
        caps: &mut IndexedCaptures,
    ) -> Self::AllCaptures<'a, H> {
        Q::lazy_all_captures(hay, caps)
    }
}

impl<I: HaystackItem, Q: LazyMatcher<I>> Debug for Lazy<I, Q> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}?", Q::default())
    }
}

pub struct LazyAllMatchesNOrMore<'a, I, H, A, const N: usize>
where
    I: HaystackItem,
    H: HaystackOf<'a, I>,
    A: Matcher<I>,
{
    hay: H,
    count: usize,
    _phantom: PhantomData<(&'a (), I, A)>,
}

impl<'a, I, H, A, const N: usize> Iterator for LazyAllMatchesNOrMore<'a, I, H, A, N>
where
    I: HaystackItem,
    H: HaystackOf<'a, I>,
    A: Matcher<I>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // N == 0 is handled before we create an Iterator.
        if A::matches(&mut self.hay) {
            self.count += 1;
            if self.count >= N {
                Some(self.hay.index())
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

pub struct LazyAllCapturesNOrMore<'a, I, H, A, const N: usize>
where
    I: HaystackItem,
    H: HaystackOf<'a, I>,
    A: Matcher<I>,
{
    hay: H,
    caps: IndexedCaptures,
    count: usize,
    _phantom: PhantomData<(&'a (), I, A)>,
}

impl<'a, I, H, A, const N: usize> Iterator for LazyAllCapturesNOrMore<'a, I, H, A, N>
where
    I: HaystackItem,
    H: HaystackOf<'a, I>,
    A: Matcher<I>,
{
    type Item = (usize, IndexedCaptures);

    fn next(&mut self) -> Option<Self::Item> {
        if A::captures(&mut self.hay, &mut self.caps) {
            self.count += 1;
            if self.count >= N {
                Some((self.hay.index(), self.caps.clone()))
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

impl<I: HaystackItem, A: Matcher<I>, const N: usize> LazyMatcher<I> for QuantifierNOrMore<I, A, N> {
    type LazyAllMatches<'a, H: HaystackOf<'a, I>> = iter::Chain<AllMatchesSingle, LazyAllMatchesNOrMore<'a, I, H, A, N>>;
    type LazyAllCaptures<'a, H: HaystackOf<'a, I>> = iter::Chain<AllCapturesSingle, LazyAllCapturesNOrMore<'a, I, H, A, N>>;

    fn lazy_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        let mut count = 0;
        loop {
            if count >= N {
                return true;
            }
            if A::matches(hay) {
                count += 1;
            } else {
                break;
            }
        }
        false
    }

    fn lazy_all_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> Self::LazyAllMatches<'a, H> {
        let zero_match = if N == 0 {
            Some(hay.index())
        } else {
            None
        };

        zero_match.into_iter().chain(
            LazyAllMatchesNOrMore {
                hay: hay.clone(),
                count: 0,
                _phantom: PhantomData,
            }
        )
    }

    fn lazy_captures<'a, H: HaystackOf<'a, I>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        let mut count = 0;
        loop {
            if count >= N {
                return true;
            }
            if A::captures(hay, caps) {
                count += 1;
            } else {
                break;
            }
        }
        false
    }

    fn lazy_all_captures<'a, H: HaystackOf<'a, I>>(
        hay: &mut H,
        caps: &mut IndexedCaptures,
    ) -> Self::LazyAllCaptures<'a, H> {
        let zero_match = if N == 0 {
            Some((hay.index(), caps.clone()))
        } else {
            None
        };

        zero_match.into_iter().chain(
            LazyAllCapturesNOrMore {
                hay: hay.clone(),
                caps: caps.clone(),
                count: 0,
                _phantom: PhantomData,
            }
        )
    }
}

pub struct LazyAllMatchesNToM<'a, I, H, A, const N: usize, const M: usize>
where
    I: HaystackItem,
    H: HaystackOf<'a, I>,
    A: Matcher<I>,
{
    hay: H,
    count: usize,
    _phantom: PhantomData<(&'a (), I, A)>,
}

impl<'a, I, H, A, const N: usize, const M: usize> Iterator for LazyAllMatchesNToM<'a, I, H, A, N, M>
where
    I: HaystackItem,
    H: HaystackOf<'a, I>,
    A: Matcher<I>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if A::matches(&mut self.hay) {
            self.count += 1;
            if N <= self.count && self.count <= M {
                Some(self.hay.index())
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

pub struct LazyAllCapturesNToM<'a, I, H, A, const N: usize, const M: usize>
where
    I: HaystackItem,
    H: HaystackOf<'a, I>,
    A: Matcher<I>,
{
    hay: H,
    caps: IndexedCaptures,
    count: usize,
    _phantom: PhantomData<(&'a (), I, A)>,
}

impl<'a, I, H, A, const N: usize, const M: usize> Iterator for LazyAllCapturesNToM<'a, I, H, A, N, M>
where
    I: HaystackItem,
    H: HaystackOf<'a, I>,
    A: Matcher<I>,
{
    type Item = (usize, IndexedCaptures);

    fn next(&mut self) -> Option<Self::Item> {
        if A::captures(&mut self.hay, &mut self.caps) {
            self.count += 1;
            if N <= self.count && self.count <= M {
                Some((self.hay.index(), self.caps.clone()))
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

impl<I: HaystackItem, A: Matcher<I>, const N: usize, const M: usize> LazyMatcher<I> for QuantifierNToM<I, A, N, M> {
    type LazyAllMatches<'a, H: HaystackOf<'a, I>> = iter::Chain<AllMatchesSingle, LazyAllMatchesNToM<'a, I, H, A, N, M>>;
    type LazyAllCaptures<'a, H: HaystackOf<'a, I>> = iter::Chain<AllCapturesSingle, LazyAllCapturesNToM<'a, I, H, A, N, M>>;

    fn lazy_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        let mut count = 0;
        loop {
            if count >= N && count <= M {
                return true;
            }
            if A::matches(hay) {
                count += 1;
            } else {
                break;
            }
        }
        false
    }

    fn lazy_all_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> Self::LazyAllMatches<'a, H> {
        let zero_match = if N == 0 {
            Some(hay.index())
        } else {
            None
        };

        zero_match.into_iter().chain(
            LazyAllMatchesNToM {
                hay: hay.clone(),
                count: 0,
                _phantom: PhantomData,
            }
        )
    }

    fn lazy_captures<'a, H: HaystackOf<'a, I>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        let mut count = 0;
        loop {
            if count >= N && count <= M {
                return true;
            }
            if A::captures(hay, caps) {
                count += 1;
            } else {
                break;
            }
        }
        false
    }

    fn lazy_all_captures<'a, H: HaystackOf<'a, I>>(
        hay: &mut H,
        caps: &mut IndexedCaptures,
    ) -> Self::LazyAllCaptures<'a, H> {
        let zero_match = if N == 0 {
            Some((hay.index(), caps.clone()))
        } else {
            None
        };

        zero_match.into_iter().chain(
            LazyAllCapturesNToM {
                hay: hay.clone(),
                caps: caps.clone(),
                count: 0,
                _phantom: PhantomData,
            }
        )
    }
}
