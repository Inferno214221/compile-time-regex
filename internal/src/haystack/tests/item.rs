use super::*;

mod char_item {
    use super::*;

    #[test]
    fn first_char_helpers() {
        assert_eq!(first_char_and_width("ab"), (1, Some('a')));
        assert_eq!(first_char_and_width("😀"), (4, Some('😀')));
        assert_eq!(first_char_and_width("🧑‍🔬"), (4, Some('🧑')));
        assert_eq!(first_char("ab"), Some('a'));
        assert_eq!(first_char("😀"), Some('😀'));
        assert_eq!(first_char("🧑‍🔬"), Some('🧑'));
    }

    #[test]
    fn vec_from_str() {
        assert_eq!(char::vec_from_str("abc"), vec!['a', 'b', 'c']);
        assert_eq!(char::vec_from_str("😀🧑‍🔬"), vec!['😀', '🧑', '\u{200d}', '🔬']);
    }

    #[test]
    fn is_newline() {
        assert!('\n'.is_newline());
        assert!(!'\r'.is_newline());
        assert!(!' '.is_newline());
        assert!(!'a'.is_newline());
    }

    #[test]
    fn is_return() {
        assert!('\r'.is_return());
        assert!(!'\n'.is_return());
        assert!(!' '.is_return());
        assert!(!'a'.is_return());
    }
}

mod u8_item {
    use super::*;

    #[test]
    fn vec_from_str() {
        assert_eq!(u8::vec_from_str("abc"), vec![b'a', b'b', b'c']);
        assert_eq!(
            u8::vec_from_str("😀🧑‍🔬"),
            vec![
                0xf0, 0x9f, 0x98, 0x80,
                0xf0, 0x9f, 0xa7, 0x91,
                0xe2, 0x80, 0x8d,
                0xf0, 0x9f, 0x94, 0xac
            ]
        );
    }

    #[test]
    fn is_newline() {
        assert!(b'\n'.is_newline());
        assert!(!b'\r'.is_newline());
        assert!(!b' '.is_newline());
        assert!(!b'a'.is_newline());
    }

    #[test]
    fn is_return() {
        assert!(b'\r'.is_return());
        assert!(!b'\n'.is_return());
        assert!(!b' '.is_return());
        assert!(!b'a'.is_return());
    }
}