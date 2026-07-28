use super::*;

mod byte_slice {
    use super::*;

    #[test]
    fn basic_iteration() {
        let mut haystack: ByteStack = b"abc".into_haystack();
        assert_eq!(haystack.item(), Some(b'a'));
        assert_eq!(haystack.item(), Some(b'a'));
        assert_eq!(haystack.index(), 0);
        assert_eq!(haystack.next(), Some(b'a'));

        assert_eq!(haystack.item(), Some(b'b'));
        assert_eq!(haystack.index(), 1);
        assert_eq!(haystack.prev_item(), Some(b'a'));
        assert_eq!(haystack.item(), Some(b'b'));
        assert_eq!(haystack.remainder_as_slice(), b"bc");
        assert_eq!(haystack.inner_slice(), b"abc");
        assert_eq!(haystack.next(), Some(b'b'));

        assert_eq!(haystack.item(), Some(b'c'));
        assert_eq!(haystack.next(), Some(b'c'));
        assert_eq!(haystack.item(), None);
        assert_eq!(haystack.next(), None);
    }

    #[test]
    fn doesnt_understand_unicode_boundaries() {
        let mut haystack: ByteStack = "😀🧑‍🔬".as_bytes().into_haystack();
        assert_eq!(haystack.item(), Some("😀".as_bytes()[0]));
        assert_eq!(haystack.index(), 0);
        haystack.next();

        assert_eq!(haystack.item(), Some("😀".as_bytes()[1]));
        assert_eq!(haystack.index(), 1);
        haystack.next();
        haystack.next();
        haystack.next();

        assert_eq!(haystack.item(), Some("🧑".as_bytes()[0]));
        assert_eq!(haystack.index(), 4);
        haystack.next();
        haystack.next();
        haystack.next();

        assert_eq!(haystack.item(), Some("🧑".as_bytes()[3]));
        assert_eq!(haystack.index(), 7);
        haystack.next();

        assert_eq!(haystack.item(), Some("\u{200d}".as_bytes()[0]));
        assert_eq!(haystack.index(), 8);

        assert_eq!(haystack.remainder_as_slice(), "\u{200d}🔬".as_bytes());
    }

    #[test]
    fn rolls_back_successfully() {
        let mut haystack: ByteStack = b"abc".into_haystack();
        haystack.go_to(2);
        haystack.item();
        haystack.go_to(1);
        haystack.item();

        let mut haystack: ByteStack = "😀🧑‍🔬".as_bytes().into_haystack();
        haystack.go_to(8);
        haystack.item();
        haystack.go_to(4);
        haystack.item();
    }

    #[test]
    fn doesnt_panic_on_char_boundary_rollback() {
        let mut haystack: ByteStack = "😀".as_bytes().into_haystack();
        haystack.go_to(1);
        haystack.item();
    }
}

mod byte_owned {
    use super::*;

    #[test]
    fn basic_conversions_and_replace() {
        let mut hay = Vec::<u8>::from(b"abcd");
        assert_eq!(hay.as_haystack().inner_slice(), b"abcd");
        assert_eq!(OwnedHaystackable::as_slice(&hay), b"abcd");
        assert_eq!(OwnedHaystackable::len(&hay), 4);
        OwnedHaystackable::replace_range(&mut hay, 1..3, b"ef");
        assert_eq!(OwnedHaystackable::as_slice(&hay), b"aefd");
    }

    #[test]
    fn unicode_conversions_and_replace() {
        let mut hay = Vec::<u8>::from("a🧑‍🔬c".as_bytes());
        assert_eq!(hay.as_haystack().inner_slice(), "a🧑‍🔬c".as_bytes());
        assert_eq!(OwnedHaystackable::as_slice(&hay), "a🧑‍🔬c".as_bytes());
        assert_eq!(OwnedHaystackable::len(&hay), 13);
        OwnedHaystackable::replace_range(&mut hay, 5..8, b"b");
        assert_eq!(OwnedHaystackable::as_slice(&hay), "a🧑b🔬c".as_bytes());
    }
}