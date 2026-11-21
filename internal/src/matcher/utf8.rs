use std::{fmt::{self, Debug}, iter::FusedIterator, marker::PhantomData};

use crate::{expr::IndexedCaptures, haystack::{HaystackItem, HaystackOf}, matcher::Matcher, sealed::Sealed};

// Copied from Rust internals, not offered publicly for some reason. I am fairly certain that this
// won't change.
// https://github.com/rust-lang/rust/blob/5f4effd579500175af844241eb179e756190f1e7/library/core/src/num/mod.rs#L1236
pub(crate) fn is_utf8_char_boundary(byte: u8) -> bool {
    // Byte is outside this range.
    !(128..192).contains(&byte)
}

pub(crate) fn check_item(item: Option<u8>) -> bool {
    item.map(is_utf8_char_boundary).unwrap_or(true)
}

#[derive(Default, Clone, Copy)]
pub struct ZeroWidthCheck<I: HaystackItem, A: Matcher<I>>(pub(crate) PhantomData<(I, A)>);

impl<I: HaystackItem, A: Matcher<I>> Sealed for ZeroWidthCheck<I, A> {}

// Not matching over a generic A doesn't matter, because A is sealed and always concrete when used
// by the macro.

impl<A: Matcher<u8>> Matcher<u8> for ZeroWidthCheck<u8, A> {
    type AllMatches<'a, H: HaystackOf<'a, u8>> = AllMatchesZeroWidthCheck<'a, H, A>;
    type AllCaptures<'a, H: HaystackOf<'a, u8>> = AllCapturesZeroWidthCheck<'a, H, A>;

    fn matches<'a, H: HaystackOf<'a, u8>>(hay: &mut H) -> bool {
        // We only have to check zero-width matches that are in the middle of a codepoint.
        // Therefore, if the next (current) item is a utf8 boundary, we can match before it.
        // Otherwise, we're in the middle of a codepoint.
        A::matches(hay) && check_item(hay.item())
    }

    fn captures<'a, H: HaystackOf<'a, u8>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        A::captures(hay, caps) && check_item(hay.item())
    }

    fn all_matches<'a, H: HaystackOf<'a, u8>>(hay: &mut H) -> Self::AllMatches<'a, H> {
        AllMatchesZeroWidthCheck {
            hay: hay.clone(),
            inner: A::all_matches(hay),
            _phantom: PhantomData,
        }
    }

    fn all_captures<'a, H: HaystackOf<'a, u8>>(
        hay: &mut H,
        caps: &mut IndexedCaptures,
    ) -> Self::AllCaptures<'a, H> {
        AllCapturesZeroWidthCheck {
            hay: hay.clone(),
            inner: A::all_captures(hay, caps),
            _phantom: PhantomData,
        }
    }
}

impl<A: Matcher<char>> Matcher<char> for ZeroWidthCheck<char, A> {
    type AllMatches<'a, H: HaystackOf<'a, char>> = A::AllMatches<'a, H>;
    type AllCaptures<'a, H: HaystackOf<'a, char>> = A::AllCaptures<'a, H>;

    fn matches<'a, H: HaystackOf<'a, char>>(hay: &mut H) -> bool {
        A::matches(hay)
    }

    fn captures<'a, H: HaystackOf<'a, char>>(hay: &mut H, caps: &mut IndexedCaptures) -> bool {
        A::captures(hay, caps)
    }

    fn all_matches<'a, H: HaystackOf<'a, char>>(hay: &mut H) -> Self::AllMatches<'a, H> {
        A::all_matches(hay)
    }

    fn all_captures<'a, H: HaystackOf<'a, char>>(
        hay: &mut H,
        caps: &mut IndexedCaptures,
    ) -> Self::AllCaptures<'a, H> {
        A::all_captures(hay, caps)
    }
}

impl<I: HaystackItem, A: Matcher<I>> Debug for ZeroWidthCheck<I, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", A::default())
    }
}

#[derive(Debug, Clone, Hash)]
pub struct AllMatchesZeroWidthCheck<'a, H, A>
where
    H: HaystackOf<'a, u8>,
    A: Matcher<u8>
{
    hay: H,
    inner: A::AllMatches<'a, H>,
    _phantom: PhantomData<(&'a (), A)>
}

impl<'a, H, A> Iterator for AllMatchesZeroWidthCheck<'a, H, A>
where
    H: HaystackOf<'a, u8>,
    A: Matcher<u8>
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next();
        if check_item(
            next.and_then(|index| self.hay.rollback(index).item())
        ) {
            // TODO: Should Matcher require a fused iterator?
            // This forwards the case where next = None.
            next
        } else {
            self.next()
        }
    }
}

impl<'a, H, A> FusedIterator for AllCapturesZeroWidthCheck<'a, H, A>
where
    H: HaystackOf<'a, u8>,
    A: Matcher<u8>
{}

#[derive(Debug, Clone, Hash)]
pub struct AllCapturesZeroWidthCheck<'a, H, A>
where
    H: HaystackOf<'a, u8>,
    A: Matcher<u8>
{
    hay: H,
    inner: A::AllCaptures<'a, H>,
    _phantom: PhantomData<(&'a (), A)>
}

impl<'a, H, A> Iterator for AllCapturesZeroWidthCheck<'a, H, A>
where
    H: HaystackOf<'a, u8>,
    A: Matcher<u8>
{
    type Item = (usize, IndexedCaptures);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next();
        if check_item(
            next.as_ref()
                .and_then(|(index, _)| self.hay.rollback(*index).item())
        ) {
            // This forwards the case where next = None.
            next
        } else {
            self.next()
        }
    }
}

impl<'a, H, A> FusedIterator for AllMatchesZeroWidthCheck<'a, H, A>
where
    H: HaystackOf<'a, u8>,
    A: Matcher<u8>
{}

