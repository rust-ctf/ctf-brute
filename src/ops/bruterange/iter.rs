use crate::ops::resetiter::ResetIter;

use super::{BruteRange, BruteRangeIter};

impl Iterator for BruteRangeIter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next() {
            return None;
        }
        Some(self.get_next())
    }
}

impl ResetIter for BruteRangeIter<'_> {
    type Item<'a> = char where Self: 'a;

    fn has_next<'a>(&'a self) -> bool {
        self.index <= self.end && self.index <= 0x10FFFFFF
    }

    fn move_next<'a>(&'a mut self) {
        debug_assert_ne!(self.index.checked_add(1), None);
        self.index += 1;
        if self.index == 0xD800 {
            self.index += 0xE000 - 0xD800;
        }
    }

    fn get_next<'a>(&'a mut self) -> Self::Item<'a> {
        let value = self.peek();
        self.move_next();
        value
    }

    fn peek<'a>(&'a self) -> Self::Item<'a> {
        debug_assert_ne!(char::from_u32(self.index), None);
        unsafe { char::from_u32_unchecked(self.index) }
    }

    fn reset<'a>(&'a mut self) {
        self.index = self.range.start as u32;
    }
}

impl BruteRange {
    pub const fn iter(&self) -> BruteRangeIter {
        BruteRangeIter {
            range: &self,
            end: self.end as u32,
            index: self.start as u32,
        }
    }

    pub fn nth(&self, index: u32) -> Option<char> {
        let mut index = u32::checked_add(self.start as u32, index)?;
        if self.start <= '\u{d7ff}' && index > 0xd7ff {
            index = u32::checked_add(index, 0xE000 - 0xD800)?;
        }
        if index > (self.end as u32) {
            return None;
        }
        char::from_u32(index)
    }

    pub unsafe fn nth_unchecked(&self, index: u32) -> char {
        debug_assert!(u32::checked_add(self.start as u32, index).is_some());
        let mut index = (self.start as u32) + index;
        if self.start <= '\u{d7ff}' && index > 0xd7ff {
            debug_assert!(u32::checked_add(index, 0xE000 - 0xD800).is_some());
            index += 0xE000 - 0xD800;
        }
        debug_assert!(index <= (self.end as u32));
        debug_assert_ne!(char::from_u32(index), None);
        char::from_u32_unchecked(index)
    }
}

// impl <'a> IntoIterator for &'a BruteRange {
//     type Item = char;

