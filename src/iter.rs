//! A collection of iterators used in return types for Regex methods. Although also usually
//! inferred, these may be needed to name types in some cases.
//!
//! Unfortunately, all of these types have many type parameters. To make code more concise, try
//! elliding them where possible with `_`. [`Regex::Pattern`](crate::Regex) should be used to obtain
//! the `Matcher` type if required.

pub use ct_regex_internal::expr::{FindAllCaptures, RangeOfAllMatches, SliceAllMatches};