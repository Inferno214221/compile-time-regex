use super::*;
use regex_syntax::Parser;

// Helper function to parse regex and convert to type expression
fn parse_and_convert_char(pattern: &str) -> String {
    let hir = Parser::new().parse(pattern).unwrap();
    hir.into_type_expr::<char>()
}

fn parse_and_convert_byte(pattern: &str) -> String {
    let hir = Parser::new().parse(pattern).unwrap();
    hir.into_type_expr::<u8>()
}

// Tests for type_name function
#[test]
fn test_type_name_scalar() {
    let name = type_name::<Scalar<'a'>>();
    assert!(name.contains("Scalar"));
}

#[test]
fn test_type_name_byte() {
    let name = type_name::<Byte<0>>();
    assert!(name.contains("Byte"));
}

#[test]
fn test_type_name_or() {
    let name = type_name::<Or<char, Scalar<'a'>, Scalar<'b'>>>();
    assert!(name.contains("Or"));
}

#[test]
fn test_type_name_then() {
    let name = type_name::<Then<char, Scalar<'a'>, Scalar<'b'>>>();
    assert!(name.contains("Then"));
}

// Tests for Empty HIR
#[test]
fn test_empty_pattern() {
    let result = parse_and_convert_char("");
    assert!(result.contains("Always"));
}

// Tests for Literal patterns
#[test]
fn test_literal_single_char() {
    let result = parse_and_convert_char("a");
    assert!(result.contains("Scalar"));
    assert!(result.contains("a"));
}

#[test]
fn test_literal_multiple_chars() {
    let result = parse_and_convert_char("abc");
    assert!(result.contains("Then"));
    assert!(result.contains("Scalar"));
}

#[test]
fn test_literal_unicode() {
    let result = parse_and_convert_char("🦀");
    assert!(result.contains("Scalar"));
}

#[test]
fn test_literal_escaped() {
    let result = parse_and_convert_char(r"\n");
    assert!(result.contains("Scalar"));
}

// Tests for character classes
#[test]
fn test_class_single_range() {
    let result = parse_and_convert_char("[a-z]");
    assert!(result.contains("ScalarRange") || result.contains("Or"));
}

#[test]
fn test_class_multiple_ranges() {
    let result = parse_and_convert_char("[a-zA-Z]");
    assert!(result.contains("Or"));
}

#[test]
fn test_class_digit() {
    let result = parse_and_convert_char(r"\d");
    assert!(result.contains("Or") || result.contains("ScalarRange"));
}

#[test]
fn test_class_word() {
    let result = parse_and_convert_char(r"\w");
    assert!(result.contains("Or") || result.contains("ScalarRange"));
}

// Tests for byte patterns
#[test]
fn test_byte_literal() {
    let result = parse_and_convert_byte("x");
    assert!(result.contains("Byte"));
}

#[test]
fn test_byte_range() {
    // Parse as bytes by using (?-u) to disable Unicode mode
    let result = parse_and_convert_byte("(?-u)[a-z]");
    assert!(result.contains("ByteRange") || result.contains("Or") || result.contains("Byte"));
}

// Tests for anchors (Look)
#[test]
fn test_anchor_start() {
    let result = parse_and_convert_char("^");
    assert!(result.contains("Beginning"));
}

#[test]
fn test_anchor_end() {
    let result = parse_and_convert_char("$");
    assert!(result.contains("End"));
}

#[test]
fn test_anchor_both() {
    let result = parse_and_convert_char("^a$");
    assert!(result.contains("Beginning"));
    assert!(result.contains("End"));
    assert!(result.contains("Scalar"));
}

// Tests for repetition (quantifiers)
#[test]
fn test_quantifier_zero_or_more() {
    let result = parse_and_convert_char("a*");
    assert!(result.contains("QuantifierNOrMore"));
    assert!(result.contains("0"));
}

#[test]
fn test_quantifier_one_or_more() {
    let result = parse_and_convert_char("a+");
    assert!(result.contains("QuantifierNOrMore"));
    assert!(result.contains("1"));
}

