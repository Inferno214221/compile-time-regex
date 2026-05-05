use crate::expr::IndexedCaptures;
use crate::haystack::{Haystack, IntoHaystack};

use super::*;

use crate::{implements_debug, test_matches_with_index};

/// Macro to test unsuccessful matches and default implementations for Matcher methods.
///
/// # Arguments
/// ```ignore
/// test_doesnt_match_no_index!(pattern, hay+progress?)
/// ```
///
/// # Generates
/// Code to test the following functionality:
/// - `pattern` doesn't matches the haystack when starting at `progress`.
/// - `pattern::all_matches` and `pattern::all_captures` produce no values.
#[macro_export]
macro_rules! test_doesnt_match_no_index {
    ($pattern:ty, $hay:literal) => {
        test_doesnt_match_no_index!($pattern, $hay+0)
    };
    ($pattern:ty, $hay:literal+$progress:literal) => {
        let mut hay = $hay.into_haystack();
        hay.rollback($progress);

        let mut hay_match = hay.clone();
        let mut hay_capture = hay_match.clone();
        let caps = IndexedCaptures::default();
        let mut caps_capture = caps.clone();

        assert!(!<$pattern>::matches(&mut hay_match));
        assert!(!<$pattern>::captures(&mut hay_capture, &mut caps_capture));

        assert!(<$pattern>::all_matches(&mut hay.clone()).eq(vec![]));
        assert!(<$pattern>::all_captures(&mut hay.clone(), &mut caps.clone()).eq(vec![]));
    };
}

/// Macro to test successful matches with multiples haystack indices.
///
/// # Arguments
/// ```ignore
/// test_matches_with_indices!(pattern, hay+progress?, indices?)
/// ```
///
/// # Generates
/// Code to test the following functionality:
/// - `pattern` matches the haystack when starting at `progress`, leaving the haystack at the last
///   value of `indices`.
/// - `pattern::all_matches` and `pattern::all_captures` produce values equal to `indices`.
#[macro_export]
macro_rules! test_matches_with_indices {
    ($pattern:ty, $hay:literal, $indices:expr) => {
        test_matches_with_indices!($pattern, $hay+0, $indices)
    };
    ($pattern:ty, $hay:literal+$progress:literal) => {
        test_matches_with_indices!($pattern, $hay+$progress, vec![$progress])
    };
    ($pattern:ty, $hay:literal+$progress:literal, $indices:expr) => {
        let mut hay = $hay.into_haystack();
        hay.rollback($progress);

        let mut hay_match = hay.clone();
        let mut hay_capture = hay_match.clone();
        let caps = IndexedCaptures::default();
        let mut caps_capture = caps.clone();

        assert!(<$pattern>::matches(&mut hay_match));
        assert!(<$pattern>::captures(&mut hay_capture, &mut caps_capture));

        assert_eq!(caps_capture, caps);

        assert_eq!(hay_match.index(), *$indices.first().unwrap());
        assert_eq!(hay_capture.index(), *$indices.first().unwrap());

        assert!(<$pattern>::all_matches(&mut hay.clone()).eq($indices));
        assert!(
            <$pattern>::all_captures(&mut hay.clone(), &mut caps.clone()).eq(
                $indices.into_iter()
                    .zip(std::iter::repeat(caps))
            )
        );
    };
}

type ScalarA = Scalar<'a'>;

type QuantifierNA<const N: usize> = QuantifierN<char, ScalarA, N>;
type QuantifierNOrMoreA<const N: usize> = QuantifierNOrMore<char, ScalarA, N>;
type QuantifierNToMA<const N: usize, const M: usize> = QuantifierNToM<char, ScalarA, N, M>;

mod quantifier_n {
    use super::*;

    #[test]
    fn n_match() {
        test_matches_with_index!(QuantifierNA<0>, "", 0);
        test_matches_with_index!(QuantifierNA<1>, "a", 1);
        test_matches_with_index!(QuantifierNA<2>, "aa", 2);
        test_matches_with_index!(QuantifierNA<5>, "aaaaa", 5);
    }

