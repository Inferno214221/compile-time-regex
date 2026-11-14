use crate::haystack::HaystackSlice;

// To start with, this will be implemented with UTF8 only when the entire expression matches UTF8.
// More atomic group-specific flags may be implemented in the future.

/// An pre-sliced representation of a group's capture.
#[derive(Debug, Clone)]
pub struct CaptureGroup<'a, S: 'a + HaystackSlice + ?Sized, const UTF8: bool> {
    slice: &'a S,
}

impl<'a, S: 'a + HaystackSlice + ?Sized, const U: bool> CaptureGroup<'a, S, U> {
    pub fn new(slice: &'a S) -> Self {
        CaptureGroup {
            slice,
        }
    }

    pub fn as_slice(self) -> &'a S {
        self.slice
    }

    pub fn as_bytes(self) -> &'a [u8] {
        self.slice.as_bytes()
    }
}

impl<'a, S: 'a + HaystackSlice + ?Sized> CaptureGroup<'a, S, true> {
    pub fn as_str(self) -> &'a str {
        unsafe { self.slice.as_str() }
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