use super::*;

// Tests for Haystack creation from &str
#[test]
fn test_haystack_from_str() {
    let mut hay = Haystack::from("test");
    assert!(!hay.is_end());
}

#[test]
fn test_haystack_from_empty_str() {
    let mut hay = Haystack::from("");
    assert!(hay.is_end());
}

#[test]
fn test_haystack_from_unicode_str() {
    let mut hay = Haystack::from("Hello 🦀 World");
    assert!(!hay.is_end());
}

// Tests for Haystack creation from &[u8]
#[test]
fn test_haystack_from_bytes() {
    let mut hay = Haystack::from(b"test" as &[u8]);
    assert!(!hay.is_end());
}

#[test]
fn test_haystack_from_empty_bytes() {
    let mut hay = Haystack::from(b"" as &[u8]);
    assert!(hay.is_end());
}

// Tests for item() method with chars
#[test]
fn test_item_char_some() {
    let mut hay = Haystack::from("a");
    assert_eq!(hay.item(), Some('a'));
}

#[test]
fn test_item_char_none() {
    let mut hay = Haystack::from("");
    assert_eq!(hay.item(), None);
}

#[test]
fn test_item_char_unicode() {
    let mut hay = Haystack::from("🦀");
    assert_eq!(hay.item(), Some('🦀'));
}

#[test]
fn test_item_char_multibyte() {
    let mut hay = Haystack::from("é");
    assert_eq!(hay.item(), Some('é'));
}

// Tests for item() method with bytes
#[test]
fn test_item_byte_some() {
    let mut hay = Haystack::from(b"a" as &[u8]);
    assert_eq!(hay.item(), Some(b'a'));
}

#[test]
fn test_item_byte_none() {
    let mut hay = Haystack::from(b"" as &[u8]);
    assert_eq!(hay.item(), None);
}

// Tests for progress() method with chars
#[test]
fn test_progress_char() {
    let mut hay = Haystack::from("ab");
    assert_eq!(hay.item(), Some('a'));
    hay.progress();
    assert_eq!(hay.item(), Some('b'));
    hay.progress();
    assert_eq!(hay.item(), None);
}

#[test]
fn test_progress_char_unicode() {
    let mut hay = Haystack::from("🦀🎉");
    assert_eq!(hay.item(), Some('🦀'));
    hay.progress();
    assert_eq!(hay.item(), Some('🎉'));
    hay.progress();
    assert_eq!(hay.item(), None);
}

#[test]
fn test_progress_char_single() {
    let mut hay = Haystack::from("x");
    hay.progress();
    assert_eq!(hay.item(), None);
    assert!(hay.is_end());
}

// Tests for progress() method with bytes
#[test]
fn test_progress_byte() {
    let mut hay = Haystack::from(b"ab" as &[u8]);
    assert_eq!(hay.item(), Some(b'a'));
    hay.progress();
    assert_eq!(hay.item(), Some(b'b'));
    hay.progress();
    assert_eq!(hay.item(), None);
}

// Tests for is_start() method
#[test]
fn test_is_start_initially_true() {
    let mut hay = Haystack::from("test");
    assert!(hay.is_start());
}

#[test]
fn test_is_start_initially_true_empty() {
    let mut hay = Haystack::from("");
    assert!(hay.is_start());
}

#[test]
fn test_is_start_false_after_progress() {
    let mut hay = Haystack::from("test");
    hay.progress();
    assert!(!hay.is_start());
}

#[test]
fn test_is_start_with_bytes() {
    let mut hay = Haystack::from(b"test" as &[u8]);
    assert!(hay.is_start());
    hay.progress();
    assert!(!hay.is_start());
}

// Tests for is_end() method with chars
#[test]
fn test_is_end_empty() {
    let mut hay = Haystack::from("");
    assert!(hay.is_end());
}

#[test]
fn test_is_end_not_empty() {
    let mut hay = Haystack::from("a");
    assert!(!hay.is_end());
}

#[test]
fn test_is_end_after_consuming() {
    let mut hay = Haystack::from("a");
    hay.progress();
    assert!(hay.is_end());
}

#[test]
fn test_is_end_after_partial_consuming() {
    let mut hay = Haystack::from("abc");
    hay.progress();
    assert!(!hay.is_end());
    hay.progress();
    assert!(!hay.is_end());
    hay.progress();
    assert!(hay.is_end());
}

// Tests for is_end() method with bytes
#[test]
fn test_is_end_bytes_empty() {
    let mut hay = Haystack::from(b"" as &[u8]);
    assert!(hay.is_end());
}

#[test]
fn test_is_end_bytes_not_empty() {
    let mut hay = Haystack::from(b"a" as &[u8]);
    assert!(!hay.is_end());
}

#[test]
fn test_is_end_bytes_after_consuming() {
    let mut hay = Haystack::from(b"a" as &[u8]);
    hay.progress();
    assert!(hay.is_end());
}

// Tests for clone behavior
#[test]
fn test_clone_independence() {
    let mut hay1 = Haystack::from("abc");
    let mut hay2 = hay1.clone();

    hay1.progress();
    assert_eq!(hay1.item(), Some('b'));
    assert_eq!(hay2.item(), Some('a'));
}

#[test]
fn test_clone_preserves_state() {
    let mut hay1 = Haystack::from("abc");
    hay1.progress();

    let mut hay2 = hay1.clone();
    assert_eq!(hay1.item(), hay2.item());
}

#[test]
fn test_clone_start_flag() {
    let mut hay1 = Haystack::from("abc");
    assert!(hay1.is_start());

    let mut hay2 = hay1.clone();
    assert!(hay2.is_start());

    hay1.progress();
    let mut hay3 = hay1.clone();
    assert!(!hay3.is_start());
}

// Tests for HaystackItem trait implementations
#[test]
fn test_haystack_item_u8_from_str() {
    let iter = <u8 as HaystackItem>::from_str("abc");
    let bytes: Vec<u8> = iter.collect();
    assert_eq!(bytes, vec![b'a', b'b', b'c']);
}

#[test]
fn test_haystack_item_char_from_str() {
    let iter = <char as HaystackItem>::from_str("abc");
    let chars: Vec<char> = iter.collect();
    assert_eq!(chars, vec!['a', 'b', 'c']);
}

#[test]
fn test_haystack_item_char_from_str_unicode() {
    let iter = <char as HaystackItem>::from_str("🦀🎉");
    let chars: Vec<char> = iter.collect();
    assert_eq!(chars, vec!['🦀', '🎉']);
}

// Integration tests
#[test]
fn test_multiple_operations() {
    let mut hay = Haystack::from("hello");
    assert!(hay.is_start());
    assert!(!hay.is_end());
    assert_eq!(hay.item(), Some('h'));

    hay.progress();
    assert!(!hay.is_start());
    assert!(!hay.is_end());
    assert_eq!(hay.item(), Some('e'));

    hay.progress();
    hay.progress();
    hay.progress();
    hay.progress();
    assert!(!hay.is_start());
    assert!(hay.is_end());
    assert_eq!(hay.item(), None);
}

#[test]
fn test_peek_does_not_consume() {
    let mut hay = Haystack::from("test");
    let first = hay.item();
    let second = hay.item();
    let third = hay.item();
    assert_eq!(first, second);
    assert_eq!(second, third);
    assert_eq!(first, Some('t'));
}
