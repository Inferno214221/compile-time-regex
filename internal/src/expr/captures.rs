use std::fmt::Debug;
use std::ops::Range;

use standard_lib::collections::cons::ConsBranch;

use crate::haystack::HaystackSlice;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct IndexedCaptures(pub ConsBranch<IndexedCapture>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexedCapture {
    pub index: usize,
    pub cap: Range<usize>,
}

impl IndexedCaptures {
    pub fn push(&mut self, index: usize, cap: Range<usize>) {
        self.0.push(IndexedCapture { index, cap });
    }

    // May contain duplicates for a certain index. To avoid backtracking and overriding, we deal
    // with this here.
    pub fn into_array<const N: usize>(self) -> [Option<Range<usize>>; N] {
        let mut res = [const { None }; N];

        for item in self.0.into_iter_owned() {
            match res.get(item.index) {
                None => panic!("capture index exceeds maximum group number"),
                Some(None) => res[item.index] = Some(item.cap.clone()),
                // We're traversing captures backwards, so we need to keep the value written into
                // the array first.
                Some(_) => (),
            }
        }

        res
    }
}

pub trait CaptureFromRanges<'a, S: HaystackSlice<'a>, const N: usize>: Sized + Debug {
    fn from_ranges(ranges: [Option<Range<usize>>; N], slice: S) -> Option<Self>;
    fn whole_match_range(&self) -> Range<usize>;
}
