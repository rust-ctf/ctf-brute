use super::{BruteRange, BruteRangeIter};

impl Iterator for BruteRangeIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.end {
            return None;
        }

        assert_ne!(char::from_u32(self.index), None);
        let chr = unsafe { char::from_u32_unchecked(self.index) };

        self.index = self.index.checked_add(1)?;
        if self.index == 0xD800 {
            self.index += 0xE000 - 0xD800;
        }

        return Some(chr);
    }
}

impl BruteRange {
    pub fn iter(&self) -> BruteRangeIter {
        BruteRangeIter {
            end: self.end as u32,
            index: self.start as u32,
        }
    }
}

impl IntoIterator for BruteRange {
    type Item = char;

    type IntoIter = BruteRangeIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

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
        assert_eq!(range.len(), result.len());
        assert_eq!(result, vec!['c', 'd', 'e', 'f', 'g']);
    }

    #[test]
    fn test_bruterange_iter_multi_reversed() {
        let range = BruteRange::from_range('G'..='C');
        let result: Vec<char> = range.iter().collect();
        assert_eq!(range.len(), result.len());
        assert_eq!(result, vec!['C', 'D', 'E', 'F', 'G']);
    }

    #[test]
    fn test_bruterange_iter_multi_invalid_chars_in_bound() {
        let range = BruteRange::from_range('\u{d7fe}'..='\u{e001}');
        let result: Vec<char> = range.iter().collect();
        assert_eq!(range.len(), result.len());
        assert_eq!(result, vec!['\u{d7fe}', '\u{d7ff}', '\u{e000}', '\u{e001}']);
    }

    #[test]
    fn test_bruterange_iter_bound_end() {
        let range = BruteRange::from_range('\u{10fffe}'..='\u{10ffff}');
        let result: Vec<char> = range.iter().collect();
        assert_eq!(range.len(), result.len());
        assert_eq!(result, vec!['\u{10fffe}', '\u{10ffff}']);
    }

    #[test]
    fn test_bruterange_iter_bound_all() {
        let range = BruteRange::from_range('\u{0}'..='\u{10ffff}');
        let result: Vec<char> = range.iter().collect();
        assert_eq!(range.len(), result.len());
        let valid_chars: Vec<char> = (0..=0x10ffff)
            .into_iter()
            .map(|x| char::from_u32(x))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        assert_eq!(result, valid_chars);
    }
}
