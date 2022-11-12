use crate::ops::bruterange::BruteRangeIter;
use crate::ops::resetiter::ResetIter;

use super::MBruteRange;
use super::MBruteRangeIter;

impl Iterator for MBruteRangeIter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next() {
            return None;
        }
        Some(self.get_next())
    }
}

impl MBruteRange {
    pub fn iter(&self) -> MBruteRangeIter {
        let iters: Vec<BruteRangeIter> = self.ranges.iter().map(|r| r.iter()).collect();
        MBruteRangeIter { index: 0, iters }
    }

    pub fn nth(&self, index: u32) -> Option<char> {
        let range_index = match self.indexes.binary_search(&index) {
            Ok(index) => index,
            //If the value is not found then Result::Err is returned,
            //containing the index where a matching element could be inserted
            //while maintaining sorted order.
            Err(index) => usize::checked_sub(index, 1)?,
        };
        let start = self.indexes.get(range_index)?;
        let range = self.ranges.get(range_index)?;
        let index = u32::checked_sub(index, *start)?;
        range.nth(index)
    }

    pub unsafe fn nth_unchecked(&self, index: u32) -> char {
        let range_index = match self.indexes.binary_search(&index) {
            Ok(index) => index,
            //If the value is not found then Result::Err is returned,
            //containing the index where a matching element could be inserted
            //while maintaining sorted order.
            Err(index) => index - 1,
        };
        let start = self.indexes[range_index];
        let range = self.ranges[range_index];
        let index = index - start;
        range.nth_unchecked(index)
    }
}

impl ResetIter for MBruteRangeIter<'_> {
    type Item<'a> = char where Self: 'a;

    fn has_next<'a>(&'a self) -> bool {
        self.index < self.iters.len()
    }

    fn move_next<'a>(&'a mut self) {
        debug_assert!(self.iters.get(self.index).is_some());
        let iter = &mut self.iters[self.index];
        iter.move_next();
        if !iter.has_next() {
            debug_assert_ne!(self.index.checked_add(1), None);
            self.index += 1;
        }
    }

    fn get_next<'a>(&'a mut self) -> Self::Item<'a> {
        let value = self.peek();
        self.move_next();
        value
    }

    fn peek<'a>(&'a self) -> Self::Item<'a> {
        debug_assert!(self.iters.get(self.index).is_some());
        let iter = &self.iters[self.index];
        debug_assert!(iter.has_next());
        iter.peek()
    }

    fn reset<'a>(&'a mut self) {
        self.index = 0;
        for iter in self.iters.iter_mut() {
            iter.reset();
        }
    }
}

impl<'a> IntoIterator for &'a MBruteRange {
    type Item = char;

    type IntoIter = MBruteRangeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::BruteRange;
    use nonempty::{nonempty, NonEmpty};

    #[test]
    fn test_multi_single() {
        let range = MBruteRange::from_ranges(nonempty![BruteRange::from_range('a'..='c')]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['a', 'b', 'c']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_multi() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('0'..='3'),
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('b'..='d')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', 'A', 'B', 'C', 'b', 'c', 'd']
        );
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_multi_sort() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('b'..='d'),
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('0'..='3')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', 'A', 'B', 'C', 'b', 'c', 'd']
        );
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_single_reversed1() {
        let range = MBruteRange::from_ranges(nonempty![BruteRange::from_range('d'..='a')]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['a', 'b', 'c', 'd']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_multi_reversed2() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('d'..='a'),
            BruteRange::from_range('D'..='A'),
            BruteRange::from_range('3'..='0')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', 'A', 'B', 'C', 'D', 'a', 'b', 'c', 'd']
        );
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_one_reversed2() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('3'..='0'),
            BruteRange::from_range('A'..='D'),
            BruteRange::from_range('a'..='d')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', 'A', 'B', 'C', 'D', 'a', 'b', 'c', 'd']
        );
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_one_reversed3() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('0'..='3'),
            BruteRange::from_range('D'..='A'),
            BruteRange::from_range('a'..='d')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', 'A', 'B', 'C', 'D', 'a', 'b', 'c', 'd']
        );
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_one_reversed4() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('0'..='3'),
            BruteRange::from_range('A'..='D'),
            BruteRange::from_range('d'..='a')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', 'A', 'B', 'C', 'D', 'a', 'b', 'c', 'd']
        );
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_single_overlap() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('a'..='d'),
            BruteRange::from_range('b'..='f')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['a', 'b', 'c', 'd', 'e', 'f']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_under_overlap() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('a'..='d'),
            BruteRange::from_range('c'..='f'),
            BruteRange::from_range('b'..='d')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['a', 'b', 'c', 'd', 'e', 'f']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_multi_overlap() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='D'),
            BruteRange::from_range('B'..='E'),
            BruteRange::from_range('a'..='d'),
            BruteRange::from_range('b'..='e')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(
            result,
            vec!['A', 'B', 'C', 'D', 'E', 'a', 'b', 'c', 'd', 'e']
        );
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_overlap_start() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('A'..='E')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D', 'E']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_overlap_end() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('C'..='E')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D', 'E']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_overlap_end2() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('C'..='E'),
            BruteRange::from_range('E'..='G')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D', 'E', 'F', 'G']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_alongside() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('D'..='E')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D', 'E']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_multi_alongside() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('D'..='E'),
            BruteRange::from_range('F'..='H')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_not_alongside() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='B'),
            BruteRange::from_range('D'..='E')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['A', 'B', 'D', 'E']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_char() {
        let range = MBruteRange::from_ranges(nonempty![BruteRange::from_range('1'..='1')]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['1']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_chars() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='A'),
            BruteRange::from_range('a'..='a')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['A', 'a']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_chars_alongside() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='A'),
            BruteRange::from_range('B'..='B')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['A', 'B']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_chars_overlap() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='A'),
            BruteRange::from_range('A'..='A')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['A']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_range_char_overlap() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='D'),
            BruteRange::from_range('D'..='D')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_range_char_alongside() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='D'),
            BruteRange::from_range('E'..='E')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D', 'E']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_char_range_overlap() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='A'),
            BruteRange::from_range('A'..='D')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_char_range_alongside() {
        let range = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='A'),
            BruteRange::from_range('B'..='E')
        ]);
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D', 'E']);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }

    #[test]
    fn test_multi_char_range_alongside_largest_case() {
        let chars: Vec<char> = ('\0'..='\u{10ffff}').step_by(2).collect();
        let ranges: Vec<BruteRange> = chars
            .clone()
            .into_iter()
            .map(BruteRange::from_char)
            .collect();
        let range = MBruteRange::from_ranges(NonEmpty::from_vec(ranges).unwrap());
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, chars);
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i])
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }
}
