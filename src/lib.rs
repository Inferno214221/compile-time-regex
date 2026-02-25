pub use ct_regex_internal::{haystack::{Haystack, HaystackItem}, traits::{AnonRegex, Regex}};
pub use ct_regex_macro::{anon_regex, regex};

#[cfg(test)]
mod tests {
    mod codegen;
    mod matches;
}