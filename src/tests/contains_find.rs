// Tests that verify contains_match and find_match methods work correctly

use crate::*;

// ============================================================================
// CONTAINS_MATCH TESTS - NAMED REGEX
// ============================================================================

#[test]
fn test_contains_match_literal_present() {
    regex!(Literal = "hello");

    let mut hay = Haystack::from("say hello world");
    assert!(Literal::contains_match(&mut hay));
}

#[test]
fn test_contains_match_literal_absent() {
    regex!(Literal = "hello");

    let mut hay = Haystack::from("goodbye world");
    assert!(!Literal::contains_match(&mut hay));
}

#[test]
fn test_contains_match_at_start() {
    regex!(Pattern = "abc");

    let mut hay = Haystack::from("abcdef");
    assert!(Pattern::contains_match(&mut hay));
}

#[test]
fn test_contains_match_at_end() {
    regex!(Pattern = "xyz");

    let mut hay = Haystack::from("abcxyz");
    assert!(Pattern::contains_match(&mut hay));
}

#[test]
fn test_contains_match_in_middle() {
    regex!(Pattern = "middle");

    let mut hay = Haystack::from("start middle end");
    assert!(Pattern::contains_match(&mut hay));
}

#[test]
fn test_contains_match_exact_string() {
    regex!(Pattern = "exact");

    let mut hay = Haystack::from("exact");
    assert!(Pattern::contains_match(&mut hay));
}

#[test]
fn test_contains_match_empty_pattern() {
    regex!(Empty = "");

    let mut hay = Haystack::from("anything");
    assert!(Empty::contains_match(&mut hay));
}

#[test]
fn test_contains_match_empty_haystack() {
    regex!(Pattern = "test");

    let mut hay = Haystack::from("");
    assert!(!Pattern::contains_match(&mut hay));
}

#[test]
fn test_contains_match_pattern_with_quantifier() {
    regex!(Digits = "[0-9]+");

    let mut hay_has_digits = Haystack::from("abc123def");
    let mut hay_no_digits = Haystack::from("abcdef");

    assert!(Digits::contains_match(&mut hay_has_digits));
    assert!(!Digits::contains_match(&mut hay_no_digits));
}

#[test]
fn test_contains_match_alternation() {
    regex!(Alt = "foo|bar");

    let mut hay_foo = Haystack::from("prefix foo suffix");
    let mut hay_bar = Haystack::from("prefix bar suffix");
    let mut hay_none = Haystack::from("prefix baz suffix");

    assert!(Alt::contains_match(&mut hay_foo));
    assert!(Alt::contains_match(&mut hay_bar));
    assert!(!Alt::contains_match(&mut hay_none));
}

#[test]
fn test_contains_match_multiple_occurrences() {
    regex!(Pattern = "ab");

    let mut hay = Haystack::from("ab ab ab");
    // Should find the first occurrence
    assert!(Pattern::contains_match(&mut hay));
}

#[test]
fn test_contains_match_bytes() {
    regex!(BytePattern = "test");

    let mut hay_match = Haystack::from(b"this is a test string" as &[u8]);
    let mut hay_no_match = Haystack::from(b"nothing here" as &[u8]);

    assert!(BytePattern::contains_match(&mut hay_match));
    assert!(!BytePattern::contains_match(&mut hay_no_match));
}

#[test]
fn test_contains_match_unicode() {
    regex!(Unicode = "🦀");

    let mut hay_has_crab = Haystack::from("Hello 🦀 World");
    let mut hay_no_crab = Haystack::from("Hello World");

    assert!(Unicode::contains_match(&mut hay_has_crab));
    assert!(!Unicode::contains_match(&mut hay_no_crab));
}

// ============================================================================
// CONTAINS_MATCH TESTS - ANONYMOUS REGEX
// ============================================================================

#[test]
fn test_anon_contains_match_literal() {
    let pattern = regex!("hello");

    let mut hay_match = Haystack::from("say hello world");
    let mut hay_no_match = Haystack::from("goodbye world");

    assert!(pattern.contains_match(&mut hay_match));
    assert!(!pattern.contains_match(&mut hay_no_match));
}

#[test]
fn test_anon_contains_match_in_expression() {
    let result = regex!("needle").contains_match(&mut Haystack::from("haystack needle here"));
    assert!(result);
}

