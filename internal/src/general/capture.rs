use std::{mem, ops::Range, rc::Rc};

use crate::haystack::{Haystack, HaystackItem};

#[derive(Debug, Clone)]
pub struct Capture(pub Range<usize>);

#[derive(Debug, Clone)]
pub struct IndexedCapturesInner {
    pub index: usize,
    pub cap: Capture,
    // Tada: A reference counted cones list for hydra-style head clones and an auto Drop impl.
    pub prev: Rc<IndexedCaptures>,
}

#[derive(Debug, Default, Clone)]
pub struct IndexedCaptures(pub Option<IndexedCapturesInner>);

impl IndexedCaptures {
    pub fn push(&mut self, index: usize, cap: Capture) {
        let old = mem::replace(self, IndexedCaptures(None));
        *self = IndexedCaptures(Some(IndexedCapturesInner {
            index,
            cap,
            prev: Rc::new(old),
        }));
    }

    // May contain duplicates for a certain index. To avoid backtracking and overriding, we deal
    // with this here.
    pub fn into_array<const N: usize>(&self) -> [Option<Capture>; N] {
        let mut res = [const { None }; N];

        let mut next = self;

        while let IndexedCaptures(Some(item)) = next {
            // We're traversing captures backwards, so we need to keep the value written into the
            // array first.
            if res[item.index].is_none() {
                res[item.index] = Some(item.cap.clone());
            }

            next = &*item.prev;
        }

        res
    }
}

pub trait FromCaptures<'a, I: HaystackItem, const N: usize>: Sized {
    fn from_captures(captures: [Option<Capture>; N], hay: Haystack<'a, I>) -> Option<Self>;
}