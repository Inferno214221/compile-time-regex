use std::{fmt::{self, Debug}, marker::PhantomData};

use crate::{expr::IndexedCaptures, haystack::{HaystackItem, HaystackOf}, matcher::Matcher};

#[derive(Default)]
pub struct CaptureGroup<I: HaystackItem, A: Matcher<I>, const N: usize>(
    pub PhantomData<I>,
    pub PhantomData<A>,
);

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Matcher<I> for CaptureGroup<I, A, N> {
    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        A::matches(hay)
    }

    fn all_matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> Vec<usize> {
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
    ) -> Vec<(usize, IndexedCaptures)> {
        let start = hay.index();
        let mut captures = A::all_captures(hay, caps);

        for (state_fork, caps_fork) in captures.iter_mut() {
            caps_fork.push(N, start..*state_fork);
        }

        captures
    }
}

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Debug for CaptureGroup<I, A, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", A::default())
    }
}