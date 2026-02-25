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
