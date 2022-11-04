use nonempty::{nonempty, NonEmpty};
use std::iter::Flatten;

mod constant;
mod iter;
mod merge;

use super::BruteRange;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct MBruteRange {
    pub(crate) ranges: NonEmpty<BruteRange>,
}

pub struct MBruteRangeIter {
    pub(crate) iter:
        Flatten<std::iter::Chain<std::iter::Once<BruteRange>, std::vec::IntoIter<BruteRange>>>,
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
        self.ranges.iter().map(BruteRange::len).sum()
    }
}

impl From<BruteRange> for MBruteRange {
    fn from(range: BruteRange) -> Self {
        Self::from_range(range)
    }
}