//     type IntoIter = BruteRangeIter<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bruterange_iter_single() {
        let range = BruteRange::from_range('0'..='0');
        let result: Vec<char> = range.iter().collect();
        assert_eq!(result, vec!['0']);
    }

    #[test]
    fn test_bruterange_iter_multi() {
        let range = BruteRange::from_range('c'..='g');
        let result: Vec<char> = range.iter().collect();
        assert_eq!(range.len() as usize, result.len());
        assert_eq!(result, vec!['c', 'd', 'e', 'f', 'g']);
    }

    #[test]
    fn test_bruterange_iter_multi_reversed() {
        let range = BruteRange::from_range('G'..='C');
        let result: Vec<char> = range.iter().collect();
        assert_eq!(range.len() as usize, result.len());
        assert_eq!(result, vec!['C', 'D', 'E', 'F', 'G']);
    }

    #[test]
    fn test_bruterange_iter_multi_invalid_chars_in_bound() {
        let range = BruteRange::from_range('\u{d7fe}'..='\u{e001}');
        let result: Vec<char> = range.iter().collect();
        assert_eq!(range.len() as usize, result.len());
        assert_eq!(result, vec!['\u{d7fe}', '\u{d7ff}', '\u{e000}', '\u{e001}']);
    }

    #[test]
    fn test_bruterange_iter_bound_end() {
        let range = BruteRange::from_range('\u{10fffe}'..='\u{10ffff}');
        let result: Vec<char> = range.iter().collect();
        assert_eq!(range.len() as usize, result.len());
        assert_eq!(result, vec!['\u{10fffe}', '\u{10ffff}']);
    }

    #[test]
    fn test_bruterange_iter_bound_all() {
        let range = BruteRange::from_range('\u{0}'..='\u{10ffff}');
        let result: Vec<char> = range.iter().collect();
        assert_eq!(range.len() as usize, result.len());
        let valid_chars: Vec<char> = (0..=0x10ffff)
            .into_iter()
            .map(|x| char::from_u32(x))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        assert_eq!(result, valid_chars);
    }

    #[test]
    fn test_nth_single() {
        let range = BruteRange::from_range('0'..='0');
        assert_eq!(range.nth(0), Some('0'));
        assert_eq!(range.nth(1), None);
        assert_eq!(unsafe { range.nth_unchecked(0) }, '0');
    }

    #[test]
    fn test_nth_multi() {
        let range = BruteRange::from_range('a'..='d');
        assert_eq!(range.nth(0), Some('a'));
        assert_eq!(range.nth(1), Some('b'));
        assert_eq!(range.nth(2), Some('c'));
        assert_eq!(range.nth(3), Some('d'));
        assert_eq!(range.nth(4), None);
        assert_eq!(unsafe { range.nth_unchecked(0) }, 'a');
        assert_eq!(unsafe { range.nth_unchecked(1) }, 'b');
        assert_eq!(unsafe { range.nth_unchecked(2) }, 'c');
        assert_eq!(unsafe { range.nth_unchecked(3) }, 'd');
    }

    #[test]
    fn test_nth_multi_reversed() {
        let range = BruteRange::from_range('d'..='a');
        assert_eq!(range.nth(0), Some('a'));
        assert_eq!(range.nth(1), Some('b'));
        assert_eq!(range.nth(2), Some('c'));
        assert_eq!(range.nth(3), Some('d'));
        assert_eq!(range.nth(4), None);
        assert_eq!(unsafe { range.nth_unchecked(0) }, 'a');
        assert_eq!(unsafe { range.nth_unchecked(1) }, 'b');
        assert_eq!(unsafe { range.nth_unchecked(2) }, 'c');
        assert_eq!(unsafe { range.nth_unchecked(3) }, 'd');
    }

    #[test]
    fn test_nth_invalid_chars_in_bound() {
        let range = BruteRange::from_range('\u{d7fe}'..='\u{e001}');
        assert_eq!(range.nth(0), Some('\u{d7fe}'));
        assert_eq!(range.nth(1), Some('\u{d7ff}'));
        assert_eq!(range.nth(2), Some('\u{e000}'));
        assert_eq!(range.nth(3), Some('\u{e001}'));
        assert_eq!(range.nth(4), None);
        assert_eq!(unsafe { range.nth_unchecked(0) }, '\u{d7fe}');
        assert_eq!(unsafe { range.nth_unchecked(1) }, '\u{d7ff}');
        assert_eq!(unsafe { range.nth_unchecked(2) }, '\u{e000}');
        assert_eq!(unsafe { range.nth_unchecked(3) }, '\u{e001}');
    }
    #[test]
    fn test_nth_after_invalid_chars() {
        let range = BruteRange::from_range('\u{e000}'..='\u{e003}');
        assert_eq!(range.nth(0), Some('\u{e000}'));
        assert_eq!(range.nth(1), Some('\u{e001}'));
        assert_eq!(range.nth(2), Some('\u{e002}'));
        assert_eq!(range.nth(3), Some('\u{e003}'));
        assert_eq!(range.nth(4), None);
        assert_eq!(unsafe { range.nth_unchecked(0) }, '\u{e000}');
        assert_eq!(unsafe { range.nth_unchecked(1) }, '\u{e001}');
        assert_eq!(unsafe { range.nth_unchecked(2) }, '\u{e002}');
        assert_eq!(unsafe { range.nth_unchecked(3) }, '\u{e003}');
    }

    #[test]
    fn test_nth_after_invalid_chars2() {
        let range = BruteRange::from_range('\u{e001}'..='\u{e003}');
        assert_eq!(range.nth(0), Some('\u{e001}'));
        assert_eq!(range.nth(1), Some('\u{e002}'));
        assert_eq!(range.nth(2), Some('\u{e003}'));
        assert_eq!(range.nth(3), None);
        assert_eq!(unsafe { range.nth_unchecked(0) }, '\u{e001}');
        assert_eq!(unsafe { range.nth_unchecked(1) }, '\u{e002}');
        assert_eq!(unsafe { range.nth_unchecked(2) }, '\u{e003}');
    }

    #[test]
    fn test_nth_all() {
        let range = BruteRange::from_range('\u{0}'..='\u{10ffff}');
        let result: Vec<char> = range.iter().collect();
        for i in 0..result.len() {
            assert_eq!(range.nth(i as u32), Some(result[i]));
            assert_eq!(unsafe { range.nth_unchecked(i as u32) }, result[i]);
        }
        assert_eq!(range.nth(result.len() as u32), None);
    }
}
