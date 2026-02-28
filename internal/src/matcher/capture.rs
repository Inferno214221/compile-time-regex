use std::marker::PhantomData;

use crate::{general::IndexedCaptures, haystack::{Haystack, HaystackItem}, matcher::Matcher};

pub struct CaptureGroup<I: HaystackItem, A: Matcher<I>, const N: usize>(
    pub PhantomData<I>,
    pub PhantomData<A>,
);

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Matcher<I> for CaptureGroup<I, A, N> {
    fn matches(hay: &mut Haystack<I>) -> bool {
        A::matches(hay)
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
        let captures = A::all_captures(hay, caps);

        for (mut h, mut c) in A::all_captures(hay, caps) {
            c.push(N, start..h.index());
        }

        captures
    }
}