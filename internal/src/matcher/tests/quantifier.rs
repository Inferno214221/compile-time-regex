use crate::expr::IndexedCaptures;
use crate::haystack::{Haystack, IntoHaystack};

use super::*;

use crate::test_matches;

// TODO: Rename to indicate no index checking
/// Macro to test unsuccessful matches and default implementations for Matcher methods.
///
/// # Arguments
/// ```ignore
/// test_no_matches!(pattern, hay+progress?)
/// ```
///
/// # Generates
/// Code to test the following functionality:
/// - `pattern` doesn't matches the haystack when starting at `progress`.
/// - `pattern::all_matches` and `pattern::all_captures` produce no values.
macro_rules! test_no_matches {
    ($pattern:ty, $hay:literal) => {
        test_no_matches!($pattern, $hay+0)
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

        assert_eq!(<$pattern>::all_matches(&mut hay.clone()), vec![]);
        assert_eq!(<$pattern>::all_captures(&mut hay.clone(), &mut caps.clone()), vec![]);
    };
}

/// Macro to test successful matches with multiples hastack indicies.
///
/// # Arguments
/// ```ignore
/// test_matches_multiple!(pattern, hay+progress?, indices?)
/// ```
///
/// # Generates
/// Code to test the following functionality:
/// - `pattern` matches the haystack when starting at `progress`, leaving the haystack at the last
/// value of `indices`.
/// - `pattern::all_matches` and `pattern::all_captures` produce values equal to `indices`.
macro_rules! test_matches_multiple {
    ($pattern:ty, $hay:literal, $indices:expr) => {
        test_matches_multiple!($pattern, $hay+0, $indices)
    };
    ($pattern:ty, $hay:literal+$progress:literal) => {
        test_matches_multiple!($pattern, $hay+$progress, vec![$progress])
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

        assert_eq!(hay_match.index(), *$indices.last().unwrap());
        assert_eq!(hay_capture.index(), *$indices.last().unwrap());

        assert_eq!(
            <$pattern>::all_matches(&mut hay.clone()), $indices
        );
        assert_eq!(
            <$pattern>::all_captures(&mut hay.clone(), &mut caps.clone()),
            $indices.into_iter()
                .zip(std::iter::repeat(caps))
                .collect::<Vec<_>>()
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
        test_matches!(QuantifierNA<0>, "", 0);
        test_matches!(QuantifierNA<1>, "a", 1);
        test_matches!(QuantifierNA<2>, "aa", 2);
        test_matches!(QuantifierNA<5>, "aaaaa", 5);
    }

    #[test]
    fn not_n_doesnt_match() {
        test_no_matches!(QuantifierNA<1>, "");
        test_no_matches!(QuantifierNA<2>, "a");
        test_no_matches!(QuantifierNA<2>, "aaa");
    }
}

mod quantifier_n_or_more {
    use super::*;

    #[test]
    fn n_match() {
        test_matches!(QuantifierNOrMoreA<0>, "", 0);
        test_matches!(QuantifierNOrMoreA<1>, "a", 1);
        test_matches!(QuantifierNOrMoreA<2>, "aa", 2);
        test_matches!(QuantifierNOrMoreA<5>, "aaaaa", 5);
    }

    #[test]
    fn more_than_n_match() {
        test_matches_multiple!(QuantifierNOrMoreA<1>, "aa", vec![1, 2]);
        test_matches_multiple!(QuantifierNOrMoreA<2>, "aaaaa", vec![2, 3, 4, 5]);
    }

    #[test]
    fn less_than_n_doesnt_match() {
        test_no_matches!(QuantifierNOrMoreA<1>, "");
        test_no_matches!(QuantifierNOrMoreA<2>, "a");
    }

    #[test]
    fn not_a_doesnt_match() {
        test_no_matches!(QuantifierNOrMoreA<1>, "b");
        test_no_matches!(QuantifierNOrMoreA<2>, "bb");
    }
}

mod quantifier_n_to_m {
    use super::*;

    #[test]
    fn n_to_m_match() {
        test_matches_multiple!(QuantifierNToMA<1, 3>, "a", vec![1]);
        test_matches_multiple!(QuantifierNToMA<1, 3>, "aa", vec![1, 2]);
        test_matches_multiple!(QuantifierNToMA<1, 3>, "aaa", vec![1, 2, 3]);
        test_matches_multiple!(QuantifierNToMA<0, 1>, "", vec![0]);
        test_matches_multiple!(QuantifierNToMA<0, 1>, "a", vec![0, 1]);
    }

    #[test]
    fn n_eq_m_match() {
        test_matches!(QuantifierNToMA<0, 0>, "", 0);
        test_matches!(QuantifierNToMA<0, 0>, "a", 0);
        test_matches!(QuantifierNToMA<1, 1>, "a", 1);
        test_matches!(QuantifierNToMA<1, 1>, "aa", 1);
    }

    #[test]
    fn more_than_m_restricted_match() {
        test_matches_multiple!(QuantifierNToMA<0, 1>, "aa", vec![0, 1]);
        test_matches_multiple!(QuantifierNToMA<1, 3>, "aaaa", vec![1, 2, 3]);
    }

    #[test]
    fn more_than_m_restricted_capture() {
        let mut hay = "abcd".into_haystack();
        let mut caps = IndexedCaptures::default();

        type CapturingLetters = CaptureGroup<char, ScalarRange<'a', 'z'>, 0>;
        type QuantifierNToMCapturingLetters = QuantifierNToM<char, CapturingLetters, 1, 3>;

        assert!(QuantifierNToMCapturingLetters::captures(&mut hay, &mut caps));
        let cap_1 = caps.into_array::<1>()[0].clone().unwrap();
        assert_eq!(hay.slice_with(cap_1), "c");
    }

    #[test]
    fn not_a_doesnt_match() {
        test_no_matches!(QuantifierNToMA<1, 2>, "b");
        test_no_matches!(QuantifierNToMA<1, 2>, "bb");
    }
}

mod quantifer_then {
    use super::*;

    #[test]
    fn performs_rollback_match() {
        test_no_matches!(QuantifierThen<_, QuantifierNOrMoreA<2>, ScalarA>, "aa");
        test_matches!(QuantifierThen<_, QuantifierNOrMoreA<2>, ScalarA>, "aaa", 3);
        test_matches_multiple!(QuantifierThen<_, QuantifierNOrMoreA<2>, ScalarA>, "aaaa", vec![3, 4]);
    }

    #[test]
    fn quadratic_all_matches() {
        type QuadraticA2 = QuantifierThen<char, QuantifierNOrMoreA<2>, QuantifierNOrMoreA<2>>;

        test_matches_multiple!(QuadraticA2, "aaaa", vec![4]);
        test_matches_multiple!(QuadraticA2, "aaaaa", vec![4, 5, 5]);
    }

    type Quantifier2OrMore<A> = QuantifierNOrMore<char, A, 2>;
    type Capturing2OrMoreLetters = CaptureGroup<char, Quantifier2OrMore<ScalarRange<'a', 'z'>>, 0>;
    type Capturing2OrMoreA = CaptureGroup<char, Quantifier2OrMore<ScalarA>, 1>;
    type QuadraticLetterOrA = QuantifierThen<char, Capturing2OrMoreLetters, Capturing2OrMoreA>;

    #[test]
    fn captures_prefers_first() {
        let mut hay = "bbaaa".into_haystack();
        let mut caps = IndexedCaptures::default();

        assert!(QuadraticLetterOrA::captures(&mut hay, &mut caps));
        let [cap_1, cap_2] = caps.into_array::<2>();
        assert_eq!(hay.slice_with(cap_1.unwrap()), "bba");
        assert_eq!(hay.slice_with(cap_2.unwrap()), "aa");
    }

    #[test]
    fn captures_prioritised_correctly() {
        let expected_caps = [
            ["bb", "aa"],
            ["bb", "aaa"],
            ["bba", "aa"]
        ];

        let mut hay = "bbaaa".into_haystack();
        let mut caps = IndexedCaptures::default();

        let all_caps = QuadraticLetterOrA::all_captures(&mut hay, &mut caps)
            .into_iter()
            .map(|(_, caps)| caps);

        for (index, caps) in all_caps.enumerate() {
            let caps_array = caps.into_array::<2>()
                .into_iter()
                .map(|cap_range| hay.slice_with(cap_range.unwrap()))
                .collect::<Vec<_>>();
            assert_eq!(caps_array, expected_caps[index]);
        }
    }
}