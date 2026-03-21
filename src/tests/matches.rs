// Tests that verify the generated regex patterns match correctly

use crate::*;

// ============================================================================
// BASIC MATCHING TESTS
// ============================================================================

#[test]
fn test_literal_match() {
    regex!(Literal = "hello");

    assert!(Literal::is_match("hello"));
    assert!(!Literal::is_match("world"));
}

#[test]
fn test_literal_match_bytes() {
    regex!(LiteralBytes = "test");

    assert!(LiteralBytes::is_match(b"test" as &[u8]));
    assert!(!LiteralBytes::is_match(b"fail" as &[u8]));
}

#[test]
fn test_anon_regex_literal() {
    assert!(regex!("pattern").is_match("pattern"));
    assert!(!regex!("pattern").is_match("other"));
}

// ============================================================================
// ALTERNATION TESTS
// ============================================================================

#[test]
fn test_alternation_first_branch() {
    regex!(Alt = "foo|bar|baz");
    assert!(Alt::is_match("foo"));
}

#[test]
fn test_alternation_second_branch() {
    regex!(Alt = "foo|bar|baz");
    assert!(Alt::is_match("bar"));
}

#[test]
fn test_alternation_third_branch() {
    regex!(Alt = "foo|bar|baz");
    assert!(Alt::is_match("baz"));
}

#[test]
fn test_alternation_no_match() {
    regex!(Alt = "foo|bar|baz");
    assert!(!Alt::is_match("qux"));
}

// ============================================================================
// QUANTIFIER TESTS
// ============================================================================

#[test]
fn test_zero_or_more_zero_matches() {
    regex!(ZeroOrMore = "a*");
    assert!(ZeroOrMore::is_match(""));
}

#[test]
fn test_zero_or_more_multiple_matches() {
    regex!(ZeroOrMore = "a*");
    assert!(ZeroOrMore::is_match("aaa"));
}

#[test]
fn test_one_or_more_single_match() {
    regex!(OneOrMore = "b+");
    assert!(OneOrMore::is_match("b"));
}

#[test]
fn test_one_or_more_multiple_matches() {
    regex!(OneOrMore = "b+");
    assert!(OneOrMore::is_match("bbbb"));
}

#[test]
fn test_one_or_more_no_match() {
    regex!(OneOrMore = "b+");
    assert!(!OneOrMore::is_match(""));
}

#[test]
fn test_optional_present() {
    regex!(Optional = "c?");
    assert!(Optional::is_match("c"));
}

#[test]
fn test_optional_absent() {
    regex!(Optional = "c?");
    assert!(Optional::is_match(""));
}

#[test]
fn test_exact_count() {
    regex!(Exact = "x{3}");

    assert!(Exact::is_match("xxx"));
    assert!(!Exact::is_match("xx"));
    assert!(!Exact::is_match("xxxx"));
}

#[test]
fn test_range_count_lower() {
    regex!(Range = "y{2,4}");
    assert!(Range::is_match("yy"));
}

#[test]
fn test_range_count_middle() {
    regex!(Range = "y{2,4}");
    assert!(Range::is_match("yyy"));
}

#[test]
fn test_range_count_upper() {
    regex!(Range = "y{2,4}");
    assert!(Range::is_match("yyyy"));
}

#[test]
fn test_range_count_below() {
    regex!(Range = "y{2,4}");
    assert!(!Range::is_match("y"));
}

#[test]
fn test_range_count_above() {
    regex!(Range = "^y{2,4}$");
    assert!(!Range::is_match("yyyyy"));
}

#[test]
fn test_at_least_count() {
    regex!(AtLeast = "z{3,}");

    assert!(AtLeast::is_match("zzz"));
    assert!(AtLeast::is_match("zzzzzz"));
    assert!(!AtLeast::is_match("zz"));
}

// ============================================================================
// CHARACTER CLASS TESTS
// ============================================================================

#[test]
fn test_char_range_lowercase() {
    regex!(Lower = "[a-z]");

    assert!(Lower::is_match("m"));
    assert!(!Lower::is_match("M"));
    assert!(!Lower::is_match("5"));
}

