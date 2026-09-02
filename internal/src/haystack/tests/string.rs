use super::*;

mod string_slice {
    use super::*;

    #[test]
    fn basic_iteration() {
        let mut haystack: StrStack = "abc".into_haystack();
        assert_eq!(haystack.item(), Some('a'));
        assert_eq!(haystack.item(), Some('a'));
        assert_eq!(haystack.index(), 0);
        assert_eq!(haystack.next(), Some('a'));

        assert_eq!(haystack.item(), Some('b'));
        assert_eq!(haystack.index(), 1);
        assert_eq!(haystack.prev_item(), Some('a'));
        assert_eq!(haystack.item(), Some('b'));
        assert_eq!(haystack.remainder_as_slice(), "bc");
        assert_eq!(haystack.inner_slice(), "abc");
        assert_eq!(haystack.next(), Some('b'));

        assert_eq!(haystack.item(), Some('c'));
        assert_eq!(haystack.next(), Some('c'));
        assert_eq!(haystack.item(), None);
        assert_eq!(haystack.next(), None);
    }

    #[test]
    fn understands_unicode_boundaries() {
        let mut haystack: StrStack = "😀🧑‍🔬".into_haystack();
        assert_eq!(haystack.item(), Some('😀'));
        assert_eq!(haystack.index(), 0);
        haystack.next();

        assert_eq!(haystack.item(), Some('🧑'));
        assert_eq!(haystack.index(), 4);
        haystack.next();

        assert_eq!(haystack.item(), Some('\u{200d}'));
        assert_eq!(haystack.index(), 8);

        assert_eq!(haystack.remainder_as_slice(), "\u{200d}🔬");
    }

    #[test]
    fn rolls_back_successfully() {
        let mut haystack: StrStack = "abc".into_haystack();
        haystack.go_to(2);
        haystack.item();
        haystack.go_to(1);
        haystack.item();

        let mut haystack: StrStack = "😀🧑‍🔬".into_haystack();
        haystack.go_to(8);
        haystack.item();
        haystack.go_to(4);
        haystack.item();
    }

    #[test]
    #[should_panic]
    fn panics_on_char_boundary_rollback() {
        let mut haystack: StrStack = "😀".into_haystack();
        haystack.go_to(1);
        haystack.item();
    }
}

mod string_owned {
    use super::*;

    #[test]
    fn basic_conversions_and_replace() {
        let mut hay = String::from("abcd");
        assert_eq!(hay.as_haystack().inner_slice(), "abcd");
        assert_eq!(OwnedHaystackable::as_slice(&hay), "abcd");
        assert_eq!(OwnedHaystackable::len(&hay), 4);
        OwnedHaystackable::replace_range(&mut hay, 1..3, "ef");
        assert_eq!(OwnedHaystackable::as_slice(&hay), "aefd");
    }

    #[test]
    fn unicode_conversions_and_replace() {
        let mut hay = String::from("a🧑‍🔬c");
        assert_eq!(hay.as_haystack().inner_slice(), "a🧑‍🔬c");
        assert_eq!(OwnedHaystackable::as_slice(&hay), "a🧑‍🔬c");
        assert_eq!(OwnedHaystackable::len(&hay), 13);
        OwnedHaystackable::replace_range(&mut hay, 5..8, "b");
        assert_eq!(OwnedHaystackable::as_slice(&hay), "a🧑b🔬c");
    }
}
