use crate::{haystack::{Haystack, HaystackItem}, matcher::Matcher};

pub trait Regex<I: HaystackItem> {
    type Pattern: Matcher<I>;

    fn matches(hay: &mut Haystack<I>) -> bool {
        Self::Pattern::matches(hay)
    }
}

pub trait AnonRegex<I: HaystackItem> {
    type Pattern: Matcher<I>;

    fn matches(&self, hay: &mut Haystack<I>) -> bool {
        Self::Pattern::matches(hay)
    }
}