use std::{mem, ops::Range, rc::Rc};

use crate::haystack::{Haystack, HaystackItem};

#[derive(Debug, Clone)]
pub struct IndexedCapturesInner {
    pub index: usize,
    pub cap: Range<usize>,
    // Tada: A reference counted cones list for hydra-style head clones and an auto Drop impl.
    pub prev: Rc<IndexedCaptures>,
}

#[derive(Debug, Default, Clone)]
pub struct IndexedCaptures(pub Option<IndexedCapturesInner>);

impl IndexedCaptures {
    pub fn push(&mut self, index: usize, cap: Range<usize>) {
        let old = mem::replace(self, IndexedCaptures(None));
        *self = IndexedCaptures(Some(IndexedCapturesInner {
            index,
            cap,
            prev: Rc::new(old),
        }));
    }

    // May contain duplicates for a certain index. To avoid backtracking and overriding, we deal
    // with this here.
    pub fn into_array<const N: usize>(&self) -> [Option<Range<usize>>; N] {
        let mut res = [const { None }; N];

        let mut next = self;

        while let IndexedCaptures(Some(item)) = next {
            // We're traversing captures backwards, so we need to keep the value written into the
            // array first.
            match res.get(item.index) {
                None => panic!("capture index exceeds maximum group number"),
                Some(None) => res[item.index] = Some(item.cap.clone()),
                Some(_) => (),
            }
            if let Some(None) | None = res.get(item.index) {
                res[item.index] = Some(item.cap.clone());
            }

            next = &*item.prev;
        }

        res
    }
}

pub trait FromCaptures<'a, I: HaystackItem, const N: usize>: Sized {
    fn from_captures(captures: [Option<Range<usize>>; N], hay: Haystack<'a, I>) -> Option<Self>;
}