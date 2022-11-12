use std::ops::{Range, RangeInclusive};

use itertools::Itertools;

use crate::ops::resetiter::ResetIter;

use super::{Pattern, PatternIter};

// impl<'a> Iterator for &'a PatternIter<'_> {
//     type Item = &'a str;

//     fn next(&mut self) -> Option<Self::Item> {
//         if !self.has_next() {
//             return None;
//         }
//         Some(self.get_next())
//     }
// }

impl ResetIter for PatternIter<'_> {
    //TODO: Return string slice to avoid allocations
    type Item<'a> = &'a str where
        Self: 'a;

    fn has_next<'a>(&'a self) -> bool {
        match self {
            Self::Base(pattern, _, _) => pattern.has_next(),
            Self::Range(iter) => iter.has_next(),
            Self::MRange(iter) => iter.has_next(),
            Self::Group(iters) => iters[0].has_next(),
            Self::Length(iters, length, _) => {
                iters.len() != 0 && iters[0].has_next() || *length == 0
            }
        }
    }

    fn move_next<'a>(&'a mut self) {
        match self {
            Self::Base(pattern, buffer, init) => {
                *init = false;
                pattern.move_next();
            }
            Self::Range(iter) => iter.move_next(),
            Self::MRange(iter) => iter.move_next(),
            Self::Group(iters) => {
                for i in (0..iters.len()).rev() {
                    debug_assert!(iters.get(i).is_some());
                    let iter = &mut iters[i];
                    debug_assert!(iter.has_next());
                    iter.move_next();
                    if iter.has_next() {
                        break;
                    }
                    if i != 0 {
                        iter.reset()
                    }
                }
            }
            Self::Length(iters, length, _) => {
                if *length == 0 {
                    *length = 1;
                    return;
                }
                debug_assert!(*length <= iters.len());
                let start = iters.len() - *length;
                for i in (start..iters.len()).rev() {
                    debug_assert!(iters.get(i).is_some());
                    let iter = &mut iters[i];
                    debug_assert!(iter.has_next());
                    iter.move_next();
                    if iter.has_next() {
                        break;
                    }
                    if i != start {
                        iter.reset()
                    }
                }

                debug_assert!(iters.get(start).is_some());

                if !iters[start].has_next() {
                    *length += 1;
                    if start != 0 {
                        iters[start].reset()
                    }
                }
            }
        }
    }

    fn get_next<'a>(&'a mut self) -> Self::Item<'a> {
        let value = self.peek();
        // unsafe{
        //     let  this = self as &mut Self;
        //     this.move_next()
        //  };
        value
    }

    fn peek<'a>(&'a mut self) -> Self::Item<'a> {
        match self {
            Self::Base(pattern, buffer, init) =>
            {
                if !*init
                {
                    buffer.clear();
                    pattern.peek_buffered(buffer);
                    *init = true;
                }
                
                buffer.as_str()
            }
           _ => panic!("Calling peek from non base pattern is not allowed")
        }
    }

    fn reset<'a>(&'a mut self) {
        match self {
            Self::Base(pattern, buffer , init) => {
                pattern.reset();
                *init = false;
                buffer.clear();
            }
            Self::Range(iter) => {
                iter.reset();
            }
            Self::MRange(iter) => {
                iter.reset();
            }
            Self::Group(iters) => {
                iters.iter_mut().for_each(|i| i.reset());
            }
            Self::Length(iters, length, start_length) => {
                iters.iter_mut().for_each(|i| i.reset());
                *length = *start_length;
            }
        }
    }
}

impl PatternIter<'_>
{
    fn peek_buffered<'a>(&'a mut self, buffer: &mut String)
    {
        match self {
            Self::Base(pattern, _, _) =>
            {
                pattern.peek_buffered(buffer);
            }
            Self::Range(iter) => {
                debug_assert!(iter.has_next());
                buffer.push(iter.peek())
            }
            Self::MRange(iter) => {
                debug_assert!(iter.has_next());
                buffer.push(iter.peek())
            }
            Self::Group(iters) => {
                for iter in iters.iter_mut() {
                    iter.peek_buffered(buffer);
                }
            }
            Self::Length(iters, length, _) => {
                debug_assert!(*length <= iters.len());
                let start = iters.len() - *length;
                for i in start..iters.len() {
                    debug_assert!(iters.get(i as usize).is_some());
                    let iter = &mut iters[i as usize];
                    debug_assert!(iter.has_next());
                    iter.peek_buffered(buffer);
                }
            }
        }
    }
}

impl Pattern {
    pub fn iter(&self) -> PatternIter {
        match self {
            Self::Range(range) => PatternIter::Range(range.iter()),
            Self::MRange(range) => PatternIter::MRange(range.iter()),
            Self::Group(patterns) => {
                PatternIter::Group(patterns.iter().map(|p| p.iter()).collect())
            }
            Self::Length(pattern, range) => {
                let max_len = *range.end();
                let start = *range.start();
                let iters = (0..max_len).into_iter().map(|_| pattern.iter()).collect();
                PatternIter::Length(iters, start as usize, start as usize)
            }
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
