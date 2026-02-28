// Enable the crate to reference itself by name (needed for macro expansion)
extern crate self as ct_regex;

pub use ct_regex_internal::{haystack::{Haystack, HaystackItem}, general::{AnonRegex, Regex}};
pub use ct_regex_macro::{regex};

#[doc(hidden)]
pub mod internal {
    pub use ct_regex_internal::*;
}

#[cfg(test)]
mod tests;