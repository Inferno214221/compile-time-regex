use crate::expr::IndexedCaptures;
use crate::haystack::{Haystack, IntoHaystack};

use super::*;

use crate::{implements_debug, test_doesnt_match_with_index};

type ScalarA = Scalar<'a'>;
type ScalarB = Scalar<'b'>;
type ScalarC = Scalar<'c'>;

/// Macro to test primitive successful matches with the provided `indices` and `caps`.
///
/// # Arguments
/// ```ignore
/// test_matches_with_captures!(pattern, hay, caps)
/// ```
///
/// # Generates
/// Code to test the following functionality:
/// - `pattern` matches the haystack when starting at `progress`, leaving the haystack at the last
///   value of `indices`.
/// - `pattern::all_matches` and `pattern::all_captures` produce values equal to `indices` and
///   `caps`.
#[macro_export]
macro_rules! test_matches_with_captures {
    ($pattern:ty, $hay:literal, $indices:expr, $caps:expr) => {
        let hay = $hay.into_haystack();

        let mut hay_match = hay.clone();
        let mut hay_capture = hay_match.clone();
        let expected_caps = $caps.last().unwrap().clone();
        let mut real_caps = IndexedCaptures::default();

        assert!(<$pattern>::matches(&mut hay_match));
        assert!(<$pattern>::captures(&mut hay_capture, &mut real_caps));

        assert_eq!(real_caps, expected_caps);

        assert_eq!(hay_match.index(), *$indices.last().unwrap());
        assert_eq!(hay_capture.index(), *$indices.last().unwrap());

        assert!(<$pattern>::all_matches(&mut hay.clone()).eq($indices));
        assert_eq!(
            <$pattern>::all_captures(&mut hay.clone(), &mut IndexedCaptures::default()),
            $indices.into_iter()
                .zip($caps)
                .collect::<Vec<_>>()
        );
    };
}

/// A vararg constructor for IndexedCaptures.
macro_rules! caps {
    () => {
        IndexedCaptures::default()
    };
    ($(($indices:literal, $ranges:expr)),+) => {
        {
            let mut caps = IndexedCaptures::default();
            caps!(caps, $(($indices, $ranges)),+);
            caps
        }
    };
    ($caps:expr, ($index:literal, $range:expr)) => {
        {
            $caps.push($index, $range);
        }
    };
    ($caps:expr, ($index:literal, $range:expr), $(($indices:literal, $ranges:expr)),+) => {
        {
            $caps.push($index, $range);
            caps!($caps, $(($indices, $ranges)),+);
        }
    };
}

#[allow(clippy::module_inception)]
mod capture {
    use super::*;

    type CapA0 = CaptureGroup<char, ScalarA, 0>;
    type CapB1 = CaptureGroup<char, ScalarB, 1>;
    type CapC2 = CaptureGroup<char, ScalarC, 2>;

    #[test]
    fn correct_match() {
        test_matches_with_captures!(CapA0, "a", vec![1], vec![caps![(0, 0..1)]]);
    }

    #[test]
    fn incorrect_match() {
        test_doesnt_match_with_index!(CapA0, "b", 0);
    }

    #[test]
    fn capture_to_correct_index() {
        test_matches_with_captures!(
            QuantifierNOrMore<_, Or<_, Or<_, CapA0, CapB1>, CapC2>, 1>,
            "acb",
            vec![1, 2, 3],
            vec![
                caps![(0, 0..1)],
                caps![(0, 0..1), (2, 1..2)],
                caps![(0, 0..1), (2, 1..2), (1, 2..3)]
            ]
        );
    }

    #[test]
    fn capture_prefers_last() {
        test_matches_with_captures!(
            QuantifierN<_, CapA0, 3>,
            "aaa",
            vec![3],
            vec![caps![(0, 0..1), (0, 1..2), (0, 2..3)]]
        );
        let mut hay = "aaa".into_haystack();
        let mut caps = IndexedCaptures::default();

        assert!(<QuantifierN<char, CapA0, 3>>::captures(&mut hay, &mut caps));
        let [cap] = caps.into_array::<1>();
        assert_eq!(cap, Some(2..3));
    }
}

#[test]
fn implements_debug() {
    implements_debug!(CaptureGroup<char, ScalarA, 0>);
}