#[test]
fn test_quantifier_optional() {
    let result = parse_and_convert_char("a?");
    assert!(result.contains("QuantifierNToM"));
    assert!(result.contains("0"));
    assert!(result.contains("1"));
}

#[test]
fn test_quantifier_exact() {
    let result = parse_and_convert_char("a{3}");
    assert!(result.contains("QuantifierN"));
    assert!(result.contains("3"));
}

#[test]
fn test_quantifier_range() {
    let result = parse_and_convert_char("a{2,5}");
    assert!(result.contains("QuantifierNToM"));
    assert!(result.contains("2"));
    assert!(result.contains("5"));
}

#[test]
fn test_quantifier_at_least() {
    let result = parse_and_convert_char("a{3,}");
    assert!(result.contains("QuantifierNOrMore"));
    assert!(result.contains("3"));
}

// Tests for alternation (Or)
#[test]
fn test_alternation_consecutive_chars() {
    // Consecutive chars like a|b should optimize to ScalarRange
    let result = parse_and_convert_char("a|b");
    assert!(result.contains("ScalarRange"));
    assert!(result.contains("'\\u{61}'") && result.contains("'\\u{62}'")); // 'a' and 'b'
}

#[test]
fn test_alternation_non_consecutive_chars() {
    // Non-consecutive chars like a|c should use Or
    let result = parse_and_convert_char("a|c");
    assert!(result.contains("Or"));
    assert!(result.contains("Scalar"));
}

#[test]
fn test_alternation_three_consecutive_chars() {
    // Three consecutive chars like a|b|c should optimize to ScalarRange
    let result = parse_and_convert_char("a|b|c");
    assert!(result.contains("ScalarRange"));
    assert!(result.contains("'\\u{61}'") && result.contains("'\\u{63}'")); // 'a' and 'c'
}

#[test]
fn test_alternation_three_non_consecutive_chars() {
    // Non-consecutive chars like a|c|e should use Or
    let result = parse_and_convert_char("a|c|e");
    assert!(result.contains("Or"));
}

#[test]
fn test_alternation_with_sequences() {
    let result = parse_and_convert_char("ab|cd");
    assert!(result.contains("Or"));
    assert!(result.contains("Then"));
}

// Tests for concatenation (Then)
#[test]
fn test_concat_two_chars() {
    let result = parse_and_convert_char("ab");
    assert!(result.contains("Then"));
}

#[test]
fn test_concat_three_chars() {
    let result = parse_and_convert_char("abc");
    assert!(result.contains("Then"));
}

#[test]
fn test_concat_with_quantifiers() {
    let result = parse_and_convert_char("a+b");
    assert!(result.contains("Then"));
    assert!(result.contains("QuantifierNOrMore"));
}

// Tests for groups (capture)
#[test]
fn test_capture_group() {
    let result = parse_and_convert_char("(a)");
    // Captures are currently transparent, just check the content is there
    assert!(result.contains("Scalar"));
}

#[test]
fn test_capture_group_with_alternation_consecutive() {
    // Consecutive chars in capture group should optimize to ScalarRange
    let result = parse_and_convert_char("(a|b)");
    assert!(result.contains("ScalarRange"));
}

#[test]
fn test_capture_group_with_alternation_non_consecutive() {
    // Non-consecutive chars in capture group should use Or
    let result = parse_and_convert_char("(a|c)");
    assert!(result.contains("Or"));
}

#[test]
fn test_non_capturing_group() {
    let result = parse_and_convert_char("(?:abc)");
    assert!(result.contains("Then"));
}

// Complex integration tests
#[test]
fn test_complex_email_like() {
    let result = parse_and_convert_char(r"[a-z]+@[a-z]+\.[a-z]+");
    assert!(result.contains("Then"));
    assert!(result.contains("QuantifierNOrMore"));
}

#[test]
fn test_complex_optional_prefix() {
    let result = parse_and_convert_char("(https?://)?example");
    assert!(result.contains("Then"));
    assert!(result.contains("QuantifierNToM"));
}

