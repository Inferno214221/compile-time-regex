use std::{option, vec};

use crate::{haystack::{HaystackItem, HaystackOf}, matcher::Matcher};

pub type AllMatchesSingle = option::IntoIter<usize>;
pub type AllMatchesMultiple = vec::IntoIter<usize>;

pub fn all_matches_single<'a, I: HaystackItem, H: HaystackOf<'a, I>, M: Matcher<I>>(hay: &mut H) -> AllMatchesSingle {
    if M::matches(hay) {
        Some(hay.index()).into_iter()
    } else {
        None.into_iter()
    }
}

// This is just a helper to get around the fact that there are no default associated types yet.
#[macro_export]
#[doc(hidden)]
macro_rules! impl_all_matches_single {
    ($I:ident) => {
        type AllMatches<'a, H: HaystackOf<'a, $I>> = $crate::matcher::AllMatchesSingle;

        fn all_matches<'a, H: HaystackOf<'a, $I>>(hay: &mut H) -> Self::AllMatches<'a, H> {
            $crate::matcher::all_matches_single::<$I, H, Self>(hay)
        }
    };
}

#[doc(inline)]
pub use impl_all_matches_single;