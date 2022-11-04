use std::ops::RangeInclusive;

mod constant;
pub mod iter;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct BruteRange {
    pub(crate) start: char,
    pub(crate) end: char,
}

#[derive(Clone, Debug)]
pub struct BruteRangeIter {
    pub(crate) end: u32,
    pub(crate) index: u32,
}

impl BruteRange {
    pub const fn new(start: char, end: char) -> Self {
        let (mut start, mut end) = (start, end);
        if start > end {
            (start, end) = (end, start);
        }
        Self { start, end }
    }

    pub const fn from_range(range: RangeInclusive<char>) -> Self {
        Self::new(*range.start(), *range.end())
    }

    pub fn from_range_u32(range: RangeInclusive<u32>) -> Option<Self> {
        let start = char::from_u32(*range.start())?;
        let end = char::from_u32(*range.end())?;
        Some(Self::new(start, end))
    }

    pub fn len(&self) -> usize {
        let start = self.start as u32;
        let end = self.end as u32;
        let mut count = end - start;
        if start < 0xD800 && 0xE000 < end {
            count -= 0x800;
        }
        count += 1;
        count as usize
    }
}

impl From<RangeInclusive<char>> for BruteRange {
    fn from(range: RangeInclusive<char>) -> Self {
        Self::from_range(range)
    }
}
