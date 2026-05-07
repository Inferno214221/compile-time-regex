use super::*;

mod string_slice {
    use super::*;

    #[test]
    fn basic_iteration() {
        let mut haystack: StrStack = "abc".into_haystack();
        assert_eq!(haystack.current_item(), Some('a'));
        assert_eq!(haystack.current_item(), Some('a'));
        assert_eq!(haystack.current_index(), 0);
        assert_eq!(haystack.next(), Some('a'));

        assert_eq!(haystack.current_item(), Some('b'));
        assert_eq!(haystack.current_index(), 1);
        assert_eq!(haystack.prev_item(), Some('a'));
        assert_eq!(haystack.current_item(), Some('b'));
        assert_eq!(haystack.remainder_as_slice(), "bc");
        assert_eq!(haystack.whole_slice(), "abc");
        assert_eq!(haystack.next(), Some('b'));

        assert_eq!(haystack.current_item(), Some('c'));
        assert_eq!(haystack.next(), Some('c'));
        assert_eq!(haystack.current_item(), None);
        assert_eq!(haystack.next(), None);
    }

    #[test]
    fn understands_unicode_boundaries() {
        let mut haystack: StrStack = "😀🧑‍🔬".into_haystack();
        assert_eq!(haystack.current_item(), Some('😀'));
        assert_eq!(haystack.current_index(), 0);
        haystack.next();

        assert_eq!(haystack.current_item(), Some('🧑'));
        assert_eq!(haystack.current_index(), 4);
        haystack.next();

        assert_eq!(haystack.current_item(), Some('\u{200d}'));
        assert_eq!(haystack.current_index(), 8);

        assert_eq!(haystack.remainder_as_slice(), "\u{200d}🔬");
    }

    #[test]
    fn rolls_back_successfully() {
        let mut haystack: StrStack = "abc".into_haystack();
        haystack.go_to(2);
        haystack.current_item();
        haystack.go_to(1);
        haystack.current_item();

        let mut haystack: StrStack = "😀🧑‍🔬".into_haystack();
        haystack.go_to(8);
        haystack.current_item();
        haystack.go_to(4);
        haystack.current_item();
    }

    #[test]
    #[should_panic]
    fn panics_on_invalid_rollback() {
        let mut haystack: StrStack = "😀".into_haystack();
        haystack.go_to(1);
        haystack.current_item();
    }
}

mod string_owned {
    use super::*;

    #[test]
    fn basic_conversions_and_replace() {
        let mut hay = String::from("abcd");
        assert_eq!(hay.as_haystack().whole_slice(), "abcd");
        assert_eq!(hay.as_slice(), "abcd");
        assert_eq!(OwnedHaystackable::len(&hay), 4);
        OwnedHaystackable::replace_range(&mut hay, 1..3, "ef");
        assert_eq!(hay.as_slice(), "aefd");
    }

    #[test]
    fn unicode_conversions_and_replace() {
        let mut hay = String::from("a🧑‍🔬c");
        assert_eq!(hay.as_haystack().whole_slice(), "a🧑‍🔬c");
        assert_eq!(hay.as_slice(), "a🧑‍🔬c");
        assert_eq!(OwnedHaystackable::len(&hay), 13);
        hay.replace_range(5..8, "b");
        assert_eq!(hay.as_slice(), "a🧑b🔬c");
    }
}