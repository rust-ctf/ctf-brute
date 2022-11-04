use super::MBruteRange;
use super::MBruteRangeIter;

impl Iterator for MBruteRangeIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl MBruteRange {
    pub fn iter(&self) -> MBruteRangeIter {
        let iter = self.ranges.clone().into_iter().flatten();
        MBruteRangeIter { iter }
    }
}

impl IntoIterator for MBruteRange {
    type Item = char;

    type IntoIter = MBruteRangeIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::BruteRange;
    use nonempty::nonempty;

    #[test]
    fn test_iter_single() {
        let result: Vec<char> =
            MBruteRange::from_ranges(nonempty![BruteRange::from_range('a'..='c')])
                .into_iter()
                .collect();
        assert_eq!(result, vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_iter_multi() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('0'..='3'),
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('b'..='d')
        ])
        .into_iter()
        .collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', 'A', 'B', 'C', 'b', 'c', 'd']
        );
    }

    #[test]
    fn test_iter_multi_sort() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('b'..='d'),
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('0'..='3')
        ])
        .into_iter()
        .collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', 'A', 'B', 'C', 'b', 'c', 'd']
        );
    }

    #[test]
    fn test_iter_single_reversed1() {
        let result: Vec<char> =
            MBruteRange::from_ranges(nonempty![BruteRange::from_range('d'..='a')])
                .into_iter()
                .collect();
        assert_eq!(result, vec!['a', 'b', 'c', 'd']);
    }

    #[test]
    fn test_iter_multi_reversed2() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('d'..='a'),
            BruteRange::from_range('D'..='A'),
            BruteRange::from_range('3'..='0')
        ])
        .into_iter()
        .collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', 'A', 'B', 'C', 'D', 'a', 'b', 'c', 'd']
        );
    }

    #[test]
    fn test_iter_one_reversed2() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('3'..='0'),
            BruteRange::from_range('A'..='D'),
            BruteRange::from_range('a'..='d')
        ])
        .into_iter()
        .collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', 'A', 'B', 'C', 'D', 'a', 'b', 'c', 'd']
        );
    }

    #[test]
    fn test_iter_one_reversed3() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('0'..='3'),
            BruteRange::from_range('D'..='A'),
            BruteRange::from_range('a'..='d')
        ])
        .into_iter()
        .collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', 'A', 'B', 'C', 'D', 'a', 'b', 'c', 'd']
        );
    }

    #[test]
    fn test_iter_one_reversed4() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('0'..='3'),
            BruteRange::from_range('A'..='D'),
            BruteRange::from_range('d'..='a')
        ])
        .into_iter()
        .collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', 'A', 'B', 'C', 'D', 'a', 'b', 'c', 'd']
        );
    }

    #[test]
    fn test_iter_single_overlap() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('a'..='d'),
            BruteRange::from_range('b'..='f')
        ])
        .into_iter()
        .collect();
        assert_eq!(result, vec!['a', 'b', 'c', 'd', 'e', 'f']);
    }

    #[test]
    fn test_iter_under_overlap() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('a'..='d'),
            BruteRange::from_range('c'..='f'),
            BruteRange::from_range('b'..='d')
        ])
        .into_iter()
        .collect();
        assert_eq!(result, vec!['a', 'b', 'c', 'd', 'e', 'f']);
    }

    #[test]
    fn test_iter_multi_overlap() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='D'),
            BruteRange::from_range('B'..='E'),
            BruteRange::from_range('a'..='d'),
            BruteRange::from_range('b'..='e')
        ])
        .into_iter()
        .collect();
        assert_eq!(
            result,
            vec!['A', 'B', 'C', 'D', 'E', 'a', 'b', 'c', 'd', 'e']
        );
    }

    #[test]
    fn test_iter_overlap_start() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('A'..='E')
        ])
        .into_iter()
        .collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D', 'E']);
    }

    #[test]
    fn test_iter_overlap_end() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('C'..='E')
        ])
        .into_iter()
        .collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D', 'E']);
    }

    #[test]
    fn test_iter_overlap_end2() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('C'..='E'),
            BruteRange::from_range('E'..='G')
        ])
        .into_iter()
        .collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D', 'E', 'F', 'G']);
    }

    #[test]
    fn test_iter_allongside() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('D'..='E')
        ])
        .into_iter()
        .collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D', 'E']);
    }

    #[test]
    fn test_iter_multi_allongside() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('D'..='E'),
            BruteRange::from_range('F'..='H')
        ])
        .into_iter()
        .collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']);
    }

    #[test]
    fn test_iter_not_allongside() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='B'),
            BruteRange::from_range('D'..='E')
        ])
        .into_iter()
        .collect();
        assert_eq!(result, vec!['A', 'B', 'D', 'E']);
    }

    #[test]
    fn test_iter_char() {
        let result: Vec<char> =
            MBruteRange::from_ranges(nonempty![BruteRange::from_range('1'..='1')])
                .into_iter()
                .collect();
        assert_eq!(result, vec!['1']);
    }

    #[test]
    fn test_iter_chars() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='A'),
            BruteRange::from_range('a'..='a')
        ])
        .into_iter()
        .collect();
        assert_eq!(result, vec!['A', 'a']);
    }

    #[test]
    fn test_iter_chars_allongside() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='A'),
            BruteRange::from_range('B'..='B')
        ])
        .into_iter()
        .collect();
        assert_eq!(result, vec!['A', 'B']);
    }

    #[test]
    fn test_iter_chars_overlap() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='A'),
            BruteRange::from_range('A'..='A')
        ])
        .into_iter()
        .collect();
        assert_eq!(result, vec!['A']);
    }

    #[test]
    fn test_iter_range_char_overlap() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='D'),
            BruteRange::from_range('D'..='D')
        ])
        .into_iter()
        .collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D']);
    }

    #[test]
    fn test_iter_range_char_allongside() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='D'),
            BruteRange::from_range('E'..='E')
        ])
        .into_iter()
        .collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D', 'E']);
    }

    #[test]
    fn test_iter_char_range_overlap() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='A'),
            BruteRange::from_range('A'..='D')
        ])
        .into_iter()
        .collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D']);
    }

    #[test]
    fn test_iter_char_range_allongside() {
        let result: Vec<char> = MBruteRange::from_ranges(nonempty![
            BruteRange::from_range('A'..='A'),
            BruteRange::from_range('B'..='E')
        ])
        .into_iter()
        .collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D', 'E']);
    }
}
