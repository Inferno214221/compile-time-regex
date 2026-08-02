use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ops::Range;

use crate::expr::{CaptureFromRanges, IndexedCaptures, Regex};
use crate::haystack::{HaystackItem, HaystackOf};
use crate::matcher::Matcher;
use crate::matcher::anchor::Anchor;

/// An `Iterator` over each match in the haystack, as a [`Range<usize>`](Range). See
/// [`Regex::range_of_all_matches`].
#[derive(Debug, Clone, Hash)]
pub struct RangeOfAllMatches<'a, R, I, H, const N: usize>
where
    R: Regex<I, N> + ?Sized,
    I: HaystackItem,
    H: HaystackOf<'a, I>,
{
    pub(crate) hay: H,
    pub(crate) overlapping: bool,
    pub(crate) last_check: bool,
    pub(crate) _phantom: PhantomData<(&'a (), I, R)>,
}

impl<'a, R, I, H, const N: usize> RangeOfAllMatches<'a, R, I, H, N>
where
    R: Regex<I, N> + ?Sized,
    I: HaystackItem,
    H: HaystackOf<'a, I>,
{
    pub fn new(hay: H, overlapping: bool) -> Self {
        RangeOfAllMatches {
            hay,
            overlapping,
            last_check: false,
            _phantom: PhantomData,
        }
    }
}

impl<'a, R, I, H, const N: usize> Iterator for RangeOfAllMatches<'a, R, I, H, N>
where
    R: Regex<I, N> + ?Sized,
    I: HaystackItem,
    H: HaystackOf<'a, I>,
{
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last_check {
            return None;
        }

        self.last_check = self.hay.item().is_none();
        let mut ret = None;

        if R::Anchors::assert(&self.hay).continue_value()? {
            let start = self.hay.index();

            if let Some(state_fork) = R::Pattern::all_matches(&mut self.hay).next() {
                ret = Some(start..state_fork);

                // If start == state_fork, we have a zero-width pattern and have already matched
                // this index. We need to progress normally.

                if !self.overlapping && start != state_fork {
                    self.hay.rollback(state_fork);
                    return ret.or_else(|| self.next());
                }
            }

            self.hay.rollback(start);
        }

        self.hay.progress();
        ret.or_else(|| self.next())
    }
}

impl<'a, R, I, H, const N: usize> FusedIterator for RangeOfAllMatches<'a, R, I, H, N>
where
    R: Regex<I, N> + ?Sized,
    I: HaystackItem,
    H: HaystackOf<'a, I>,
{}

/// An `Iterator` over each match in the haystack, as an `H::Slice`. See
/// [`Regex::slice_all_matches`].
#[derive(Debug, Clone, Hash)]
pub struct SliceAllMatches<'a, R, I, H, const N: usize>
where
    R: Regex<I, N> + ?Sized,
    I: HaystackItem,
    H: HaystackOf<'a, I>,
{
    pub(crate) inner: RangeOfAllMatches<'a, R, I, H, N>,
}

impl<'a, R, I, H, const N: usize> Iterator for SliceAllMatches<'a, R, I, H, N>
where
    R: Regex<I, N> + ?Sized,
    I: HaystackItem,
    H: HaystackOf<'a, I>,
{
    type Item = H::Slice;

    fn next(&mut self) -> Option<Self::Item> {
        let range = self.inner.next()?;
        Some(self.inner.hay.slice_with(range))
    }
}

impl<'a, R, I, H, const N: usize> FusedIterator for SliceAllMatches<'a, R, I, H, N>
where
    R: Regex<I, N> + ?Sized,
    I: HaystackItem,
    H: HaystackOf<'a, I>,
{}

/// An `Iterator` over each capture in the haystack, as an `R::Capture`. See
/// [`Regex::find_all_captures`].
#[derive(Debug, Clone, Hash)]
pub struct FindAllCaptures<'a, R, I, H, const N: usize>
where
    R: Regex<I, N> + ?Sized,
    I: HaystackItem,
    H: HaystackOf<'a, I>,
{
    pub(crate) hay: H,
    pub(crate) overlapping: bool,
    pub(crate) last_check: bool,
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
            last_check: false,
            _phantom: PhantomData,
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
        if self.last_check {
            return None;
        }

        self.last_check = self.hay.item().is_none();
        let mut ret = None;

        if R::Anchors::assert(&self.hay).continue_value()? {
            let start = self.hay.index();
            let mut caps = IndexedCaptures::default();

            if let Some((
                state_fork,
                mut caps_fork
            )) = R::Pattern::all_captures(&mut self.hay, &mut caps).next() {
                caps_fork.push(0, start..state_fork);
                ret = Some(
                    R::Capture::from_ranges(caps_fork.into_array(), self.hay.inner_slice())
                        .expect("failed to convert captures despite matching correctly")
                );

                // If start == state_fork, we have a zero-width pattern and have already matched
                // this index. We need to progress normally.

                if !self.overlapping && start != state_fork {
                    self.hay.rollback(state_fork);
                    return ret.or_else(|| self.next());
                }
            }

            self.hay.rollback(start);
        }

        self.hay.progress();
        ret.or_else(|| self.next())
    }
}

impl<'a, R, I, H, const N: usize> FusedIterator for FindAllCaptures<'a, R, I, H, N>
where
    R: Regex<I, N> + ?Sized,
    I: HaystackItem + 'a,
    H: HaystackOf<'a, I>,
{}
