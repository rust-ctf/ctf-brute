use std::ops::RangeInclusive;

pub mod iter;
mod parser;

use super::bruterange::{BruteRange, BruteRangeIter};
use super::mbruterange::{MBruteRange, MBruteRangeIter};

#[derive(Clone, Debug)]
pub enum Pattern {
    Range {
        range: BruteRange,
        size: u128,
    },
    MRange {
        range: MBruteRange,
        size: u128,
    },
    Group {
        patterns: Vec<Pattern>,
        size: Option<u128>,
        width: usize,
    },
    Length {
        pattern: Box<Pattern>,
        range: RangeInclusive<u32>,
        size: Option<u128>,
        indexes: Option<Vec<u128>>,
        max_width: usize,
    },
}

#[derive(Clone, Debug)]
pub enum PatternIter<'st, 'bf> {
    Base(Box<PatternIter<'st, 'bf>>, &'bf String, bool),
    Range(BruteRangeIter<'st>),
    MRange(MBruteRangeIter<'st>),
    Group(Vec<PatternIter<'st, 'bf>>),
    Length(Vec<PatternIter<'st, 'bf>>, usize, usize),
}

impl Pattern {
    pub fn from_pattern(pattern: &str) -> Option<Pattern> {
        parser::parse_pattern(pattern)
    }

    pub(crate) fn new_range(range: BruteRange) -> Self {
        let size = range.len() as u128;
        Self::Range {
            range,
            size,
        }
    }

    pub(crate) fn new_multi_range(range: MBruteRange) -> Self {
        let size = range.len() as u128;
        Self::MRange {
            range,
            size,
        }
    }

    pub(crate) fn new_group(patterns: Vec<Self>) -> Self {
        let size = patterns
            .iter()
            .fold(Some(1u128), |b, x| x.len()?.checked_mul(b?));
        let width = patterns.len();
        Self::Group { patterns, size, width }
    }

    pub(crate) fn new_length(pattern: Self, range: RangeInclusive<u32>) -> Self {
        let len = pattern.len();
        let mut size = None;
        let mut indexes = None;
        let max_width = pattern.max_width() * (*range.end() as usize);
        if let Some(len) = len
        {
            let mut indexes_vec = Vec::with_capacity((*range.end() - *range.start()) as usize);
            size = range.clone()
                .into_iter()
                .skip(1)
                .fold(len.checked_pow(*range.start()), |b, _|
                {
                    if let Some(b) = b
                    {
                        indexes_vec.push(b);
                        return u128::checked_mul(b, len)
                    }
                    b
                });
            if size.is_some()
            {
                indexes = Some(indexes_vec);
            }
        }
        let pattern = Box::new(pattern);
        Self::Length { pattern, range, size, indexes, max_width }
    }

    pub fn max_width(&self) -> usize
    {
        0
    }

    pub fn len(&self) -> Option<u128> {
        match self {
            Self::Range { range, size } => Some(*size),
            Self::MRange { range: _, size } => Some(*size),
            Self::Group {
                patterns,
                size,
                width,
            } => *size,
            Self::Length {
                pattern,
                range,
                size,
                indexes,
                max_width,
            } => *size,
            // Self::Group(patterns) => patterns
            //     .iter()
            //     .fold(Some(1u128), |b, x| x.len()?.checked_mul(b?)),
            // //Self::Empty() => Some(1u128),
            // Self::Length(pattern, range, _) => {
            //     let mut range = range.clone();
            //     let pattern_len = pattern.len()?;
            //     let first = range.next()?;
            //     let mut sum = pattern_len.checked_pow(first)?;
            //     let mut pow = sum;
            //     for _ in range {
            //         pow = pow.checked_mul(pattern_len as u128)?;
            //         sum = sum.checked_add(pow)?;
            //     }
            //     Some(sum)
            // }
        }
    }
}
