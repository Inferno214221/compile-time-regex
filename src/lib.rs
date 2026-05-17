//! A crate for creating types that match regular expressions at compile time.
//! [`Regex`]-implementing types can be created with the [`regex!`] macro.
//!
//! This crate was heavily inspired by [`ctreg`](https://docs.rs/ctreg/latest/ctreg/), which
//! provides named, infallible capture groups and syntax error checking at compile time.
//!
//! I'm yet to do a complexity analysis on this crate, but it should generally have time complexity
//! `O(n*m)` where `n` and `m` are the length of the pattern and haystack, the same as most regex
//! crates.
//!
//! # Approach
//!
//! How does this crate differ from the many other regex crates on crates.io?
//!
//! The answer is in the name: it creates types that match regular expressions at _compile time_, as
//! opposed to runtime like most other implementations.
//!
//! 1. As with most crates, this one starts by parsing the provided expressions using the
//!    [`regex_syntax`](https://docs.rs/regex-syntax/latest/regex_syntax/) crate, producing an
//!    _abstract syntax tree_ before translating and optimising into a _high-level intermedite
//!    representation_ (HIR).
//!
//! 2. Rather than using [NFAs](https://en.wikipedia.org/wiki/Thompson%27s_construction) or DFAs,
//!    the macro converts the HIR into a Rust type expression, made of
//!    [`Matcher`](ct_regex_internal::matcher::Matcher) components that describe the various actions
//!    needed to match / capture a regular expression. An _simple_ example of this generated type
//!    expression can be seen at
//!    [`demo::Email::Pattern`](demo/struct.Email.html#associatedtype.Pattern-1).
//!
//! 3. The macro finishes and the binary is compiled normally, using a collection of associated
//!    functions on each `Matcher` to perform the relvant matching / capturing. In short, matching
//!    or capturing at runtime boils down to a series of function calls, which the Rust compile can
//!    optimise as it sees fit.
//!
//! # When To Use This Crate
//!
//! If you're writing a small piece of software using expressions known at compile time and you
//! don't want to package a whole regex interpreter into your binary. It's also a good idea if you
//! only use an expression a few times.
//!
//! Parsing strings from an input file or command line arguments using the named captures is
//! probably one of the major benefits, and the reason I started writing this in the first place.
//!
//! # When Not To Use This Crate
//!
//! For runtime regular expressions (_gasp_). Seriously though, most of the work done by this crate
//! occurs when building the binary, so it isn't possible to create expressions on the fly. See one
//! of the other crates listed above if this is something you want.
//!
//! After the other regular expressions are compiled, they can achieve speeds a fair bit faster than
//! this crate, using parallel operations and such.
//!
//! Some complex functionality isn't implement yet, primarily complex look-arounds. An error will
//! occur at **compile-time** if you try to use an unimplemented feature.

#![feature(doc_cfg)]

// TODO: Double check time complexity, write some benchmarks.

// Enable the crate to reference itself by name (needed for macro expansion)
extern crate self as ct_regex;

pub use ct_regex_internal::expr::{AnonRegex, Regex};
pub use ct_regex_macro::regex;

#[cfg(feature = "demo")]
#[doc(cfg(feature = "demo"))]
pub mod demo;

pub mod haystack;
pub mod iter;

#[doc(hidden)]
pub mod internal {
    pub use ct_regex_internal::*;
}

#[cfg(test)]
mod tests;
