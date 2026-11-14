use std::fmt::{self, Debug};
use std::marker::PhantomData;

use crate::expr::IndexedCaptures;
use crate::haystack::{HaystackItem, HaystackOf, HaystackSlice};
use crate::matcher::{Matcher, impl_all_captures_single, impl_all_matches_single};
use crate::sealed::Sealed;

pub trait Literal: Default + Clone + Copy {
    const LITERAL: &[u8];
}

#[derive(Default, Clone, Copy)]
pub struct LiteralMatcher<L: Literal>(pub PhantomData<L>);

impl<L: Literal> Sealed for LiteralMatcher<L> {}

impl<L: Literal, I: HaystackItem> Matcher<I> for LiteralMatcher<L> {
    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        let success = hay.remainder_as_slice()
            .as_bytes()
            .starts_with(L::LITERAL);

        // This should avoid unnecessary branching here, because hay's state is undefined in the
        // case of a fail.
        hay.skip(L::LITERAL.len());

        success
    }

    impl_all_matches_single!(I);
    impl_all_captures_single!(I);
}

impl<L: Literal> Debug for LiteralMatcher<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", L::LITERAL)
    }
}