    #[test]
    fn not_n_doesnt_match() {
        test_doesnt_match_no_index!(QuantifierNA<1>, "");
        test_doesnt_match_no_index!(QuantifierNA<2>, "a");
        test_doesnt_match_no_index!(QuantifierNA<2>, "aaa");
    }
}

mod quantifier_n_or_more {
    use super::*;

    #[test]
    fn n_match() {
        test_matches_with_index!(QuantifierNOrMoreA<0>, "", 0);
        test_matches_with_index!(QuantifierNOrMoreA<1>, "a", 1);
        test_matches_with_index!(QuantifierNOrMoreA<2>, "aa", 2);
        test_matches_with_index!(QuantifierNOrMoreA<5>, "aaaaa", 5);
    }

    #[test]
    fn more_than_n_match() {
        test_matches_with_indices!(QuantifierNOrMoreA<1>, "aa", vec![1, 2]);
        test_matches_with_indices!(QuantifierNOrMoreA<2>, "aaaaa", vec![2, 3, 4, 5]);
    }

    #[test]
    fn less_than_n_doesnt_match() {
        test_doesnt_match_no_index!(QuantifierNOrMoreA<1>, "");
        test_doesnt_match_no_index!(QuantifierNOrMoreA<2>, "a");
    }

    #[test]
    fn not_a_doesnt_match() {
        test_doesnt_match_no_index!(QuantifierNOrMoreA<1>, "b");
        test_doesnt_match_no_index!(QuantifierNOrMoreA<2>, "bb");
    }
}

mod quantifier_n_to_m {
    use super::*;

    type CapturingLetters = CaptureGroup<char, ScalarRange<'a', 'z'>, 0>;
    type QuantifierNToMCapturingLetters = QuantifierNToM<char, CapturingLetters, 1, 3>;

    #[test]
    fn n_to_m_match() {
        test_matches_with_indices!(QuantifierNToMA<1, 3>, "a", vec![1]);
        test_matches_with_indices!(QuantifierNToMA<1, 3>, "aa", vec![1, 2]);
        test_matches_with_indices!(QuantifierNToMA<1, 3>, "aaa", vec![1, 2, 3]);
        test_matches_with_indices!(QuantifierNToMA<0, 1>, "", vec![0]);
        test_matches_with_indices!(QuantifierNToMA<0, 1>, "a", vec![0, 1]);
    }

    #[test]
    fn n_eq_m_match() {
        test_matches_with_index!(QuantifierNToMA<0, 0>, "", 0);
        test_matches_with_index!(QuantifierNToMA<0, 0>, "a", 0);
        test_matches_with_index!(QuantifierNToMA<1, 1>, "a", 1);
        test_matches_with_index!(QuantifierNToMA<1, 1>, "aa", 1);
    }

    #[test]
    fn more_than_m_restricted_match() {
        test_matches_with_indices!(QuantifierNToMA<0, 1>, "aa", vec![0, 1]);
        test_matches_with_indices!(QuantifierNToMA<1, 3>, "aaaa", vec![1, 2, 3]);
    }

    #[test]
    fn more_than_m_restricted_capture() {
        let mut hay = "abcd".into_haystack();
        let mut caps = IndexedCaptures::default();

        assert!(QuantifierNToMCapturingLetters::captures(&mut hay, &mut caps));
        let cap_1 = caps.into_array::<1>()[0].clone().unwrap();
        assert_eq!(hay.slice_with(cap_1), "c");
    }

    #[test]
    fn not_a_doesnt_match() {
        test_doesnt_match_no_index!(QuantifierNToMA<1, 2>, "b");
        test_doesnt_match_no_index!(QuantifierNToMA<1, 2>, "bb");
    }
}

#[test]
fn implements_debug() {
    implements_debug!(
        QuantifierNA<1>,
        QuantifierNOrMoreA<1>,
        QuantifierNToMA<1, 2>
    );
}