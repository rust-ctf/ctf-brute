
use std::ops::RangeInclusive;

pub mod iter;
mod parser;



use std::ops::{Range};

use itertools::Itertools;

use crate::ops::resetiter::ResetIter;

use super::bruterange::{BruteRange, BruteRangeIter};
use super::mbruterange::{MBruteRangeIter, MBruteRange};

#[derive(Clone, Debug)]
pub enum Pattern {
    //TODO: Send read only references
    Range(BruteRange),
    MRange(MBruteRange),
    Group(Vec<Pattern>),
    Length(Box<Pattern>, RangeInclusive<u32>, Vec<u128>),
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

    pub fn len(&self) -> Option<u128> {
        match &self {
            Self::Range(range) => Some(range.len() as u128),
            Self::MRange(range) => Some(range.len() as u128),
            Self::Group(patterns) => patterns
                .iter()
                .fold(Some(1u128), |b, x| x.len()?.checked_mul(b?)),
            //Self::Empty() => Some(1u128),
            Self::Length(pattern, range, _) => {
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

