use std::marker::PhantomData;

use crate::{haystack::{Haystack, HaystackItem}, matcher::Matcher};

pub struct CaptureGroup<I: HaystackItem, A: Matcher<I>, const N: usize>(
    pub PhantomData<I>,
    pub PhantomData<A>,
);

impl<I: HaystackItem, A: Matcher<I>, const N: usize> Matcher<I> for CaptureGroup<I, A, N> {
    fn matches(hay: &mut Haystack<I>) -> bool {
        A::matches(hay)
    }

    // capture
}