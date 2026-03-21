use std::ops::Range;

use super::*;
use crate::haystack::{Haystack, HaystackItem};
use crate::matcher::{Scalar, Then, Byte};

// Stub captures type for testing - doesn't capture anything
#[derive(Debug)]
struct NoCaptures;

impl<'a, I: HaystackItem> CaptureFromRanges<'a, I, 1> for NoCaptures {
    fn from_ranges(_captures: [Option<Range<usize>>; 1], _hay: Haystack<'a, I>) -> Option<Self> {
        Some(NoCaptures)
    }
}

// Test struct implementing Regex trait
#[derive(Debug)]
struct TestRegexChar;

impl Regex<char, 1> for TestRegexChar {
    type Pattern = Scalar<'a'>;
    type Capture<'a> = NoCaptures;
}

// Test struct implementing Regex trait with bytes
#[derive(Debug)]
struct TestRegexByte;

impl Regex<u8, 1> for TestRegexByte {
    type Pattern = Byte<b'x'>;
    type Capture<'a> = NoCaptures;
}

// Test struct implementing Regex trait with complex pattern
#[derive(Debug)]
struct TestRegexComplex;

impl Regex<char, 1> for TestRegexComplex {
    type Pattern = Then<char, Scalar<'h'>, Scalar<'i'>>;
    type Capture<'a> = NoCaptures;
}

// Test struct implementing AnonRegex trait
#[derive(Debug)]
struct TestAnonRegexChar;

impl Regex<char, 1> for TestAnonRegexChar {
    type Pattern = Scalar<'b'>;
    type Capture<'a> = NoCaptures;
}

impl AnonRegex<char, 1> for TestAnonRegexChar {}

// Test struct implementing AnonRegex trait with bytes
#[derive(Debug)]
struct TestAnonRegexByte;

impl Regex<u8, 1> for TestAnonRegexByte {
    type Pattern = Byte<b'y'>;
    type Capture<'a> = NoCaptures;
}

impl AnonRegex<u8, 1> for TestAnonRegexByte {}

// Tests for Regex trait with chars
#[test]
fn test_regex_char_matches() {
    assert!(TestRegexChar::is_match("a"));
}

#[test]
fn test_regex_char_no_match() {
    assert!(!TestRegexChar::is_match("b"));
}

#[test]
fn test_regex_char_empty() {
    assert!(!TestRegexChar::is_match(""));
}

// Tests for Regex trait with bytes
#[test]
fn test_regex_byte_matches() {
    assert!(TestRegexByte::is_match(b"x" as &[u8]));
}

#[test]
fn test_regex_byte_no_match() {
    assert!(!TestRegexByte::is_match(b"z" as &[u8]));
}

#[test]
fn test_regex_byte_empty() {
    assert!(!TestRegexByte::is_match(b"" as &[u8]));
}

// Tests for Regex trait with complex patterns
#[test]
fn test_regex_complex_matches() {
    assert!(TestRegexComplex::is_match("hi"));
}

#[test]
fn test_regex_complex_partial_match() {
    assert!(!TestRegexComplex::is_match("h"));
}

#[test]
fn test_regex_complex_no_match() {
    assert!(!TestRegexComplex::is_match("hello"));
}

#[test]
fn test_regex_complex_wrong_order() {
    assert!(!TestRegexComplex::is_match("ih"));
}

// Tests for AnonRegex trait with chars
#[test]
fn test_anon_regex_char_matches() {
    let regex = TestAnonRegexChar;
    assert!(regex.is_match("b"));
}

#[test]
fn test_anon_regex_char_no_match() {
    let regex = TestAnonRegexChar;
    assert!(!regex.is_match("a"));
}

#[test]
fn test_anon_regex_char_empty() {
    let regex = TestAnonRegexChar;
    assert!(!regex.is_match(""));
}

// Tests for AnonRegex trait with bytes
#[test]
fn test_anon_regex_byte_matches() {
    let regex = TestAnonRegexByte;
    assert!(regex.is_match(b"y" as &[u8]));
}

#[test]
fn test_anon_regex_byte_no_match() {
    let regex = TestAnonRegexByte;
    assert!(!regex.is_match(b"z" as &[u8]));
}

#[test]
fn test_anon_regex_byte_empty() {
    let regex = TestAnonRegexByte;
    assert!(!regex.is_match(b"" as &[u8]));
}

