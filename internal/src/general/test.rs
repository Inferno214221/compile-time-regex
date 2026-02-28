use super::*;
use crate::haystack::{Haystack, HaystackItem};
use crate::matcher::{Scalar, Then, Byte};

// Stub captures type for testing - doesn't capture anything
struct NoCaptures;

impl<'a, I: HaystackItem> FromCaptures<'a, I, 1> for NoCaptures {
    fn from_captures(_captures: [Option<Capture>; 1], _hay: Haystack<'a, I>) -> Option<Self> {
        Some(NoCaptures)
    }
}

// Test struct implementing Regex trait
struct TestRegexChar;

impl Regex<char, 1> for TestRegexChar {
    type Pattern = Scalar<'a'>;
    type Captures<'a> = NoCaptures;
}

// Test struct implementing Regex trait with bytes
struct TestRegexByte;

impl Regex<u8, 1> for TestRegexByte {
    type Pattern = Byte<b'x'>;
    type Captures<'a> = NoCaptures;
}

// Test struct implementing Regex trait with complex pattern
struct TestRegexComplex;

impl Regex<char, 1> for TestRegexComplex {
    type Pattern = Then<char, Scalar<'h'>, Scalar<'i'>>;
    type Captures<'a> = NoCaptures;
}

// Test struct implementing AnonRegex trait
struct TestAnonRegexChar;

impl AnonRegex<char, 1> for TestAnonRegexChar {
    type Pattern = Scalar<'b'>;
    type Captures<'a> = NoCaptures;
}

// Test struct implementing AnonRegex trait with bytes
struct TestAnonRegexByte;

impl AnonRegex<u8, 1> for TestAnonRegexByte {
    type Pattern = Byte<b'y'>;
    type Captures<'a> = NoCaptures;
}

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
