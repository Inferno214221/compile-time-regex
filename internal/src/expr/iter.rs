use std::{marker::PhantomData, ops::Range};

use crate::{expr::{CaptureFromRanges, IndexedCaptures, Regex}, haystack::{HaystackItem, HaystackOf}, matcher::Matcher};

#[derive(Debug, Clone)]
pub struct RangeOfAllMatches<'a, I: HaystackItem, H: HaystackOf<'a, I>, M: Matcher<I>> {
    pub(crate) hay: H,
    pub(crate) overlapping: bool,
    pub(crate) _phantom: PhantomData<(&'a (), I, M)>,
}

impl<'a, I: HaystackItem, H: HaystackOf<'a, I>, M: Matcher<I>> RangeOfAllMatches<'a, I, H, M> {
    pub fn new(hay: H, overlapping: bool) -> Self {
        RangeOfAllMatches {
            hay,
            overlapping,
            _phantom: PhantomData,
        }
    }
}

impl<'a, I, H, M> Iterator for RangeOfAllMatches<'a, I, H, M>
where
    I: HaystackItem,
    H: HaystackOf<'a, I>,
    M: Matcher<I>,
{
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let _ = self.hay.item()?;

        let start = self.hay.index();

        if let Some(state_fork) = M::all_matches(&mut self.hay).next() {
            if self.overlapping {
                self.hay.rollback(start).progress();
            } else {
                self.hay.rollback(state_fork);

                // This doesn't seem to make a difference...
                debug_assert_ne!(start, state_fork)
                // if start == state_fork {
                //     // We've already matched at this index.
                //     hay.progress();
                // }
            }

            Some(start..state_fork)
        } else {
            self.hay.rollback(start).progress();
            self.next()
        }
    }
}

#[derive(Debug, Clone)]
pub struct SliceAllMatches<'a, I: HaystackItem, H: HaystackOf<'a, I>, M: Matcher<I>> {
    pub(crate) inner: RangeOfAllMatches<'a, I, H, M>,
}

impl<'a, I, H, M> Iterator for SliceAllMatches<'a, I, H, M>
where
    I: HaystackItem,
    H: HaystackOf<'a, I>,
    M: Matcher<I>,
{
    type Item = H::Slice;

    fn next(&mut self) -> Option<Self::Item> {
        let range = self.inner.next()?;
        Some(self.inner.hay.slice_with(range))
    }
}

#[derive(Debug, Clone)]
pub struct FindAllCaptures<'a, R, I, H, const N: usize>
where
    R: Regex<I, N> + ?Sized,
    I: HaystackItem,
    H: HaystackOf<'a, I>,
{
    pub(crate) hay: H,
    pub(crate) overlapping: bool,
    pub(crate) _phantom: PhantomData<(&'a (), I, R)>,
}

impl<'a, R, I, H, const N: usize> FindAllCaptures<'a, R, I, H, N>
where
    R: Regex<I, N> + ?Sized,
    I: HaystackItem,
    H: HaystackOf<'a, I>,
{
    pub fn new(hay: H, overlapping: bool) -> Self {
        Self {
            hay,
            overlapping,
            _phantom: PhantomData
        }
    }
}

impl<'a, R, I, H, const N: usize> Iterator for FindAllCaptures<'a, R, I, H, N>
where
    R: Regex<I, N> + ?Sized,
    I: HaystackItem + 'a,
    H: HaystackOf<'a, I>,
{
    type Item = R::Capture<'a, H::Slice>;

    fn next(&mut self) -> Option<Self::Item> {
        let _ = self.hay.item()?;

        let start = self.hay.index();

        let mut caps = IndexedCaptures::default();

        let first = R::Pattern::all_captures(&mut self.hay, &mut caps).next();

        if let Some((state_fork, mut caps_fork)) = first {
            caps_fork.push(0, start..state_fork);

            if self.overlapping {
                self.hay.rollback(start).progress();
            } else {
                self.hay.rollback(state_fork);
                debug_assert_ne!(start, state_fork);
            }

            Some(
                R::Capture::from_ranges(caps_fork.into_array(), self.hay.inner_slice())
                    .expect("failed to convert captures despite matching correctly")
            )
        } else {
            self.hay.rollback(start).progress();
            self.next()
        }
    }
}