#[test]
fn test_char_range_uppercase() {
    regex!(Upper = "[A-Z]");

    assert!(Upper::is_match("M"));
    assert!(!Upper::is_match("m"));
}

#[test]
fn test_char_range_digits() {
    regex!(Digits = "[0-9]");

    assert!(Digits::is_match("7"));
    assert!(!Digits::is_match("a"));
}

#[test]
fn test_digit_class() {
    regex!(DigitClass = r"\d");

    assert!(DigitClass::is_match("5"));
    assert!(!DigitClass::is_match("x"));
}

#[test]
fn test_multiple_ranges() {
    regex!(AlphaNum = "[a-zA-Z0-9]");

    assert!(AlphaNum::is_match("f"));
    assert!(AlphaNum::is_match("F"));
    assert!(AlphaNum::is_match("6"));
    assert!(!AlphaNum::is_match("!"));
}

// ============================================================================
// ANCHOR TESTS
// ============================================================================

#[test]
fn test_start_anchor_at_start() {
    regex!(StartAnchor = "^abc");
    assert!(StartAnchor::is_match("abc"));
}

#[test]
fn test_start_anchor_not_at_start() {
    regex!(StartAnchor = "^abc");
    assert!(!StartAnchor::is_match("xabc"));
}

#[test]
fn test_end_anchor_at_end() {
    regex!(EndAnchor = "abc$");
    assert!(EndAnchor::is_match("abc"));
}

#[test]
fn test_end_anchor_not_at_end() {
    regex!(EndAnchor = "abc$");
    assert!(!EndAnchor::is_match("abcx"));
}

#[test]
fn test_both_anchors_exact_match() {
    regex!(BothAnchors = "^test$");

    assert!(BothAnchors::is_match("test"));
    assert!(!BothAnchors::is_match("xtest"));
    assert!(!BothAnchors::is_match("testx"));
}

// ============================================================================
// COMBINATION TESTS
// ============================================================================

#[test]
fn test_quantifier_in_sequence() {
    regex!(QuantSeq = "a+b");

    assert!(QuantSeq::is_match("aaab"));
    assert!(!QuantSeq::is_match("b"));
    assert!(!QuantSeq::is_match("aaa"));
}

#[test]
fn test_alternation_with_quantifiers() {
    regex!(AltQuant = "a+|b*");

    assert!(AltQuant::is_match("aaa"));
    assert!(AltQuant::is_match("bbb"));
    assert!(AltQuant::is_match("")); // b* matches empty
}

#[test]
fn test_range_with_quantifier() {
    regex!(RangeQuant = "[0-9]+");

    assert!(RangeQuant::is_match("5"));
    assert!(RangeQuant::is_match("12345"));
    assert!(!RangeQuant::is_match("abc"));
}

#[test]
fn test_anchored_range_quantifier() {
    regex!(Anchored = "^[a-z]+$");

    assert!(Anchored::is_match("hello"));
    assert!(!Anchored::is_match("Hello"));
    assert!(!Anchored::is_match("123"));
}

// ============================================================================
// ESCAPE SEQUENCE TESTS
// ============================================================================

#[test]
fn test_newline_escape() {
    regex!(Newline = "line\n");
    assert!(Newline::is_match("line\n"));
}

#[test]
fn test_tab_escape() {
    regex!(Tab = "col\t");
    assert!(Tab::is_match("col\t"));
}

#[test]
fn test_backslash_escape() {
    regex!(Backslash = r"path\\file");
    assert!(Backslash::is_match("path\\file"));
}

// ============================================================================
// UNICODE TESTS
// ============================================================================

#[test]
fn test_unicode_literal() {
    regex!(Unicode = "hello🦀");

    assert!(Unicode::is_match("hello🦀"));
    assert!(!Unicode::is_match("hello"));
}

