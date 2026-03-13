use std::ops::Range;

use crate::{haystack::{Haystack, HaystackItem}, util::cons_tree::ConsTree};

#[derive(Debug, Default, Clone)]
pub struct IndexedCaptures(pub ConsTree<IndexedCapture>);

#[derive(Debug, Clone)]
pub struct IndexedCapture {
    pub index: usize,
    pub cap: Range<usize>,
}

impl IndexedCaptures {
    pub fn push(&mut self, index: usize, cap: Range<usize>) {
        self.0.push(IndexedCapture {
            index,
            cap
        });
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

pub trait FromCaptures<'a, I: HaystackItem, const N: usize>: Sized {
    fn from_captures(captures: [Option<Range<usize>>; N], hay: Haystack<'a, I>) -> Option<Self>;
}