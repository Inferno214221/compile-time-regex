/// Extra haystack implementations for the [`arcstr`](https://docs.rs/arcstr/latest/arcstr/) crate.
#[cfg(feature = "arcstr")]
pub mod arcstr;

/// Extra haystack implementations for the [`bstr`](https://docs.rs/bstr/latest/bstr/) crate.
#[cfg(feature = "bstr")]
pub mod bstr;

/// Extra haystack implementations for the [`ecow`](https://docs.rs/bstr/latest/ecow/) crate.
#[cfg(feature = "ecow")]
pub mod ecow;

/// Extra haystack implementations for the [`hipstr`](https://docs.rs/hipstr/latest/hipstr/) crate.
#[cfg(feature = "hipstr")]
pub mod hipstr;