// Tests for behavior differences between Regex and AnonRegex
#[test]
fn test_regex_is_static_method() {
    // Regex is called as a static method
    assert!(TestRegexChar::is_match("a"));
}

#[test]
fn test_anon_regex_is_instance_method() {
    let regex = TestAnonRegexChar;
    // AnonRegex is called as an instance method
    assert!(regex.is_match("b"));
}

// Tests for multiple matches on same pattern
#[test]
fn test_regex_multiple_attempts() {
    assert!(TestRegexChar::is_match("a"));
    assert!(TestRegexChar::is_match("a"));
}

#[test]
fn test_anon_regex_multiple_attempts() {
    let regex = TestAnonRegexChar;

    assert!(regex.is_match("b"));
    assert!(regex.is_match("b"));
}

// Tests for partial matches (full haystack not consumed)
#[test]
fn test_regex_requires_full_match() {
    // "abc" contains 'a' but is_match requires entire haystack to match
    assert!(!TestRegexChar::is_match("abc"));
}

#[test]
fn test_anon_regex_requires_full_match() {
    let regex = TestAnonRegexChar;
    // "bcd" contains 'b' but is_match requires entire haystack to match
    assert!(!regex.is_match("bcd"));
}

// ============================================================================
// INDEXED CAPTURES TESTS
// ============================================================================

#[test]
fn test_indexed_captures_default_is_empty() {
    let caps = IndexedCaptures::default();
    assert!(caps.0.is_empty());
}

#[test]
fn test_indexed_captures_push_single() {
    let mut caps = IndexedCaptures::default();
    caps.push(0, 0..5);

    assert!(!caps.0.is_empty());
    let inner = caps.0.inner.as_ref().unwrap();
    assert_eq!(inner.index, 0);
    assert_eq!(inner.cap, 0..5);
}

#[test]
fn test_indexed_captures_push_multiple() {
    let mut caps = IndexedCaptures::default();
    caps.push(0, 0..10);
    caps.push(1, 0..5);
    caps.push(2, 5..10);

    // Most recent push should be at the front
    let inner = caps.0.inner.as_ref().unwrap();
    assert_eq!(inner.index, 2);
    assert_eq!(inner.cap, 5..10);
}

#[test]
fn test_indexed_captures_into_array_single() {
    let mut caps = IndexedCaptures::default();
    caps.push(0, 0..5);

    let arr: [Option<Range<usize>>; 1] = caps.into_array();
    assert_eq!(arr[0], Some(0..5));
}

#[test]
fn test_indexed_captures_into_array_multiple() {
    let mut caps = IndexedCaptures::default();
    caps.push(0, 0..10);
    caps.push(1, 0..5);
    caps.push(2, 5..10);

    let arr: [Option<Range<usize>>; 3] = caps.into_array();
    assert_eq!(arr[0], Some(0..10));
    assert_eq!(arr[1], Some(0..5));
    assert_eq!(arr[2], Some(5..10));
}

#[test]
fn test_indexed_captures_into_array_with_gaps() {
    let mut caps = IndexedCaptures::default();
    caps.push(0, 0..10);
    caps.push(2, 5..10);
    // index 1 is not pushed

    let arr: [Option<Range<usize>>; 3] = caps.into_array();
    assert_eq!(arr[0], Some(0..10));
    assert_eq!(arr[1], None);  // gap
    assert_eq!(arr[2], Some(5..10));
}

#[test]
fn test_indexed_captures_keeps_last_for_duplicates() {
    let mut caps = IndexedCaptures::default();
    caps.push(0, 0..5);
    caps.push(0, 0..10);  // duplicate index, pushed later

    let arr: [Option<Range<usize>>; 1] = caps.into_array();
    // Keeps the last one pushed (0..10), since into_array traverses backwards
    // and only writes if the slot is None
    assert_eq!(arr[0], Some(0..10));
}

#[test]
fn test_indexed_captures_clone_independence() {
    let mut caps = IndexedCaptures::default();
    caps.push(0, 0..5);

    let caps_clone = caps.clone();

    caps.push(1, 5..10);

    // Original should have two captures
    let arr_orig: [Option<Range<usize>>; 2] = caps.into_array();
    assert_eq!(arr_orig[0], Some(0..5));
    assert_eq!(arr_orig[1], Some(5..10));

    // Clone should still only have one
    let arr_clone: [Option<Range<usize>>; 2] = caps_clone.into_array();
    assert_eq!(arr_clone[0], Some(0..5));
    assert_eq!(arr_clone[1], None);
}

