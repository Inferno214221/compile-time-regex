//! A collection of iterators used in return types for Regex methods. Although also usually
//! inferred, these may be needed to name types in some cases.

pub use ct_regex_internal::expr::{FindAllCaptures, RangeOfAllMatches, SliceAllMatches};