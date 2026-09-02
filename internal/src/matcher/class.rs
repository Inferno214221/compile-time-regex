use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::marker::PhantomData;

use crate::expr::IndexedCaptures;
use crate::haystack::{HaystackItem, HaystackOf};
use crate::matcher::{Matcher, impl_all_captures_single, impl_all_matches_single};
use crate::sealed::Sealed;

#[derive(Clone)]
pub struct ClassEntry<I: HaystackItem> {
    pub value: I,
    pub is_upper_bound: bool,
}

impl<I: HaystackItem> ClassEntry<I> {
    pub const fn new(value: I, is_upper_bound: bool) -> ClassEntry<I> {
        ClassEntry { value, is_upper_bound }
    }

    pub fn cmp_item(&self, item: &I) -> Ordering {
        self.value.cmp(item)
    }
}

impl<I: HaystackItem> PartialEq<I> for ClassEntry<I> {
    fn eq(&self, other: &I) -> bool {
        &self.value == other
    }
}

impl<I: HaystackItem> Debug for ClassEntry<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_upper_bound {
            write!(f, "-")?;
        }
        write!(f, "{:?}", self.value)
    }
}

pub trait Class<I: HaystackItem>: Default + Clone + Copy {
    const ENTRIES: &[ClassEntry<I>];
}

#[derive(Default, Clone, Copy)]
pub struct ClassMatcher<I: HaystackItem, C: Class<I>>(pub PhantomData<(I, C)>);

impl<I: HaystackItem, C: Class<I>> Sealed for ClassMatcher<I, C> {}

impl<I: HaystackItem, C: Class<I>> Matcher<I> for ClassMatcher<I, C> {
    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        let Some(item) = hay.next() else {
            return false;
        };
        // FIXME: could be out of bounds?
        match C::ENTRIES.binary_search_by(|entry| entry.cmp_item(&item)) {
            Ok(_) => true,
            // We've failed the exact binary search, but if the target index for insertion is an
            // upper bound, we're in the middle of a range. Still counts as a match.
            Err(index) if C::ENTRIES.get(index).is_some_and(|entry| entry.is_upper_bound) => true,
            _ => false,
        }
    }

    impl_all_matches_single!(I);
    impl_all_captures_single!(I);
}

impl<I: HaystackItem, C: Class<I>> Debug for ClassMatcher<I, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for entry in C::ENTRIES {
            write!(f, "{:?}", entry)?;
        }
        write!(f, "]")
    }
}