#[test]
fn test_complex_multiple_quantifiers() {
    let result = parse_and_convert_char("a*b+c?");
    assert!(result.contains("Then"));
    assert!(result.contains("QuantifierNOrMore"));
    assert!(result.contains("QuantifierNToM"));
}

#[test]
fn test_complex_nested_groups_consecutive() {
    // b|c are consecutive and should optimize to ScalarRange
    let result = parse_and_convert_char("(a(b|c)d)");
    assert!(result.contains("Then"));
    assert!(result.contains("ScalarRange"));
}

#[test]
fn test_complex_nested_groups_non_consecutive() {
    // b|d are non-consecutive and should use Or
    let result = parse_and_convert_char("(a(b|d)e)");
    assert!(result.contains("Then"));
    assert!(result.contains("Or"));
}

#[test]
fn test_complex_anchored_pattern() {
    let result = parse_and_convert_char("^[a-z]+$");
    assert!(result.contains("Beginning"));
    assert!(result.contains("End"));
    assert!(result.contains("QuantifierNOrMore"));
}

// Tests for WriteTypeExpr trait implementations
#[test]
fn test_write_type_expr_u8() {
    let mut s = String::new();
    (b'x').write_type_expr::<u8>(&mut s).unwrap();
    assert!(s.contains("Byte"));
    assert!(s.contains("120")); // ASCII value of 'x'
}

#[test]
fn test_write_type_expr_char() {
    let mut s = String::new();
    'a'.write_type_expr::<char>(&mut s).unwrap();
    assert!(s.contains("Scalar"));
    assert!(s.contains("a"));
}

#[test]
fn test_write_type_expr_char_unicode() {
    let mut s = String::new();
    '🦀'.write_type_expr::<char>(&mut s).unwrap();
    assert!(s.contains("Scalar"));
}

#[test]
fn test_write_type_expr_char_escape() {
    let mut s = String::new();
    '\n'.write_type_expr::<char>(&mut s).unwrap();
    assert!(s.contains("Scalar"));
    assert!(s.contains("\\u{a}")); // escaped newline
}

// Edge case tests
#[test]
fn test_empty_alternation() {
    let result = parse_and_convert_char("||");
    // Empty patterns should produce Always
    assert!(result.contains("Always") || result.contains("Or"));
}

#[test]
fn test_single_char_quantifier_zero() {
    let result = parse_and_convert_char("a{0}");
    // a{0} might be optimized to Always or empty pattern
    assert!(result.contains("QuantifierN") || result.contains("Always"));
}

#[test]
fn test_unicode_escape() {
    let result = parse_and_convert_char(r"\u{1F980}");
    assert!(result.contains("Scalar"));
}

// Tests for HirExtension trait
#[test]
fn test_hir_extension_into_type_expr() {
    let hir = Parser::new().parse("abc").unwrap();
    let result: String = hir.into_type_expr::<char>();
    assert!(result.contains("Then"));
    assert!(result.contains("Scalar"));
}

#[test]
fn test_hir_extension_with_quantifier() {
    let hir = Parser::new().parse("a+").unwrap();
    let result: String = hir.into_type_expr::<char>();
    assert!(result.contains("QuantifierNOrMore"));
}

// Tests to ensure proper type name extraction
#[test]
fn test_type_name_removes_generic_params() {
    let name = type_name::<Scalar<'z'>>();
    // Should only contain the base type name, not the full path with generics
    assert!(!name.contains('<'));
}

#[test]
fn test_type_name_consistent() {
    let name1 = type_name::<Scalar<'a'>>();
    let name2 = type_name::<Scalar<'b'>>();
    // Type names should be the same regardless of const generic parameter
    assert_eq!(name1, name2);
}

// ============================================================================
// CHUNKED TYPE GENERATION TESTS
// Tests for Or4, Or8, Or16, Then4, Then8, Then16 type generation
// ============================================================================

