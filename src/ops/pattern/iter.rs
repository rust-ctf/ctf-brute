use std::ops::{Range, RangeInclusive};

use super::{Pattern, PatternIter};

impl Iterator for PatternIter<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Range(range) => Some(range.next()?.to_string()),
            Self::MRange(range) => Some(range.next()?.to_string()),
            Self::Empty(b) => {
                if *b {
                    None
                } else {
                    *b = true;
                    Some(String::new())
                }
            }
            Self::Group(patterns, iterators, last) => {
                todo!()
                // assert_eq!(patterns.len(), iterators.len());
                // assert_eq!(iterators.len(), last.len());
                // let range = Range {
                //     start: 0,
                //     end: iterators.len(),
                // };
                // let mut next = true;
                // let mut result = String::new();
                // for i in range.into_iter().rev() {
                //     if last[i].is_none() || next {
                //         let mut next_val = iterators[i].next();
                //         if i == 0 && next_val.is_none() {
                //             return None; //End
                //         }
                //         if next_val.is_none() {
                //             iterators[i] = patterns[i].iter();
                //             next_val = iterators[i].next();
                //             last[i] = next_val;
                //         } else {
                //             next = false;
                //             last[i] = next_val;
                //         }
                //     }
                //     //Error one of iterators had 0 results (shouldnt be possible)
                //     if last[i].is_none() {
                //         return None;
                //     }
                //     let res = last[i].as_ref().unwrap();
                //     result.insert_str(0, res.as_str());
                //}
                //Some(result)
            }
            Self::Length(iteratrs) => loop {
                if iteratrs.is_empty() {
                    return None;
                }
                let next = iteratrs[0].next();
                if next.is_none() {
                    iteratrs.remove(0);
                    continue;
                }
                return next;
            },
        }
    }
}

impl Pattern {
    pub fn iter(&self) -> PatternIter {
        match &self {
            Self::Range(range) => PatternIter::Range(range.iter()),
            Self::MRange(range) => PatternIter::MRange(range.iter()),
            Self::Group(patterns) => PatternIter::Group(
                patterns.clone(),
                patterns.iter().map(Pattern::iter).collect(),
                patterns.iter().map(|_| None).collect(),
            ),
            Self::Length(pattern, range) => {
                // let patterns: Vec<PatternIter> = range
                //     .clone()
                //     .into_iter()
                //     .map(|i| {
                //         if i == 0 {
                //             return Self::Empty().iter();
                //         }
                //         let patterns: Vec<Self> = RangeInclusive::new(1, i)
                //             .map(|_| *pattern.clone())
                //             .collect();
                //         Self::Group(patterns).iter()
                //     })
                //     .collect();
                // PatternIter::Length(patterns)
                todo!()
            }
            Self::Empty() => PatternIter::Empty(false),
        }
    }
}

// impl<'a> IntoIterator for &'a Pattern {
//     type Item = String;

//     type IntoIter = PatternIter<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }

#[cfg(test)]
mod tests {
    use crate::ops::BruteRange;

    use super::*;
    use nonempty::{nonempty, NonEmpty};

    #[test]
    fn test_group() {
        let pattern = Pattern::Group(vec![
            Pattern::Range(BruteRange::from_range('a'..='b')),
            Pattern::Range(BruteRange::from_range('a'..='c')),
        ]);
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(result, vec!["aa", "ab", "ac", "ba", "bb", "bc"]);
    }

    #[test]
    fn test_lenght() {
        let pattern = Pattern::Length(
            Box::new(Pattern::Range(BruteRange::from_range('a'..='c'))),
            1..=2,
        );
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(
            result,
            vec!["a", "b", "c", "aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc"]
        );
    }

    #[test]
    fn test_lenght2() {
        let pattern = Pattern::Length(
            Box::new(Pattern::Range(BruteRange::from_range('a'..='c'))),
            0..=3,
        );
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(
            result,
            vec![
                "", "a", "b", "c", "aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc", "aaa",
                "aab", "aac", "aba", "abb", "abc", "aca", "acb", "acc", "baa", "bab", "bac", "bba",
                "bbb", "bbc", "bca", "bcb", "bcc", "caa", "cab", "cac", "cba", "cbb", "cbc", "cca",
                "ccb", "ccc"
            ]
        );
    }
}
