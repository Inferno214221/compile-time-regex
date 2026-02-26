pub use ct_regex_internal::{haystack::{Haystack, HaystackItem}, traits::{AnonRegex, Regex}};
pub use ct_regex_macro::{regex};

#[cfg(test)]
mod tests {
    mod codegen;
    mod contains_find;
    mod matches;
}