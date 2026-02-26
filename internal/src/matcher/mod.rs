#[cfg(test)]
mod test;

pub mod primitive;
pub mod composite;
pub mod quanitfier;

pub use primitive::*;
pub use composite::*;
pub use quanitfier::*;