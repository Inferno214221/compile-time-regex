use std::collections::HashMap;

use crate::{haystack::{Haystack, HaystackItem, HaystackIter}, matcher::Matcher};

pub trait Regex<I: HaystackItem> {
    type Pattern: Matcher<I>;

    fn is_match(hay: &mut Haystack<I>) -> bool {
        Self::Pattern::matches(hay) && hay.is_end()
    }

    fn contains_match(hay: &mut Haystack<I>) -> bool {
        while hay.item().is_some() {
            if Self::Pattern::matches(hay) {
                return true;
            }
            hay.progress()
        }
        false
    }

    fn find_match<'a>(hay: &'a mut Haystack<'a, I>) -> Option<<I::Iter<'a> as HaystackIter<'a>>::Slice<'a>> {
        while hay.item().is_some() {
            let start = hay.index();
            if Self::Pattern::matches(hay) {
                let end = hay.index();
                return Some(hay.slice(start..end));
            }
            hay.progress()
        }
        None
    }

    fn find_all_matches<'a>(_hay: &'a mut Haystack<'a, I>) -> Vec<<I::Iter<'a> as HaystackIter<'a>>::Slice<'a>> {
        todo!("all matches including overlaps")
    }

    fn captures<'a>(_hay: &'a mut Haystack<'a, I>) -> Option<HashMap<String, &'a str>> {
        todo!("matches equivalent")
    }

    fn find_captures<'a>(_hay: &'a mut Haystack<'a, I>) -> Option<HashMap<String, &'a str>> {
        todo!("find_matches equivalent")
    }

    fn find_all_captures<'a>(_hay: &'a mut Haystack<'a, I>) -> Vec<HashMap<String, &'a str>> {
        todo!("find_all_matches equivalent")
    }
}

pub trait AnonRegex<I: HaystackItem> {
    type Pattern: Matcher<I>;

    fn is_match(&self, hay: &mut Haystack<I>) -> bool {
        Self::Pattern::matches(hay) && hay.is_end()
    }

    fn contains_match(&self, hay: &mut Haystack<I>) -> bool {
        while hay.item().is_some() {
            if Self::Pattern::matches(hay) {
                return true;
            }
            hay.progress()
        }
        false
    }

    fn find_match<'a>(&self, hay: &'a mut Haystack<'a, I>) -> Option<<I::Iter<'a> as HaystackIter<'a>>::Slice<'a>> {
        while hay.item().is_some() {
            let start = hay.index();
            if Self::Pattern::matches(hay) {
                let end = hay.index();
                return Some(hay.slice(start..end));
            }
            hay.progress()
        }
        None
    }
}