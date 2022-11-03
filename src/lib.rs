use std::{ops::RangeInclusive, result};

mod rangeutils;

pub struct BruteIterator {
    pattern: Pattern,
}

pub enum Pattern {
    Range(RangeInclusive<char>),
    Ranges(Vec<RangeInclusive<char>>),
    Repeater(Box<Pattern>, RangeInclusive<u32>),
    Patterns(Vec<Box<Pattern>>),
}


impl IntoIterator for BruteIterator
{
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl Pattern {
    fn size(&self) -> Option<u128> {
        match &self {
            Self::Range(range) => rangeutils::range_size(range),
            Self::Ranges(ranges) => ranges.iter().fold(Some(0u128), |buf, r| {
                buf?.checked_mul(rangeutils::range_size(r)?)
            }),
            Self::Repeater(pattern, range) => {
                let len = pattern.size()?;
                if range.is_empty() {
                    return Some(0);
                }
                let combo = len.checked_pow(*range.start())?;
                Some(
                    RangeInclusive::new(*range.start() + 1, *range.end())
                        .into_iter()
                        .fold(Some((combo, combo)), |buf, _| {
                            let (total_size, last_size) = buf?;
                            let size = last_size.checked_mul(len)?;
                            Some((total_size.checked_mul(size)?, size))
                        })?
                        .0,
                )
            }
            Self::Patterns(patterns) => patterns.iter().fold(Some(1u128), |buf, p| {
                let size = p.as_ref().size()?;
                if size == 0 {
                    buf
                } else {
                    buf?.checked_mul(size)
                }
            }),
        }
    }
}
