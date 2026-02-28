use std::{mem, ops::Range, rc::Rc};

use crate::haystack::{Haystack, HaystackItem};

#[derive(Debug, Clone)]
pub struct Capture(pub Range<usize>);

#[derive(Debug, Clone)]
pub struct IndexedCaptures {
    pub index: usize,
    pub range: Capture,
    // Tada: A reference counted cones list for hydra-style head clones and an auto Drop impl.
    pub prev: Option<Rc<IndexedCaptures>>,
}

impl IndexedCaptures {
    pub fn push(&mut self, index: usize, range: Capture) {
        let old = mem::replace(self, IndexedCaptures {
            index,
            range,
            prev: None,
        });
        self.prev = Some(Rc::new(old));
    }

    // May contain duplicates for a certain index. To avoid backtracking and overriding, we deal
    // with this here.
    pub fn into_array<const N: usize>(&self) -> [Option<Capture>; N] {
        let mut res = [const { None }; N];

        let mut item = self;

        res[item.index] = Some(item.range.clone());
        
        while let Some(prev) = &item.prev {
            item = &**prev;

            // We're traversing captures backwards, so we need to keep the value written into the
            // array first.
            if res[item.index].is_none() {
                res[item.index] = Some(item.range.clone());
            }
        }

        res
    }
}

pub trait FromCaptures<'a, I: HaystackItem, const N: usize>: Sized {
    fn from_captures(captures: [Option<Capture>; N], hay: &'a Haystack<'a, I>) -> Option<Self>;
}