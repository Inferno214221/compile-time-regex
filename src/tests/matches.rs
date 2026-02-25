// Tests that verify the generated regex patterns match correctly

use crate::*;

// ============================================================================
// BASIC MATCHING TESTS
// ============================================================================

#[test]
fn test_literal_match() {
    regex!(Literal = "hello");

    let mut hay_match = Haystack::from("hello");
    let mut hay_no_match = Haystack::from("world");

    assert!(Literal::matches(&mut hay_match));
    assert!(!Literal::matches(&mut hay_no_match));
}

#[test]
fn test_literal_match_bytes() {
    regex!(LiteralBytes = "test");

    let mut hay_match = Haystack::from(b"test" as &[u8]);
    let mut hay_no_match = Haystack::from(b"fail" as &[u8]);

    assert!(LiteralBytes::matches(&mut hay_match));
    assert!(!LiteralBytes::matches(&mut hay_no_match));
}

#[test]
fn test_anon_regex_literal() {
    let mut hay_match = Haystack::from("pattern");
    let mut hay_no_match = Haystack::from("other");

    assert!(anon_regex!("pattern").matches(&mut hay_match));
    assert!(!anon_regex!("pattern").matches(&mut hay_no_match));
}

// ============================================================================
// ALTERNATION TESTS
// ============================================================================

