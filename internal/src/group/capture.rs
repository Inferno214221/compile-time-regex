use std::{marker::PhantomData};

use crate::{haystack::HaystackSlice};

// To start with, this will be implemented with UTF8 only when the entire expression matches UTF8.
// More atomic group-specific flags may be implemented in the future.

/// An pre-sliced representation of a group's capture.
#[derive(Debug, Clone)]
pub struct CaptureGroup<'a, S: HaystackSlice<'a>, const UTF8: bool> {
    pub slice: S,
    pub _phantom: PhantomData<&'a ()>,
}

impl<'a, S: HaystackSlice<'a>, const U: bool> CaptureGroup<'a, S, U> {
    pub fn into_slice(self) -> S {
        self.slice
    }

    pub fn into_bytes(self) -> &'a [u8] {
        self.slice.into_bytes()
    }
}

impl<'a, S: HaystackSlice<'a>> CaptureGroup<'a, S, true> {
    pub fn into_str(self) -> &'a str {
        unsafe { self.slice.into_str() }
    }
}

// Don't implement Deref, which would make method origins unclear.
// impl<'a, S: HaystackSlice<'a>> Deref for CaptureGroup<'a, S> {
//     type Target = S;
//
//     fn deref(&self) -> &Self::Target {
//         &self.slice
//     }
// }