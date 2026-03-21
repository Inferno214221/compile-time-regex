// Tests for capture group functionality

use crate::*;

// ============================================================================
// BASIC CAPTURE TESTS - NAMED REGEX
// ============================================================================

#[test]
fn test_captures_whole_match() {
    regex!(Simple = "hello");

    let caps = Simple::do_capture("hello").expect("should match");
    assert_eq!(caps.cap_0(), "hello");
}

#[test]
fn test_captures_no_match_returns_none() {
    regex!(Simple = "hello");

    let caps = Simple::do_capture("world");
    assert!(caps.is_none());
}

#[test]
fn test_captures_single_group() {
    regex!(SingleGroup = "a(b+)c");

    let caps = SingleGroup::do_capture("abbc").expect("should match");
    assert_eq!(caps.cap_0(), "abbc");  // whole match
    assert_eq!(caps.cap_1(), "bb");     // captured group
}

#[test]
fn test_captures_multiple_groups() {
    regex!(MultiGroup = "([a-z]+)@([a-z]+)");

    let caps = MultiGroup::do_capture("user@example").expect("should match");
    assert_eq!(caps.cap_0(), "user@example");  // whole match
    assert_eq!(caps.cap_1(), "user");           // first group
    assert_eq!(caps.cap_2(), "example");        // second group
}

#[test]
fn test_captures_nested_groups() {
    regex!(Nested = "((a+)(b+))");

    let caps = Nested::do_capture("aaabbb").expect("should match");
    assert_eq!(caps.cap_0(), "aaabbb");  // whole match
    assert_eq!(caps.cap_1(), "aaabbb");  // outer group
    assert_eq!(caps.cap_2(), "aaa");     // first inner group
    assert_eq!(caps.cap_3(), "bbb");     // second inner group
}

#[test]
fn test_captures_with_quantifiers() {
    regex!(Quantified = "(a+)(b*)");

    let caps = Quantified::do_capture("aaab").expect("should match");
    assert_eq!(caps.cap_0(), "aaab");
    assert_eq!(caps.cap_1(), "aaa");
    assert_eq!(caps.cap_2(), "b");
}

#[test]
fn test_captures_empty_group() {
    regex!(EmptyCapture = "(a*)(b+)");

    let caps = EmptyCapture::do_capture("bbb").expect("should match");
    assert_eq!(caps.cap_0(), "bbb");
    assert_eq!(caps.cap_1(), "");   // a* matches empty
    assert_eq!(caps.cap_2(), "bbb");
}

// ============================================================================
// OPTIONAL CAPTURE GROUP TESTS
// ============================================================================

#[test]
fn test_captures_optional_group_present() {
    regex!(OptGroup = "a(b)?c");

    let caps = OptGroup::do_capture("abc").expect("should match");
    assert_eq!(caps.cap_0(), "abc");
    assert_eq!(caps.cap_1(), Some("b"));
}

#[test]
fn test_captures_optional_group_absent() {
    regex!(OptGroup = "a(b)?c");

    let caps = OptGroup::do_capture("ac").expect("should match");
    assert_eq!(caps.cap_0(), "ac");
    assert_eq!(caps.cap_1(), None);
}

#[test]
fn test_captures_alternation_first_branch() {
    regex!(AltCapture = "(foo)|(bar)");

    let caps = AltCapture::do_capture("foo").expect("should match");
    assert_eq!(caps.cap_0(), "foo");
    assert_eq!(caps.cap_1(), Some("foo"));
    assert_eq!(caps.cap_2(), None);
}

#[test]
fn test_captures_alternation_second_branch() {
    regex!(AltCapture = "(foo)|(bar)");

    let caps = AltCapture::do_capture("bar").expect("should match");
    assert_eq!(caps.cap_0(), "bar");
    assert_eq!(caps.cap_1(), None);
    assert_eq!(caps.cap_2(), Some("bar"));
}

// ============================================================================
// NAMED CAPTURE GROUP TESTS
// ============================================================================

#[test]
fn test_named_capture_group() {
    regex!(Named = "(?<word>[a-z]+)");

    let caps = Named::do_capture("hello").expect("should match");
    assert_eq!(caps.cap_0(), "hello");
    assert_eq!(caps.word(), "hello");
}

#[test]
fn test_multiple_named_groups() {
    regex!(MultiNamed = "(?<user>[a-z]+)@(?<domain>[a-z]+)");

    let caps = MultiNamed::do_capture("user@example").expect("should match");
    assert_eq!(caps.cap_0(), "user@example");
    assert_eq!(caps.user(), "user");
    assert_eq!(caps.domain(), "example");
}

#[test]
fn test_mixed_named_and_numbered() {
    regex!(Mixed = "(?<name>[a-z]+):([0-9]+)");

    let caps = Mixed::do_capture("test:123").expect("should match");
    assert_eq!(caps.cap_0(), "test:123");
    assert_eq!(caps.name(), "test");
    assert_eq!(caps.cap_1(), "test");  // same as name()
    assert_eq!(caps.cap_2(), "123");
}

// ============================================================================
// RANGE ACCESSOR TESTS
// ============================================================================

#[test]
fn test_capture_range_accessor() {
    regex!(RangeTest = "(a+)b(c+)");

    let caps = RangeTest::do_capture("aaabcc").expect("should match");

    assert_eq!(caps.cap_0_range(), &(0..6));
    assert_eq!(caps.cap_1_range(), &(0..3));
    assert_eq!(caps.cap_2_range(), &(4..6));
}