// ============================================================================
// FROM CAPTURES TRAIT TESTS
// ============================================================================

// Test captures struct that stores actual capture data
#[derive(Debug)]
struct TestCaptures<'a> {
    hay: Haystack<'a, char>,
    cap0: Range<usize>,
    cap1: Option<Range<usize>>,
}

impl<'a> CaptureFromRanges<'a, char, 2> for TestCaptures<'a> {
    fn from_ranges(captures: [Option<Range<usize>>; 2], hay: Haystack<'a, char>) -> Option<Self> {
        Some(TestCaptures {
            hay,
            cap0: captures[0].clone()?,
            cap1: captures[1].clone(),
        })
    }
}

#[test]
fn test_from_captures_basic() {
    let hay = Haystack::from("hello");
    let captures = [Some(0..5), Some(0..3)];

    let result = TestCaptures::from_ranges(captures, hay);
    assert!(result.is_some());

    let caps = result.unwrap();
    assert_eq!(caps.cap0, 0..5);
    assert_eq!(caps.cap1, Some(0..3));
}

#[test]
fn test_from_captures_with_none() {
    let hay = Haystack::from("hello");
    let captures = [Some(0..5), None];

    let result = TestCaptures::from_ranges(captures, hay);
    assert!(result.is_some());

    let caps = result.unwrap();
    assert_eq!(caps.cap0, 0..5);
    assert_eq!(caps.cap1, None);
}

#[test]
fn test_from_captures_required_missing_returns_none() {
    let hay = Haystack::from("hello");
    let captures = [None, Some(0..3)];  // cap0 is required but None

    let result = TestCaptures::from_ranges(captures, hay);
    assert!(result.is_none());
}

#[test]
fn test_from_captures_slicing() {
    let hay = Haystack::from("hello world");
    let captures = [Some(0..5), Some(6..11)];

    let result = TestCaptures::from_ranges(captures, hay);
    let caps = result.unwrap();

    assert_eq!(caps.hay.slice(caps.cap0.clone()), "hello");
    assert_eq!(caps.hay.slice(caps.cap1.clone().unwrap()), "world");
}

// ============================================================================
// CAPTURES METHOD INTEGRATION TESTS
// ============================================================================

// A more complete test regex that actually uses captures
use crate::matcher::CaptureGroup;

#[derive(Debug)]
struct TestRegexWithCaptures;

// Captures struct for TestRegexWithCaptures
#[derive(Debug)]
struct TwoGroupCaptures<'a> {
    hay: Haystack<'a, char>,
    whole_match: Range<usize>,
    group1: Range<usize>,
}

impl<'a> CaptureFromRanges<'a, char, 2> for TwoGroupCaptures<'a> {
    fn from_ranges(captures: [Option<Range<usize>>; 2], hay: Haystack<'a, char>) -> Option<Self> {
        Some(TwoGroupCaptures {
            hay,
            whole_match: captures[0].clone()?,
            group1: captures[1].clone()?,
        })
    }
}

impl<'a> TwoGroupCaptures<'a> {
    fn whole_match(&'a self) -> &'a str {
        self.hay.slice(self.whole_match.clone())
    }

    fn group1(&'a self) -> &'a str {
        self.hay.slice(self.group1.clone())
    }
}

impl Regex<char, 2> for TestRegexWithCaptures {
    // Pattern: (a+) - matches one or more 'a' and captures it
    type Pattern = CaptureGroup<char, crate::matcher::QuantifierNOrMore<char, Scalar<'a'>, 1>, 1>;
    type Capture<'a> = TwoGroupCaptures<'a>;
}

#[test]
fn test_regex_captures_method() {
    let caps = TestRegexWithCaptures::do_capture("aaa");

    assert!(caps.is_some());
    let caps = caps.unwrap();

    assert_eq!(caps.whole_match(), "aaa");
    assert_eq!(caps.group1(), "aaa");
}

#[test]
fn test_regex_captures_no_match() {
    let caps = TestRegexWithCaptures::do_capture("bbb");
    assert!(caps.is_none());
}

#[test]
fn test_regex_captures_requires_full_match() {
    // do_capture requires the entire haystack to be consumed, so "aaab" doesn't match (a+)
    let caps = TestRegexWithCaptures::do_capture("aaab");
    assert!(caps.is_none());
}