#[test]
fn test_anon_contains_match_pattern() {
    let pattern = regex!("[a-z]+@[a-z]+");

    let mut hay_has_email = Haystack::from("contact us at user@example for info");
    let mut hay_no_email = Haystack::from("no email here");

    assert!(pattern.contains_match(&mut hay_has_email));
    assert!(!pattern.contains_match(&mut hay_no_email));
}

// ============================================================================
// FIND_MATCH TESTS - NAMED REGEX
// ============================================================================

#[test]
fn test_find_match_literal_present() {
    regex!(Literal = "hello");

    let mut hay = Haystack::from("say hello world");
    let result = Literal::find_match(&mut hay);

    assert_eq!(result, Some("hello"));
}

#[test]
fn test_find_match_literal_absent() {
    regex!(Literal = "hello");

    let mut hay = Haystack::from("goodbye world");
    let result = Literal::find_match(&mut hay);

    assert_eq!(result, None);
}

#[test]
fn test_find_match_at_start() {
    regex!(Pattern = "abc");

    let mut hay = Haystack::from("abcdef");
    let result = Pattern::find_match(&mut hay);

    assert_eq!(result, Some("abc"));
}

#[test]
fn test_find_match_at_end() {
    regex!(Pattern = "xyz");

    let mut hay = Haystack::from("abcxyz");
    let result = Pattern::find_match(&mut hay);

    assert_eq!(result, Some("xyz"));
}

#[test]
fn test_find_match_in_middle() {
    regex!(Pattern = "middle");

    let mut hay = Haystack::from("start middle end");
    let result = Pattern::find_match(&mut hay);

    assert_eq!(result, Some("middle"));
}

#[test]
fn test_find_match_exact_string() {
    regex!(Pattern = "exact");

    let mut hay = Haystack::from("exact");
    let result = Pattern::find_match(&mut hay);

    assert_eq!(result, Some("exact"));
}

#[test]
fn test_find_match_empty_pattern() {
    regex!(Empty = "");

    let mut hay = Haystack::from("anything");
    let result = Empty::find_match(&mut hay);

    // Empty pattern matches at the start with empty slice
    assert_eq!(result, Some(""));
}

#[test]
fn test_find_match_empty_haystack() {
    regex!(Pattern = "test");

    let mut hay = Haystack::from("");
    let result = Pattern::find_match(&mut hay);

    assert_eq!(result, None);
}

#[test]
fn test_find_match_pattern_with_quantifier() {
    regex!(Digits = "[0-9]+");

    let mut hay = Haystack::from("abc123def");
    let result = Digits::find_match(&mut hay);

    assert_eq!(result, Some("123"));
}

#[test]
fn test_find_match_alternation() {
    regex!(Alt = "foo|bar");

    let mut hay = Haystack::from("prefix bar suffix");
    let result = Alt::find_match(&mut hay);

    assert_eq!(result, Some("bar"));
}

#[test]
fn test_find_match_first_occurrence() {
    regex!(Pattern = "ab");

    let mut hay = Haystack::from("xxabyyabzz");
    let result = Pattern::find_match(&mut hay);

    // Should find the first occurrence
    assert_eq!(result, Some("ab"));
}

#[test]
fn test_find_match_greedy_quantifier() {
    regex!(Greedy = "a+");

    let mut hay = Haystack::from("xaaaay");
    let result = Greedy::find_match(&mut hay);

    // Should match all consecutive 'a's
    assert_eq!(result, Some("aaaa"));
}

#[test]
fn test_find_match_bytes() {
    regex!(BytePattern = "test");

    let mut hay = Haystack::from(b"this is a test string" as &[u8]);
    let result = BytePattern::find_match(&mut hay);

    assert_eq!(result, Some(b"test" as &[u8]));
}

#[test]
fn test_find_match_bytes_no_match() {
    regex!(BytePattern = "test");

    let mut hay = Haystack::from(b"nothing here" as &[u8]);
    let result = BytePattern::find_match(&mut hay);

    assert_eq!(result, None);
}

#[test]
fn test_find_match_unicode() {
    regex!(Unicode = "🦀+");

    let mut hay = Haystack::from("Hello 🦀🦀🦀 World");
    let result = Unicode::find_match(&mut hay);

    assert_eq!(result, Some("🦀🦀🦀"));
}

#[test]
fn test_find_match_complex_pattern() {
    regex!(Email = r"[a-z]+@[a-z]+\.[a-z]+");

    let mut hay = Haystack::from("Contact us at user@example.com for info");
    let result = Email::find_match(&mut hay);

    assert_eq!(result, Some("user@example.com"));
}

