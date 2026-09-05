# Compile Time Regex (`ct-regex`)

A crate for compiling regular expressions into Rust code at compile-time.

Documentation and a better description is available at [rust.inferno214221.com/ct_regex](https://rust.inferno214221.com/ct_regex).

## Structure

This repository contains three crates:

- `ct_regex`, which provide the public interface for this crate.
  - This crate is separate to allow exporting public types and the macro side by side but also as a method of controlling which types are a part of the public API.
- `ct_regex_macro`, a proc_macro crate that defines nothing other than the main `regex!` macro.
  - As proc_macro crates have restrictions about what they can do, this crate exists only to export a macro for use by Rust, redirecting the actual implementation to `ct_regex_internal`.
- `ct_regex_internal`, which provides all of the types both public and private that form the implementation.
  - This is where everything interesting happens. All public types and traits are implemented here but many internal types are also exported for use by macro generated code. This specific sub-crate will not follow semver, any types exported here and not in the main `ct_regex` crate may change at any point and are intended for usage only by macro-generated code.

## Feature Flags

- `demo` - Intended for documentation only, exports a macro generated type as a demonstration.
- `arcstr` - Adds haystack implementations for the [arcstr](https://docs.rs/arcstr/) crate.
- `bstr` - Adds haystack implementations for the [bstr](https://docs.rs/bstr/) crate.
- `ecow` - Adds haystack implementations for the [ecow](https://docs.rs/ecow/) crate.
- `hipstr` - Adds haystack implementations for the [hipstr](https://docs.rs/hipstr/) crate.