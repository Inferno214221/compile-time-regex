use crate::expr::IndexedCaptures;
use crate::haystack::{Haystack, IntoHaystack};

use super::*;

/// Macro to test primitive successful matches and default implementations for Matcher methods.
///
/// # Arguments
/// ```ignore
/// test_matches!(pattern, hay+progress?, index?)
/// ```
///
/// # Generates
/// Code to test the following functionality:
/// - `pattern` matches the haystack when starting at `progress`, leaving the haystack at `index`.
/// - `pattern::captures` matches exactly the same as `pattern::matches` without performing any
/// capturing.
/// - `pattern::all_matches` and `pattern::all_captures` produce a single value which is `index`.
#[macro_export]
macro_rules! test_matches {
    ($pattern:ty, $hay:literal, $index:literal) => {
        test_matches!($pattern, $hay+0, $index)
    };
    ($pattern:ty, $hay:literal+$progress:literal) => {
        test_matches!($pattern, $hay+$progress, $progress)
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

        assert_eq!(
            <$pattern>::all_matches(&mut hay.clone()),
            vec![$index]
        );
        assert_eq!(
            <$pattern>::all_captures(&mut hay.clone(), &mut caps.clone()),
            vec![($index, caps)]
        );
    };
}

/// Macro to test primitive unsuccessful matches and default implementations for Matcher methods.
///
/// # Arguments
/// ```ignore
/// test_no_matches!(pattern, hay+progress?, index?)
/// ```
///
/// # Generates
/// Code to test the following functionality:
/// - `pattern` doesn't matches the haystack when starting at `progress`, leaving the haystack at
/// `index`. Note that in general, `Matcher` doesn't specify where the index of the haystack should
/// sit after a failed match but for primitives, we're testing it anyway.
/// - `pattern::all_matches` and `pattern::all_captures` produce no values.
#[macro_export]
macro_rules! test_no_matches {
    ($pattern:ty, $hay:literal, $index:literal) => {
        test_no_matches!($pattern, $hay+0, $index)
    };
    ($pattern:ty, $hay:literal+$progress:literal) => {
        test_no_matches!($pattern, $hay+$progress, $progress)
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

        assert_eq!(<$pattern>::all_matches(&mut hay.clone()), vec![]);
        assert_eq!(<$pattern>::all_captures(&mut hay.clone(), &mut caps.clone()), vec![]);
    };
}

mod byte {
    use super::*;

    type ByteA = Byte<b'a'>;

    #[test]
    fn correct_match() {
        test_matches!(ByteA, b"a", 1);
    }

    #[test]
    fn incorrect_doesnt_match_or_progress() {
        test_no_matches!(ByteA, b"b", 0);
    }

    #[test]
    fn empty_doesnt_match_or_progress() {
        test_no_matches!(ByteA, b"", 0);
    }
}

mod byte_range {
    use super::*;

    type ByteRangeAC = ByteRange<b'a', b'c'>;

    #[test]
    fn correct_match() {
        test_matches!(ByteRangeAC, b"b", 1);
    }

    #[test]
    fn bounds_match() {
        test_matches!(ByteRangeAC, b"a", 1);
        test_matches!(ByteRangeAC, b"c", 1);
    }

    #[test]
    fn incorrect_match() {
        test_no_matches!(ByteRangeAC, b"d", 0);
    }

    #[test]
    fn empty_doesnt_match_or_progress() {
        test_no_matches!(ByteRangeAC, b"", 0);
    }
}

mod scalar {
    use super::*;

    type ScalarA = Scalar<'a'>;

    #[test]
    fn correct_match() {
        test_matches!(ScalarA, "a", 1);
    }

    #[test]
    fn incorrect_doesnt_match_or_progress() {
        test_no_matches!(ScalarA, "b", 0);
    }

    #[test]
    fn empty_doesnt_match_or_progress() {
        test_no_matches!(ScalarA, "", 0);
    }
}

mod scalar_range {
    use super::*;

    type ScalarRangeAC = ScalarRange<'a', 'c'>;

    #[test]
    fn correct_match() {
        test_matches!(ScalarRangeAC, "b", 1);
    }

    #[test]
    fn bounds_match() {
        test_matches!(ScalarRangeAC, "a", 1);
        test_matches!(ScalarRangeAC, "c", 1);
    }

    #[test]
    fn incorrect_doesnt_match() {
        test_no_matches!(ScalarRangeAC, "d", 0);
    }

    #[test]
    fn empty_doesnt_match() {
        test_no_matches!(ScalarRangeAC, "", 0);
    }
}

mod always {
    use super::*;

    #[test]
    fn full_match() {
        test_matches!(Always, "a", 0);
    }

    #[test]
    fn empty_match() {
        test_matches!(Always, "", 0);
    }
}

mod start {
    use super::*;

    #[test]
    fn correct_match() {
        test_matches!(Start, "ab", 0);
    }

    #[test]
    fn incorrect_doesnt_match() {
        test_no_matches!(Start, "ab"+1);
    }

    #[test]
    fn empty_match() {
        test_matches!(Start, "", 0);
    }
}

mod end {
    use super::*;

