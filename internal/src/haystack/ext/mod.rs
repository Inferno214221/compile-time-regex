/// Extra haystack implementations for the [`arcstr`](https://docs.rs/arcstr/latest/arcstr/) crate.
#[cfg(feature = "arcstr")]
pub mod arcstr;

/// Extra haystack implementations for the [`bstr`](https://docs.rs/bstr/latest/bstr/) crate.
#[cfg(feature = "bstr")]
pub mod bstr;

/// Extra haystack implementations for the [`ecow`](https://docs.rs/bstr/latest/ecow/) crate.
///
/// This module doesn't actually contain any types but the feature gate enables
/// [`OwnedHaystackable`](super::OwnedHaystackable) implementations for
/// [`EcoString`](../trait.OwnedHaystackable.html#impl-OwnedHaystackable<char>-for-EcoString) and
/// [`EcoVec<u8>`](../trait.OwnedHaystackable.html#impl-OwnedHaystackable<u8>-for-EcoVec<u8>).
#[cfg(feature = "ecow")]
pub mod ecow;

/// Extra haystack implementations for the [`hipstr`](https://docs.rs/hipstr/latest/hipstr/) crate.
#[cfg(feature = "hipstr")]
pub mod hipstr;
