// Tests that verify contains_match and find_match methods work correctly

use crate::*;

// ============================================================================
// CONTAINS_MATCH TESTS - NAMED REGEX
// ============================================================================

#[test]
fn test_contains_match_literal_present() {
    regex!(Literal = "hello");
    assert!(Literal::contains_match("say hello world"));
}

#[test]
fn test_contains_match_literal_absent() {
    regex!(Literal = "hello");
    assert!(!Literal::contains_match("goodbye world"));
}

#[test]
fn test_contains_match_at_start() {
    regex!(Pattern = "abc");
    assert!(Pattern::contains_match("abcdef"));
}

#[test]
fn test_contains_match_at_end() {
    regex!(Pattern = "xyz");
    assert!(Pattern::contains_match("abcxyz"));
}

#[test]
fn test_contains_match_in_middle() {
    regex!(Pattern = "middle");
    assert!(Pattern::contains_match("start middle end"));
}

#[test]
fn test_contains_match_exact_string() {
    regex!(Pattern = "exact");
    assert!(Pattern::contains_match("exact"));
}

#[test]
fn test_contains_match_empty_pattern() {
    regex!(Empty = "");
    assert!(Empty::contains_match("anything"));
}

#[test]
fn test_contains_match_empty_haystack() {
    regex!(Pattern = "test");
    assert!(!Pattern::contains_match(""));
}

#[test]
fn test_contains_match_pattern_with_quantifier() {
    regex!(Digits = "[0-9]+");

    assert!(Digits::contains_match("abc123def"));
    assert!(!Digits::contains_match("abcdef"));
}

#[test]
fn test_contains_match_alternation() {
    regex!(Alt = "foo|bar");

    assert!(Alt::contains_match("prefix foo suffix"));
    assert!(Alt::contains_match("prefix bar suffix"));
    assert!(!Alt::contains_match("prefix baz suffix"));
}

#[test]
fn test_contains_match_multiple_occurrences() {
    regex!(Pattern = "ab");
    // Should find the first occurrence
    assert!(Pattern::contains_match("ab ab ab"));
}

#[test]
fn test_contains_match_bytes() {
    regex!(BytePattern = "test");

    assert!(BytePattern::contains_match(b"this is a test string" as &[u8]));
    assert!(!BytePattern::contains_match(b"nothing here" as &[u8]));
}

#[test]
fn test_contains_match_unicode() {
    regex!(Unicode = "🦀");

    assert!(Unicode::contains_match("Hello 🦀 World"));
    assert!(!Unicode::contains_match("Hello World"));
}

// ============================================================================
// CONTAINS_MATCH TESTS - ANONYMOUS REGEX
// ============================================================================

#[test]
fn test_anon_contains_match_literal() {
    let pattern = regex!("hello");

    assert!(pattern.contains_match("say hello world"));
    assert!(!pattern.contains_match("goodbye world"));
}

#[test]
fn test_anon_contains_match_in_expression() {
    let result = regex!("needle").contains_match("haystack needle here");
    assert!(result);
}

#[test]
fn test_anon_contains_match_pattern() {
    let pattern = regex!("[a-z]+@[a-z]+");

    assert!(pattern.contains_match("contact us at user@example for info"));
    assert!(!pattern.contains_match("no email here"));
}

// ============================================================================
// FIND_MATCH TESTS - NAMED REGEX
// ============================================================================

#[test]
fn test_find_match_literal_present() {
    regex!(Literal = "hello");
    assert_eq!(Literal::slice_match("say hello world"), Some("hello"));
}

#[test]
fn test_find_match_literal_absent() {
    regex!(Literal = "hello");
    assert_eq!(Literal::slice_match("goodbye world"), None);
}

#[test]
fn test_find_match_at_start() {
    regex!(Pattern = "abc");
    assert_eq!(Pattern::slice_match("abcdef"), Some("abc"));
}

#[test]
fn test_find_match_at_end() {
    regex!(Pattern = "xyz");
    assert_eq!(Pattern::slice_match("abcxyz"), Some("xyz"));
}

#[test]
fn test_find_match_in_middle() {
    regex!(Pattern = "middle");
    assert_eq!(Pattern::slice_match("start middle end"), Some("middle"));
}

#[test]
fn test_find_match_exact_string() {
    regex!(Pattern = "exact");
    assert_eq!(Pattern::slice_match("exact"), Some("exact"));
}

#[test]
fn test_find_match_empty_pattern() {
    regex!(Empty = "");
    // Empty pattern matches at the start with empty slice
    assert_eq!(Empty::slice_match("anything"), Some(""));
}

#[test]
fn test_find_match_empty_haystack() {
    regex!(Pattern = "test");
    assert_eq!(Pattern::slice_match(""), None);
}

