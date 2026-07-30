use std::ops::ControlFlow;

use regex_syntax::hir::Properties;

use crate::haystack::{Haystack, HaystackSlice};

#[derive(Debug, Clone, Copy)]
pub struct Anchors {
    pub min_len: Option<usize>,
    pub max_len: Option<usize>,
    pub start: bool,
    pub end: bool,
}

pub enum AnchorHint {
    Continue,
}

impl Anchors {
    pub fn from_properties(props: &Properties) -> Anchors {
        Anchors {
            min_len: props.minimum_len(),
            max_len: props.maximum_len(),
            start: props.look_set_prefix().contains_anchor_haystack(),
            end: props.look_set_suffix().contains_anchor_haystack(),
        }
    }

    /// Asserts that each anchor in this set could possibly succeed with the given haystack state.
    /// In the presence of a start anchor, the haystack's position at the start doesn't need to be
    /// checked again.
    ///
    /// The return value represents two things:
    ///
    /// - The outer [`ControlFlow`] represents whether the haystack has reached a point of
    ///   no-return. If the value is [`ControlFlow::Break`], no more matches will succeed; no
    ///   further attempts should be made to match against the haystack.
    ///   For functions that return an option, it should be possible to try the return value with
    ///   `.continue_value()?`.
    ///
    /// - If the value of the outer type is [`ControlFlow::Continue`], the inner [`bool`] represents
    ///   whether the current position should be checked for a match. A value of `false` indicates
    ///   that the haystack should be progressed before checking again.
    pub fn assert<'a, H: Haystack<'a>>(
        &self,
        hay: &H
    ) -> ControlFlow<(), bool> {
        if self.start && !hay.is_start() {
            return ControlFlow::Break(());
        }
        let len = hay.remainder_as_slice().as_bytes().len();
        if let Some(min) = self.min_len && len < min {
            return ControlFlow::Break(());
        }
        if self.end && let Some(max) = self.max_len && len > max {
            return ControlFlow::Continue(false);
        }
        ControlFlow::Continue(true)
    }

    /// Asserts that each anchor in this set could possibly succeed with the given haystack state.
    /// In the presence of a start anchor, the haystack's position at the start doesn't need to be
    /// checked again. This variant provide no information about whether the search should continue
    /// and should be called by searches that intend to match the entirety of the provided haystack.
    pub fn assert_fixed<'a, H: Haystack<'a>>(&self, hay: &H) -> bool {
        let len = hay.remainder_as_slice().as_bytes().len();
        if let Some(min) = self.min_len && len < min {
            return false;
        }
        if self.end && let Some(max) = self.max_len && len > max {
            return false;
        }
        true
    }
}