    #[test]
    fn correct_match() {
        test_matches!(End, "a"+1);
    }

    #[test]
    fn incorrect_doesnt_match() {
        test_no_matches!(End, "a", 0);
    }

    #[test]
    fn empty_match() {
        test_matches!(End, "", 0);
    }
}

mod line_start {
    use super::*;

    #[test]
    fn first_pos_match() {
        test_matches!(LineStart, "", 0);
        test_matches!(LineStart, "a", 0);
    }

    #[test]
    fn post_newline_match() {
        test_matches!(LineStart, "a\n"+2);
        test_matches!(LineStart, "a\nb"+2);
    }

    #[test]
    fn non_first_pos_doesnt_match() {
        test_no_matches!(LineStart, "a"+1);
        test_no_matches!(LineStart, "ab"+1);
    }

    #[test]
    fn newline_adjacent_doesnt_match() {
        test_no_matches!(LineStart, "a\nb"+1);
        test_no_matches!(LineStart, "a\nb"+3);
    }

    #[test]
    fn post_return_doesnt_match() {
        test_no_matches!(LineStart, "a\r"+2);
        test_no_matches!(LineStart, "a\rb"+2);
    }
}

mod line_end {
    use super::*;

    #[test]
    fn last_pos_match() {
        test_matches!(LineEnd, ""+0);
        test_matches!(LineEnd, "a"+1);
    }

    #[test]
    fn pre_newline_match() {
        test_matches!(LineEnd, "\n"+0);
        test_matches!(LineEnd, "a\n"+1);
    }

    #[test]
    fn non_last_pos_doesnt_match() {
        test_no_matches!(LineEnd, "a"+0);
        test_no_matches!(LineEnd, "ab"+1);
    }

    #[test]
    fn newline_adjacent_doesnt_match() {
        test_no_matches!(LineEnd, "a\nb"+0);
        test_no_matches!(LineEnd, "a\nb"+2);
    }

    #[test]
    fn pre_return_doesnt_match() {
        test_no_matches!(LineEnd, "\r"+0);
        test_no_matches!(LineEnd, "a\rb"+1);
    }
}

mod crlf_start {
    use super::*;

    #[test]
    fn first_pos_match() {
        test_matches!(CRLFStart, "", 0);
        test_matches!(CRLFStart, "a", 0);
    }

    #[test]
    fn post_newline_match() {
        test_matches!(CRLFStart, "a\n"+2);
        test_matches!(CRLFStart, "a\nb"+2);
    }

    #[test]
    fn post_return_match() {
        test_matches!(CRLFStart, "a\r"+2);
        test_matches!(CRLFStart, "a\rb"+2);
    }

    #[test]
    fn post_return_newline_match() {
        test_matches!(CRLFStart, "a\r\n"+3);
        test_matches!(CRLFStart, "a\r\nb"+3);
    }

    #[test]
    fn non_first_pos_doesnt_match() {
        test_no_matches!(CRLFStart, "a"+1);
        test_no_matches!(CRLFStart, "ab"+1);
    }

    #[test]
    fn newline_adjacent_doesnt_match() {
        test_no_matches!(CRLFStart, "a\nb"+1);
        test_no_matches!(CRLFStart, "a\nb"+3);
    }

    #[test]
    fn return_adjacent_doesnt_match() {
        test_no_matches!(CRLFStart, "a\rb"+1);
        test_no_matches!(CRLFStart, "a\rb"+3);
    }

    #[test]
    fn post_return_pre_newline_match() {
        test_no_matches!(CRLFStart, "a\r\n"+2);
        test_no_matches!(CRLFStart, "a\r\nb"+2);
    }
}

mod crlf_end {
    use super::*;

    #[test]
    fn last_pos_match() {
        test_matches!(CRLFEnd, ""+0);
        test_matches!(CRLFEnd, "a"+1);
    }

    #[test]
    fn pre_newline_match() {
        test_matches!(CRLFEnd, "\n"+0);
        test_matches!(CRLFEnd, "a\n"+1);
    }

    #[test]
    fn pre_return_match() {
        test_matches!(CRLFEnd, "a\r"+1);
        test_matches!(CRLFEnd, "a\rb"+1);
    }

    #[test]
    fn pre_return_newline_match() {
        test_matches!(CRLFEnd, "a\r\n"+1);
        test_matches!(CRLFEnd, "a\r\nb"+1);
    }

    #[test]
    fn non_last_pos_doesnt_match() {
        test_no_matches!(CRLFEnd, "a"+0);
        test_no_matches!(CRLFEnd, "ab"+1);
    }

    #[test]
    fn newline_adjacent_doesnt_match() {
        test_no_matches!(CRLFEnd, "a\nb"+0);
        test_no_matches!(CRLFEnd, "a\nb"+2);
    }

    #[test]
    fn return_adjacent_doesnt_match() {
        test_no_matches!(CRLFEnd, "a\rb"+0);
        test_no_matches!(CRLFEnd, "a\rb"+2);
    }

    #[test]
    fn post_return_pre_newline_match() {
        test_no_matches!(CRLFEnd, "a\r\n"+2);
        test_no_matches!(CRLFEnd, "a\r\nb"+2);
    }
}