use crate::expr::IndexedCaptures;
use crate::haystack::{Haystack, IntoHaystack};

use super::*;

use crate::{implements_debug, test_matches_with_index, test_matches_with_indices};

type ScalarA = Scalar<'a'>;
type ScalarB = Scalar<'b'>;

mod or {
    use super::*;

    #[test]
    fn a_or_b_match() {
        test_matches_with_index!(Or<_, ScalarA, ScalarB>, "a", 1);
        test_matches_with_index!(Or<_, ScalarA, ScalarB>, "b", 1);
        test_matches_with_indices!(
            QuantifierNOrMore<_, Or<_, ScalarA, ScalarB>, 1>,
            "ab",
            vec![1, 2]
        );
    }

    #[test]
    fn rollback_as_required() {}

    #[test]
    fn captures_prefers_first() {}

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
        type CapRangeAB = CaptureGroup<char, QuantifierNOrMore<char, ScalarRange<'a', 'b'>, 1>, 1>;
        type OrRanges = Or<char, CapA, CapRangeAB>;

        let all_caps = OrRanges::all_captures(&mut hay, &mut caps)
            .into_iter()
            .map(|(_, caps)| caps)
            .rev();

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
        type CapAThenB = CaptureGroup<char, QuantifierNOrMore<char, Then<char, ScalarA, ScalarB>, 1>, 1>;
        type OrRanges = Or<char, CapA, CapAThenB>;

        let all_caps = OrRanges::all_captures(&mut hay, &mut caps)
            .into_iter()
            .map(|(_, caps)| caps)
            .rev();

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
        type CapAThenB = CaptureGroup<char, QuantifierNOrMore<char, Then<char, ScalarA, ScalarB>, 1>, 2>;
        type OrRanges = CaptureGroup<char, Then<char, Or<char, CapA, CapAThenB>, ScalarB>, 0>;

        let all_caps = OrRanges::all_captures(&mut hay, &mut caps)
            .into_iter()
            .map(|(_, caps)| caps)
            .rev();

        for (index, caps) in all_caps.enumerate() {
            let caps_array = caps.into_array::<3>()
                .into_iter()
                .map(|cap_range| cap_range.map_or("", |some_range| hay.slice_with(some_range)))
                .collect::<Vec<_>>();

            assert_eq!(caps_array, expected_caps[index]);
        }
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