#[test]
fn test_find_match_char_class() {
    regex!(Word = "[a-zA-Z]+");

    let mut hay = Haystack::from("123 Hello 456");
    let result = Word::find_match(&mut hay);

    assert_eq!(result, Some("Hello"));
}

// ============================================================================
// FIND_MATCH TESTS - ANONYMOUS REGEX
// ============================================================================

#[test]
fn test_anon_find_match_literal() {
    let pattern = regex!("hello");

    let mut hay = Haystack::from("say hello world");
    let result = pattern.find_match(&mut hay);

    assert_eq!(result, Some("hello"));
}

#[test]
fn test_anon_find_match_pattern() {
    let pattern = regex!("[0-9]+");

    let mut hay = Haystack::from("item 42 in stock");
    let result = pattern.find_match(&mut hay);

    assert_eq!(result, Some("42"));
}

#[test]
fn test_anon_find_match_no_match() {
    let pattern = regex!("xyz");

    let mut hay = Haystack::from("abc def");
    let result = pattern.find_match(&mut hay);

    assert_eq!(result, None);
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_contains_match_overlapping_patterns() {
    regex!(Pattern = "aa");

    let mut hay = Haystack::from("aaa");
    // Should find "aa" starting at position 0
    assert!(Pattern::contains_match(&mut hay));
}

#[test]
fn test_find_match_overlapping_patterns() {
    regex!(Pattern = "aa");

    let mut hay = Haystack::from("baaa");
    let result = Pattern::find_match(&mut hay);

    // Should find first "aa" match
    assert_eq!(result, Some("aa"));
}

#[test]
fn test_contains_match_partial_match_then_full() {
    regex!(Pattern = "abc");

    // "ab" is a partial match, then "abc" is found
    let mut hay = Haystack::from("ab abc");
    assert!(Pattern::contains_match(&mut hay));
}

#[test]
fn test_find_match_partial_match_then_full() {
    regex!(Pattern = "abc");

    let mut hay = Haystack::from("ab abc");
    let result = Pattern::find_match(&mut hay);

    assert_eq!(result, Some("abc"));
}

#[test]
fn test_contains_match_with_anchored_pattern() {
    regex!(Anchored = "^start");

    let mut hay_at_start = Haystack::from("start of string");
    let mut hay_not_at_start = Haystack::from("not start of string");

    assert!(Anchored::contains_match(&mut hay_at_start));
    // The pattern requires ^ anchor, so it won't match in the middle
    assert!(!Anchored::contains_match(&mut hay_not_at_start));
}

#[test]
fn test_find_match_with_end_anchor() {
    regex!(EndAnchored = "end$");

    let mut hay_at_end = Haystack::from("this is the end");
    let result = EndAnchored::find_match(&mut hay_at_end);

    assert_eq!(result, Some("end"));
}

#[test]
fn test_find_match_range_quantifier() {
    regex!(Range = "a{2,4}");

    let mut hay = Haystack::from("xaaaaay");
    let result = Range::find_match(&mut hay);

    // Should match greedily up to 4 'a's
    assert_eq!(result, Some("aaaa"));
}

#[test]
fn test_find_match_optional() {
    regex!(Optional = "colou?r");

    let mut hay_us = Haystack::from("my favorite color is blue");
    let mut hay_uk = Haystack::from("my favourite colour is blue");

    assert_eq!(Optional::find_match(&mut hay_us), Some("color"));
    assert_eq!(Optional::find_match(&mut hay_uk), Some("colour"));
}

#[test]
fn test_contains_and_find_consistency() {
    regex!(Pattern = "test");

    let mut hay1 = Haystack::from("this is a test");
    let mut hay2 = Haystack::from("this is a test");

    // If contains_match returns true, find_match should return Some
    let contains = Pattern::contains_match(&mut hay1);
    let found = Pattern::find_match(&mut hay2);

    assert!(contains);
    assert!(found.is_some());
}

#[test]
fn test_contains_and_find_consistency_no_match() {
    regex!(Pattern = "xyz");

    let mut hay1 = Haystack::from("abc def");
    let mut hay2 = Haystack::from("abc def");

    // If contains_match returns false, find_match should return None
    let contains = Pattern::contains_match(&mut hay1);
    let found = Pattern::find_match(&mut hay2);

    assert!(!contains);
    assert!(found.is_none());
}
