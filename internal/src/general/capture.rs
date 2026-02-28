use std::{mem, ops::Range, rc::Rc};

use crate::haystack::{HaystackItem, HaystackIter};

#[derive(Debug, Clone)]
pub struct Capture<'a, I: HaystackItem + 'a> {
    pub range: Range<usize>,
    pub content: <I::Iter<'a> as HaystackIter<'a>>::Slice<'a>,
}

impl<'a, I: HaystackItem> Capture<'a, I> {
    pub fn content(&'a self) -> <I::Iter<'a> as HaystackIter<'a>>::Slice<'a> {
        self.content
    }

    pub fn range(&self) -> &Range<usize> {
        &self.range
    }
}

#[derive(Debug, Clone)]
pub struct IndexedCaptures {
    pub index: usize,
    pub range: Range<usize>,
    // Tada: A reference counted cones list for hydra-style head clones and an auto Drop impl.
    pub prev: Option<Rc<IndexedCaptures>>,
}

impl IndexedCaptures {
    pub fn push(&mut self, index: usize, range: Range<usize>) {
        let old = mem::replace(self, IndexedCaptures {
            index,
            range,
            prev: None,
        });
        self.prev = Some(Rc::new(old));
    }

    // May contain duplicates for a certain index. To avoid backtracking and overriding, we deal
    // with this here.
    pub fn into_array<const N: usize>(&self) -> [Option<Range<usize>>; N] {
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
    fn from_captures(captures: [Option<Capture<'a, I>>; N]) -> Option<Self>;
}