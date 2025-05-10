use std::ops::{self, Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl Span {
    pub fn new(lo: usize, hi: usize) -> Self {
        Self { lo, hi }
    }

    pub fn len(&self) -> usize {
        self.hi - self.lo
    }

    pub fn is_empty(&self) -> bool {
        self.hi == self.lo
    }

    pub fn contains(&self, other: Span) -> bool {
        self.lo <= other.lo && self.hi >= other.hi
    }

    pub fn offset(self, offset: usize) -> Self {
        Self {
            lo: self.lo + offset,
            hi: self.hi + offset,
        }
    }

    pub fn adjacent(&self, rhs: Span) -> bool {
        self.hi == rhs.lo || self.lo == rhs.hi
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[index.lo..index.hi]
    }
}

impl IndexMut<Span> for str {
    fn index_mut(&mut self, index: Span) -> &mut Self::Output {
        &mut self[index.lo..index.hi]
    }
}

impl ops::Add for Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            lo: self.lo.min(rhs.lo),
            hi: self.hi.max(rhs.hi),
        }
    }
}
