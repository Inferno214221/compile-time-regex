pub mod anchor;
pub mod codegen;
pub mod expr;
pub mod haystack;
pub mod matcher;

pub(crate) mod sealed {
    pub trait Sealed {}
}