#[test]
fn test_alternation_first_branch() {
    regex!(Alt = "foo|bar|baz");

    let mut hay = Haystack::from("foo");
    assert!(Alt::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_alternation_second_branch() {
    regex!(Alt = "foo|bar|baz");

    let mut hay = Haystack::from("bar");
    assert!(Alt::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_alternation_third_branch() {
    regex!(Alt = "foo|bar|baz");

    let mut hay = Haystack::from("baz");
    assert!(Alt::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_alternation_no_match() {
    regex!(Alt = "foo|bar|baz");

    let mut hay = Haystack::from("qux");
    assert!(!Alt::matches(&mut hay));
}

// ============================================================================
// QUANTIFIER TESTS
// ============================================================================

#[test]
fn test_zero_or_more_zero_matches() {
    regex!(ZeroOrMore = "a*");

    let mut hay = Haystack::from("");
    assert!(ZeroOrMore::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_zero_or_more_multiple_matches() {
    regex!(ZeroOrMore = "a*");

    let mut hay = Haystack::from("aaa");
    assert!(ZeroOrMore::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_one_or_more_single_match() {
    regex!(OneOrMore = "b+");

    let mut hay = Haystack::from("b");
    assert!(OneOrMore::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_one_or_more_multiple_matches() {
    regex!(OneOrMore = "b+");

    let mut hay = Haystack::from("bbbb");
    assert!(OneOrMore::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_one_or_more_no_match() {
    regex!(OneOrMore = "b+");

    let mut hay = Haystack::from("");
    assert!(!OneOrMore::matches(&mut hay));
}

#[test]
fn test_optional_present() {
    regex!(Optional = "c?");

    let mut hay = Haystack::from("c");
    assert!(Optional::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_optional_absent() {
    regex!(Optional = "c?");

    let mut hay = Haystack::from("");
    assert!(Optional::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_exact_count() {
    regex!(Exact = "x{3}");

    let mut hay_match = Haystack::from("xxx");
    let mut hay_too_few = Haystack::from("xx");
    let mut hay_too_many = Haystack::from("xxxx");

    assert!(Exact::matches(&mut hay_match));
    assert!(!Exact::matches(&mut hay_too_few));
    assert!(!Exact::matches(&mut hay_too_many));
}

#[test]
fn test_range_count_lower() {
    regex!(Range = "y{2,4}");

    let mut hay = Haystack::from("yy");
    assert!(Range::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_range_count_middle() {
    regex!(Range = "y{2,4}");

    let mut hay = Haystack::from("yyy");
    assert!(Range::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_range_count_upper() {
    regex!(Range = "y{2,4}");

    let mut hay = Haystack::from("yyyy");
    assert!(Range::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_range_count_below() {
    regex!(Range = "y{2,4}");

    let mut hay = Haystack::from("y");
    assert!(!Range::matches(&mut hay));
}

#[test]
fn test_range_count_above() {
    regex!(Range = "y{2,4}");

    let mut hay = Haystack::from("yyyyy");
    assert!(!Range::matches(&mut hay));
}

#[test]
fn test_at_least_count() {
    regex!(AtLeast = "z{3,}");

    let mut hay_exact = Haystack::from("zzz");
    let mut hay_more = Haystack::from("zzzzzz");
    let mut hay_less = Haystack::from("zz");

    assert!(AtLeast::matches(&mut hay_exact));
    assert!(AtLeast::matches(&mut hay_more));
    assert!(!AtLeast::matches(&mut hay_less));
}

// ============================================================================
// CHARACTER CLASS TESTS
// ============================================================================

#[test]
fn test_char_range_lowercase() {
    regex!(Lower = "[a-z]");

    let mut hay_match = Haystack::from("m");
    let mut hay_upper = Haystack::from("M");
    let mut hay_digit = Haystack::from("5");

    assert!(Lower::matches(&mut hay_match));
    assert!(!Lower::matches(&mut hay_upper));
    assert!(!Lower::matches(&mut hay_digit));
}

#[test]
fn test_char_range_uppercase() {
    regex!(Upper = "[A-Z]");

    let mut hay_match = Haystack::from("M");
    let mut hay_lower = Haystack::from("m");

    assert!(Upper::matches(&mut hay_match));
    assert!(!Upper::matches(&mut hay_lower));
}

#[test]
fn test_char_range_digits() {
    regex!(Digits = "[0-9]");

    let mut hay_match = Haystack::from("7");
    let mut hay_letter = Haystack::from("a");

    assert!(Digits::matches(&mut hay_match));
    assert!(!Digits::matches(&mut hay_letter));
}

#[test]
fn test_digit_class() {
    regex!(DigitClass = r"\d");

    let mut hay_digit = Haystack::from("5");
    let mut hay_letter = Haystack::from("x");

    assert!(DigitClass::matches(&mut hay_digit));
    assert!(!DigitClass::matches(&mut hay_letter));
}

#[test]
fn test_multiple_ranges() {
    regex!(AlphaNum = "[a-zA-Z0-9]");

    let mut hay_lower = Haystack::from("f");
    let mut hay_upper = Haystack::from("F");
    let mut hay_digit = Haystack::from("6");
    let mut hay_punct = Haystack::from("!");

    assert!(AlphaNum::matches(&mut hay_lower));
    assert!(AlphaNum::matches(&mut hay_upper));
    assert!(AlphaNum::matches(&mut hay_digit));
    assert!(!AlphaNum::matches(&mut hay_punct));
}

// ============================================================================
// ANCHOR TESTS
// ============================================================================

#[test]
fn test_start_anchor_at_start() {
    regex!(StartAnchor = "^abc");

    let mut hay = Haystack::from("abc");
    assert!(StartAnchor::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_start_anchor_not_at_start() {
    regex!(StartAnchor = "^abc");

    let mut hay = Haystack::from("xabc");
    assert!(!StartAnchor::matches(&mut hay));
}

#[test]
fn test_end_anchor_at_end() {
    regex!(EndAnchor = "abc$");

    let mut hay = Haystack::from("abc");
    assert!(EndAnchor::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_end_anchor_not_at_end() {
    regex!(EndAnchor = "abc$");

    let mut hay = Haystack::from("abcx");
    assert!(!EndAnchor::matches(&mut hay));
}

#[test]
fn test_both_anchors_exact_match() {
    regex!(BothAnchors = "^test$");

    let mut hay_exact = Haystack::from("test");
    let mut hay_prefix = Haystack::from("xtest");
    let mut hay_suffix = Haystack::from("testx");

    assert!(BothAnchors::matches(&mut hay_exact));
    assert!(!BothAnchors::matches(&mut hay_prefix));
    assert!(!BothAnchors::matches(&mut hay_suffix));
}

// ============================================================================
// COMBINATION TESTS
// ============================================================================

#[test]
fn test_quantifier_in_sequence() {
    regex!(QuantSeq = "a+b");

    let mut hay_match = Haystack::from("aaab");
    let mut hay_no_a = Haystack::from("b");
    let mut hay_no_b = Haystack::from("aaa");

    assert!(QuantSeq::matches(&mut hay_match));
    assert!(!QuantSeq::matches(&mut hay_no_a));
    assert!(!QuantSeq::matches(&mut hay_no_b));
}

#[test]
fn test_alternation_with_quantifiers() {
    regex!(AltQuant = "a+|b*");

    let mut hay_a = Haystack::from("aaa");
    let mut hay_b = Haystack::from("bbb");
    let mut hay_empty = Haystack::from("");

    assert!(AltQuant::matches(&mut hay_a));
    assert!(AltQuant::matches(&mut hay_b));
    assert!(AltQuant::matches(&mut hay_empty)); // b* matches empty
}

#[test]
fn test_range_with_quantifier() {
    regex!(RangeQuant = "[0-9]+");

    let mut hay_single = Haystack::from("5");
    let mut hay_multiple = Haystack::from("12345");
    let mut hay_letter = Haystack::from("abc");

    assert!(RangeQuant::matches(&mut hay_single));
    assert!(RangeQuant::matches(&mut hay_multiple));
    assert!(!RangeQuant::matches(&mut hay_letter));
}

#[test]
fn test_anchored_range_quantifier() {
    regex!(Anchored = "^[a-z]+$");

    let mut hay_match = Haystack::from("hello");
    let mut hay_mixed = Haystack::from("Hello");
    let mut hay_digits = Haystack::from("123");

    assert!(Anchored::matches(&mut hay_match));
    assert!(!Anchored::matches(&mut hay_mixed));
    assert!(!Anchored::matches(&mut hay_digits));
}

// ============================================================================
// ESCAPE SEQUENCE TESTS
// ============================================================================

#[test]
fn test_newline_escape() {
    regex!(Newline = "line\n");

    let mut hay = Haystack::from("line\n");
    assert!(Newline::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_tab_escape() {
    regex!(Tab = "col\t");

    let mut hay = Haystack::from("col\t");
    assert!(Tab::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_backslash_escape() {
    regex!(Backslash = r"path\\file");

    let mut hay = Haystack::from("path\\file");
    assert!(Backslash::matches(&mut hay));
    assert!(hay.is_end());
}

// ============================================================================
// UNICODE TESTS
// ============================================================================

#[test]
fn test_unicode_literal() {
    regex!(Unicode = "hello🦀");

    let mut hay_match = Haystack::from("hello🦀");
    let mut hay_no_match = Haystack::from("hello");

    assert!(Unicode::matches(&mut hay_match));
    assert!(!Unicode::matches(&mut hay_no_match));
}

#[test]
fn test_unicode_pattern() {
    regex!(UnicodeEmoji = "🎉+");

    let mut hay_single = Haystack::from("🎉");
    let mut hay_multiple = Haystack::from("🎉🎉🎉");

    assert!(UnicodeEmoji::matches(&mut hay_single));
    assert!(UnicodeEmoji::matches(&mut hay_multiple));
}

// ============================================================================
// REAL-WORLD PATTERNS
// ============================================================================

#[test]
fn test_simple_email_pattern() {
    regex!(Email = r"[a-z]+@[a-z]+\.[a-z]+");

    let mut hay_valid = Haystack::from("user@example.com");
    let mut hay_invalid = Haystack::from("not-an-email");

    assert!(Email::matches(&mut hay_valid));
    assert!(!Email::matches(&mut hay_invalid));
}

#[test]
fn test_url_protocol() {
    regex!(Protocol = r"^https?://");

    let mut hay_https = Haystack::from("https://example.com");
    let mut hay_http = Haystack::from("http://example.com");
    let mut hay_ftp = Haystack::from("ftp://example.com");

    assert!(Protocol::matches(&mut hay_https));
    assert!(Protocol::matches(&mut hay_http));
    assert!(!Protocol::matches(&mut hay_ftp));
}

#[test]
fn test_three_digit_code() {
    regex!(ThreeDigits = r"^[0-9]{3}$");

    let mut hay_valid = Haystack::from("123");
    let mut hay_short = Haystack::from("12");
    let mut hay_long = Haystack::from("1234");

    assert!(ThreeDigits::matches(&mut hay_valid));
    assert!(!ThreeDigits::matches(&mut hay_short));
    assert!(!ThreeDigits::matches(&mut hay_long));
}

#[test]
fn test_word_boundary_pattern() {
    regex!(Word = r"[a-z]+");

    let mut hay_word = Haystack::from("word");
    let mut hay_mixed = Haystack::from("word123");

    assert!(Word::matches(&mut hay_word));
    // Will match "word" part and leave "123"
    assert!(Word::matches(&mut hay_mixed));
    assert_eq!(hay_mixed.item(), Some('1'));
}

// ============================================================================
// ANON_REGEX MATCHING TESTS
// ============================================================================

#[test]
fn test_anon_regex_in_expression() {
    let result = anon_regex!("^test$").matches(&mut Haystack::from("test"));
    assert!(result);
}

#[test]
fn test_anon_regex_stored() {
    let pattern = anon_regex!("[0-9]+");

    let mut hay1 = Haystack::from("123");
    let mut hay2 = Haystack::from("abc");

    assert!(pattern.matches(&mut hay1));
    assert!(!pattern.matches(&mut hay2));
}

#[test]
fn test_anon_regex_complex() {
    let pattern = anon_regex!(r"^[ab]+c$");

    let mut hay_valid1 = Haystack::from("ac");
    let mut hay_valid2 = Haystack::from("aaabbc");
    let mut hay_invalid = Haystack::from("cd");

    assert!(pattern.matches(&mut hay_valid1));
    assert!(pattern.matches(&mut hay_valid2));
    assert!(!pattern.matches(&mut hay_invalid));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_empty_pattern() {
    regex!(Empty = "");

    let mut hay_empty = Haystack::from("");
    let mut hay_non_empty = Haystack::from("x");

    assert!(Empty::matches(&mut hay_empty));
    // Empty pattern matches without consuming
    assert!(Empty::matches(&mut hay_non_empty));
    assert_eq!(hay_non_empty.item(), Some('x'));
}

#[test]
fn test_pattern_leaves_remainder() {
    regex!(Partial = "abc");

    let mut hay = Haystack::from("abcdef");
    assert!(Partial::matches(&mut hay));
    assert_eq!(hay.item(), Some('d')); // "def" remains
}

#[test]
fn test_greedy_matching() {
    regex!(Greedy = "a+");

    let mut hay = Haystack::from("aaab");
    assert!(Greedy::matches(&mut hay));
    assert_eq!(hay.item(), Some('b')); // Consumed all 'a's, 'b' remains
}

#[test]
fn test_alternation_precedence() {
    regex!(AltPrec = "ab|abc");

    let mut hay = Haystack::from("abc");
    assert!(AltPrec::matches(&mut hay));
    // First alternative "ab" matches, leaving "c"
    assert_eq!(hay.item(), Some('c'));
}

// ============================================================================
// BYTE VS CHAR MATCHING
// ============================================================================

#[test]
fn test_byte_and_char_same_pattern() {
    regex!(BothTypes = "test");

    let mut hay_char = Haystack::from("test");
    let mut hay_byte = Haystack::from(b"test" as &[u8]);

    assert!(BothTypes::matches(&mut hay_char));
    assert!(BothTypes::matches(&mut hay_byte));
}

#[test]
fn test_unicode_char_vs_byte() {
    regex!(UnicodeBoth = "🦀");

    // Unicode works with char
    let mut hay_char = Haystack::from("🦀");
    assert!(UnicodeBoth::matches(&mut hay_char));

    // Unicode with bytes (UTF-8 encoded)
    let mut hay_byte = Haystack::from("🦀".as_bytes());
    assert!(UnicodeBoth::matches(&mut hay_byte));
}