#[test]
fn test_chunked_or4_generation() {
    // 4 alternations should generate Or4
    let result = parse_and_convert_char("a|c|e|g");
    assert!(result.contains("Or4"), "Expected Or4 for 4 alternations, got: {}", result);
}

#[test]
fn test_chunked_or8_generation() {
    // 8 non-adjacent ranges in a character class should generate Or8
    // Using ranges that don't merge: a-b, d-e, g-h, j-k, m-n, p-q, s-t, v-w
    let result = parse_and_convert_char("[a-bd-eg-hj-km-np-qs-tv-w]");
    assert!(result.contains("Or8") || result.contains("Or4"),
        "Expected Or8 or Or4 for 8 non-adjacent ranges, got: {}", result);
}

#[test]
fn test_chunked_then4_generation() {
    // 4 characters in sequence should generate Then4
    let result = parse_and_convert_char("abcd");
    assert!(result.contains("Then4"), "Expected Then4 for 4-char literal, got: {}", result);
}

#[test]
fn test_chunked_then8_generation() {
    // 8 characters in sequence should generate Then8
    let result = parse_and_convert_char("abcdefgh");
    assert!(result.contains("Then8"), "Expected Then8 for 8-char literal, got: {}", result);
}

#[test]
fn test_chunked_then16_generation() {
    // 16 characters in sequence should generate Then16
    let result = parse_and_convert_char("abcdefghijklmnop");
    assert!(result.contains("Then16"), "Expected Then16 for 16-char literal, got: {}", result);
}

#[test]
fn test_chunked_mixed_size_5() {
    // 5 items = Or4 + 1 remainder, wrapped in Or
    let result = parse_and_convert_char("a|c|e|g|i");
    // Should have Or wrapper and Or4 inside
    assert!(result.contains("Or4"), "Expected Or4 inside for 5 alternations, got: {}", result);
    // Count Or occurrences - should have both Or4 and regular Or
    let or_count = result.matches("Or<").count();
    assert!(or_count >= 1, "Expected at least one Or wrapper, got: {}", result);
}

#[test]
fn test_chunked_mixed_size_6() {
    // 6 items = Or4 + Or (2 items)
    let result = parse_and_convert_char("a|c|e|g|i|k");
    assert!(result.contains("Or4"), "Expected Or4 for first 4 of 6, got: {}", result);
}

#[test]
fn test_chunked_literal_5_chars() {
    // 5 chars = Then4 + 1 remainder
    let result = parse_and_convert_char("abcde");
    assert!(result.contains("Then4"), "Expected Then4 for first 4 of 5, got: {}", result);
}

#[test]
fn test_chunked_literal_10_chars() {
    // 10 chars = Then8 + Then (2 items)
    let result = parse_and_convert_char("abcdefghij");
    assert!(result.contains("Then8"), "Expected Then8 for first 8 of 10, got: {}", result);
}

#[test]
fn test_chunked_preserves_order() {
    // Verify that chunking preserves the order of items
    let result = parse_and_convert_char("abcd");
    // Should have all chars in order: a, b, c, d
    let a_pos = result.find("'\\u{61}'").expect("'a' not found");
    let b_pos = result.find("'\\u{62}'").expect("'b' not found");
    let c_pos = result.find("'\\u{63}'").expect("'c' not found");
    let d_pos = result.find("'\\u{64}'").expect("'d' not found");
    assert!(a_pos < b_pos && b_pos < c_pos && c_pos < d_pos,
        "Characters should appear in order, got: {}", result);
}

// ============================================================================
// DOT (ANY CHARACTER) MATCHER TESTS
// Tests for the . metacharacter
// ============================================================================

#[test]
fn test_dot_basic() {
    let result = parse_and_convert_char(".");
    // Dot should generate a class that matches any character
    // In Unicode mode, this is typically a very large character class
    assert!(result.contains("Or") || result.contains("ScalarRange"),
        "Expected Or or ScalarRange for dot, got: {}", result);
}

