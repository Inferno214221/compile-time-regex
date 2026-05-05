//! A demonstration of the types produced by the `regex!` macro.
//!
//! The [`Email`] and [`EmailCapture`] types have both been implemented by the following code:
//!
//! ```
//! regex! {
//!     pub Email = r"(\w+)@(?<domain>(\w+)(\.\w+)?)"
//! }
//! ```
//!
//! Notable features include:
//! - A fully expanded 'matcher' type expression under
//! [`Email::Pattern`](struct.Email.html#associatedtype.Pattern-1), used to call a set of associated
//! methods and perform the compile time matching.
//! - Numbered and named captures generated specfic to the regular expression.
//!     - Optional and essential captures use [`Option`] as required.
//! - Lazily sliced capture groups.

use ct_regex_macro::regex;

regex! {
    pub Email = r"(\w+)@(?<domain>(\w+)(\.\w+)?)"
}
