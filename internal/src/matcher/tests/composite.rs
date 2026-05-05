use crate::expr::IndexedCaptures;
use crate::haystack::{Haystack, IntoHaystack};

use super::*;

use crate::{implements_debug, test_doesnt_match_no_index, test_matches_with_index, test_matches_with_indices};

type ScalarA = Scalar<'a'>;
type ScalarB = Scalar<'b'>;
type ScalarC = Scalar<'c'>;
type ScalarD = Scalar<'d'>;

type RangeAB = ScalarRange<'a', 'b'>;

type AOrB = Or<char, ScalarA, ScalarB>;
type AThenB = Then<char, ScalarA, ScalarB>;

type QuantifierNOrMoreA<const N: usize> = QuantifierNOrMore<char, ScalarA, N>;

mod or {
    use super::*;

    #[test]
    fn a_or_b_match() {
        test_matches_with_index!(AOrB, "a", 1);
        test_matches_with_index!(AOrB, "b", 1);
        test_matches_with_indices!(QuantifierNOrMore<_, AOrB, 1>, "ab", vec![1, 2]);
    }

    #[test]
    fn rollback_as_required() {
        test_matches_with_index!(
            Then<_, QuantifierNOrMore<_, AOrB, 1>, ScalarB>,
            "ab",
            2
        );
    }

    #[test]
    fn captures_prefers_first() {
        // Despite the second match being longer, the first is preferred and filtered out later if
        // required.
        test_matches_with_indices!(Or<_, AThenB, Then<_, AThenB, ScalarB>>, "abb", vec![3, 2]);
    }

    #[test]
    fn captures_prioritised_correctly_when_both_match() {
        let expected_caps = [
            ["aa", ""],
            ["a", ""],
            ["", "aa"],
            ["", "a"],
        ];

        let mut hay = "aa".into_haystack();
        let mut caps = IndexedCaptures::default();

        type CapA = CaptureGroup<char, QuantifierNOrMore<char, ScalarA, 1>, 0>;
        type CapRangeAB = CaptureGroup<char, QuantifierNOrMore<char, RangeAB, 1>, 1>;
        type CapOr = Or<char, CapA, CapRangeAB>;

        let all_caps = CapOr::all_captures(&mut hay, &mut caps)
            .map(|(_, caps)| caps);

        for (index, caps) in all_caps.enumerate() {
            let caps_array = caps.into_array::<2>()
                .into_iter()
                .map(|cap_range| cap_range.map_or("", |some_range| hay.slice_with(some_range)))
                .collect::<Vec<_>>();

            assert_eq!(caps_array, expected_caps[index]);
        }
    }

    #[test]
    fn captures_prioritised_correctly_when_one_matches() {
        let expected_caps = [
            ["aa", ""],
            ["a", ""],
            ["", "aab"],
            ["", "aa"],
            ["", "a"],
        ];

        let mut hay = "aab".into_haystack();
        let mut caps = IndexedCaptures::default();

        // /(b+)|((bc)+)/
        type CapA = CaptureGroup<char, QuantifierNOrMore<char, ScalarA, 1>, 0>;
        type CapAThenB = CaptureGroup<char, QuantifierNOrMore<char, AThenB, 1>, 1>;
        type CapOr = Or<char, CapA, CapAThenB>;

        let all_caps = CapOr::all_captures(&mut hay, &mut caps)
            .map(|(_, caps)| caps);

        for (index, caps) in all_caps.enumerate() {
            let caps_array = caps.into_array::<2>()
                .into_iter()
                .map(|cap_range| cap_range.map_or("", |some_range| hay.slice_with(some_range)))
                .collect::<Vec<_>>();

            assert_eq!(caps_array, expected_caps[index]);
        }
    }

    #[test]
    fn captures_prioritised_correctly_with_external_requirements() {
        let expected_caps = [
            ["aab", "aa", ""],
        ];

        let mut hay = "aab".into_haystack();
        let mut caps = IndexedCaptures::default();

        type CapA = CaptureGroup<char, QuantifierNOrMore<char, ScalarA, 1>, 1>;
        type CapAThenB = CaptureGroup<char, QuantifierNOrMore<char, AThenB, 1>, 2>;
        type CapOr = CaptureGroup<char, Then<char, Or<char, CapA, CapAThenB>, ScalarB>, 0>;

        let all_caps = CapOr::all_captures(&mut hay, &mut caps)
            .map(|(_, caps)| caps);

        for (index, caps) in all_caps.enumerate() {
            let caps_array = caps.into_array::<3>()
                .into_iter()
                .map(|cap_range| cap_range.map_or("", |some_range| hay.slice_with(some_range)))
                .collect::<Vec<_>>();

            assert_eq!(caps_array, expected_caps[index]);
        }
    }
}