#[test]
fn test_dot_with_quantifier() {
    let result = parse_and_convert_char(".*");
    assert!(result.contains("QuantifierNOrMore"),
        "Expected QuantifierNOrMore for .*, got: {}", result);
}

#[test]
fn test_dot_one_or_more() {
    let result = parse_and_convert_char(".+");
    assert!(result.contains("QuantifierNOrMore"),
        "Expected QuantifierNOrMore for .+, got: {}", result);
    assert!(result.contains("1"), "Expected min=1 for .+");
}

#[test]
fn test_dot_in_sequence() {
    let result = parse_and_convert_char("a.b");
    assert!(result.contains("Then"), "Expected Then for a.b sequence");
    // The middle part should be the dot class
    assert!(result.contains("Or") || result.contains("ScalarRange"),
        "Expected dot to generate Or or ScalarRange, got: {}", result);
}

#[test]
fn test_dot_anchored() {
    let result = parse_and_convert_char("^.$");
    assert!(result.contains("Beginning"), "Expected Beginning anchor");
    assert!(result.contains("End"), "Expected End anchor");
}

// ============================================================================
// CONSECUTIVE ALTERNATION OPTIMIZATION TESTS
// Tests that verify consecutive character alternations optimize to ScalarRange
// ============================================================================

#[test]
fn test_consecutive_two_chars_optimizes() {
    // a|b (consecutive) should optimize to ScalarRange<'a', 'b'>
    let result = parse_and_convert_char("a|b");
    assert!(result.contains("ScalarRange"),
        "Expected ScalarRange for consecutive a|b, got: {}", result);
    assert!(!result.contains("Or<"),
        "Should NOT use Or for consecutive chars, got: {}", result);
}

#[test]
fn test_consecutive_three_chars_optimizes() {
    // a|b|c (all consecutive) should optimize to ScalarRange<'a', 'c'>
    let result = parse_and_convert_char("a|b|c");
    assert!(result.contains("ScalarRange"),
        "Expected ScalarRange for consecutive a|b|c, got: {}", result);
}

#[test]
fn test_non_consecutive_uses_or() {
    // a|c (non-consecutive) should use Or
    let result = parse_and_convert_char("a|c");
    assert!(result.contains("Or"),
        "Expected Or for non-consecutive a|c, got: {}", result);
}

#[test]
fn test_mixed_consecutive_non_consecutive() {
    // a|b|d - a|b are consecutive, but d is not
    let result = parse_and_convert_char("a|b|d");
    // This might optimize a|b to ScalarRange, then Or with d
    // Or it might just use Or for all
    assert!(result.contains("Or") || result.contains("ScalarRange"),
        "Expected some optimization, got: {}", result);
}

#[test]
fn test_digits_consecutive() {
    // 0|1|2 should optimize to ScalarRange
    let result = parse_and_convert_char("0|1|2");
    assert!(result.contains("ScalarRange"),
        "Expected ScalarRange for consecutive 0|1|2, got: {}", result);
}

#[test]
fn test_uppercase_consecutive() {
    // A|B|C should optimize to ScalarRange
    let result = parse_and_convert_char("A|B|C");
    assert!(result.contains("ScalarRange"),
        "Expected ScalarRange for consecutive A|B|C, got: {}", result);
}

// ============================================================================
// CHARACTER CLASS EXPANSION TESTS
// Tests for \w, \s, \d and their optimization with chunked types
// ============================================================================

#[test]
fn test_digit_class_generates_type() {
    let result = parse_and_convert_char(r"\d");
    // \d should generate ScalarRange for 0-9
    assert!(result.contains("ScalarRange") || result.contains("Or"),
        "Expected ScalarRange or Or for \\d, got: {}", result);
}