#[test]
fn test_find_match_pattern_with_quantifier() {
    regex!(Digits = "[0-9]+");
    assert_eq!(Digits::slice_match("abc123def"), Some("123"));
}

#[test]
fn test_find_match_alternation() {
    regex!(Alt = "foo|bar");
    assert_eq!(Alt::slice_match("prefix bar suffix"), Some("bar"));
}

#[test]
fn test_find_match_first_occurrence() {
    regex!(Pattern = "ab");
    // Should find the first occurrence
    assert_eq!(Pattern::slice_match("xxabyyabzz"), Some("ab"));
}

#[test]
fn test_find_match_greedy_quantifier() {
    regex!(Greedy = "a+");
    // Should match all consecutive 'a's
    assert_eq!(Greedy::slice_match("xaaaay"), Some("aaaa"));
}

#[test]
fn test_find_match_bytes() {
    regex!(BytePattern = "test");
    assert_eq!(BytePattern::slice_match(b"this is a test string" as &[u8]), Some(b"test" as &[u8]));
}

#[test]
fn test_find_match_bytes_no_match() {
    regex!(BytePattern = "test");
    assert_eq!(BytePattern::slice_match(b"nothing here" as &[u8]), None);
}

#[test]
fn test_find_match_unicode() {
    regex!(Unicode = "🦀+");
    assert_eq!(Unicode::slice_match("Hello 🦀🦀🦀 World"), Some("🦀🦀🦀"));
}

#[test]
fn test_find_match_complex_pattern() {
    regex!(Email = r"[a-z]+@[a-z]+\.[a-z]+");
    assert_eq!(Email::slice_match("Contact us at user@example.com for info"), Some("user@example.com"));
}

#[test]
fn test_find_match_char_class() {
    regex!(Word = "[a-zA-Z]+");
    assert_eq!(Word::slice_match("123 Hello 456"), Some("Hello"));
}

// ============================================================================
// FIND_MATCH TESTS - ANONYMOUS REGEX
// ============================================================================

#[test]
fn test_anon_find_match_literal() {
    let pattern = regex!("hello");
    assert_eq!(pattern.slice_match("say hello world"), Some("hello"));
}

#[test]
fn test_anon_find_match_pattern() {
    let pattern = regex!("[0-9]+");
    assert_eq!(pattern.slice_match("item 42 in stock"), Some("42"));
}

#[test]
fn test_anon_find_match_no_match() {
    let pattern = regex!("xyz");
    assert_eq!(pattern.slice_match("abc def"), None);
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_contains_match_overlapping_patterns() {
    regex!(Pattern = "aa");
    // Should find "aa" starting at position 0
    assert!(Pattern::contains_match("aaa"));
}

#[test]
fn test_find_match_overlapping_patterns() {
    regex!(Pattern = "aa");
    // Should find first "aa" match
    assert_eq!(Pattern::slice_match("baaa"), Some("aa"));
}

#[test]
fn test_contains_match_partial_match_then_full() {
    regex!(Pattern = "abc");
    // "ab" is a partial match, then "abc" is found
    assert!(Pattern::contains_match("ab abc"));
}

#[test]
fn test_find_match_partial_match_then_full() {
    regex!(Pattern = "abc");
    assert_eq!(Pattern::slice_match("ab abc"), Some("abc"));
}

#[test]
fn test_contains_match_with_anchored_pattern() {
    regex!(Anchored = "^start");

    assert!(Anchored::contains_match("start of string"));
    // The pattern requires ^ anchor, so it won't match in the middle
    assert!(!Anchored::contains_match("not start of string"));
}

#[test]
fn test_find_match_with_end_anchor() {
    regex!(EndAnchored = "end$");
    assert_eq!(EndAnchored::slice_match("this is the end"), Some("end"));
}

#[test]
fn test_find_match_range_quantifier() {
    regex!(Range = "a{2,4}");
    // Should match greedily up to 4 'a's
    assert_eq!(Range::slice_match("xaaaaay"), Some("aaaa"));
}

#[test]
fn test_find_match_optional() {
    regex!(Optional = "colou?r");

    assert_eq!(Optional::slice_match("my favorite color is blue"), Some("color"));
    assert_eq!(Optional::slice_match("my favourite colour is blue"), Some("colour"));
}

#[test]
fn test_contains_and_find_consistency() {
    regex!(Pattern = "test");

    // If contains_match returns true, find_match should return Some
    let contains = Pattern::contains_match("this is a test");
    let found = Pattern::slice_match("this is a test");

    assert!(contains);
    assert!(found.is_some());
}

#[test]
fn test_contains_and_find_consistency_no_match() {
    regex!(Pattern = "xyz");

    // If contains_match returns false, find_match should return None
    let contains = Pattern::contains_match("abc def");
    let found = Pattern::slice_match("abc def");

    assert!(!contains);
    assert!(found.is_none());
}
