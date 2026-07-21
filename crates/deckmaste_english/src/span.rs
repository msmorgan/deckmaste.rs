use std::ops::Range;

/// A half-open UTF-8 byte range in the Oracle text passed to [`crate::parse`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    #[must_use]
    pub const fn len(self) -> usize {
        self.end.saturating_sub(self.start)
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start >= self.end
    }

    /// Returns the source text covered by this span, or `None` if the span did
    /// not originate from this source string.
    #[must_use]
    pub fn text(self, source: &str) -> Option<&str> {
        source.get(self.start..self.end)
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self::new(range.start, range.end)
    }
}
