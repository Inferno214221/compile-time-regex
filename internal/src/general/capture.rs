use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Capture<'a> {
    pub range: Range<usize>,
    pub content: &'a str,
}

impl<'a> Capture<'a> {
    pub fn content(&self) -> &str {
        self.content
    }

    pub fn range(&self) -> &Range<usize> {
        &self.range
    }
}