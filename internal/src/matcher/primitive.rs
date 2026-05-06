use std::fmt::{self, Debug};

use crate::expr::IndexedCaptures;
use crate::haystack::{HaystackItem, HaystackOf};
use crate::matcher::{Matcher, impl_all_captures_single, impl_all_matches_single};

#[derive(Default, Clone, Copy)]
pub struct Byte<const N: u8>;

impl<const N: u8> Matcher<u8> for Byte<N> {
    fn matches<'a, H: HaystackOf<'a, u8>>(hay: &mut H) -> bool {
        if hay.item() == Some(N) {
            hay.progress();
            true
        } else {
            false
        }
    }

    impl_all_matches_single!(u8);
    impl_all_captures_single!(u8);
}

impl<const N: u8> Debug for Byte<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{N:#04x}")
    }
}

#[derive(Default, Clone, Copy)]
pub struct ByteRange<const A: u8, const B: u8>;

impl<const A: u8, const B: u8> Matcher<u8> for ByteRange<A, B> {
    fn matches<'a, H: HaystackOf<'a, u8>>(hay: &mut H) -> bool {
        if let Some(byte) = hay.item()
            && A <= byte
            && byte <= B
        {
            hay.progress();
            true
        } else {
            false
        }
    }

    impl_all_matches_single!(u8);
    impl_all_captures_single!(u8);
}

impl<const A: u8, const B: u8> Debug for ByteRange<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{A:#04x}-{B:#04x}]")
    }
}

#[derive(Default, Clone, Copy)]
pub struct Scalar<const N: char>;

impl<const N: char> Matcher<char> for Scalar<N> {
    fn matches<'a, H: HaystackOf<'a, char>>(hay: &mut H) -> bool {
        if hay.item() == Some(N) {
            hay.progress();
            true
        } else {
            false
        }
    }

    impl_all_matches_single!(char);
    impl_all_captures_single!(char);
}

impl<const N: char> Debug for Scalar<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", N.escape_debug())
    }
}

#[derive(Default, Clone, Copy)]
pub struct ScalarRange<const A: char, const B: char>;

impl<const A: char, const B: char> Matcher<char> for ScalarRange<A, B> {
    fn matches<'a, H: HaystackOf<'a, char>>(hay: &mut H) -> bool {
        if let Some(scalar) = hay.item()
            && A <= scalar
            && scalar <= B
        {
            hay.progress();
            true
        } else {
            false
        }
    }

    impl_all_matches_single!(char);
    impl_all_captures_single!(char);
}

impl<const A: char, const B: char> Debug for ScalarRange<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}-{}]", A.escape_debug(), B.escape_debug())
    }
}

#[derive(Default, Clone, Copy)]
pub struct Always;

impl<I: HaystackItem> Matcher<I> for Always {
    fn matches<'a, H: HaystackOf<'a, I>>(_hay: &mut H) -> bool {
        true
    }

    impl_all_matches_single!(I);
    impl_all_captures_single!(I);
}

impl Debug for Always {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "()")
    }
}

#[derive(Default, Clone, Copy)]
pub struct Start;

impl<I: HaystackItem> Matcher<I> for Start {
    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        hay.is_start()
    }

    impl_all_matches_single!(I);
    impl_all_captures_single!(I);
}

impl Debug for Start {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\A")
    }
}

#[derive(Default, Clone, Copy)]
pub struct End;

impl<I: HaystackItem> Matcher<I> for End {
    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        hay.is_end()
    }

    impl_all_matches_single!(I);
    impl_all_captures_single!(I);
}

impl Debug for End {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\z")
    }
}

#[derive(Default, Clone, Copy)]
pub struct LineStart;

impl<I: HaystackItem> Matcher<I> for LineStart {
    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        hay.is_line_start()
    }

    impl_all_matches_single!(I);
    impl_all_captures_single!(I);
}

impl Debug for LineStart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "^")
    }
}

#[derive(Default, Clone, Copy)]
pub struct LineEnd;

impl<I: HaystackItem> Matcher<I> for LineEnd {
    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        hay.is_line_end()
    }

    impl_all_matches_single!(I);
    impl_all_captures_single!(I);
}

impl Debug for LineEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "$")
    }
}

#[derive(Default, Clone, Copy)]
pub struct CRLFStart;

impl<I: HaystackItem> Matcher<I> for CRLFStart {
    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        hay.is_crlf_start()
    }

    impl_all_matches_single!(I);
    impl_all_captures_single!(I);
}

impl Debug for CRLFStart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "^")
    }
}

#[derive(Default, Clone, Copy)]
pub struct CRLFEnd;

impl<I: HaystackItem> Matcher<I> for CRLFEnd {
    fn matches<'a, H: HaystackOf<'a, I>>(hay: &mut H) -> bool {
        hay.is_crlf_end()
    }

    impl_all_matches_single!(I);
    impl_all_captures_single!(I);
}

impl Debug for CRLFEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "$")
    }
}
