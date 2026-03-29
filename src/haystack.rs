//! A collection of traits and structs that form the haystack system. Although usually implicit,
//! these type may be needed on occasion for full type names etc.
//!
//! Additionally, it _should_ be possible to implement these traits for other types to allow
//! matching different strings and other types, but not all of the traits will be required.
//!
//! The main traits in this crate are chained together with associated items:
//! ```
//! trait HaystackItem {}
//!
//! trait HaystackSlice<'a> {
//!     type Item: HaystackItem;
//! }
//!
//! trait HaystackIter<'a> {
//!     type Slice: HaystackSlice<'a>;
//! }
//! ```
//!
//! The primary types that will fill these roles are:
//!
//! - `Item`: [`char`]
//! - `Slice<'a>`: [`&'a str`](str)
//! - `HaystackIter<'a>`: [`StrIter<'a>`]
//!
//! but byte-based types may also be used:
//!
//! - `Item`: [`u8`]
//! - `Slice<'a>`: [`&'a [u8]`](slice)
//! - `HaystackIter<'a>`: [`ByteIter<'a>`]
//!
//! It needs to be noted that regardless of the haystack type being matched, the regular expression
//! provided to the `regex!` macro needs to be valid UTF-8.

pub use ct_regex_internal::haystack::{ByteIter, Haystack, HaystackItem, HaystackIter, HaystackSlice, HaystackWith, StrIter};