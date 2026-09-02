use super::*;
use crate::expr::IndexedCaptures;
use crate::haystack::{Haystack, IntoHaystack};
use crate::{implements_debug, test_doesnt_match_no_index, test_matches_with_index};

#[derive(Debug, Default, Clone, Copy)]
struct BasicCharClass;

impl Class<char> for BasicCharClass {
    const ENTRIES: &[ClassEntry<char>] = &[
        ClassEntry::new('a', false),
        ClassEntry::new('c', false),
        ClassEntry::new('e', false),
        ClassEntry::new('g', true),
    ];
}

type BasicCharClassMatcher = ClassMatcher<char, BasicCharClass>;

#[derive(Debug, Default, Clone, Copy)]
struct BasicByteClass;

impl Class<u8> for BasicByteClass {
    const ENTRIES: &[ClassEntry<u8>] = &[
        ClassEntry::new(b'a', false),
        ClassEntry::new(b'c', false),
        ClassEntry::new(b'e', false),
        ClassEntry::new(b'g', true),
        ClassEntry::new(0xF0, false),
    ];
}

type BasicByteClassMatcher = ClassMatcher<u8, BasicByteClass>;

#[derive(Debug, Default, Clone, Copy)]
struct ScalarClass;

impl Class<char> for ScalarClass {
    const ENTRIES: &[ClassEntry<char>] = &[
        ClassEntry::new('\u{2160}', false),
        ClassEntry::new('\u{216f}', true),
        ClassEntry::new('😀', false),
    ];
}

type ScalarClassMatcher = ClassMatcher<char, ScalarClass>;

mod class {
    use super::*;

    #[test]
    fn correct_match() {
        test_matches_with_index!(BasicCharClassMatcher, "a", 1);
        test_matches_with_index!(BasicCharClassMatcher, "c", 1);
        test_matches_with_index!(BasicCharClassMatcher, "e", 1);
        test_matches_with_index!(BasicCharClassMatcher, "f", 1);
        test_matches_with_index!(BasicCharClassMatcher, "g", 1);
        test_matches_with_index!(BasicCharClassMatcher, "ab", 1);

        test_matches_with_index!(ScalarClassMatcher, "\u{2160}", 3);
        test_matches_with_index!(ScalarClassMatcher, "\u{216e}", 3);
        test_matches_with_index!(ScalarClassMatcher, "\u{216f}", 3);
        test_matches_with_index!(ScalarClassMatcher, "😀", 4);

        test_matches_with_index!(BasicByteClassMatcher, b"a", 1);
        test_matches_with_index!(BasicByteClassMatcher, b"c", 1);
        test_matches_with_index!(BasicByteClassMatcher, b"e", 1);
        test_matches_with_index!(BasicByteClassMatcher, b"f", 1);
        test_matches_with_index!(BasicByteClassMatcher, b"g", 1);
        test_matches_with_index!(BasicByteClassMatcher, b"ab", 1);
        test_matches_with_index!(BasicByteClassMatcher, b"\xF0", 1);
    }

    #[test]
    fn incorrect_doesnt_match() {
        test_doesnt_match_no_index!(BasicCharClassMatcher, "b");
        test_doesnt_match_no_index!(BasicCharClassMatcher, "d");
        test_doesnt_match_no_index!(BasicCharClassMatcher, "h");

        test_doesnt_match_no_index!(ScalarClassMatcher, "a");
        test_doesnt_match_no_index!(ScalarClassMatcher, "\u{2170}");

        test_doesnt_match_no_index!(BasicByteClassMatcher, b"b");
        test_doesnt_match_no_index!(BasicByteClassMatcher, b"d");
        test_doesnt_match_no_index!(BasicByteClassMatcher, b"h");
    }
}

#[test]
fn implements_debug() {
    implements_debug!(
        BasicCharClassMatcher,
        ScalarClassMatcher,
        BasicByteClassMatcher
    );
}
