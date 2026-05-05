use crate::expr::IndexedCaptures;
use crate::haystack::{Haystack, IntoHaystack};

use super::*;

/// Macro to test primitive successful matches and default implementations for Matcher methods.
///
/// # Arguments
/// ```ignore
/// test_matches_with_index!(pattern, hay+progress?, index?)
/// ```
///
/// # Generates
/// Code to test the following functionality:
/// - `pattern` matches the haystack when starting at `progress`, leaving the haystack at `index`.
/// - `pattern::captures` matches exactly the same as `pattern::matches` without performing any
///   capturing.
/// - `pattern::all_matches` and `pattern::all_captures` produce a single value which is `index`.
#[macro_export]
macro_rules! test_matches_with_index {
    ($pattern:ty, $hay:literal, $index:literal) => {
        test_matches_with_index!($pattern, $hay+0, $index)
    };
    ($pattern:ty, $hay:literal+$progress:literal) => {
        test_matches_with_index!($pattern, $hay+$progress, $progress)
    };
    ($pattern:ty, $hay:literal+$progress:literal, $index:literal) => {
        let mut hay = $hay.into_haystack();
        hay.rollback($progress);

        let mut hay_match = hay.clone();
        let mut hay_capture = hay_match.clone();
        let caps = IndexedCaptures::default();
        let mut caps_capture = caps.clone();

        assert!(<$pattern>::matches(&mut hay_match));
        assert!(<$pattern>::captures(&mut hay_capture, &mut caps_capture));

        assert_eq!(caps_capture, caps);

        assert_eq!(hay_match.index(), $index);
        assert_eq!(hay_capture.index(), $index);

        assert!(<$pattern>::all_matches(&mut hay.clone()).eq(vec![$index]));
        assert!(
            <$pattern>::all_captures(&mut hay.clone(), &mut caps.clone()).eq(vec![($index, caps)])
        );
    };
}

/// Macro to test primitive unsuccessful matches and default implementations for Matcher methods.
///
/// # Arguments
/// ```ignore
/// test_doesnt_match_with_index!(pattern, hay+progress?, index?)
/// ```
///
/// # Generates
/// Code to test the following functionality:
/// - `pattern` doesn't matches the haystack when starting at `progress`, leaving the haystack at
///   `index`. Note that in general, `Matcher` doesn't specify where the index of the haystack
///   should sit after a failed match but for primitives, we're testing it anyway.
/// - `pattern::all_matches` and `pattern::all_captures` produce no values.
#[macro_export]
macro_rules! test_doesnt_match_with_index {
    ($pattern:ty, $hay:literal, $index:literal) => {
        test_doesnt_match_with_index!($pattern, $hay+0, $index)
    };
    ($pattern:ty, $hay:literal+$progress:literal) => {
        test_doesnt_match_with_index!($pattern, $hay+$progress, $progress)
    };
    ($pattern:ty, $hay:literal+$progress:literal, $index:literal) => {
        let mut hay = $hay.into_haystack();
        hay.rollback($progress);

        let mut hay_match = hay.clone();
        let mut hay_capture = hay_match.clone();
        let caps = IndexedCaptures::default();
        let mut caps_capture = caps.clone();

        assert!(!<$pattern>::matches(&mut hay_match));
        assert!(!<$pattern>::captures(&mut hay_capture, &mut caps_capture));

        assert_eq!(hay_match.index(), $index);
        assert_eq!(hay_capture.index(), $index);

        assert!(<$pattern>::all_matches(&mut hay.clone()).eq(vec![]));
        assert!(<$pattern>::all_captures(&mut hay.clone(), &mut caps.clone()).eq(vec![]));
    };
}

type ByteA = Byte<b'a'>;
type ByteRangeAC = ByteRange<b'a', b'c'>;

type ScalarA = Scalar<'a'>;
type ScalarRangeAC = ScalarRange<'a', 'c'>;

mod byte {
    use super::*;

    #[test]
    fn correct_match() {
        test_matches_with_index!(ByteA, b"a", 1);
    }

    #[test]
    fn incorrect_doesnt_match_or_progress() {
        test_doesnt_match_with_index!(ByteA, b"b", 0);
    }

    #[test]
    fn empty_doesnt_match_or_progress() {
        test_doesnt_match_with_index!(ByteA, b"", 0);
    }
}

mod byte_range {
    use super::*;

