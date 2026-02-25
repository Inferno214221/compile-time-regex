// Tests that verify the regex! and anon_regex! macros generate correct code structure

use crate::*;

// Test that regex! macro creates a struct that implements Regex trait
#[test]
fn test_regex_macro_creates_struct() {
    regex!(TestPattern = "a");

    // Should be able to instantiate (zero-sized type)
    let _instance = TestPattern;
}

#[test]
fn test_regex_macro_implements_regex_trait_for_char() {
    regex!(TestCharRegex = "abc");

    // Should implement Regex<char>
    fn assert_implements_regex<T: Regex<char>>() {}
    assert_implements_regex::<TestCharRegex>();
}

#[test]
fn test_regex_macro_implements_regex_trait_for_u8() {
    regex!(TestByteRegex = "xyz");

    // Should implement Regex<u8>
    fn assert_implements_regex<T: Regex<u8>>() {}
    assert_implements_regex::<TestByteRegex>();
}

#[test]
fn test_regex_macro_with_public_visibility() {
    // This should compile without errors
    regex!(pub PublicPattern = "test");

    // Can use it like any other struct
    let _instance = PublicPattern;
}

#[test]
fn test_regex_macro_with_private_visibility() {
    // Default (no visibility) should work
    regex!(PrivatePattern = "test");

    let _instance = PrivatePattern;
}

#[test]
fn test_regex_macro_with_pub_crate_visibility() {
    // pub(crate) visibility should work
    regex!(pub(crate) CratePattern = "test");

    let _instance = CratePattern;
}

// Test that regex! creates something that implements AnonRegex when used anonymously
#[test]
fn test_anon_regex_implements_anon_regex_trait_char() {
    let pattern = regex!("test");

    // Should implement AnonRegex<char>
    fn assert_anon_regex<T: AnonRegex<char>>(_: &T) {}
    assert_anon_regex(&pattern);
}

#[test]
fn test_anon_regex_implements_anon_regex_trait_u8() {
    let pattern = regex!("test");

    // Should implement AnonRegex<u8>
    fn assert_anon_regex<T: AnonRegex<u8>>(_: &T) {}
    assert_anon_regex(&pattern);
}

#[test]
fn test_anon_regex_is_expression() {
    // regex! should be usable directly in expressions (anonymous form)
    let result = regex!("x").matches(&mut Haystack::from("x"));
    assert!(result);
}

// Test that complex patterns compile correctly
#[test]
fn test_regex_macro_with_alternation() {
    regex!(AlternationPattern = "foo|bar");
    let _instance = AlternationPattern;
}

#[test]
fn test_regex_macro_with_quantifiers() {
    regex!(QuantPattern = "a+b*c?d{3}e{2,5}");
    let _instance = QuantPattern;
}

#[test]
fn test_regex_macro_with_character_classes() {
    regex!(ClassPattern = r"[a-z][A-Z][0-9]");
    let _instance = ClassPattern;
}

#[test]
fn test_regex_macro_with_anchors() {
    regex!(AnchorPattern = r"^start[a-z]+end$");
    let _instance = AnchorPattern;
}

#[test]
fn test_regex_macro_with_groups() {
    regex!(GroupPattern = r"(abc)(def)(?:xyz)");
    let _instance = GroupPattern;
}

#[test]
fn test_regex_macro_with_escape_sequences() {
    regex!(EscapePattern = r"\n\t\r\\");
    let _instance = EscapePattern;
}

#[test]
fn test_regex_macro_with_unicode() {
    regex!(UnicodePattern = "hello🦀world");
    let _instance = UnicodePattern;
}

// Test that both forms work with the same pattern
#[test]
fn test_both_forms_with_same_pattern() {
    regex!(NamedTest = "test");
    let anon = regex!("test");

    let _named_instance = NamedTest;
    let _anon_instance = anon;
}

// Test that multiple regex! invocations work
#[test]
fn test_multiple_regex_declarations() {
    regex!(Pattern1 = "a");
    regex!(Pattern2 = "b");
    regex!(Pattern3 = "c");

    let _p1 = Pattern1;
    let _p2 = Pattern2;
    let _p3 = Pattern3;
}

// Test that multiple anonymous regex! invocations work
#[test]
fn test_multiple_anon_regex_uses() {
    let _p1 = regex!("a");
    let _p2 = regex!("b");
    let _p3 = regex!("c");
}

// Test empty pattern compiles
#[test]
fn test_empty_pattern() {
    regex!(EmptyPattern = "");
    let _instance = EmptyPattern;
}

// Test very complex pattern
#[test]
fn test_complex_realistic_pattern() {
    // Email-like pattern
    regex!(EmailLike = r"[a-z]+@[a-z]+\.[a-z]+");
    let _instance = EmailLike;
}

#[test]
fn test_url_like_pattern() {
    // URL-like pattern
    regex!(UrlLike = r"^https?://[a-z]+(\.[a-z]+)*");
    let _instance = UrlLike;
}

// Test that the generated types have expected properties
#[test]
fn test_generated_type_is_zero_sized() {
    regex!(ZstPattern = "test");

    // Zero-sized types have size 0
    assert_eq!(std::mem::size_of::<ZstPattern>(), 0);
}

#[test]
fn test_anon_regex_type_is_zero_sized() {
    let pattern = regex!("test");

    // The generated anonymous type should also be zero-sized
    assert_eq!(std::mem::size_of_val(&pattern), 0);
}

// Test that regex! can be used in const contexts
#[test]
fn test_regex_in_const_context() {
    regex!(ConstPattern = "const");

    const _PATTERN: ConstPattern = ConstPattern;
}

// Test pattern with special regex metacharacters
#[test]
fn test_pattern_with_metacharacters() {
    regex!(MetaPattern = r"\+\?\[\]\(\)\{\}\|\^$");
    let _instance = MetaPattern;
}

// Test that both implementations exist simultaneously
#[test]
fn test_both_implementations_exist() {
    regex!(DualImpl = "test");

    // Can create haystacks for both types
    let mut hay_char = Haystack::from("test");
    let mut hay_byte = Haystack::from(b"test" as &[u8]);

    // Both should work
    let _matches_char = DualImpl::matches(&mut hay_char);
    let _matches_byte = DualImpl::matches(&mut hay_byte);
}

#[test]
fn test_anon_regex_both_implementations() {
    let pattern = regex!("test");

    let mut hay_char = Haystack::from("test");
    let mut hay_byte = Haystack::from(b"test" as &[u8]);

    let _matches_char = pattern.matches(&mut hay_char);
    let _matches_byte = pattern.matches(&mut hay_byte);
}
