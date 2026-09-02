use super::*;
use crate::expr::IndexedCaptures;
use crate::haystack::{Haystack, IntoHaystack};
use crate::{implements_debug, test_doesnt_match_no_index, test_matches_with_index};

#[derive(Debug, Default, Clone, Copy)]
struct HayLiteral;

impl Literal for HayLiteral {
    const LITERAL: &[u8] = b"hay";
}

type HayLitMatcher = LiteralMatcher<HayLiteral>;

#[derive(Debug, Default, Clone, Copy)]
struct HaystackLiteral;

impl Literal for HaystackLiteral {
    const LITERAL: &[u8] = b"HayStack";
}

type HaystackLitMatcher = LiteralMatcher<HaystackLiteral>;

#[derive(Debug, Default, Clone, Copy)]
struct EmojiLiteral;

impl Literal for EmojiLiteral {
    const LITERAL: &[u8] = "😀🧑‍🔬".as_bytes();
}

type EmojiLitMatcher = LiteralMatcher<EmojiLiteral>;

mod literal {
    use super::*;

    #[test]
    fn correct_match() {
        test_matches_with_index!(HayLitMatcher, "hay", 3);
        test_matches_with_index!(HayLitMatcher, "hays", 3);
        test_matches_with_index!(HaystackLitMatcher, "HayStack", 8);
        test_matches_with_index!(HaystackLitMatcher, "HayStackk", 8);
        test_matches_with_index!(EmojiLitMatcher, "😀🧑‍🔬", 15);
    }

    #[test]
    fn incorrect_doesnt_match() {
        test_doesnt_match_no_index!(HayLitMatcher, "ha");
        test_doesnt_match_no_index!(HayLitMatcher, "b");
        test_doesnt_match_no_index!(HaystackLitMatcher, "haystack");
        test_doesnt_match_no_index!(HaystackLitMatcher, "Hay");
        test_doesnt_match_no_index!(EmojiLitMatcher, "😀");
        test_doesnt_match_no_index!(EmojiLitMatcher, "no");
    }
}

#[test]
fn implements_debug() {
    implements_debug!(
        HayLitMatcher,
        HaystackLitMatcher
    );
}