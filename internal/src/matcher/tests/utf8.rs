use super::*;
use crate::expr::IndexedCaptures;
use crate::haystack::{Haystack, IntoHaystack};
use crate::test_matches_with_index;

type Empty = ZeroWidthCheck<u8, Always>;

mod zero_width_check {
    use super::*;

    #[test]
    fn correct_match() {
        test_matches_with_index!(Empty, b"", 0);
        test_matches_with_index!(Empty, b"a", 0);
    }

    #[test]
    fn all_matches_eq_scalar_version() {
        let caps = IndexedCaptures::default();
        let mut hay_scalar = "😀🧑‍🔬".into_haystack();
        let mut hay_byte = "😀🧑‍🔬".as_bytes().into_haystack();

        let mut all_scalar_indices = Vec::new();

        while !hay_scalar.is_end() {
            let index = hay_scalar.index();

            if Always::matches(&mut hay_scalar) {
                all_scalar_indices.push(index);
            }

            assert_eq!(
                Always::matches(hay_scalar.rollback(index)),
                Always::captures(hay_scalar.rollback(index), &mut caps.clone())
            );

            hay_scalar.rollback(index).progress();
        }

        let mut all_byte_indices = Vec::new();

        while !hay_byte.is_end() {
            let index = hay_byte.index();

            if Empty::matches(&mut hay_byte) {
                all_byte_indices.push(index);
            }

            assert_eq!(
                Empty::matches(hay_byte.rollback(index)),
                Empty::captures(hay_byte.rollback(index), &mut caps.clone())
            );

            hay_byte.rollback(index).progress();
        }

        assert_eq!(all_scalar_indices, all_byte_indices);
    }

    #[test]
    fn no_check_produces_extra_matches() {

        let caps = IndexedCaptures::default();
        let mut hay_scalar = "😀🧑‍🔬".into_haystack();
        let mut hay_byte = "😀🧑‍🔬".as_bytes().into_haystack();

        let mut all_scalar_indices = Vec::new();

        while !hay_scalar.is_end() {
            let index = hay_scalar.index();

            if Always::matches(&mut hay_scalar) {
                all_scalar_indices.push(index);
            }

            assert_eq!(
                Always::matches(hay_scalar.rollback(index)),
                Always::captures(hay_scalar.rollback(index), &mut caps.clone())
            );

            hay_scalar.rollback(index).progress();
        }

        let mut all_byte_indices = Vec::new();

        while !hay_byte.is_end() {
            let index = hay_byte.index();

            // Use Always rather than the wrapped version.
            if Always::matches(&mut hay_byte) {
                all_byte_indices.push(index);
            }

            assert_eq!(
                Always::matches(hay_byte.rollback(index)),
                Always::captures(hay_byte.rollback(index), &mut caps.clone())
            );

            hay_byte.rollback(index).progress();
        }

        assert_ne!(all_scalar_indices, all_byte_indices);
    }
}