use std::ops::RangeInclusive;

pub mod iter;
mod parser;

use super::{bruterange::BruteRangeIter, mbruterange::MBruteRangeIter, BruteRange, MBruteRange};

#[derive(Clone, Debug)]
pub enum Pattern {
    //TODO: Send read only references
    Range(BruteRange),
    MRange(MBruteRange),
    Group(Vec<Pattern>),
    Length(Box<Pattern>, RangeInclusive<u32>),
}

#[derive(Clone, Debug)]
pub enum PatternIter<'a> {
    Range(BruteRangeIter<'a>),
    MRange(MBruteRangeIter<'a>),
    Group(Vec<PatternIter<'a>>),
    Length(Vec<PatternIter<'a>>, usize, usize),
}

impl Pattern {
    pub fn from_pattern(pattern: &str) -> Option<Pattern> {
        parser::parse_pattern(pattern)
    }

    pub fn len(&self) -> Option<u128> {
        match &self {
            Self::Range(range) => Some(range.len() as u128),
            Self::MRange(range) => Some(range.len() as u128),
            Self::Group(patterns) => patterns
                .iter()
                .fold(Some(1u128), |b, x| x.len()?.checked_mul(b?)),
            //Self::Empty() => Some(1u128),
            Self::Length(pattern, range) => {
                let mut range = range.clone();
                let pattern_len = pattern.len()?;
                let first = range.next()?;
                let mut sum = pattern_len.checked_pow(first)?;
                let mut pow = sum;
                for _ in range {
                    pow = pow.checked_mul(pattern_len as u128)?;
                    sum = sum.checked_add(pow)?;
                }
                Some(sum)
            }
        }
    }
}
