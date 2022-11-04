use std::{
    iter::{Flatten, Map},
    ops::{Range, RangeInclusive, RangeTo},
    vec::IntoIter,
};

mod iter;

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
pub enum PatternIter {
    Range(BruteRangeIter),
    MRange(MBruteRangeIter),
    Group(Vec<Pattern>, Vec<PatternIter>, Vec<Option<String>>),
    Length(Vec<PatternIter>), //Flatten<Map<RangeInclusive<u32>, Fn>>)
}
