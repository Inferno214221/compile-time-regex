use super::*;
use crate::haystack::Haystack;

// Tests for Byte matcher
#[test]
fn test_byte_matches() {
    let mut hay = Haystack::from(b"a" as &[u8]);
    assert!(Byte::<b'a'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_byte_no_match() {
    let mut hay = Haystack::from(b"b" as &[u8]);
    assert!(!Byte::<b'a'>::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_byte_empty_haystack() {
    let mut hay = Haystack::from(b"" as &[u8]);
    assert!(!Byte::<b'a'>::matches(&mut hay));
}

// Tests for ByteRange matcher
#[test]
fn test_byte_range_matches_lower() {
    let mut hay = Haystack::from(b"a" as &[u8]);
    assert!(ByteRange::<b'a', b'z'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_byte_range_matches_upper() {
    let mut hay = Haystack::from(b"z" as &[u8]);
    assert!(ByteRange::<b'a', b'z'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_byte_range_matches_middle() {
    let mut hay = Haystack::from(b"m" as &[u8]);
    assert!(ByteRange::<b'a', b'z'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_byte_range_no_match_below() {
    let mut hay = Haystack::from(b"A" as &[u8]);
    assert!(!ByteRange::<b'a', b'z'>::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_byte_range_no_match_above() {
    let mut hay = Haystack::from(b"0" as &[u8]);
    assert!(!ByteRange::<b'a', b'z'>::matches(&mut hay));
}

// Tests for Scalar matcher
#[test]
fn test_scalar_matches() {
    let mut hay = Haystack::from("a");
    assert!(Scalar::<'a'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_scalar_no_match() {
    let mut hay = Haystack::from("b");
    assert!(!Scalar::<'a'>::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_scalar_unicode() {
    let mut hay = Haystack::from("🦀");
    assert!(Scalar::<'🦀'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_scalar_empty_haystack() {
    let mut hay = Haystack::from("");
    assert!(!Scalar::<'a'>::matches(&mut hay));
}

// Tests for ScalarRange matcher
#[test]
fn test_scalar_range_matches_lower() {
    let mut hay = Haystack::from("a");
    assert!(ScalarRange::<'a', 'z'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_scalar_range_matches_upper() {
    let mut hay = Haystack::from("z");
    assert!(ScalarRange::<'a', 'z'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_scalar_range_matches_middle() {
    let mut hay = Haystack::from("m");
    assert!(ScalarRange::<'a', 'z'>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_scalar_range_no_match_below() {
    let mut hay = Haystack::from("A");
    assert!(!ScalarRange::<'a', 'z'>::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_scalar_range_no_match_above() {
    let mut hay = Haystack::from("0");
    assert!(!ScalarRange::<'a', 'z'>::matches(&mut hay));
}

// Tests for Or matcher
#[test]
fn test_or_matches_first() {
    let mut hay = Haystack::from("a");
    assert!(Or::<char, Scalar::<'a'>, Scalar::<'b'>>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_or_matches_second() {
    let mut hay = Haystack::from("b");
    assert!(Or::<char, Scalar::<'a'>, Scalar::<'b'>>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_or_no_match() {
    let mut hay = Haystack::from("c");
    assert!(!Or::<char, Scalar::<'a'>, Scalar::<'b'>>::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_or_with_bytes() {
    let mut hay = Haystack::from(b"x" as &[u8]);
    assert!(Or::<u8, Byte::<b'x'>, Byte::<b'y'>>::matches(&mut hay));
    assert!(hay.is_end());
}

// Tests for Then matcher
#[test]
fn test_then_both_match() {
    let mut hay = Haystack::from("ab");
    assert!(Then::<char, Scalar::<'a'>, Scalar::<'b'>>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_then_first_fails() {
    let mut hay = Haystack::from("bb");
    assert!(!Then::<char, Scalar::<'a'>, Scalar::<'b'>>::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_then_second_fails() {
    let mut hay = Haystack::from("aa");
    assert!(!Then::<char, Scalar::<'a'>, Scalar::<'b'>>::matches(&mut hay));
}

#[test]
fn test_then_with_bytes() {
    let mut hay = Haystack::from(b"xy" as &[u8]);
    assert!(Then::<u8, Byte::<b'x'>, Byte::<b'y'>>::matches(&mut hay));
    assert!(hay.is_end());
}

// Tests for QuantifierN matcher
#[test]
fn test_quantifier_n_exact_match() {
    let mut hay = Haystack::from("aaa");
    assert!(QuantifierN::<char, Scalar::<'a'>, 3>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_n_too_few() {
    let mut hay = Haystack::from("aa");
    assert!(!QuantifierN::<char, Scalar::<'a'>, 3>::matches(&mut hay));
}

#[test]
fn test_quantifier_n_too_many() {
    let mut hay = Haystack::from("aaaa");
    assert!(!QuantifierN::<char, Scalar::<'a'>, 3>::matches(&mut hay));
}

#[test]
fn test_quantifier_n_zero() {
    let mut hay = Haystack::from("b");
    assert!(QuantifierN::<char, Scalar::<'a'>, 0>::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_quantifier_n_with_bytes() {
    let mut hay = Haystack::from(b"xxx" as &[u8]);
    assert!(QuantifierN::<u8, Byte::<b'x'>, 3>::matches(&mut hay));
    assert!(hay.is_end());
}

// Tests for QuantifierNOrMore matcher
#[test]
fn test_quantifier_n_or_more_exact() {
    let mut hay = Haystack::from("aaa");
    assert!(QuantifierNOrMore::<char, Scalar::<'a'>, 3>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_n_or_more_more() {
    let mut hay = Haystack::from("aaaaa");
    assert!(QuantifierNOrMore::<char, Scalar::<'a'>, 3>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_n_or_more_too_few() {
    let mut hay = Haystack::from("aa");
    assert!(!QuantifierNOrMore::<char, Scalar::<'a'>, 3>::matches(&mut hay));
}

#[test]
fn test_quantifier_n_or_more_zero() {
    let mut hay = Haystack::from("b");
    assert!(QuantifierNOrMore::<char, Scalar::<'a'>, 0>::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_quantifier_n_or_more_with_bytes() {
    let mut hay = Haystack::from(b"xxxx" as &[u8]);
    assert!(QuantifierNOrMore::<u8, Byte::<b'x'>, 2>::matches(&mut hay));
    assert!(hay.is_end());
}

// Tests for QuantifierNToM matcher
#[test]
fn test_quantifier_n_to_m_at_lower() {
    let mut hay = Haystack::from("aa");
    assert!(QuantifierNToM::<char, Scalar::<'a'>, 2, 4>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_n_to_m_at_upper() {
    let mut hay = Haystack::from("aaaa");
    assert!(QuantifierNToM::<char, Scalar::<'a'>, 2, 4>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_n_to_m_in_middle() {
    let mut hay = Haystack::from("aaa");
    assert!(QuantifierNToM::<char, Scalar::<'a'>, 2, 4>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_quantifier_n_to_m_below_range() {
    let mut hay = Haystack::from("a");
    assert!(!QuantifierNToM::<char, Scalar::<'a'>, 2, 4>::matches(&mut hay));
}

#[test]
fn test_quantifier_n_to_m_above_range() {
    let mut hay = Haystack::from("aaaaa");
    assert!(!QuantifierNToM::<char, Scalar::<'a'>, 2, 4>::matches(&mut hay));
}

#[test]
fn test_quantifier_n_to_m_with_bytes() {
    let mut hay = Haystack::from(b"xxx" as &[u8]);
    assert!(QuantifierNToM::<u8, Byte::<b'x'>, 2, 4>::matches(&mut hay));
    assert!(hay.is_end());
}

// Tests for Beginning matcher
#[test]
fn test_beginning_at_start() {
    let mut hay = Haystack::from("abc");
    assert!(Beginning::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_beginning_after_progress() {
    let mut hay = Haystack::from("abc");
    hay.progress();
    assert!(!Beginning::matches(&mut hay));
}

#[test]
fn test_beginning_empty_haystack() {
    let mut hay = Haystack::from("");
    assert!(Beginning::matches(&mut hay));
}

#[test]
fn test_beginning_with_bytes() {
    let mut hay = Haystack::from(b"test" as &[u8]);
    assert!(Beginning::matches(&mut hay));
}

// Tests for End matcher
#[test]
fn test_end_at_end() {
    let mut hay = Haystack::from("");
    assert!(End::matches(&mut hay));
}

#[test]
fn test_end_not_at_end() {
    let mut hay = Haystack::from("a");
    assert!(!End::matches(&mut hay));
}

#[test]
fn test_end_after_consuming() {
    let mut hay = Haystack::from("a");
    Scalar::<'a'>::matches(&mut hay);
    assert!(End::matches(&mut hay));
}

#[test]
fn test_end_with_bytes() {
    let mut hay = Haystack::from(b"" as &[u8]);
    assert!(End::matches(&mut hay));
}

// Tests for Always matcher
#[test]
fn test_always_matches() {
    let mut hay = Haystack::from("anything");
    assert!(Always::matches(&mut hay));
    assert!(!hay.is_end());
}

#[test]
fn test_always_empty() {
    let mut hay = Haystack::from("");
    assert!(Always::matches(&mut hay));
}

#[test]
fn test_always_with_bytes() {
    let mut hay = Haystack::from(b"data" as &[u8]);
    assert!(Always::matches(&mut hay));
}

// Complex combination tests
#[test]
fn test_complex_or_then() {
    let mut hay = Haystack::from("ab");
    type AB = Then<char, Scalar<'a'>, Scalar<'b'>>;
    type CD = Then<char, Scalar<'c'>, Scalar<'d'>>;
    assert!(Or::<char, AB, CD>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_complex_quantifier_with_range() {
    let mut hay = Haystack::from("abc");
    assert!(QuantifierN::<char, ScalarRange::<'a', 'z'>, 3>::matches(&mut hay));
    assert!(hay.is_end());
}

#[test]
fn test_anchored_pattern() {
    let mut hay = Haystack::from("test");
    type Pattern = Then<char, Beginning, Then<char, Scalar<'t'>, End>>;
    assert!(!Pattern::matches(&mut hay));
}

#[test]
fn test_anchored_single_char() {
    let mut hay = Haystack::from("t");
    type Pattern = Then<char, Beginning, Then<char, Scalar<'t'>, End>>;
    assert!(Pattern::matches(&mut hay));
    assert!(hay.is_end());
}
