use std::fmt::Debug;

use crate::{general::{CaptureFromRanges, IndexedCaptures}, haystack::{Haystack, HaystackItem}, matcher::Matcher};

pub trait Regex<I: HaystackItem, const N: usize>: Debug {
    type Pattern: Matcher<I>;
    type Capture<'a>: CaptureFromRanges<'a, I, N> where I: 'a;

    fn is_match<'a>(hay: impl Into<Haystack<'a, I>>) -> bool {
        let mut hay = hay.into();

        Self::Pattern::matches(&mut hay) && hay.is_end()
    }

    fn contains_match<'a>(hay: impl Into<Haystack<'a, I>>) -> bool {
        let mut hay = hay.into();

        while hay.item().is_some() {
            if Self::Pattern::matches(&mut hay.clone()) {
                return true;
            }
            hay.progress()
        }
        false
    }

    // TODO: Is hay.progress really the right semantics?
    fn find_match<'a>(hay: impl Into<Haystack<'a, I>>) -> Option<I::Slice<'a>> {
        let mut hay = hay.into();

        while hay.item().is_some() {
            let start = hay.index();
            let mut fork = hay.clone();

            if Self::Pattern::matches(&mut fork) {
                let cap = start..fork.index();
                return Some(hay.slice(cap));
            }
            hay.progress()
        }
        None
    }

    fn find_all_matches<'a>(
        hay: impl Into<Haystack<'a, I>>,
        overlapping: bool
    ) -> Vec<I::Slice<'a>> {
        let mut hay = hay.into();

        let mut all_matches = vec![];

        while hay.item().is_some() {
            let start = hay.index();
            let rollback = hay.clone();

            if Self::Pattern::matches(&mut hay) {
                all_matches.push(start..hay.index());
            }

            if overlapping {
                hay = rollback;
                hay.progress();
            }
        }

        all_matches.into_iter().map(|m| hay.slice(m)).collect()
    }

    // TODO: switch capture methods to rollback enabled versions
    // TODO: switch to lazy rollback via iterators

    fn do_capture<'a>(hay: impl Into<Haystack<'a, I>>) -> Option<Self::Capture<'a>> {
        let mut hay = hay.into();

        let mut caps = IndexedCaptures::default();

        let start = hay.index();
        if Self::Pattern::captures(&mut hay, &mut caps) && hay.is_end() {
            caps.push(0, start..hay.index());
            Some(
                Self::Capture::from_ranges(caps.into_array(), hay)
                    .expect("failed to convert captures despite matching correctly")
            )
        } else {
            None
        }
    }

    fn find_capture<'a>(hay: impl Into<Haystack<'a, I>>) -> Option<Self::Capture<'a>> {
        let mut hay = hay.into();

        let mut caps = IndexedCaptures::default();

        while hay.item().is_some() {
            let start = hay.index();
            let mut fork = hay.clone();

            if Self::Pattern::captures(&mut fork, &mut caps) {
                caps.push(0, start..fork.index());
                return Some(
                    Self::Capture::from_ranges(caps.into_array(), hay)
                        .expect("failed to convert captures despite matching correctly")
                );
            }
            hay.progress()
        }
        None
    }

    fn find_all_captures<'a>(
        _hay: impl Into<Haystack<'a, I>>,
        _overlapping: bool
    ) -> Vec<Self::Capture<'a>> {
        todo!("find_all_matches equivalent")
    }
}

pub trait AnonRegex<I: HaystackItem, const N: usize>: Debug {
    type Pattern: Matcher<I>;
    type Capture<'a>: CaptureFromRanges<'a, I, N>;

    fn is_match<'a>(&self, hay: impl Into<Haystack<'a, I>>) -> bool {
        let mut hay = hay.into();
        
        Self::Pattern::matches(&mut hay) && hay.is_end()
    }

    fn contains_match<'a>(&self, hay: impl Into<Haystack<'a, I>>) -> bool {
        let mut hay = hay.into();
        while hay.item().is_some() {
            if Self::Pattern::matches(&mut hay) {
                return true;
            }
            hay.progress()
        }
        false
    }

    fn find_match<'a>(&self, hay: impl Into<Haystack<'a, I>>) -> Option<I::Slice<'a>> {
        let mut hay = hay.into();

        while hay.item().is_some() {
            let start = hay.index();
            if Self::Pattern::matches(&mut hay) {
                let end = hay.index();
                return Some(hay.slice(start..end));
            }
            hay.progress()
        }
        None
    }

    fn find_all_matches<'a>(
        &self,
        hay: impl Into<Haystack<'a, I>>,
        overlapping: bool
    ) -> Vec<I::Slice<'a>> {
        let mut hay = hay.into();

        let mut all_matches = vec![];

        while hay.item().is_some() {
            let start = hay.index();
            let rollback = hay.clone();

            if Self::Pattern::matches(&mut hay) {
                all_matches.push(start..hay.index());
            }

            if overlapping {
                hay = rollback;
                hay.progress();
            }
        }

        all_matches.into_iter().map(|m| hay.slice(m)).collect()
    }

    fn captures<'a>(&self, hay: impl Into<Haystack<'a, I>>) -> Option<Self::Capture<'a>> {
        let mut hay = hay.into();

        let mut caps = IndexedCaptures::default();

        let start = hay.index();
        if Self::Pattern::captures(&mut hay, &mut caps) {
            // Capture the "whole_match" group at index 0.
            caps.push(0, start..hay.index());
            Some(
                Self::Capture::from_ranges(caps.into_array(), hay)
                    .expect("failed to convert captures despite matching correctly")
            )
        } else {
            None
        }
    }

    fn find_captures<'a>(&self, _hay: impl Into<Haystack<'a, I>>) -> Option<Self::Capture<'a>> {
        todo!("find_matches equivalent")
    }

    fn find_all_captures<'a>(
        &self,
        _hay: impl Into<Haystack<'a, I>>,
        _overlapping: bool
    ) -> Vec<Self::Capture<'a>> {
        todo!("find_all_matches equivalent")
    }
}