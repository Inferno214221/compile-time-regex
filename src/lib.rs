pub use ct_regex_internal::{haystack::{Haystack, HaystackItem}, general::{AnonRegex, Capture, Regex}};
pub use ct_regex_macro::{regex};

#[doc(hidden)]
pub mod internal {
    pub use ct_regex_internal::*;
}

#[cfg(test)]
mod tests;