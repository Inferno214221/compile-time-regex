use crate::{general::{Capture, FromCaptures, IndexedCaptures}, haystack::{Haystack, HaystackItem, HaystackIter}, matcher::Matcher};

pub trait Regex<I: HaystackItem, const N: usize> {
    type Pattern: Matcher<I>;

    type Captures<'a>: FromCaptures<'a, I, N> where I: 'a;

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

    fn find_match<'a>(hay: impl Into<Haystack<'a, I>>) -> Option<<I::Iter<'a> as HaystackIter<'a>>::Slice<'a>> {
        let mut hay = hay.into();
        while hay.item().is_some() {
            let start = hay.index();
            if Self::Pattern::matches(&mut hay) {
                let cap = Capture(start..hay.index());
                return Some(hay.slice(cap));
            }
            hay.progress()
        }
        None
    }

    fn find_all_matches<'a>(hay: impl Into<Haystack<'a, I>>) -> Vec<<I::Iter<'a> as HaystackIter<'a>>::Slice<'a>> {
        let mut hay = hay.into();
        let mut all_matches = vec![];

        while hay.item().is_some() {
            let start = hay.index();
            let mut rollback = hay.clone();

            if Self::Pattern::matches(&mut rollback) {
                all_matches.push(Capture(start..rollback.index()));
            }

            hay.progress()
        }

        all_matches.into_iter().map(|m| hay.slice(m)).collect()
    }

    fn captures<'a>(hay: impl Into<Haystack<'a, I>>) -> Option<Self::Captures<'a>> {
        let mut hay = hay.into();
        let mut caps = IndexedCaptures::default();

        let start = hay.index();
        Self::Pattern::capture(&mut hay, &mut caps);
        caps.push(0, Capture(start..hay.index()));

        Self::Captures::from_captures(caps.into_array(), hay)
    }

    fn find_captures<'a>(_hay: impl Into<Haystack<'a, I>>) -> Option<Self::Captures<'a>> {
        todo!("find_matches equivalent")
    }

    fn find_all_captures<'a>(_hay: impl Into<Haystack<'a, I>>) -> Vec<Self::Captures<'a>> {
        todo!("find_all_matches equivalent")
    }
}

pub trait AnonRegex<I: HaystackItem, const N: usize> {
    type Pattern: Matcher<I>;
    type Captures<'a>: FromCaptures<'a, I, N>;

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

    fn find_match<'a>(&self, hay: impl Into<Haystack<'a, I>>) -> Option<<I::Iter<'a> as HaystackIter<'a>>::Slice<'a>> {
        let mut hay = hay.into();
        while hay.item().is_some() {
            let start = hay.index();
            if Self::Pattern::matches(&mut hay) {
                let end = hay.index();
                return Some(hay.slice(Capture(start..end)));
            }
            hay.progress()
        }
        None
    }

    fn captures<'a>(&self, hay: impl Into<Haystack<'a, I>>) -> Option<Self::Captures<'a>> {
        let mut hay = hay.into();
        let mut caps = IndexedCaptures::default();

        let start = hay.index();
        Self::Pattern::capture(&mut hay, &mut caps);
        caps.push(0, Capture(start..hay.index()));

        Self::Captures::from_captures(caps.into_array(), hay)
    }

    fn find_captures<'a>(&self, _hay: impl Into<Haystack<'a, I>>) -> Option<Self::Captures<'a>> {
        todo!("find_matches equivalent")
    }

    fn find_all_captures<'a>(&self, _hay: impl Into<Haystack<'a, I>>) -> Vec<Self::Captures<'a>> {
        todo!("find_all_matches equivalent")
    }
}