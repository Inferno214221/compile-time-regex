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
    let iter = <u8 as HaystackItem>::iter_from_str("abc");
    let bytes: Vec<u8> = iter.collect();
    assert_eq!(bytes, vec![b'a', b'b', b'c']);
}

#[test]
fn test_haystack_item_char_from_str() {
    let iter = <char as HaystackItem>::iter_from_str("abc");
    let chars: Vec<char> = iter.collect();
    assert_eq!(chars, vec!['a', 'b', 'c']);
}

#[test]
fn test_haystack_item_char_from_str_unicode() {
    let iter = <char as HaystackItem>::iter_from_str("🦀🎉");
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

// ============================================================================
// Tests for StrIter
// ============================================================================

// Tests for StrIter Iterator implementation
#[test]
fn test_str_iter_next_basic() {
    let mut iter = StrIter::from("abc");
    assert_eq!(iter.next(), Some('a'));
    assert_eq!(iter.next(), Some('b'));
    assert_eq!(iter.next(), Some('c'));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_str_iter_next_empty() {
    let mut iter = StrIter::from("");
    assert_eq!(iter.next(), None);
}

#[test]
fn test_str_iter_next_unicode() {
    let mut iter = StrIter::from("🦀🎉");
    assert_eq!(iter.next(), Some('🦀'));
    assert_eq!(iter.next(), Some('🎉'));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_str_iter_next_multibyte() {
    let mut iter = StrIter::from("café");
    assert_eq!(iter.next(), Some('c'));
    assert_eq!(iter.next(), Some('a'));
    assert_eq!(iter.next(), Some('f'));
    assert_eq!(iter.next(), Some('é'));
    assert_eq!(iter.next(), None);
}

// Tests for StrIter::current_item
#[test]
fn test_str_iter_current_item_basic() {
    let iter = StrIter::from("abc");
    assert_eq!(iter.current_item(), Some('a'));
}

#[test]
fn test_str_iter_current_item_empty() {
    let iter = StrIter::from("");
    assert_eq!(iter.current_item(), None);
}

#[test]
fn test_str_iter_current_item_does_not_advance() {
    let iter = StrIter::from("abc");
    assert_eq!(iter.current_item(), Some('a'));
    assert_eq!(iter.current_item(), Some('a'));
    assert_eq!(iter.current_item(), Some('a'));
}

#[test]
fn test_str_iter_current_item_after_next() {
    let mut iter = StrIter::from("abc");
    iter.next();
    assert_eq!(iter.current_item(), Some('b'));
}

#[test]
fn test_str_iter_current_item_unicode() {
    let iter = StrIter::from("🦀test");
    assert_eq!(iter.current_item(), Some('🦀'));
}

// Tests for StrIter::current_index
#[test]
fn test_str_iter_current_index_initial() {
    let iter = StrIter::from("abc");
    assert_eq!(iter.current_index(), 0);
}

#[test]
fn test_str_iter_current_index_after_next() {
    let mut iter = StrIter::from("abc");
    iter.next();
    assert_eq!(iter.current_index(), 1);
    iter.next();
    assert_eq!(iter.current_index(), 2);
}

#[test]
fn test_str_iter_current_index_unicode() {
    let mut iter = StrIter::from("🦀b");
    assert_eq!(iter.current_index(), 0);
    iter.next(); // Skip the 4-byte emoji
    assert_eq!(iter.current_index(), 4); // Index is byte position, not char position
}

#[test]
fn test_str_iter_current_index_empty() {
    let iter = StrIter::from("");
    assert_eq!(iter.current_index(), 0);
}

// Tests for StrIter::is_start
#[test]
fn test_str_iter_is_start_initial() {
    let iter = StrIter::from("abc");
    assert!(iter.is_start());
}

#[test]
fn test_str_iter_is_start_empty() {
    let iter = StrIter::from("");
    assert!(iter.is_start());
}

#[test]
fn test_str_iter_is_start_after_next() {
    let mut iter = StrIter::from("abc");
    iter.next();
    assert!(!iter.is_start());
}

// Tests for StrIter::as_slice
#[test]
fn test_str_iter_as_slice() {
    let iter = StrIter::from("hello");
    assert_eq!(iter.as_slice(), "hello");
}

#[test]
fn test_str_iter_as_slice_after_progress() {
    let mut iter = StrIter::from("hello");
    iter.next();
    iter.next();
    assert_eq!(iter.as_slice(), "hello"); // as_slice returns full string
}

#[test]
fn test_str_iter_as_slice_empty() {
    let iter = StrIter::from("");
    assert_eq!(iter.as_slice(), "");
}

// Tests for StrIter::rem_as_slice
#[test]
fn test_str_iter_rem_as_slice_initial() {
    let iter = StrIter::from("hello");
    assert_eq!(iter.rem_as_slice(), "hello");
}

#[test]
fn test_str_iter_rem_as_slice_after_progress() {
    let mut iter = StrIter::from("hello");
    iter.next();
    assert_eq!(iter.rem_as_slice(), "ello");
    iter.next();
    assert_eq!(iter.rem_as_slice(), "llo");
}

#[test]
fn test_str_iter_rem_as_slice_at_end() {
    let mut iter = StrIter::from("ab");
    iter.next();
    iter.next();
    assert_eq!(iter.rem_as_slice(), "");
}

#[test]
fn test_str_iter_rem_as_slice_unicode() {
    let mut iter = StrIter::from("🦀bc");
    iter.next();
    assert_eq!(iter.rem_as_slice(), "bc");
}

// Tests for StrIter::slice_with
#[test]
fn test_str_iter_slice_with() {
    let iter = StrIter::from("hello");
    assert_eq!(iter.slice_with(0..5), "hello");
    assert_eq!(iter.slice_with(1..4), "ell");
    assert_eq!(iter.slice_with(0..0), "");
}

#[test]
fn test_str_iter_slice_with_unicode() {
    let iter = StrIter::from("🦀bc");
    assert_eq!(iter.slice_with(0..4), "🦀"); // First 4 bytes are the emoji
    assert_eq!(iter.slice_with(4..6), "bc");
}

// Tests for StrIter clone
#[test]
fn test_str_iter_clone_independence() {
    let mut iter1 = StrIter::from("abc");
    iter1.next();
    let iter2 = iter1.clone();

    iter1.next();
    assert_eq!(iter1.current_item(), Some('c'));
    assert_eq!(iter2.current_item(), Some('b'));
}

// ============================================================================
// Tests for ByteIter
// ============================================================================

// Tests for ByteIter Iterator implementation
#[test]
fn test_byte_iter_next_basic() {
    let mut iter = ByteIter::from(b"abc" as &[u8]);
    assert_eq!(iter.next(), Some(b'a'));
    assert_eq!(iter.next(), Some(b'b'));
    assert_eq!(iter.next(), Some(b'c'));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_byte_iter_next_empty() {
    let mut iter = ByteIter::from(b"" as &[u8]);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_byte_iter_next_binary() {
    let data: &[u8] = &[0x00, 0xFF, 0x7F];
    let mut iter = ByteIter::from(data);
    assert_eq!(iter.next(), Some(0x00));
    assert_eq!(iter.next(), Some(0xFF));
    assert_eq!(iter.next(), Some(0x7F));
    assert_eq!(iter.next(), None);
}

// Tests for ByteIter::current_item
#[test]
fn test_byte_iter_current_item_basic() {
    let iter = ByteIter::from(b"abc" as &[u8]);
    assert_eq!(iter.current_item(), Some(b'a'));
}

#[test]
fn test_byte_iter_current_item_empty() {
    let iter = ByteIter::from(b"" as &[u8]);
    assert_eq!(iter.current_item(), None);
}

#[test]
fn test_byte_iter_current_item_does_not_advance() {
    let iter = ByteIter::from(b"abc" as &[u8]);
    assert_eq!(iter.current_item(), Some(b'a'));
    assert_eq!(iter.current_item(), Some(b'a'));
    assert_eq!(iter.current_item(), Some(b'a'));
}

#[test]
fn test_byte_iter_current_item_after_next() {
    let mut iter = ByteIter::from(b"abc" as &[u8]);
    iter.next();
    assert_eq!(iter.current_item(), Some(b'b'));
}

// Tests for ByteIter::current_index
#[test]
fn test_byte_iter_current_index_initial() {
    let iter = ByteIter::from(b"abc" as &[u8]);
    assert_eq!(iter.current_index(), 0);
}

#[test]
fn test_byte_iter_current_index_after_next() {
    let mut iter = ByteIter::from(b"abc" as &[u8]);
    iter.next();
    assert_eq!(iter.current_index(), 1);
    iter.next();
    assert_eq!(iter.current_index(), 2);
}

#[test]
fn test_byte_iter_current_index_empty() {
    let iter = ByteIter::from(b"" as &[u8]);
    assert_eq!(iter.current_index(), 0);
}

// Tests for ByteIter::is_start
#[test]
fn test_byte_iter_is_start_initial() {
    let iter = ByteIter::from(b"abc" as &[u8]);
    assert!(iter.is_start());
}

#[test]
fn test_byte_iter_is_start_empty() {
    let iter = ByteIter::from(b"" as &[u8]);
    assert!(iter.is_start());
}

#[test]
fn test_byte_iter_is_start_after_next() {
    let mut iter = ByteIter::from(b"abc" as &[u8]);
    iter.next();
    assert!(!iter.is_start());
}

// Tests for ByteIter::as_slice
#[test]
fn test_byte_iter_as_slice() {
    let iter = ByteIter::from(b"hello" as &[u8]);
    assert_eq!(iter.as_slice(), b"hello");
}

#[test]
fn test_byte_iter_as_slice_after_progress() {
    let mut iter = ByteIter::from(b"hello" as &[u8]);
    iter.next();
    iter.next();
    assert_eq!(iter.as_slice(), b"hello"); // as_slice returns full slice
}

#[test]
fn test_byte_iter_as_slice_empty() {
    let iter = ByteIter::from(b"" as &[u8]);
    assert_eq!(iter.as_slice(), b"");
}

// Tests for ByteIter::rem_as_slice
#[test]
fn test_byte_iter_rem_as_slice_initial() {
    let iter = ByteIter::from(b"hello" as &[u8]);
    assert_eq!(iter.rem_as_slice(), b"hello");
}

#[test]
fn test_byte_iter_rem_as_slice_after_progress() {
    let mut iter = ByteIter::from(b"hello" as &[u8]);
    iter.next();
    assert_eq!(iter.rem_as_slice(), b"ello");
    iter.next();
    assert_eq!(iter.rem_as_slice(), b"llo");
}

#[test]
fn test_byte_iter_rem_as_slice_at_end() {
    let mut iter = ByteIter::from(b"ab" as &[u8]);
    iter.next();
    iter.next();
    assert_eq!(iter.rem_as_slice(), b"");
}

// Tests for ByteIter::slice_with
#[test]
fn test_byte_iter_slice_with() {
    let iter = ByteIter::from(b"hello" as &[u8]);
    assert_eq!(iter.slice_with(0..5), b"hello");
    assert_eq!(iter.slice_with(1..4), b"ell");
    assert_eq!(iter.slice_with(0..0), b"");
}

// Tests for ByteIter clone
#[test]
fn test_byte_iter_clone_independence() {
    let mut iter1 = ByteIter::from(b"abc" as &[u8]);
    iter1.next();
    let iter2 = iter1.clone();

    iter1.next();
    assert_eq!(iter1.current_item(), Some(b'c'));
    assert_eq!(iter2.current_item(), Some(b'b'));
}
