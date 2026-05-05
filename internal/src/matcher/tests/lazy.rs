use crate::expr::IndexedCaptures;
use crate::haystack::{Haystack, IntoHaystack};

use super::*;

use crate::{implements_debug, test_doesnt_match_no_index, test_matches_with_indices};

type ScalarA = Scalar<'a'>;
type ScalarB = Scalar<'b'>;

type QuantifierNOrMoreA<const N: usize> = QuantifierNOrMore<char, ScalarA, N>;
type QuantifierNToMA<const N: usize, const M: usize> = QuantifierNToM<char, ScalarA, N, M>;

#[allow(clippy::module_inception)]
mod lazy {
    use super::*;

    #[test]
    fn n_matches_minimally() {
        test_matches_with_indices!(Lazy<_, QuantifierNOrMoreA<2>>, "aaa", vec![2, 3]);
        test_matches_with_indices!(Lazy<_, QuantifierNToMA<2, 3>>, "aaaa", vec![2, 3]);
    }

    #[test]
    fn n_matches_zero() {
        test_matches_with_indices!(Lazy<_, QuantifierNOrMoreA<0>>, "aaa", vec![0, 1, 2, 3]);
        test_matches_with_indices!(Lazy<_, QuantifierNToMA<0, 1>>, "aaa", vec![0, 1]);
    }

    #[test]
    fn performs_rollback_as_required() {
        test_matches_with_indices!(Then<_, Lazy<_, QuantifierNOrMoreA<0>>, ScalarB>, "aab", vec![3]);
        test_doesnt_match_no_index!(Then<_, Lazy<_, QuantifierNToMA<0, 2>>, ScalarB>, "aaab");
        test_matches_with_indices!(Then<_, Lazy<_, QuantifierNOrMore<_, Or<_, ScalarA, ScalarB>, 0>>, ScalarB>, "abb", vec![2, 3]);
    }

    #[test]
    fn less_than_n_doesnt_match() {
        test_doesnt_match_no_index!(Lazy<_, QuantifierNOrMoreA<2>>, "a");
        test_doesnt_match_no_index!(Lazy<_, QuantifierNToMA<2, 3>>, "a");
    }

    #[test]
    fn not_a_doesnt_match() {
        test_doesnt_match_no_index!(Lazy<_, QuantifierNOrMoreA<1>>, "b");
        test_doesnt_match_no_index!(Lazy<_, QuantifierNToMA<1, 3>>, "b");
    }
}

#[test]
fn implements_debug() {
    implements_debug!(
        Lazy<char, QuantifierNOrMoreA<0>>
    );
}