#[test]
fn test_unicode_pattern() {
    regex!(UnicodeEmoji = "🎉+");

    assert!(UnicodeEmoji::is_match("🎉"));
    assert!(UnicodeEmoji::is_match("🎉🎉🎉"));
}

// ============================================================================
// REAL-WORLD PATTERNS
// ============================================================================

#[test]
fn test_simple_email_pattern() {
    regex!(Email = r"[a-z]+@[a-z]+\.[a-z]+");

    assert!(Email::is_match("user@example.com"));
    assert!(!Email::is_match("not-an-email"));
}

#[test]
fn test_url_protocol() {
    regex!(Protocol = r"^https?://$");

    // is_match() requires the entire haystack to match
    assert!(Protocol::is_match("https://"));
    assert!(Protocol::is_match("http://"));
    assert!(!Protocol::is_match("ftp://"));
}

#[test]
fn test_three_digit_code() {
    regex!(ThreeDigits = r"^[0-9]{3}$");

    assert!(ThreeDigits::is_match("123"));
    assert!(!ThreeDigits::is_match("12"));
    assert!(!ThreeDigits::is_match("1234"));
}

#[test]
fn test_word_boundary_pattern() {
    regex!(Word = r"[a-z]+");

    assert!(Word::is_match("word"));
    // is_match() requires entire haystack to match, so "word123" fails
    assert!(!Word::is_match("word123"));
}

// ============================================================================
// ANONYMOUS REGEX MATCHING TESTS
// ============================================================================

#[test]
fn test_anon_regex_in_expression() {
    let result = regex!("^test$").is_match("test");
    assert!(result);
}

#[test]
fn test_anon_regex_stored() {
    let pattern = regex!("[0-9]+");

    assert!(pattern.is_match("123"));
    assert!(!pattern.is_match("abc"));
}

#[test]
fn test_anon_regex_complex() {
    let pattern = regex!(r"^[ab]+c$");

    assert!(pattern.is_match("ac"));
    assert!(pattern.is_match("aaabbc"));
    assert!(!pattern.is_match("cd"));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_empty_pattern() {
    regex!(Empty = "");

    // Empty pattern only matches empty haystack when is_match() requires full match
    assert!(Empty::is_match(""));
    assert!(!Empty::is_match("x"));
}

#[test]
fn test_pattern_requires_full_match() {
    regex!(Partial = "abc");

    // is_match() requires the entire haystack to match
    assert!(Partial::is_match("abc"));
    assert!(!Partial::is_match("abcdef"));
}

#[test]
fn test_greedy_matching() {
    regex!(Greedy = "a+");

    // is_match() requires the entire haystack to match
    assert!(Greedy::is_match("aaaa"));
    assert!(!Greedy::is_match("aaab"));
}

#[test]
fn test_alternation_precedence() {
    regex!(AltPrec = "ab|abc");

    // is_match() requires the entire haystack to match
    assert!(AltPrec::is_match("ab"));
    // all_matches explores all alternatives, so "abc" is correctly matched
    // even though "ab" is listed first
    assert!(AltPrec::is_match("abc"));
}

#[test]
fn test_alternation_precedence_do_capture() {
    regex!(AltPrec = "ab|abc");

    // do_capture must consume the entire haystack, so it finds the "abc" match
    assert!(AltPrec::do_capture("abc").is_some());
    assert!(AltPrec::do_capture("ab").is_some());
    assert!(AltPrec::do_capture("a").is_none());
}

// ============================================================================
// BYTE VS CHAR MATCHING
// ============================================================================

#[test]
fn test_byte_and_char_same_pattern() {
    regex!(BothTypes = "test");

    assert!(BothTypes::is_match("test"));
    assert!(BothTypes::is_match(b"test" as &[u8]));
}

#[test]
fn test_unicode_char_vs_byte() {
    regex!(UnicodeBoth = "🦀");

    // Unicode works with char
    assert!(UnicodeBoth::is_match("🦀"));

    // Unicode with bytes (UTF-8 encoded)
    assert!(UnicodeBoth::is_match("🦀".as_bytes()));
}
