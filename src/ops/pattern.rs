use std::ops::{RangeTo, RangeInclusive, Range};

use itertools::Itertools;

use super::{BruteRange, MBruteRange, bruterange::BruteRangeIter, mbruterange::MBruteRangeIter};

#[derive(Clone, Debug)]
pub enum Pattern
{
    Range(BruteRange),
    MRange(MBruteRange),
    Group(Vec<Pattern>)
}

#[derive(Clone, Debug)]
pub enum PatternIter
{
    Range(BruteRangeIter),
    MRange(MBruteRangeIter),
    Group(Vec<Pattern>, Vec<PatternIter>, Vec<Option<String>>)
}

impl Iterator for PatternIter {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Range(range) => Some(range.next()?.to_string()),
            Self::MRange(range) => Some(range.next()?.to_string()),
            Self::Group(patterns, iterators, last) =>
            {
                assert_eq!(patterns.len(), iterators.len());
                assert_eq!(iterators.len(), last.len());
                let range = Range{ start: 0, end: iterators.len()};
                let mut next = true;
                let mut result = String::new();
                for i in range.into_iter().rev()
                {
                    if last[i].is_none() || next
                    {
                        let mut next_val = iterators[i].next();
                        if i == 0 && next_val.is_none()
                        {
                            return None; //End
                        }
                        if next_val.is_none()
                        {
                            iterators[i] = patterns[i].iter();
                            next_val = iterators[i].next();
                            last[i] = next_val;
                        }
                        else
                        {
                            next = false;
                            last[i] = next_val
                        }
                    }
                    //Error one of iterators had 0 results (shouldnt be possible)
                    if last[i].is_none()
                    {
                        return None;
                    }
                    else
                    {
                        let res = last[i].as_ref().unwrap();
                        result.insert_str(0, res.as_str())
                    }
                }
                Some(result)
            }
            _ => None
        }
    }
}

impl Pattern {
    pub fn iter(&self) -> PatternIter {
        match &self {
            Self::Range(range) => PatternIter::Range(range.iter()),
            Self::MRange(range) => PatternIter::MRange(range.iter()),
            Self::Group(patterns) => PatternIter::Group(patterns.clone(), patterns.into_iter().map(|p|p.iter()).collect(),patterns.into_iter().map(|p|None).collect()),
            _ => todo!()
        }
    }
}

impl IntoIterator for Pattern {
    type Item = String;

    type IntoIter = PatternIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nonempty::{nonempty, NonEmpty};

    #[test]
    fn test_merge_ranges_single() {
        let pattern =  Pattern::Group(vec![Pattern::Range(BruteRange::from_range('a'..='b')), Pattern::Range(BruteRange::from_range('a'..='c'))]);
        let result:Vec<String> = pattern.iter().collect();
        assert_eq!(result, vec!["aa","ab","ac","ba","bb","bc"]);
    }
}