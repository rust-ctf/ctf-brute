use nonempty::{nonempty, NonEmpty};
mod constant;
mod iter;
mod merge;

use super::{bruterange::BruteRangeIter, BruteRange};

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct MBruteRange {
    pub(crate) ranges: NonEmpty<BruteRange>,
    pub(crate) indexes: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct MBruteRangeIter<'a> {
    pub(crate) index: usize,
    pub(crate) iters: Vec<BruteRangeIter<'a>>,
}

impl MBruteRange {
    pub fn from_range(range: BruteRange) -> Self {
        Self {
            ranges: nonempty![range],
            indexes: vec![range.len()],
        }
    }

    pub fn from_ranges(ranges: NonEmpty<BruteRange>) -> Self {
        let ranges = Self::merge_and_order_ranges(ranges);
        let mut indexes = Vec::with_capacity(ranges.len());
        _ = ranges.iter().fold(0u32, |s, r| {
            indexes.push(s);
            s + r.len()
        });
        Self { ranges, indexes }
    }

    pub fn len(&self) -> usize {
        self.ranges.iter().map(|x| x.len() as usize).sum()
    }
}

impl From<BruteRange> for MBruteRange {
    fn from(range: BruteRange) -> Self {
        Self::from_range(range)
    }
}
