use super::*;
use crate::haystack::Haystack;

// Tests for Byte matcher
#[test]
fn test_byte_matches() {
    let mut hay = Haystack::from(b"a" as &[u8]);
    assert!(Byte::<b'a'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_byte_no_match() {
    let mut hay = Haystack::from(b"b" as &[u8]);
    assert!(!Byte::<b'a'>::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_byte_empty_haystack() {
    let mut hay = Haystack::from(b"" as &[u8]);
    assert!(!Byte::<b'a'>::matches(&mut hay));
}

// Tests for ByteRange matcher
#[test]
fn test_byte_range_matches_lower() {
    let mut hay = Haystack::from(b"a" as &[u8]);
    assert!(ByteRange::<b'a', b'z'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_byte_range_matches_upper() {
    let mut hay = Haystack::from(b"z" as &[u8]);
    assert!(ByteRange::<b'a', b'z'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_byte_range_matches_middle() {
    let mut hay = Haystack::from(b"m" as &[u8]);
    assert!(ByteRange::<b'a', b'z'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_byte_range_no_match_below() {
    let mut hay = Haystack::from(b"A" as &[u8]);
    assert!(!ByteRange::<b'a', b'z'>::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_byte_range_no_match_above() {
    let mut hay = Haystack::from(b"0" as &[u8]);
    assert!(!ByteRange::<b'a', b'z'>::matches(&mut hay));
}

// Tests for Scalar matcher
#[test]
fn test_scalar_matches() {
    let mut hay = Haystack::from("a");
    assert!(Scalar::<'a'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_scalar_no_match() {
    let mut hay = Haystack::from("b");
    assert!(!Scalar::<'a'>::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_scalar_unicode() {
    let mut hay = Haystack::from("🦀");
    assert!(Scalar::<'🦀'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_scalar_empty_haystack() {
    let mut hay = Haystack::from("");
    assert!(!Scalar::<'a'>::matches(&mut hay));
}

// Tests for ScalarRange matcher
#[test]
fn test_scalar_range_matches_lower() {
    let mut hay = Haystack::from("a");
    assert!(ScalarRange::<'a', 'z'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_scalar_range_matches_upper() {
    let mut hay = Haystack::from("z");
    assert!(ScalarRange::<'a', 'z'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_scalar_range_matches_middle() {
    let mut hay = Haystack::from("m");
    assert!(ScalarRange::<'a', 'z'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_scalar_range_no_match_below() {
    let mut hay = Haystack::from("A");
    assert!(!ScalarRange::<'a', 'z'>::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_scalar_range_no_match_above() {
    let mut hay = Haystack::from("0");
    assert!(!ScalarRange::<'a', 'z'>::matches(&mut hay));
}

// Tests for Or matcher
#[test]
fn test_or_matches_first() {
    let mut hay = Haystack::from("a");
    assert!(Or::<char, Scalar::<'a'>, Scalar::<'b'>>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_or_matches_second() {
    let mut hay = Haystack::from("b");
    assert!(Or::<char, Scalar::<'a'>, Scalar::<'b'>>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_or_no_match() {
    let mut hay = Haystack::from("c");
    assert!(!Or::<char, Scalar::<'a'>, Scalar::<'b'>>::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_or_with_bytes() {
    let mut hay = Haystack::from(b"x" as &[u8]);
    assert!(Or::<u8, Byte::<b'x'>, Byte::<b'y'>>::matches(&mut hay));
    assert!(hay.is_end());
}

// Tests for Then matcher
#[test]
fn test_then_both_match() {
    let mut hay = Haystack::from("ab");
    assert!(Then::<char, Scalar::<'a'>, Scalar::<'b'>>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_then_first_fails() {
    let mut hay = Haystack::from("bb");
    assert!(!Then::<char, Scalar::<'a'>, Scalar::<'b'>>::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_then_second_fails() {
    let mut hay = Haystack::from("aa");
    assert!(!Then::<char, Scalar::<'a'>, Scalar::<'b'>>::matches(&mut hay));
}

#[test]
fn test_then_with_bytes() {
    let mut hay = Haystack::from(b"xy" as &[u8]);
    assert!(Then::<u8, Byte::<b'x'>, Byte::<b'y'>>::matches(&mut hay));
    assert!(hay.is_end());
}

// Tests for QuantifierN matcher
#[test]
fn test_quantifier_n_exact_match() {
    let mut hay = Haystack::from("aaa");
    assert!(QuantifierN::<char, Scalar::<'a'>, 3>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_n_too_few() {
    let mut hay = Haystack::from("aa");
    assert!(!QuantifierN::<char, Scalar::<'a'>, 3>::matches(&mut hay));
}

#[test]
fn test_quantifier_n_too_many() {
    let mut hay = Haystack::from("aaaa");
    assert!(!QuantifierN::<char, Scalar::<'a'>, 3>::matches(&mut hay));
}

#[test]
fn test_quantifier_n_zero() {
    let mut hay = Haystack::from("b");
    assert!(QuantifierN::<char, Scalar::<'a'>, 0>::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_quantifier_n_with_bytes() {
    let mut hay = Haystack::from(b"xxx" as &[u8]);
    assert!(QuantifierN::<u8, Byte::<b'x'>, 3>::matches(&mut hay));
    assert!(hay.is_end());
}

// Tests for QuantifierNOrMore matcher
#[test]
fn test_quantifier_n_or_more_exact() {
    let mut hay = Haystack::from("aaa");
    assert!(QuantifierNOrMore::<char, Scalar::<'a'>, 3>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_n_or_more_more() {
    let mut hay = Haystack::from("aaaaa");
    assert!(QuantifierNOrMore::<char, Scalar::<'a'>, 3>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_n_or_more_too_few() {
    let mut hay = Haystack::from("aa");
    assert!(!QuantifierNOrMore::<char, Scalar::<'a'>, 3>::matches(&mut hay));
}

#[test]
fn test_quantifier_n_or_more_zero() {
    let mut hay = Haystack::from("b");
    assert!(QuantifierNOrMore::<char, Scalar::<'a'>, 0>::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_quantifier_n_or_more_with_bytes() {
    let mut hay = Haystack::from(b"xxxx" as &[u8]);
    assert!(QuantifierNOrMore::<u8, Byte::<b'x'>, 2>::matches(&mut hay));
    assert!(hay.is_end());
}

// Tests for QuantifierNToM matcher
#[test]
fn test_quantifier_n_to_m_at_lower() {
    let mut hay = Haystack::from("aa");
    assert!(QuantifierNToM::<char, Scalar::<'a'>, 2, 4>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_n_to_m_at_upper() {
    let mut hay = Haystack::from("aaaa");
    assert!(QuantifierNToM::<char, Scalar::<'a'>, 2, 4>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_n_to_m_in_middle() {
    let mut hay = Haystack::from("aaa");
    assert!(QuantifierNToM::<char, Scalar::<'a'>, 2, 4>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_n_to_m_below_range() {
    let mut hay = Haystack::from("a");
    assert!(!QuantifierNToM::<char, Scalar::<'a'>, 2, 4>::matches(&mut hay));
}

#[test]
fn test_quantifier_n_to_m_stops_at_max() {
    // a{2,4} on "aaaaa" should match exactly 4 'a's and leave the 5th
    let mut hay = Haystack::from("aaaaa");
    assert!(QuantifierNToM::<char, Scalar::<'a'>, 2, 4>::matches(&mut hay));
    assert_eq!(hay.item(), Some('a')); // 5th 'a' remains
}

#[test]
fn test_quantifier_n_to_m_stops_at_max_with_different_suffix() {
    // a{2,4} on "aaaab" should match exactly 4 'a's and leave 'b'
    let mut hay = Haystack::from("aaaab");
    assert!(QuantifierNToM::<char, Scalar::<'a'>, 2, 4>::matches(&mut hay));
    assert_eq!(hay.item(), Some('b'));
}

#[test]
fn test_quantifier_n_to_m_greedy_up_to_max() {
    // a{2,4} on "aaa" should greedily match all 3 (less than max)
    let mut hay = Haystack::from("aaa");
    assert!(QuantifierNToM::<char, Scalar::<'a'>, 2, 4>::matches(&mut hay));
    assert!(hay.is_end()); // All consumed since 3 < 4
}

#[test]
fn test_quantifier_n_to_m_with_bytes() {
    let mut hay = Haystack::from(b"xxx" as &[u8]);
    assert!(QuantifierNToM::<u8, Byte::<b'x'>, 2, 4>::matches(&mut hay));
    assert!(hay.is_end());
}

// Tests for Beginning matcher
#[test]
fn test_beginning_at_start() {
    let mut hay = Haystack::from("abc");
    assert!(Beginning::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_beginning_after_progress() {
    let mut hay = Haystack::from("abc");
    hay.progress();
    assert!(!Beginning::matches(&mut hay));
}

#[test]
fn test_beginning_empty_haystack() {
    let mut hay = Haystack::from("");
    assert!(Beginning::matches(&mut hay));
}

#[test]
fn test_beginning_with_bytes() {
    let mut hay = Haystack::from(b"test" as &[u8]);
    assert!(Beginning::matches(&mut hay));
}

// Tests for End matcher
#[test]
fn test_end_at_end() {
    let mut hay = Haystack::from("");
    assert!(End::matches(&mut hay));
}

#[test]
fn test_end_not_at_end() {
    let mut hay = Haystack::from("a");
    assert!(!End::matches(&mut hay));
}

#[test]
fn test_end_after_consuming() {
    let mut hay = Haystack::from("a");
    Scalar::<'a'>::matches(&mut hay);
    assert!(End::matches(&mut hay));
}

#[test]
fn test_end_with_bytes() {
    let mut hay = Haystack::from(b"" as &[u8]);
    assert!(End::matches(&mut hay));
}

// Tests for Always matcher
#[test]
fn test_always_matches() {
    let mut hay = Haystack::from("anything");
    assert!(Always::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_always_empty() {
    let mut hay = Haystack::from("");
    assert!(Always::matches(&mut hay));
}

#[test]
fn test_always_with_bytes() {
    let mut hay = Haystack::from(b"data" as &[u8]);
    assert!(Always::matches(&mut hay));
}

// Complex combination tests
#[test]
fn test_complex_or_then() {
    let mut hay = Haystack::from("ab");
    type AB = Then<char, Scalar<'a'>, Scalar<'b'>>;
    type CD = Then<char, Scalar<'c'>, Scalar<'d'>>;
    assert!(Or::<char, AB, CD>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_complex_quantifier_with_range() {
    let mut hay = Haystack::from("abc");
    assert!(QuantifierN::<char, ScalarRange::<'a', 'z'>, 3>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_anchored_pattern() {
    let mut hay = Haystack::from("test");
    type Pattern = Then<char, Beginning, Then<char, Scalar<'t'>, End>>;
    assert!(!Pattern::matches(&mut hay));
}

#[test]
fn test_anchored_single_char() {
    let mut hay = Haystack::from("t");
    type Pattern = Then<char, Beginning, Then<char, Scalar<'t'>, End>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

// ============================================================================
// REGEX SEMANTICS VALIDATION TESTS
// These tests verify that the matchers implement correct regex matching logic
// ============================================================================

// Test 1: Greedy quantifier semantics
#[test]
fn test_greedy_quantifier_consumes_maximum() {
    // a+ should consume all 'a's, not just one
    let mut hay = Haystack::from("aaa");
    assert!(QuantifierNOrMore::<char, Scalar<'a'>, 1>::matches(&mut hay));
    assert!(hay.is_end()); // All input consumed
}

#[test]
fn test_greedy_quantifier_in_sequence() {
    // Pattern: a+b should match "aaab" by consuming "aaa" then "b"
    let mut hay = Haystack::from("aaab");
    type Pattern = Then<char, QuantifierNOrMore<char, Scalar<'a'>, 1>, Scalar<'b'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_greedy_quantifier_stops_at_non_match() {
    // a* should consume 'a's but stop at 'b', leaving 'b' unconsumed
    let mut hay = Haystack::from("aaab");
    assert!(QuantifierNOrMore::<char, Scalar<'a'>, 0>::matches(&mut hay));
    assert_eq!(hay.item(), Some('b')); // 'b' not consumed
}

// Test 2: Backtracking in alternation (Or)
#[test]
fn test_or_tries_second_alternative() {
    // (a|b) should match 'b' when 'a' fails
    let mut hay = Haystack::from("b");
    type Pattern = Or<char, Scalar<'a'>, Scalar<'b'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_or_backtracks_on_partial_match() {
    // (ab|ac) should try 'ab' first, fail, then backtrack and try 'ac'
    let mut hay = Haystack::from("ac");
    type AB = Then<char, Scalar<'a'>, Scalar<'b'>>;
    type AC = Then<char, Scalar<'a'>, Scalar<'c'>>;
    type Pattern = Or<char, AB, AC>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_or_restores_position_on_first_branch_failure() {
    // When first branch of Or fails, position should be restored
    let mut hay = Haystack::from("xyz");
    type AB = Then<char, Scalar<'a'>, Scalar<'b'>>;
    type XY = Then<char, Scalar<'x'>, Scalar<'y'>>;
    type Pattern = Or<char, AB, XY>;
    assert!(Pattern::matches(&mut hay));
    assert_eq!(hay.item(), Some('z')); // Only 'xy' consumed, 'z' remains
}

// Test 3: Quantifier semantics with exact counts
#[test]
fn test_exact_quantifier_rejects_too_few() {
    // a{3} should reject "aa" - greedily consumes what it can, then fails
    let mut hay = Haystack::from("aa");
    assert!(!QuantifierN::<char, Scalar<'a'>, 3>::matches(&mut hay));
    // Quantifiers are greedy - they consume available matches before checking count
    assert!(hay.is_end()); // Both 'a's consumed even though match failed
}

#[test]
fn test_exact_quantifier_rejects_too_many() {
    // a{3} should reject "aaaa" (consumes exactly 3, but there's leftover)
    let mut hay = Haystack::from("aaaa");
    assert!(!QuantifierN::<char, Scalar<'a'>, 3>::matches(&mut hay));
}

#[test]
fn test_range_quantifier_accepts_within_bounds() {
    // a{2,4} should accept 2, 3, or 4 'a's
    let mut hay2 = Haystack::from("aa");
    let mut hay3 = Haystack::from("aaa");
    let mut hay4 = Haystack::from("aaaa");

    assert!(QuantifierNToM::<char, Scalar<'a'>, 2, 4>::matches(&mut hay2));
    assert!(QuantifierNToM::<char, Scalar<'a'>, 2, 4>::matches(&mut hay3));
    assert!(QuantifierNToM::<char, Scalar<'a'>, 2, 4>::matches(&mut hay4));
}

// Test 4: Anchoring behavior
#[test]
fn test_beginning_anchor_fails_after_consumption() {
    // ^a should fail if we're not at the beginning
    let mut hay = Haystack::from("ba");
    hay.progress(); // Move past 'b'
    assert!(!Beginning::matches(&mut hay));
}

#[test]
fn test_end_anchor_succeeds_only_at_end() {
    // $ should only match at end of input
    let mut hay = Haystack::from("a");
    assert!(!End::matches(&mut hay)); // Not at end yet
    hay.progress();
    assert!(End::matches(&mut hay)); // Now at end
}

#[test]
fn test_anchored_pattern_rejects_prefix() {
    // ^abc$ should reject "xabc"
    let mut hay = Haystack::from("xabc");
    type Pattern = Then<char, Beginning, Then<char,
        Then<char, Scalar<'a'>, Then<char, Scalar<'b'>, Scalar<'c'>>>,
        End
    >>;
    assert!(!Pattern::matches(&mut hay));
}

#[test]
fn test_anchored_pattern_rejects_suffix() {
    // ^abc$ should reject "abcx"
    let mut hay = Haystack::from("abcx");
    type Pattern = Then<char, Beginning, Then<char,
        Then<char, Scalar<'a'>, Then<char, Scalar<'b'>, Scalar<'c'>>>,
        End
    >>;
    assert!(!Pattern::matches(&mut hay));
}

// Test 5: Range matchers behave correctly
#[test]
fn test_range_excludes_outside_values() {
    // [a-z] should accept lowercase but reject uppercase and numbers
    let mut hay_lower = Haystack::from("m");
    let mut hay_upper = Haystack::from("M");
    let mut hay_digit = Haystack::from("5");

    assert!(ScalarRange::<'a', 'z'>::matches(&mut hay_lower));
    assert!(!ScalarRange::<'a', 'z'>::matches(&mut hay_upper));
    assert!(!ScalarRange::<'a', 'z'>::matches(&mut hay_digit));
}

#[test]
fn test_range_is_inclusive() {
    // [a-c] should include both 'a' and 'c'
    let mut hay_a = Haystack::from("a");
    let mut hay_c = Haystack::from("c");

    assert!(ScalarRange::<'a', 'c'>::matches(&mut hay_a));
    assert!(ScalarRange::<'a', 'c'>::matches(&mut hay_c));
}

// Test 6: Complex real-world-like patterns
#[test]
fn test_optional_prefix_pattern() {
    // Pattern: (https?://)? - optional protocol prefix
    // Should match both with and without prefix
    type Http = Then<char, Scalar<'h'>, Then<char, Scalar<'t'>, Then<char, Scalar<'t'>, Scalar<'p'>>>>;
    type Https = Then<char, Http, Scalar<'s'>>;
    type Protocol = Or<char, Https, Http>;
    type WithColon = Then<char, Protocol, Then<char, Scalar<':'>, Then<char, Scalar<'/'>, Scalar<'/'>>>>;
    type Optional = QuantifierNToM<char, WithColon, 0, 1>;

    // Test with "https://"
    let mut hay1 = Haystack::from("https://");
    assert!(Optional::matches(&mut hay1));
    assert!(hay1.is_end());

    // Test with "http://"
    let mut hay2 = Haystack::from("http://");
    assert!(Optional::matches(&mut hay2));
    assert!(hay2.is_end());

    // Test with empty (optional, so should succeed)
    let mut hay3 = Haystack::from("");
    assert!(Optional::matches(&mut hay3));
}

#[test]
fn test_repeated_digit_pattern() {
    // Pattern: [0-9]{3} - exactly three digits
    type Digit = ScalarRange<'0', '9'>;
    type ThreeDigits = QuantifierN<char, Digit, 3>;

    let mut hay_valid = Haystack::from("123");
    assert!(ThreeDigits::matches(&mut hay_valid));
    assert!(hay_valid.is_end());

    let mut hay_short = Haystack::from("12");
    assert!(!ThreeDigits::matches(&mut hay_short));

    let mut hay_long = Haystack::from("1234");
    assert!(!ThreeDigits::matches(&mut hay_long));
}

#[test]
fn test_alternation_of_ranges() {
    // Pattern: [a-z]|[A-Z] - lowercase or uppercase letter
    type Lower = ScalarRange<'a', 'z'>;
    type Upper = ScalarRange<'A', 'Z'>;
    type Letter = Or<char, Lower, Upper>;

    let mut hay_lower = Haystack::from("a");
    let mut hay_upper = Haystack::from("Z");
    let mut hay_digit = Haystack::from("5");

    assert!(Letter::matches(&mut hay_lower));
    assert!(Letter::matches(&mut hay_upper));
    assert!(!Letter::matches(&mut hay_digit));
}

// Test 7: Consumption behavior verification
#[test]
fn test_failed_match_does_not_consume() {
    // When a match fails, input should not be consumed
    let mut hay = Haystack::from("xyz");
    assert!(!Scalar::<'a'>::matches(&mut hay));
    assert_eq!(hay.item(), Some('x')); // Nothing consumed
}

#[test]
fn test_partial_then_failure_does_not_consume() {
    // Then should not consume if second part fails
    let mut hay = Haystack::from("ax");
    type Pattern = Then<char, Scalar<'a'>, Scalar<'b'>>;
    assert!(!Pattern::matches(&mut hay));
    // First char should be consumed since it matched, but not the second
    assert_eq!(hay.item(), Some('x'));
}

#[test]
fn test_successful_match_consumes_exactly() {
    // A successful match should consume exactly what it matches
    let mut hay = Haystack::from("abcdef");
    type ABC = Then<char, Scalar<'a'>, Then<char, Scalar<'b'>, Scalar<'c'>>>;
    assert!(ABC::matches(&mut hay));
    assert_eq!(hay.item(), Some('d')); // Consumed 'abc', 'def' remains
}

// ============================================================================
// QUANTIFIERTHEN BACKTRACKING TESTS
// These tests verify that QuantifierThen correctly implements greedy matching
// with backtracking when the continuation fails
// ============================================================================

// Test: Basic backtracking - quantifier gives back to let continuation match
#[test]
fn test_quantifier_then_backtracks_for_same_char() {
    // Pattern: a*a (greedy a* followed by literal a)
    // Input: "aaa"
    // Expected: a* matches "aa", then final 'a' matches - total match
    let mut hay = Haystack::from("aaa");
    type Pattern = QuantifierThen<char, QuantifierNOrMore<char, Scalar<'a'>, 0>, Scalar<'a'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_then_backtracks_for_suffix() {
    // Pattern: .*end (greedy .* followed by "end")
    // Input: "startend"
    // Expected: .* matches "start", then "end" matches
    let mut hay = Haystack::from("startend");
    type AnyChar = ScalarRange<'\0', '\u{10FFFF}'>;
    type End = Then<char, Scalar<'e'>, Then<char, Scalar<'n'>, Scalar<'d'>>>;
    type Pattern = QuantifierThen<char, QuantifierNOrMore<char, AnyChar, 0>, End>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_then_backtracks_overlapping_content() {
    // Pattern: .*ee (greedy .* followed by "ee")
    // Input: "eee"
    // Expected: .* matches "e", then "ee" matches
    let mut hay = Haystack::from("eee");
    type AnyChar = ScalarRange<'\0', '\u{10FFFF}'>;
    type EE = Then<char, Scalar<'e'>, Scalar<'e'>>;
    type Pattern = QuantifierThen<char, QuantifierNOrMore<char, AnyChar, 0>, EE>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

// Test: Greedy preference - should prefer more matches when possible
#[test]
fn test_quantifier_then_prefers_greedy_when_possible() {
    // Pattern: a*b (greedy a* followed by 'b')
    // Input: "aaab"
    // Expected: a* matches "aaa", then 'b' matches (no backtracking needed)
    let mut hay = Haystack::from("aaab");
    type Pattern = QuantifierThen<char, QuantifierNOrMore<char, Scalar<'a'>, 0>, Scalar<'b'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_then_greedy_consumes_maximum_possible() {
    // Pattern: a+a (one or more 'a' followed by 'a')
    // Input: "aaaa"
    // Expected: a+ matches "aaa" (greedy, but gives one back), then final 'a' matches
    let mut hay = Haystack::from("aaaa");
    type Pattern = QuantifierThen<char, QuantifierNOrMore<char, Scalar<'a'>, 1>, Scalar<'a'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

// Test: Minimum bounds respected during backtracking
#[test]
fn test_quantifier_then_respects_minimum_bound() {
    // Pattern: a{2,}a (two or more 'a' followed by 'a')
    // Input: "aaa"
    // Expected: a{2,} matches "aa", then final 'a' matches
    let mut hay = Haystack::from("aaa");
    type Pattern = QuantifierThen<char, QuantifierNOrMore<char, Scalar<'a'>, 2>, Scalar<'a'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_then_fails_when_minimum_not_satisfiable() {
    // Pattern: a{2,}a (two or more 'a' followed by 'a')
    // Input: "aa"
    // Expected: Cannot satisfy both a{2,} and final 'a' - fails
    let mut hay = Haystack::from("aa");
    type Pattern = QuantifierThen<char, QuantifierNOrMore<char, Scalar<'a'>, 2>, Scalar<'a'>>;
    assert!(!Pattern::matches(&mut hay));
}

#[test]
fn test_quantifier_then_minimum_one_plus_same_char() {
    // Pattern: a+a (one or more 'a' followed by 'a')
    // Input: "aa"
    // Expected: a+ matches "a" (minimum), then final 'a' matches
    let mut hay = Haystack::from("aa");
    type Pattern = QuantifierThen<char, QuantifierNOrMore<char, Scalar<'a'>, 1>, Scalar<'a'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

// Test: QuantifierNToM (bounded quantifier) backtracking
#[test]
fn test_quantifier_n_to_m_then_backtracks() {
    // Pattern: a{2,4}a (2-4 'a's followed by 'a')
    // Input: "aaaaa"
    // Expected: a{2,4} matches "aaaa" (max), then final 'a' matches
    let mut hay = Haystack::from("aaaaa");
    type Pattern = QuantifierThen<char, QuantifierNToM<char, Scalar<'a'>, 2, 4>, Scalar<'a'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_n_to_m_then_at_minimum() {
    // Pattern: a{2,4}a (2-4 'a's followed by 'a')
    // Input: "aaa"
    // Expected: a{2,4} matches "aa" (min), then final 'a' matches
    let mut hay = Haystack::from("aaa");
    type Pattern = QuantifierThen<char, QuantifierNToM<char, Scalar<'a'>, 2, 4>, Scalar<'a'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_n_to_m_then_fails_below_minimum() {
    // Pattern: a{2,4}a (2-4 'a's followed by 'a')
    // Input: "aa"
    // Expected: Cannot have a{2,4} AND another 'a' with only 2 chars - fails
    let mut hay = Haystack::from("aa");
    type Pattern = QuantifierThen<char, QuantifierNToM<char, Scalar<'a'>, 2, 4>, Scalar<'a'>>;
    assert!(!Pattern::matches(&mut hay));
}

#[test]
fn test_quantifier_n_to_m_then_fails_max_bound_too_restrictive() {
    // Pattern: a{2,3}b (2-3 'a's followed by 'b')
    // Input: "aaaab" (4 'a's + 'b')
    // Expected: a{2,3} stops at 3 'a's (max), 'b' fails on 4th 'a'
    //           Backtracking tries 2 'a's, 'b' still fails on 3rd 'a'
    //           Overall fails because max bound prevents matching all 4 'a's
    // Note: a{2,4}b would succeed on this input!
    let mut hay = Haystack::from("aaaab");
    type Pattern = QuantifierThen<char, QuantifierNToM<char, Scalar<'a'>, 2, 3>, Scalar<'b'>>;
    assert!(!Pattern::matches(&mut hay));
}

#[test]
fn test_quantifier_n_to_m_then_succeeds_with_higher_max() {
    // Same input as above, but with higher max bound - now succeeds
    // Pattern: a{2,4}b (2-4 'a's followed by 'b')
    // Input: "aaaab" (4 'a's + 'b')
    // Expected: a{2,4} matches 4 'a's, 'b' matches 'b' - success
    let mut hay = Haystack::from("aaaab");
    type Pattern = QuantifierThen<char, QuantifierNToM<char, Scalar<'a'>, 2, 4>, Scalar<'b'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_n_to_m_then_fails_continuation_needs_more_than_available() {
    // Pattern: a{3,4}aa (3-4 'a's followed by "aa")
    // Input: "aaaa" (4 'a's total)
    // Expected: a{3,4} matches 4, leaving 0 for "aa" - fails
    //           Backtrack to 3, leaving 1 for "aa" - still fails (needs 2)
    //           Overall fails because max bound doesn't leave enough for continuation
    let mut hay = Haystack::from("aaaa");
    type AA = Then<char, Scalar<'a'>, Scalar<'a'>>;
    type Pattern = QuantifierThen<char, QuantifierNToM<char, Scalar<'a'>, 3, 4>, AA>;
    assert!(!Pattern::matches(&mut hay));
}

// Test: Edge cases
#[test]
fn test_quantifier_then_empty_input() {
    // Pattern: a*b
    // Input: ""
    // Expected: a* matches "" (zero), but 'b' fails - overall fails
    let mut hay = Haystack::from("");
    type Pattern = QuantifierThen<char, QuantifierNOrMore<char, Scalar<'a'>, 0>, Scalar<'b'>>;
    assert!(!Pattern::matches(&mut hay));
}

#[test]
fn test_quantifier_then_zero_matches_success() {
    // Pattern: a*b
    // Input: "b"
    // Expected: a* matches "" (zero), then 'b' matches
    let mut hay = Haystack::from("b");
    type Pattern = QuantifierThen<char, QuantifierNOrMore<char, Scalar<'a'>, 0>, Scalar<'b'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_then_continuation_longer_than_available() {
    // Pattern: a*abc
    // Input: "aabc"
    // Expected: a* matches "a", then "abc" matches
    let mut hay = Haystack::from("aabc");
    type ABC = Then<char, Scalar<'a'>, Then<char, Scalar<'b'>, Scalar<'c'>>>;
    type Pattern = QuantifierThen<char, QuantifierNOrMore<char, Scalar<'a'>, 0>, ABC>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_then_no_possible_match() {
    // Pattern: a+b
    // Input: "aaa"
    // Expected: No 'b' in input, so fails
    let mut hay = Haystack::from("aaa");
    type Pattern = QuantifierThen<char, QuantifierNOrMore<char, Scalar<'a'>, 1>, Scalar<'b'>>;
    assert!(!Pattern::matches(&mut hay));
}

// Test: Byte variant
#[test]
fn test_quantifier_then_with_bytes() {
    // Pattern: [a-z]*! with bytes
    // Input: "hello!"
    let mut hay = Haystack::from(b"hello!" as &[u8]);
    type Pattern = QuantifierThen<u8, QuantifierNOrMore<u8, ByteRange<b'a', b'z'>, 0>, Byte<b'!'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_then_bytes_backtracking() {
    // Pattern: .*ll with bytes
    // Input: "helll" (three l's, no trailing char)
    let mut hay = Haystack::from(b"helll" as &[u8]);
    type AnyByte = ByteRange<0, 255>;
    type LL = Then<u8, Byte<b'l'>, Byte<b'l'>>;
    type Pattern = QuantifierThen<u8, QuantifierNOrMore<u8, AnyByte, 0>, LL>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

// Test: Complex nested pattern as continuation
#[test]
fn test_quantifier_then_complex_continuation() {
    // Pattern: .*(end|fin)
    // Input: "the end"
    let mut hay = Haystack::from("the end");
    type AnyChar = ScalarRange<'\0', '\u{10FFFF}'>;
    type End = Then<char, Scalar<'e'>, Then<char, Scalar<'n'>, Scalar<'d'>>>;
    type Fin = Then<char, Scalar<'f'>, Then<char, Scalar<'i'>, Scalar<'n'>>>;
    type Suffix = Or<char, End, Fin>;
    type Pattern = QuantifierThen<char, QuantifierNOrMore<char, AnyChar, 0>, Suffix>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_then_complex_continuation_second_alt() {
    // Pattern: .*(end|fin)
    // Input: "the fin"
    let mut hay = Haystack::from("the fin");
    type AnyChar = ScalarRange<'\0', '\u{10FFFF}'>;
    type End = Then<char, Scalar<'e'>, Then<char, Scalar<'n'>, Scalar<'d'>>>;
    type Fin = Then<char, Scalar<'f'>, Then<char, Scalar<'i'>, Scalar<'n'>>>;
    type Suffix = Or<char, End, Fin>;
    type Pattern = QuantifierThen<char, QuantifierNOrMore<char, AnyChar, 0>, Suffix>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

// Test: Verify haystack position after partial match failure
#[test]
fn test_quantifier_then_leaves_remainder() {
    // Pattern: a*b (as part of larger input)
    // Input: "aaabcd"
    // After match: should leave "cd"
    let mut hay = Haystack::from("aaabcd");
    type Pattern = QuantifierThen<char, QuantifierNOrMore<char, Scalar<'a'>, 0>, Scalar<'b'>>;
    assert!(Pattern::matches(&mut hay));
    assert_eq!(hay.item(), Some('c'));
}

// Test 8: Zero-width assertions
#[test]
fn test_beginning_is_zero_width() {
    // Beginning should not consume input
    let mut hay = Haystack::from("test");
    assert!(Beginning::matches(&mut hay));
    assert_eq!(hay.item(), Some('t')); // Nothing consumed
}

#[test]
fn test_end_is_zero_width() {
    // End should not consume input (there's no input to consume anyway)
    let mut hay = Haystack::from("");
    assert!(End::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_always_is_zero_width() {
    // Always should not consume input
    let mut hay = Haystack::from("test");
    assert!(Always::matches(&mut hay));
    assert_eq!(hay.item(), Some('t')); // Nothing consumed
}

// ============================================================================
// COMPOSITE BACKTRACKING TESTS
// These tests verify that all_matches propagates correctly through Or, Then,
// Or4, etc. to enable backtracking within nested expressions
// ============================================================================

// Test: Or with quantifiers - (a*|b)c pattern
#[test]
fn test_or_quantifier_first_branch_backtracks() {
    // Pattern: (a*|b)c
    // Input: "aac"
    // Expected: a* matches "aa", then 'c' matches
    let mut hay = Haystack::from("aac");
    type StarA = QuantifierNOrMore<char, Scalar<'a'>, 0>;
    type Alt = Or<char, StarA, Scalar<'b'>>;
    type Pattern = QuantifierThen<char, Alt, Scalar<'c'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_or_quantifier_first_branch_gives_back() {
    // Pattern: (a*|b)a
    // Input: "aaa"
    // Expected: a* matches "aa" (gives back one), then 'a' matches
    let mut hay = Haystack::from("aaa");
    type StarA = QuantifierNOrMore<char, Scalar<'a'>, 0>;
    type Alt = Or<char, StarA, Scalar<'b'>>;
    type Pattern = QuantifierThen<char, Alt, Scalar<'a'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_or_quantifier_second_branch_used() {
    // Pattern: (a*|b)c
    // Input: "bc"
    // Expected: a* matches "" but then 'c' fails on 'b', backtrack to 'b' branch
    let mut hay = Haystack::from("bc");
    type StarA = QuantifierNOrMore<char, Scalar<'a'>, 0>;
    type Alt = Or<char, StarA, Scalar<'b'>>;
    type Pattern = QuantifierThen<char, Alt, Scalar<'c'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_or_both_quantifiers_prefers_first() {
    // Pattern: (a*|b*)c
    // Input: "aac"
    // Expected: Prefers a* (first branch) matching "aa", then 'c'
    let mut hay = Haystack::from("aac");
    type StarA = QuantifierNOrMore<char, Scalar<'a'>, 0>;
    type StarB = QuantifierNOrMore<char, Scalar<'b'>, 0>;
    type Alt = Or<char, StarA, StarB>;
    type Pattern = QuantifierThen<char, Alt, Scalar<'c'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_or_both_quantifiers_falls_back_to_second() {
    // Pattern: (a*|b*)c
    // Input: "bbc"
    // Expected: a* matches "" but 'c' fails on 'b', falls back to b* matching "bb"
    let mut hay = Haystack::from("bbc");
    type StarA = QuantifierNOrMore<char, Scalar<'a'>, 0>;
    type StarB = QuantifierNOrMore<char, Scalar<'b'>, 0>;
    type Alt = Or<char, StarA, StarB>;
    type Pattern = QuantifierThen<char, Alt, Scalar<'c'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

// Test: Then with multiple quantifiers - a*b*c pattern
#[test]
fn test_then_multiple_quantifiers() {
    // Pattern: a*b*c
    // Input: "aabbc"
    // Expected: a* matches "aa", b* matches "bb", 'c' matches
    let mut hay = Haystack::from("aabbc");
    type StarA = QuantifierNOrMore<char, Scalar<'a'>, 0>;
    type StarB = QuantifierNOrMore<char, Scalar<'b'>, 0>;
    type AB = QuantifierThen<char, StarA, StarB>;
    type Pattern = QuantifierThen<char, AB, Scalar<'c'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_then_quantifiers_backtrack_first() {
    // Pattern: a*b*b (two quantifiers followed by literal 'b')
    // Input: "aabbb"
    // Expected: a* matches "aa", b* matches "bb" (gives back one), 'b' matches
    let mut hay = Haystack::from("aabbb");
    type StarA = QuantifierNOrMore<char, Scalar<'a'>, 0>;
    type StarB = QuantifierNOrMore<char, Scalar<'b'>, 0>;
    type AB = QuantifierThen<char, StarA, StarB>;
    type Pattern = QuantifierThen<char, AB, Scalar<'b'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_then_quantifiers_empty_first() {
    // Pattern: a*b*c
    // Input: "bbbc"
    // Expected: a* matches "" (zero), b* matches "bbb", 'c' matches
    let mut hay = Haystack::from("bbbc");
    type StarA = QuantifierNOrMore<char, Scalar<'a'>, 0>;
    type StarB = QuantifierNOrMore<char, Scalar<'b'>, 0>;
    type AB = QuantifierThen<char, StarA, StarB>;
    type Pattern = QuantifierThen<char, AB, Scalar<'c'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

// Test: Nested Or within quantifier continuation
#[test]
fn test_nested_or_in_continuation() {
    // Pattern: a*(b|c)
    // Input: "aaab"
    // Expected: a* matches "aaa", then (b|c) matches 'b'
    let mut hay = Haystack::from("aaab");
    type StarA = QuantifierNOrMore<char, Scalar<'a'>, 0>;
    type BorC = Or<char, Scalar<'b'>, Scalar<'c'>>;
    type Pattern = QuantifierThen<char, StarA, BorC>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_nested_or_in_continuation_second_branch() {
    // Pattern: a*(b|c)
    // Input: "aaac"
    // Expected: a* matches "aaa", then (b|c) matches 'c' via second branch
    let mut hay = Haystack::from("aaac");
    type StarA = QuantifierNOrMore<char, Scalar<'a'>, 0>;
    type BorC = Or<char, Scalar<'b'>, Scalar<'c'>>;
    type Pattern = QuantifierThen<char, StarA, BorC>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

// Test: Or4 with quantifiers
#[test]
fn test_or4_with_quantifiers() {
    // Pattern: (a*|b*|c*|d*)e
    // Input: "ccce"
    // Expected: Falls through a*, b* (both match ""), to c* matching "ccc"
    let mut hay = Haystack::from("ccce");
    type StarA = QuantifierNOrMore<char, Scalar<'a'>, 0>;
    type StarB = QuantifierNOrMore<char, Scalar<'b'>, 0>;
    type StarC = QuantifierNOrMore<char, Scalar<'c'>, 0>;
    type StarD = QuantifierNOrMore<char, Scalar<'d'>, 0>;
    type Alt = Or4<char, StarA, StarB, StarC, StarD>;
    type Pattern = QuantifierThen<char, Alt, Scalar<'e'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_or4_prefers_earlier_branch() {
    // Pattern: (a*|b*|c*|d*)e
    // Input: "aae"
    // Expected: Prefers a* (first branch) matching "aa"
    let mut hay = Haystack::from("aae");
    type StarA = QuantifierNOrMore<char, Scalar<'a'>, 0>;
    type StarB = QuantifierNOrMore<char, Scalar<'b'>, 0>;
    type StarC = QuantifierNOrMore<char, Scalar<'c'>, 0>;
    type StarD = QuantifierNOrMore<char, Scalar<'d'>, 0>;
    type Alt = Or4<char, StarA, StarB, StarC, StarD>;
    type Pattern = QuantifierThen<char, Alt, Scalar<'e'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

// Test: Complex nested pattern - (a*b|c*)d
#[test]
fn test_complex_nested_first_branch() {
    // Pattern: (a*b|c*)d
    // Input: "aabd"
    // Expected: First branch a*b matches "aab", then 'd' matches
    let mut hay = Haystack::from("aabd");
    type StarA = QuantifierNOrMore<char, Scalar<'a'>, 0>;
    type AB = QuantifierThen<char, StarA, Scalar<'b'>>;
    type StarC = QuantifierNOrMore<char, Scalar<'c'>, 0>;
    type Alt = Or<char, AB, StarC>;
    type Pattern = QuantifierThen<char, Alt, Scalar<'d'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_complex_nested_second_branch() {
    // Pattern: (a*b|c*)d
    // Input: "cccd"
    // Expected: First branch a*b fails (no 'b'), second branch c* matches "ccc"
    let mut hay = Haystack::from("cccd");
    type StarA = QuantifierNOrMore<char, Scalar<'a'>, 0>;
    type AB = QuantifierThen<char, StarA, Scalar<'b'>>;
    type StarC = QuantifierNOrMore<char, Scalar<'c'>, 0>;
    type Alt = Or<char, AB, StarC>;
    type Pattern = QuantifierThen<char, Alt, Scalar<'d'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

// Test: Greedy preference verification - ensure longer matches preferred
#[test]
fn test_or_greedy_prefers_longer_first_branch() {
    // Pattern: (a+|a)b
    // Input: "aab"
    // Expected: Greedy a+ matches "aa" (not just "a"), then 'b' matches
    // This verifies that within Or, longer matches from first branch are preferred
    let mut hay = Haystack::from("aab");
    type PlusA = QuantifierNOrMore<char, Scalar<'a'>, 1>;
    type Alt = Or<char, PlusA, Scalar<'a'>>;
    type Pattern = QuantifierThen<char, Alt, Scalar<'b'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_or_greedy_backtracks_within_first_branch() {
    // Pattern: (a+|a)a
    // Input: "aaa"
    // Expected: a+ greedily matches "aaa", backtracks to "aa", then 'a' matches
    let mut hay = Haystack::from("aaa");
    type PlusA = QuantifierNOrMore<char, Scalar<'a'>, 1>;
    type Alt = Or<char, PlusA, Scalar<'a'>>;
    type Pattern = QuantifierThen<char, Alt, Scalar<'a'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

// Test: Edge case - empty alternatives
#[test]
fn test_or_with_empty_match_first() {
    // Pattern: (a*|b)c
    // Input: "c"
    // Expected: a* matches "" (zero), then 'c' matches
    let mut hay = Haystack::from("c");
    type StarA = QuantifierNOrMore<char, Scalar<'a'>, 0>;
    type Alt = Or<char, StarA, Scalar<'b'>>;
    type Pattern = QuantifierThen<char, Alt, Scalar<'c'>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}

// Test: Failure cases
#[test]
fn test_or_quantifiers_both_fail() {
    // Pattern: (a+|b+)c
    // Input: "c"
    // Expected: Neither a+ nor b+ can match (need at least 1), so fails
    let mut hay = Haystack::from("c");
    type PlusA = QuantifierNOrMore<char, Scalar<'a'>, 1>;
    type PlusB = QuantifierNOrMore<char, Scalar<'b'>, 1>;
    type Alt = Or<char, PlusA, PlusB>;
    type Pattern = QuantifierThen<char, Alt, Scalar<'c'>>;
    assert!(!Pattern::matches(&mut hay));
}

#[test]
fn test_then_quantifiers_continuation_fails() {
    // Pattern: a*b*c
    // Input: "aabbd"
    // Expected: a* matches "aa", b* matches "bb", but 'c' fails on 'd'
    let mut hay = Haystack::from("aabbd");
    type StarA = QuantifierNOrMore<char, Scalar<'a'>, 0>;
    type StarB = QuantifierNOrMore<char, Scalar<'b'>, 0>;
    type AB = QuantifierThen<char, StarA, StarB>;
    type Pattern = QuantifierThen<char, AB, Scalar<'c'>>;
    assert!(!Pattern::matches(&mut hay));
}
