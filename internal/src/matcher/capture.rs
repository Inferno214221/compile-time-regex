use std::fmt::{self, Debug};
use std::marker::PhantomData;

use crate::expr::IndexedCaptures;
use crate::haystack::{HaystackItem, HaystackOf};
use crate::matcher::Matcher;

pub struct AllCapturesGroup<'a, I, H, A, const N: usize>
where
    I: HaystackItem,
    H: HaystackOf<'a, I>,
    A: Matcher<I>
{
    captures: A::AllCaptures<'a, H>,
    start: usize,
}

impl<'a, I, H, A, const N: usize> Iterator for AllCapturesGroup<'a, I, H, A, N>
where
    I: HaystackItem,
    H: HaystackOf<'a, I>,
    A: Matcher<I>
{
    type Item = (usize, IndexedCaptures);

    fn next(&mut self) -> Option<Self::Item> {
        let (state_fork, mut caps_fork) = self.captures.next()?;
        caps_fork.push(N, self.start..state_fork);
        Some((state_fork, caps_fork))
    }
}

#[derive(Default)]
pub struct CaptureGroup<I: HaystackItem, A: Matcher<I>, const N: usize>(
    pub PhantomData<I>,
    pub PhantomData<A>,
);

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Matcher<I> for CaptureGroup<I, A, N> {
    type AllMatches<'a, H: HaystackOf<'a, I>> = A::AllMatches<'a, H>;
    type AllCaptures<'a, H: HaystackOf<'a, I>> = AllCapturesGroup<'a, I, H, A, N>;

    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        A::matches(hay)
    }

    fn all_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> Self::AllMatches<'a, H> {
        A::all_matches(hay)
    }

    fn captures<'a, H: HaystackOf<'a, I>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        let start = hay.index();
        if A::captures(hay, caps) {
            caps.push(N, start..hay.index());
            true
        } else {
            false
        }
    }

    fn all_captures<'a, H: HaystackOf<'a, I>>(
        hay: &mut H,
        caps: &mut IndexedCaptures
    ) -> Self::AllCaptures<'a, H> {
        AllCapturesGroup {
            start: hay.index(),
            captures: A::all_captures(hay, caps),
        }
    }
}

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Debug for CaptureGroup<I, A, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", A::default())
    }
}