#[test]
fn test_word_class_uses_chunked_or() {
    let result = parse_and_convert_char(r"\w");
    // \w = [a-zA-Z0-9_] which has multiple ranges
    // Should use Or4 or Or8 to reduce nesting
    assert!(result.contains("Or"), "Expected Or for \\w, got: {}", result);
    // Check for chunked types when there are enough ranges
    let has_chunked = result.contains("Or4") || result.contains("Or8") || result.contains("Or16");
    // \w typically has 4 ranges: a-z, A-Z, 0-9, _ - so should use Or4
    assert!(has_chunked || result.contains("ScalarRange"),
        "Expected chunked Or or ScalarRange for \\w, got: {}", result);
}

#[test]
fn test_space_class_type() {
    let result = parse_and_convert_char(r"\s");
    // \s matches whitespace characters
    assert!(result.contains("Or") || result.contains("ScalarRange"),
        "Expected Or or ScalarRange for \\s, got: {}", result);
}

#[test]
fn test_combined_classes() {
    let result = parse_and_convert_char(r"\d\w\s");
    // Three classes in sequence
    assert!(result.contains("Then"), "Expected Then for sequence");
}

#[test]
fn test_negated_digit_class() {
    let result = parse_and_convert_char(r"\D");
    // \D = not a digit, should be a large character class
    assert!(result.contains("Or") || result.contains("ScalarRange"),
        "Expected Or or ScalarRange for \\D, got: {}", result);
}

#[test]
fn test_negated_word_class() {
    let result = parse_and_convert_char(r"\W");
    // \W = not a word character
    assert!(result.contains("Or") || result.contains("ScalarRange"),
        "Expected Or or ScalarRange for \\W, got: {}", result);
}

#[test]
fn test_digit_class_range_values() {
    let result = parse_and_convert_char(r"\d");
    // Should contain '0' and '9' for the range
    assert!(result.contains("'\\u{30}'") || result.contains("ScalarRange"),
        "Expected '0' (\\u{{30}}) in \\d range, got: {}", result);
}

#[test]
fn test_multiple_ranges_uses_chunked() {
    // [a-zA-Z0-9_-] has 5 ranges, should use Or4 + Or
    let result = parse_and_convert_char("[a-zA-Z0-9_-]");
    assert!(result.contains("Or"), "Expected Or for multiple ranges");
}

// ============================================================================
// EDGE CASE TESTS FOR NEW FEATURES
// ============================================================================

#[test]
fn test_dot_vs_escaped_dot() {
    let dot_result = parse_and_convert_char(".");
    let escaped_result = parse_and_convert_char(r"\.");

    // Escaped dot should be a literal period character
    assert!(escaped_result.contains("Scalar") && escaped_result.contains("'\\u{2e}'"),
        "Escaped dot should be Scalar<'.'>, got: {}", escaped_result);

    // Unescaped dot should NOT be a simple Scalar
    // It should be a character class (Or of ranges)
    assert!(dot_result.contains("Or") || dot_result.contains("ScalarRange"),
        "Unescaped dot should be a class, got: {}", dot_result);
}

#[test]
fn test_character_class_with_dot() {
    // [.] should match literal dot
    let result = parse_and_convert_char("[.]");
    assert!(result.contains("ScalarRange") || result.contains("Scalar"),
        "Expected literal dot in class, got: {}", result);
}

#[test]
fn test_alternation_with_dot() {
    let result = parse_and_convert_char("a|.");
    assert!(result.contains("Or"), "Expected Or for a|.");
}

#[test]
fn test_long_alternation_chunking() {
    // 20 alternations should use Or16 + Or4
    let result = parse_and_convert_char("a|c|e|g|i|k|m|o|q|s|u|w|y|A|C|E|G|I|K|M");
    assert!(result.contains("Or16") || result.contains("Or8"),
        "Expected Or16 or Or8 for 20 alternations, got first 200 chars: {}",
        &result[..result.len().min(200)]);
}

#[test]
fn test_long_literal_chunking() {
    // 20 character literal should use Then16 + Then4
    let result = parse_and_convert_char("abcdefghijklmnopqrst");
    assert!(result.contains("Then16") || result.contains("Then8"),
        "Expected Then16 or Then8 for 20-char literal, got first 200 chars: {}",
        &result[..result.len().min(200)]);
}
