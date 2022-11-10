use nonempty::{nonempty, NonEmpty};
use std::iter::Flatten;

mod constant;
mod iter;
mod merge;

use super::{BruteRange, bruterange::BruteRangeIter};

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct MBruteRange {
    pub(crate) ranges: NonEmpty<BruteRange>,
}

#[derive(Clone, Debug)]
pub struct MBruteRangeIter<'a> {
    pub(crate) mrange: &'a MBruteRange,
    pub(crate) index: usize,
    pub(crate) iters: Vec<BruteRangeIter<'a>> 
}

impl MBruteRange {
    pub const fn from_range(range: BruteRange) -> Self {
        Self {
            ranges: nonempty![range],
        }
    }

    pub fn from_ranges(ranges: NonEmpty<BruteRange>) -> Self {
        let ranges = Self::merge_and_order_ranges(ranges);
        Self { ranges }
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