    #[test]
    fn correct_match() {
        test_matches_with_index!(ByteRangeAC, b"b", 1);
    }

    #[test]
    fn bounds_match() {
        test_matches_with_index!(ByteRangeAC, b"a", 1);
        test_matches_with_index!(ByteRangeAC, b"c", 1);
    }

    #[test]
    fn incorrect_match() {
        test_doesnt_match_with_index!(ByteRangeAC, b"d", 0);
    }

    #[test]
    fn empty_doesnt_match_or_progress() {
        test_doesnt_match_with_index!(ByteRangeAC, b"", 0);
    }
}

mod scalar {
    use super::*;

    #[test]
    fn correct_match() {
        test_matches_with_index!(ScalarA, "a", 1);
    }

    #[test]
    fn incorrect_doesnt_match_or_progress() {
        test_doesnt_match_with_index!(ScalarA, "b", 0);
    }

    #[test]
    fn empty_doesnt_match_or_progress() {
        test_doesnt_match_with_index!(ScalarA, "", 0);
    }
}

mod scalar_range {
    use super::*;

    #[test]
    fn correct_match() {
        test_matches_with_index!(ScalarRangeAC, "b", 1);
    }

    #[test]
    fn bounds_match() {
        test_matches_with_index!(ScalarRangeAC, "a", 1);
        test_matches_with_index!(ScalarRangeAC, "c", 1);
    }

    #[test]
    fn incorrect_doesnt_match() {
        test_doesnt_match_with_index!(ScalarRangeAC, "d", 0);
    }

    #[test]
    fn empty_doesnt_match() {
        test_doesnt_match_with_index!(ScalarRangeAC, "", 0);
    }
}

mod always {
    use super::*;

    #[test]
    fn full_match() {
        test_matches_with_index!(Always, "a", 0);
    }

    #[test]
    fn empty_match() {
        test_matches_with_index!(Always, "", 0);
    }
}

mod start {
    use super::*;

    #[test]
    fn correct_match() {
        test_matches_with_index!(Start, "ab", 0);
    }

    #[test]
    fn incorrect_doesnt_match() {
        test_doesnt_match_with_index!(Start, "ab"+1);
    }

    #[test]
    fn empty_match() {
        test_matches_with_index!(Start, "", 0);
    }
}

mod end {
    use super::*;

    #[test]
    fn correct_match() {
        test_matches_with_index!(End, "a"+1);
    }

    #[test]
    fn incorrect_doesnt_match() {
        test_doesnt_match_with_index!(End, "a", 0);
    }

    #[test]
    fn empty_match() {
        test_matches_with_index!(End, "", 0);
    }
}

mod line_start {
    use super::*;

    #[test]
    fn first_pos_match() {
        test_matches_with_index!(LineStart, "", 0);
        test_matches_with_index!(LineStart, "a", 0);
    }

    #[test]
    fn post_newline_match() {
        test_matches_with_index!(LineStart, "a\n"+2);
        test_matches_with_index!(LineStart, "a\nb"+2);
    }

    #[test]
    fn non_first_pos_doesnt_match() {
        test_doesnt_match_with_index!(LineStart, "a"+1);
        test_doesnt_match_with_index!(LineStart, "ab"+1);
    }

    #[test]
    fn newline_adjacent_doesnt_match() {
        test_doesnt_match_with_index!(LineStart, "a\nb"+1);
        test_doesnt_match_with_index!(LineStart, "a\nb"+3);
    }

    #[test]
    fn post_return_doesnt_match() {
        test_doesnt_match_with_index!(LineStart, "a\r"+2);
        test_doesnt_match_with_index!(LineStart, "a\rb"+2);
    }
}

mod line_end {
    use super::*;

    #[test]
    fn last_pos_match() {
        test_matches_with_index!(LineEnd, ""+0);
        test_matches_with_index!(LineEnd, "a"+1);
    }

    #[test]
    fn pre_newline_match() {
        test_matches_with_index!(LineEnd, "\n"+0);
        test_matches_with_index!(LineEnd, "a\n"+1);
    }

    #[test]
    fn non_last_pos_doesnt_match() {
        test_doesnt_match_with_index!(LineEnd, "a"+0);
        test_doesnt_match_with_index!(LineEnd, "ab"+1);
    }

    #[test]
    fn newline_adjacent_doesnt_match() {
        test_doesnt_match_with_index!(LineEnd, "a\nb"+0);
        test_doesnt_match_with_index!(LineEnd, "a\nb"+2);
    }

