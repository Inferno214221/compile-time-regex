use std::{marker::PhantomData};

use crate::{group::marker::IsUtf8, haystack::HaystackSlice};

// An pre-sliced representation of a group's capture.
#[derive(Debug, Clone)]
pub struct CaptureGroup<'a, S: HaystackSlice<'a>> {
    pub slice: S,
    pub _phantom: PhantomData<&'a ()>,
}

impl<'a, S: HaystackSlice<'a>> CaptureGroup<'a, S> {
    pub fn into_slice(self) -> S {
        self.slice
    }

    pub fn into_bytes(self) -> &'a [u8] {
        self.slice.into_bytes()
    }
}

impl<'a, S: HaystackSlice<'a>> CaptureGroup<'a, S> where Self: IsUtf8 {
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