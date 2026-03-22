extern crate proc_macro;

use ct_regex_internal::codegen::{RegexArgType, regex::{make_anon_regex, make_regex}};
use syn::parse_macro_input;

#[proc_macro]
pub fn regex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match parse_macro_input!(input as RegexArgType) {
        RegexArgType::Regex(args) => make_regex(args, false).into(),
        RegexArgType::Anon(pat) => make_anon_regex(pat).into(),
    }
}