mod then {
    use super::*;

    type Quantifier2OrMore<A> = QuantifierNOrMore<char, A, 2>;
    type Capturing2OrMoreRange = CaptureGroup<char, Quantifier2OrMore<RangeAB>, 0>;
    type Capturing2OrMoreA = CaptureGroup<char, Quantifier2OrMore<ScalarA>, 1>;

    type QuadraticA2 = Then<char, QuantifierNOrMoreA<2>, QuantifierNOrMoreA<2>>;
    type QuadraticRangeOrA = Then<char, Capturing2OrMoreRange, Capturing2OrMoreA>;

    #[test]
    fn a_then_b_match() {
        test_matches_with_index!(AThenB, "ab", 2);
        test_matches_with_indices!(
            QuantifierNOrMore<_, AThenB, 1>,
            "abab",
            vec![2, 4]
        );
    }

    #[test]
    fn performs_rollback_match() {
        test_doesnt_match_no_index!(Then<_, QuantifierNOrMoreA<2>, ScalarA>, "aa");
        test_matches_with_index!(Then<_, QuantifierNOrMoreA<2>, ScalarA>, "aaa", 3);
        test_matches_with_indices!(
            Then<_, QuantifierNOrMoreA<2>, ScalarA>,
            "aaaa",
            vec![3, 4]
        );
        test_matches_with_indices!(
            Then<_, Quantifier2OrMore<RangeAB>, Then<_, ScalarA, End>>,
            "bba",
            vec![3]
        );
    }

    #[test]
    fn quadratic_all_matches() {
        test_matches_with_indices!(QuadraticA2, "aaaa", vec![4]);
        test_matches_with_indices!(QuadraticA2, "aaaaa", vec![4, 5, 5]);
    }

    #[test]
    fn captures_prefers_first() {
        let mut hay = "bbaaa".into_haystack();
        let mut caps = IndexedCaptures::default();

        assert!(QuadraticRangeOrA::captures(&mut hay, &mut caps));
        let [cap_1, cap_2] = caps.into_array::<2>();
        assert_eq!(hay.slice_with(cap_1.unwrap()), "bba");
        assert_eq!(hay.slice_with(cap_2.unwrap()), "aa");
    }

    #[test]
    fn captures_prioritised_correctly() {
        let expected_caps = [
            ["bba", "aa"],
            ["bb", "aaa"],
            ["bb", "aa"],
        ];

        let mut hay = "bbaaa".into_haystack();
        let mut caps = IndexedCaptures::default();

        let all_caps = QuadraticRangeOrA::all_captures(&mut hay, &mut caps)
            .into_iter()
            .map(|(_, caps)| caps)
            .rev();

        for (index, caps) in all_caps.enumerate() {
            let caps_array = caps.into_array::<2>()
                .into_iter()
                .map(|cap_range| hay.slice_with(cap_range.unwrap()))
                .collect::<Vec<_>>();

            assert_eq!(caps_array, expected_caps[index]);
        }
    }
}

mod or4 {
    use super::*;

    type ABCOrD = Or4<char, ScalarA, ScalarB, ScalarC, ScalarD>;

    #[test]
    fn a_b_c_or_d_match() {
        test_matches_with_index!(ABCOrD, "a", 1);
        test_matches_with_index!(ABCOrD, "b", 1);
        test_matches_with_index!(ABCOrD, "c", 1);
        test_matches_with_index!(ABCOrD, "d", 1);
        test_matches_with_indices!(QuantifierNOrMore<_, ABCOrD, 1>, "abcd", vec![1, 2, 3, 4]);
    }
}

mod then4 {
    use super::*;

    type ABCThenD = Then4<char, ScalarA, ScalarB, ScalarC, ScalarD>;

    #[test]
    fn a_b_c_then_d_match() {
        test_matches_with_index!(ABCThenD, "abcd", 4);
    }
}

#[test]
fn implements_debug() {
    implements_debug!(
        Or<char, ScalarA, ScalarA>,
        Or4<char, ScalarA, ScalarA, ScalarA, ScalarA>,
        Then<char, ScalarA, ScalarA>,
        Then4<char, ScalarA, ScalarA, ScalarA, ScalarA>
    );
}