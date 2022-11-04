use std::ops::RangeInclusive;

mod constant;
pub(crate) mod iter;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct BruteRange {
    pub(crate) start: char,
    pub(crate) end: char,
}

pub struct BruteRangeIter {
    pub(crate) end: u32,
    pub(crate) index: u32,
}

impl BruteRange {
    pub const fn new(start: char, end: char) -> BruteRange {
        let (mut start, mut end) = (start, end);
        if start > end {
            (start, end) = (end, start);
        }
        BruteRange { start, end }
    }

    pub const fn from_range(range: RangeInclusive<char>) -> BruteRange {
        BruteRange::new(*range.start(), *range.end())
    }

    pub fn from_range_u32(range: RangeInclusive<u32>) -> Option<BruteRange> {
        let start = char::from_u32(*range.start())?;
        let end = char::from_u32(*range.end())?;
        Some(BruteRange::new(start, end))
    }

    pub fn len(&self) -> usize {
        let start = self.start as u32;
        let end = self.end as u32;
        let mut count = end - start;
        if start < 0xD800 && 0xE000 < end {
            count -= 0x800
        }
        count += 1;
        count as usize
    }
}

impl From<RangeInclusive<char>> for BruteRange {
    fn from(range: RangeInclusive<char>) -> Self {
        BruteRange::from_range(range)
    }
}
