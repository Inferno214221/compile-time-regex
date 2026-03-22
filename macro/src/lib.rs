extern crate proc_macro;

mod args;
mod codegen;

use args::*;
use codegen::*;

use syn::parse_macro_input;

#[proc_macro]
pub fn regex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match parse_macro_input!(input as RegexArgType) {
        RegexArgType::Regex(args) => make_regex(args, false).into(),
        RegexArgType::Anon(pat) => make_anon_regex(pat).into(),
    }
}