use std::{fmt::{self, Debug}, marker::PhantomData};

use crate::{expr::IndexedCaptures, haystack::{Haystack, HaystackItem}, matcher::Matcher};

#[derive(Default)]
pub struct CaptureGroup<I: HaystackItem, A: Matcher<I>, const N: usize>(
    pub PhantomData<I>,
    pub PhantomData<A>,
);

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Matcher<I> for CaptureGroup<I, A, N> {
    fn matches(hay: &mut Haystack<I>) -> bool {
        A::matches(hay)
    }

    fn all_matches<'a>(hay: &mut Haystack<'a, I>) -> Vec<Haystack<'a, I>> {
        A::all_matches(hay)
    }

    fn captures(hay: &mut Haystack<I>, caps: &mut IndexedCaptures) -> bool {
        let start = hay.index();
        if A::captures(hay, caps) {
            caps.push(N, start..hay.index());
            true
        } else {
            false
        }
    }

    fn all_captures<'a>(
        hay: &mut Haystack<'a, I>,
        caps: &mut IndexedCaptures
    ) -> Vec<(Haystack<'a, I>, IndexedCaptures)> {
        let start = hay.index();
        let mut captures = A::all_captures(hay, caps);

        for (hay_fork, caps_fork) in captures.iter_mut() {
            caps_fork.push(N, start..hay_fork.index());
        }

        captures
    }
}

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Debug for CaptureGroup<I, A, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", A::default())
    }
}