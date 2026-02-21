use crate::{haystack::Haystack, matcher::Matcher};

pub trait Regex {
    type Pattern: Matcher;

    fn matches(hay: &mut Haystack) -> bool {
        Self::Pattern::matches(hay)
    }
}