    #[test]
    fn pre_return_doesnt_match() {
        test_doesnt_match_with_index!(LineEnd, "\r"+0);
        test_doesnt_match_with_index!(LineEnd, "a\rb"+1);
    }
}

mod crlf_start {
    use super::*;

    #[test]
    fn first_pos_match() {
        test_matches_with_index!(CRLFStart, "", 0);
        test_matches_with_index!(CRLFStart, "a", 0);
    }

    #[test]
    fn post_newline_match() {
        test_matches_with_index!(CRLFStart, "a\n"+2);
        test_matches_with_index!(CRLFStart, "a\nb"+2);
    }

    #[test]
    fn post_return_match() {
        test_matches_with_index!(CRLFStart, "a\r"+2);
        test_matches_with_index!(CRLFStart, "a\rb"+2);
    }

    #[test]
    fn post_return_newline_match() {
        test_matches_with_index!(CRLFStart, "a\r\n"+3);
        test_matches_with_index!(CRLFStart, "a\r\nb"+3);
    }

    #[test]
    fn non_first_pos_doesnt_match() {
        test_doesnt_match_with_index!(CRLFStart, "a"+1);
        test_doesnt_match_with_index!(CRLFStart, "ab"+1);
    }

    #[test]
    fn newline_adjacent_doesnt_match() {
        test_doesnt_match_with_index!(CRLFStart, "a\nb"+1);
        test_doesnt_match_with_index!(CRLFStart, "a\nb"+3);
    }

    #[test]
    fn return_adjacent_doesnt_match() {
        test_doesnt_match_with_index!(CRLFStart, "a\rb"+1);
        test_doesnt_match_with_index!(CRLFStart, "a\rb"+3);
    }

    #[test]
    fn post_return_pre_newline_match() {
        test_doesnt_match_with_index!(CRLFStart, "a\r\n"+2);
        test_doesnt_match_with_index!(CRLFStart, "a\r\nb"+2);
    }
}

mod crlf_end {
    use super::*;

    #[test]
    fn last_pos_match() {
        test_matches_with_index!(CRLFEnd, ""+0);
        test_matches_with_index!(CRLFEnd, "a"+1);
    }

    #[test]
    fn pre_newline_match() {
        test_matches_with_index!(CRLFEnd, "\n"+0);
        test_matches_with_index!(CRLFEnd, "a\n"+1);
    }

    #[test]
    fn pre_return_match() {
        test_matches_with_index!(CRLFEnd, "a\r"+1);
        test_matches_with_index!(CRLFEnd, "a\rb"+1);
    }

    #[test]
    fn pre_return_newline_match() {
        test_matches_with_index!(CRLFEnd, "a\r\n"+1);
        test_matches_with_index!(CRLFEnd, "a\r\nb"+1);
    }

    #[test]
    fn non_last_pos_doesnt_match() {
        test_doesnt_match_with_index!(CRLFEnd, "a"+0);
        test_doesnt_match_with_index!(CRLFEnd, "ab"+1);
    }

    #[test]
    fn newline_adjacent_doesnt_match() {
        test_doesnt_match_with_index!(CRLFEnd, "a\nb"+0);
        test_doesnt_match_with_index!(CRLFEnd, "a\nb"+2);
    }

    #[test]
    fn return_adjacent_doesnt_match() {
        test_doesnt_match_with_index!(CRLFEnd, "a\rb"+0);
        test_doesnt_match_with_index!(CRLFEnd, "a\rb"+2);
    }

    #[test]
    fn post_return_pre_newline_match() {
        test_doesnt_match_with_index!(CRLFEnd, "a\r\n"+2);
        test_doesnt_match_with_index!(CRLFEnd, "a\r\nb"+2);
    }
}

#[macro_export]
macro_rules! implements_debug {
    ($pattern:ty, $($more:ty),+) => {
        println!("{:?}", <$pattern>::default());
        implements_debug!($($more),+);
    };
    ($pattern:ty) => {
        println!("{:?}", <$pattern>::default());
    };
}

#[test]
fn implements_debug() {
    implements_debug!(
        ByteA,
        ByteRangeAC,
        ScalarA,
        ScalarRangeAC,
        Always,
        Start,
        End,
        LineStart,
        LineEnd,
        CRLFStart,
        CRLFEnd
    );
}