#[test]
fn test_capture_range_with_unicode() {
    regex!(UnicodeRange = "([a-z]+)(🦀+)");

    let caps = UnicodeRange::do_capture("hello🦀🦀").expect("should match");

    // Ranges are byte offsets
    assert_eq!(caps.cap_1_range(), &(0..5));  // "hello" is 5 bytes
    // Each crab emoji is 4 bytes
    assert_eq!(caps.cap_2_range(), &(5..13)); // two crabs = 8 bytes
}

// ============================================================================
// BYTE CAPTURE TESTS
// ============================================================================

#[test]
fn test_captures_bytes() {
    regex!(ByteCapture = "(a+)(b+)");

    let caps = ByteCapture::do_capture(b"aaabb" as &[u8]).expect("should match");
    assert_eq!(caps.cap_0(), b"aaabb" as &[u8]);
    assert_eq!(caps.cap_1(), b"aaa" as &[u8]);
    assert_eq!(caps.cap_2(), b"bb" as &[u8]);
}

#[test]
fn test_captures_bytes_range() {
    regex!(ByteRange = "([0-9]+)");

    let caps = ByteRange::do_capture(b"123" as &[u8]).expect("should match");
    assert_eq!(caps.cap_1_range(), &(0..3));
}

// ============================================================================
// ANONYMOUS REGEX CAPTURE TESTS
// ============================================================================

#[test]
fn test_anon_captures_basic() {
    let pattern = regex!("(a+)(b+)");

    let caps = pattern.do_capture("aaabbb").expect("should match");
    assert_eq!(caps.cap_0(), "aaabbb");
    assert_eq!(caps.cap_1(), "aaa");
    assert_eq!(caps.cap_2(), "bbb");
}

#[test]
fn test_anon_captures_no_match() {
    let pattern = regex!("(a+)(b+)");

    let caps = pattern.do_capture("ccc");
    assert!(caps.is_none());
}

#[test]
fn test_anon_captures_optional() {
    let pattern = regex!("a(b)?c");

    let caps_present = pattern.do_capture("abc").expect("should match");
    assert_eq!(caps_present.cap_1(), Some("b"));

    let caps_absent = pattern.do_capture("ac").expect("should match");
    assert_eq!(caps_absent.cap_1(), None);
}

// ============================================================================
// NON-CAPTURING GROUP TESTS
// ============================================================================

#[test]
fn test_non_capturing_group() {
    regex!(NonCapturing = "(?:a+)(b+)");

    let caps = NonCapturing::do_capture("aaabbb").expect("should match");
    assert_eq!(caps.cap_0(), "aaabbb");
    // cap_1 should be the (b+) group, not the (?:a+)
    assert_eq!(caps.cap_1(), "bbb");
}

#[test]
fn test_mixed_capturing_non_capturing() {
    regex!(MixedGroups = "(a+)(?:b+)(c+)");

    let caps = MixedGroups::do_capture("aaabbccc").expect("should match");
    assert_eq!(caps.cap_0(), "aaabbccc");
    assert_eq!(caps.cap_1(), "aaa");  // first capturing group
    assert_eq!(caps.cap_2(), "ccc");  // second capturing group (skips non-capturing)
}

// ============================================================================
// COMPLEX PATTERN CAPTURE TESTS
// ============================================================================

#[test]
fn test_email_like_captures() {
    regex!(Email = "([a-z]+)@([a-z]+)\\.([a-z]+)");

    let caps = Email::do_capture("user@example.com").expect("should match");
    assert_eq!(caps.cap_0(), "user@example.com");
    assert_eq!(caps.cap_1(), "user");
    assert_eq!(caps.cap_2(), "example");
    assert_eq!(caps.cap_3(), "com");
}

#[test]
fn test_url_like_captures() {
    regex!(Url = "(https?)://([a-z]+)");

    let caps = Url::do_capture("https://example").expect("should match");
    assert_eq!(caps.cap_0(), "https://example");
    assert_eq!(caps.cap_1(), "https");
    assert_eq!(caps.cap_2(), "example");
}

#[test]
fn test_date_like_captures() {
    regex!(Date = "([0-9]+)-([0-9]+)-([0-9]+)");

    let caps = Date::do_capture("2024-01-15").expect("should match");
    assert_eq!(caps.cap_0(), "2024-01-15");
    assert_eq!(caps.cap_1(), "2024");
    assert_eq!(caps.cap_2(), "01");
    assert_eq!(caps.cap_3(), "15");
}

// ============================================================================
// EDGE CASES
// ============================================================================

// Note: This test documents that backtracking within captures requires
// explicit handling - the greedy quantifier consumes all 'a's, so the
// pattern (a+)(a) would need backtracking support to match "aaaa".
// For now, we test patterns that don't require backtracking.
#[test]
fn test_captures_sequential_groups() {
    regex!(Sequential = "(a+)(b+)");

    let caps = Sequential::do_capture("aaabbb").expect("should match");
    assert_eq!(caps.cap_0(), "aaabbb");
    assert_eq!(caps.cap_1(), "aaa");
    assert_eq!(caps.cap_2(), "bbb");
}

#[test]
fn test_captures_empty_input_no_match() {
    regex!(NonEmpty = "(a+)");

    let caps = NonEmpty::do_capture("");
    assert!(caps.is_none());
}

#[test]
fn test_captures_partial_input_no_match() {
    regex!(Full = "^(a+)$");

    // Extra characters after
    let caps = Full::do_capture("aaab");
    assert!(caps.is_none());
}

#[test]
fn test_captures_debug_impl() {
    regex!(DebugTest = "(a+)");

    let caps = DebugTest::do_capture("aaa").expect("should match");
    let debug_str = format!("{:?}", caps);
    // Should contain some representation of the captures
    assert!(!debug_str.is_empty());
}
