use super::*;
use crate::haystack::Haystack;
use crate::matcher::{Scalar, Then, Byte};

// Test struct implementing Regex trait
struct TestRegexChar;

impl Regex<char> for TestRegexChar {
    type Pattern = Scalar<'a'>;
}

// Test struct implementing Regex trait with bytes
struct TestRegexByte;

impl Regex<u8> for TestRegexByte {
    type Pattern = Byte<b'x'>;
}

// Test struct implementing Regex trait with complex pattern
struct TestRegexComplex;

impl Regex<char> for TestRegexComplex {
    type Pattern = Then<char, Scalar<'h'>, Scalar<'i'>>;
}

// Test struct implementing AnonRegex trait
struct TestAnonRegexChar;

impl AnonRegex<char> for TestAnonRegexChar {
    type Pattern = Scalar<'b'>;
}

// Test struct implementing AnonRegex trait with bytes
struct TestAnonRegexByte;

impl AnonRegex<u8> for TestAnonRegexByte {
    type Pattern = Byte<b'y'>;
}

// Tests for Regex trait with chars
#[test]
fn test_regex_char_matches() {
    let mut hay = Haystack::from("a");
    assert!(TestRegexChar::is_match(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_regex_char_no_match() {
    let mut hay = Haystack::from("b");
    assert!(!TestRegexChar::is_match(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_regex_char_empty() {
    let mut hay = Haystack::from("");
    assert!(!TestRegexChar::is_match(&mut hay));
}

// Tests for Regex trait with bytes
#[test]
fn test_regex_byte_matches() {
    let mut hay = Haystack::from(b"x" as &[u8]);
    assert!(TestRegexByte::is_match(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_regex_byte_no_match() {
    let mut hay = Haystack::from(b"z" as &[u8]);
    assert!(!TestRegexByte::is_match(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_regex_byte_empty() {
    let mut hay = Haystack::from(b"" as &[u8]);
    assert!(!TestRegexByte::is_match(&mut hay));
}

// Tests for Regex trait with complex patterns
#[test]
fn test_regex_complex_matches() {
    let mut hay = Haystack::from("hi");
    assert!(TestRegexComplex::is_match(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_regex_complex_partial_match() {
    let mut hay = Haystack::from("h");
    assert!(!TestRegexComplex::is_match(&mut hay));
}

#[test]
fn test_regex_complex_no_match() {
    let mut hay = Haystack::from("hello");
    assert!(!TestRegexComplex::is_match(&mut hay));
}

#[test]
fn test_regex_complex_wrong_order() {
    let mut hay = Haystack::from("ih");
    assert!(!TestRegexComplex::is_match(&mut hay));
}

// Tests for AnonRegex trait with chars
#[test]
fn test_anon_regex_char_matches() {
    let regex = TestAnonRegexChar;
    let mut hay = Haystack::from("b");
    assert!(regex.is_match(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_anon_regex_char_no_match() {
    let regex = TestAnonRegexChar;
    let mut hay = Haystack::from("a");
    assert!(!regex.is_match(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_anon_regex_char_empty() {
    let regex = TestAnonRegexChar;
    let mut hay = Haystack::from("");
    assert!(!regex.is_match(&mut hay));
}

// Tests for AnonRegex trait with bytes
#[test]
fn test_anon_regex_byte_matches() {
    let regex = TestAnonRegexByte;
    let mut hay = Haystack::from(b"y" as &[u8]);
    assert!(regex.is_match(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_anon_regex_byte_no_match() {
    let regex = TestAnonRegexByte;
    let mut hay = Haystack::from(b"z" as &[u8]);
    assert!(!regex.is_match(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_anon_regex_byte_empty() {
    let regex = TestAnonRegexByte;
    let mut hay = Haystack::from(b"" as &[u8]);
    assert!(!regex.is_match(&mut hay));
}

// Tests for behavior differences between Regex and AnonRegex
#[test]
fn test_regex_is_static_method() {
    let mut hay = Haystack::from("a");
    // Regex is called as a static method
    assert!(TestRegexChar::is_match(&mut hay));
}

#[test]
fn test_anon_regex_is_instance_method() {
    let mut hay = Haystack::from("b");
    let regex = TestAnonRegexChar;
    // AnonRegex is called as an instance method
    assert!(regex.is_match(&mut hay));
}

// Tests for multiple matches on same haystack
#[test]
fn test_regex_multiple_attempts() {
    let mut hay1 = Haystack::from("a");
    let mut hay2 = Haystack::from("a");

    assert!(TestRegexChar::is_match(&mut hay1));
    assert!(TestRegexChar::is_match(&mut hay2));
}

#[test]
fn test_anon_regex_multiple_attempts() {
    let regex = TestAnonRegexChar;
    let mut hay1 = Haystack::from("b");
    let mut hay2 = Haystack::from("b");

    assert!(regex.is_match(&mut hay1));
    assert!(regex.is_match(&mut hay2));
}

// Tests for haystack state after matching
#[test]
fn test_regex_consumes_on_match() {
    let mut hay = Haystack::from("abc");
    TestRegexChar::is_match(&mut hay);
    // 'a' should be consumed, leaving 'bc'
    assert_eq!(hay.item(), Some('b'));
}

#[test]
fn test_regex_does_not_consume_on_failure() {
    let mut hay = Haystack::from("bcd");
    TestRegexChar::is_match(&mut hay);
    // Nothing should be consumed since match failed
    assert_eq!(hay.item(), Some('b'));
}

#[test]
fn test_anon_regex_consumes_on_match() {
    let regex = TestAnonRegexChar;
    let mut hay = Haystack::from("bcd");
    regex.is_match(&mut hay);
    // 'b' should be consumed, leaving 'cd'
    assert_eq!(hay.item(), Some('c'));
}

#[test]
fn test_anon_regex_does_not_consume_on_failure() {
    let regex = TestAnonRegexChar;
    let mut hay = Haystack::from("acd");
    regex.is_match(&mut hay);
    // Nothing should be consumed since match failed
    assert_eq!(hay.item(), Some('a'));
}
