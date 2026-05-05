extern crate proc_macro;

use ct_regex_internal::codegen::{RegexArgType, regex::{make_anon_regex, make_regex}};
use syn::parse_macro_input;

// Doc links are broken until re-exported.

/// A macro to create, at compile time, a type that can match the provided regular expression.
///
/// This method can be invoked in one of two ways, each with different arguments providing slightly
/// different results:
///
/// # Type Definition
///
/// This macro invocation defines a zero-sized type that implements [`Regex`](trait.Regex.html),
/// matching and capturing against the provided expression.
///
/// ```ignore
/// regex! {
///   visibility? TypeName = r"pattern" / flags?
/// }
///
/// let _ = TypeName::is_match("haystack");
/// ```
///
/// Where the arguments are as follows:
/// - `visibility`: An optional visibility modifier for the generated type, e.g. `pub`,
///   `pub(crate)`.
/// - `TypeName`: The name for the generated type with an auto-generated impl of the
///   [`Regex`](trait.Regex.html) trait.
/// - `pattern`: A string literal providing the regular expression that you'd like this type to
///   match. Note, you probably want to pass this as a raw literal e.g. `r"expr"` or `r#expr#`.
/// - `flags`: An optional string literal providing a list of flags for this expression to use when
///   matching. See [Flags](macro.regex.html#flags) below.
///
/// A 'capture' type is also generated to represent the result of filling this expression's capture
/// groups from a valid haystack. The generated type is named the same as `TypeNameCapture` and is
/// also available via `<TypeName as Regex>::Capture`.
///
/// # Anonymous Type Expression
///
/// This macro invocation producing a similar result to the type-definition form, but as an
/// anonymous type (unnameable) that is returned as an expression to allow chaining and quick usage.
/// The type implements [`AnonRegex`](trait.AnonRegex.html) as well as [`Regex`](trait.Regex.html),
/// so that the associated functions can be called as methods.
///
/// ```ignore
/// let _ = regex!(r"pattern" / flags?).is_match("haystack");
/// ```
///
/// Where the arguments are the same as their equivalents above, but not visiblity or type name is
/// required.
///
/// # Flags
///
/// The flats available are relatively standard and are implemented according to those provided by
/// [`regex_automata::util::syntax::Config`](https://docs.rs/regex-automata/latest/regex_automata/util/syntax/struct.Config.html),
/// with one addition, `'c'`.
/// One notable exception to the standard options is the absence of a _global_ (`'g'`) flag.
///
/// Available flags:
/// ```ignore
/// match flag {
///     'i' => config.case_insensitive(true),
///     'm' => config.multi_line(true),
///     's' => config.dot_matches_new_line(true),
///     'R' => config.crlf(true),
///     'U' => config.swap_greed(true),
///     'x' => config.ignore_whitespace(true),
///     'c' => config.complex_classes(true),
///     ..
/// }
/// ```
///
/// The _complex classes_ flag (`'c'`), when enabled, expands perl character classes (`\w`, `\d`,
/// `\s`) to their full unicode versions. This is how `regex_syntax` behaves, but it could easily
/// trip people up. The default behavior (with this flag disabled) aliases the classes to their
/// ascii variants as follows:
/// - `\w` -> `[0-9A-Za-z_]`
/// - `\d` -> `[0-9]`
/// - `\s` -> `[\t\n\v\f\r ]`
///
/// ## Aside on Global Flag
///
/// I find the global flag to be unintuitive and it would be unessecarily restricting in this
/// implementation: you may want to use the same pattern with and without the global functionality,
/// but you'd need to define the same expression multiple times to do so. (This might make sense in
/// some cases where the Regex itself tracks state but we don't do that here.) Anyway, there is no
/// global flag. Instead, all methods of the [`Regex`](trait.Regex.html) trait clearly define how
/// they behave - have a read.
#[proc_macro]
pub fn regex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match parse_macro_input!(input as RegexArgType) {
        RegexArgType::Regex(args) => make_regex(args, false).into(),
        RegexArgType::Anon(pat) => make_anon_regex(pat).into(),
    }
}