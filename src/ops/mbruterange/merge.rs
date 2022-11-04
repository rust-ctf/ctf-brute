use std::cmp::max;

use itertools::Itertools;
use nonempty::{nonempty, NonEmpty};

use crate::ops::BruteRange;

use super::MBruteRange;

impl MBruteRange {
    pub(crate) fn merge_and_order_ranges(ranges: NonEmpty<BruteRange>) -> NonEmpty<BruteRange> {
        let mut sorted = ranges.into_iter().sorted_by(|l, r| l.start.cmp(&r.start));

        let mut result: Vec<BruteRange> = vec![];
        let mut range = sorted.next().unwrap(); //first
        for next_range in sorted {
            let start_next: u32 = next_range.start as u32;
            let end: u32 = range.end as u32;
            let start_next = start_next.checked_sub(1).unwrap_or(start_next);

            // if ranges colide or are allognside (BruteRange::from_range('a'..='b'), BruteRange::from_range('c'..='d')) we will merge them
            if next_range.start >= range.start && start_next <= end {
                range = BruteRange::new(range.start, max(range.end, next_range.end));
                continue;
            }
            //otherwise we add old range to result and continue checking with new range
            result.push(range);
            range = next_range;
        }
        result.push(range);
        NonEmpty::from_vec(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_ranges_single() {
        let result: NonEmpty<BruteRange> =
            MBruteRange::merge_and_order_ranges(nonempty![BruteRange::from_range('a'..='z')]);
        assert_eq!(result, nonempty![BruteRange::from_range('a'..='z')]);
    }

    #[test]
    fn test_merge_ranges_multi() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('0'..='9'),
            BruteRange::from_range('A'..='Z'),
            BruteRange::from_range('a'..='z')
        ]);
        assert_eq!(
            result,
            nonempty![
                BruteRange::from_range('0'..='9'),
                BruteRange::from_range('A'..='Z'),
                BruteRange::from_range('a'..='z')
            ]
        );
    }

    #[test]
    fn test_merge_ranges_multi_sort() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('a'..='z'),
            BruteRange::from_range('A'..='Z'),
            BruteRange::from_range('0'..='9')
        ]);
        assert_eq!(
            result,
            nonempty![
                BruteRange::from_range('0'..='9'),
                BruteRange::from_range('A'..='Z'),
                BruteRange::from_range('a'..='z')
            ]
        );
    }

    #[test]
    fn test_merge_ranges_single_reversed1() {
        let result: NonEmpty<BruteRange> =
            MBruteRange::merge_and_order_ranges(nonempty![BruteRange::from_range('z'..='a')]);
        assert_eq!(result, nonempty![BruteRange::from_range('a'..='z')]);
    }

    #[test]
    fn test_merge_ranges_multi_reversed2() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('z'..='a'),
            BruteRange::from_range('Z'..='A'),
            BruteRange::from_range('9'..='0')
        ]);
        assert_eq!(
            result,
            nonempty![
                BruteRange::from_range('0'..='9'),
                BruteRange::from_range('A'..='Z'),
                BruteRange::from_range('a'..='z')
            ]
        );
    }

    #[test]
    fn test_merge_ranges_one_reversed2() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('9'..='0'),
            BruteRange::from_range('A'..='Z'),
            BruteRange::from_range('a'..='z')
        ]);
        assert_eq!(
            result,
            nonempty![
                BruteRange::from_range('0'..='9'),
                BruteRange::from_range('A'..='Z'),
                BruteRange::from_range('a'..='z')
            ]
        );
    }

    #[test]
    fn test_merge_ranges_one_reversed3() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('0'..='9'),
            BruteRange::from_range('Z'..='A'),
            BruteRange::from_range('a'..='z')
        ]);
        assert_eq!(
            result,
            nonempty![
                BruteRange::from_range('0'..='9'),
                BruteRange::from_range('A'..='Z'),
                BruteRange::from_range('a'..='z')
            ]
        );
    }

    #[test]
    fn test_merge_ranges_one_reversed4() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('0'..='9'),
            BruteRange::from_range('A'..='Z'),
            BruteRange::from_range('z'..='a')
        ]);
        assert_eq!(
            result,
            nonempty![
                BruteRange::from_range('0'..='9'),
                BruteRange::from_range('A'..='Z'),
                BruteRange::from_range('a'..='z')
            ]
        );
    }

    #[test]
    fn test_merge_ranges_single_overlap() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('A'..='a'),
            BruteRange::from_range('Z'..='z')
        ]);
        assert_eq!(result, nonempty![BruteRange::from_range('A'..='z')]);
    }

    #[test]
    fn test_merge_ranges_under_overlap() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('A'..='a'),
            BruteRange::from_range('Z'..='z'),
            BruteRange::from_range('B'..='b')
        ]);
        assert_eq!(result, nonempty![BruteRange::from_range('A'..='z')]);
    }

    #[test]
    fn test_merge_ranges_multi_overlap() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('A'..='D'),
            BruteRange::from_range('B'..='E'),
            BruteRange::from_range('a'..='d'),
            BruteRange::from_range('b'..='e')
        ]);
        assert_eq!(
            result,
            nonempty![
                BruteRange::from_range('A'..='E'),
                BruteRange::from_range('a'..='e')
            ]
        );
    }

    #[test]
    fn test_merge_ranges_overlap_start() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('A'..='E')
        ]);
        assert_eq!(result, nonempty![BruteRange::from_range('A'..='E')]);
    }

    #[test]
    fn test_merge_ranges_overlap_end() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('C'..='E')
        ]);
        assert_eq!(result, nonempty![BruteRange::from_range('A'..='E')]);
    }

    #[test]
    fn test_merge_ranges_overlap_end2() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('C'..='E'),
            BruteRange::from_range('E'..='G')
        ]);
        assert_eq!(result, nonempty![BruteRange::from_range('A'..='G')]);
    }

    #[test]
    fn test_merge_ranges_allongside() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('D'..='E')
        ]);
        assert_eq!(result, nonempty![BruteRange::from_range('A'..='E')]);
    }

    #[test]
    fn test_merge_ranges_multi_allongside() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('A'..='C'),
            BruteRange::from_range('D'..='E'),
            BruteRange::from_range('F'..='H')
        ]);
        assert_eq!(result, nonempty![BruteRange::from_range('A'..='H')]);
    }

    #[test]
    fn test_merge_ranges_not_allongside() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('A'..='B'),
            BruteRange::from_range('D'..='E')
        ]);
        assert_eq!(
            result,
            nonempty![
                BruteRange::from_range('A'..='B'),
                BruteRange::from_range('D'..='E')
            ]
        );
    }

    #[test]
    fn test_merge_ranges_char() {
        let result: NonEmpty<BruteRange> =
            MBruteRange::merge_and_order_ranges(nonempty![BruteRange::from_range('1'..='1')]);
        assert_eq!(result, nonempty![BruteRange::from_range('1'..='1')]);
    }

    #[test]
    fn test_merge_ranges_chars() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('A'..='A'),
            BruteRange::from_range('a'..='a')
        ]);
        assert_eq!(
            result,
            nonempty![
                BruteRange::from_range('A'..='A'),
                BruteRange::from_range('a'..='a')
            ]
        );
    }

    #[test]
    fn test_merge_ranges_chars_allongside() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('A'..='A'),
            BruteRange::from_range('B'..='B')
        ]);
        assert_eq!(result, nonempty![BruteRange::from_range('A'..='B')]);
    }

    #[test]
    fn test_merge_ranges_chars_overlap() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('A'..='A'),
            BruteRange::from_range('A'..='A')
        ]);
        assert_eq!(result, nonempty![BruteRange::from_range('A'..='A')]);
    }

    #[test]
    fn test_merge_ranges_range_char_overlap() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('A'..='D'),
            BruteRange::from_range('D'..='D')
        ]);
        assert_eq!(result, nonempty![BruteRange::from_range('A'..='D')]);
    }

    #[test]
    fn test_merge_ranges_range_char_allongside() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('A'..='D'),
            BruteRange::from_range('E'..='E')
        ]);
        assert_eq!(result, nonempty![BruteRange::from_range('A'..='E')]);
    }

    #[test]
    fn test_merge_ranges_char_range_overlap() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('A'..='A'),
            BruteRange::from_range('A'..='D')
        ]);
        assert_eq!(result, nonempty![BruteRange::from_range('A'..='D')]);
    }

    #[test]
    fn test_merge_ranges_char_range_allongside() {
        let result: NonEmpty<BruteRange> = MBruteRange::merge_and_order_ranges(nonempty![
            BruteRange::from_range('A'..='A'),
            BruteRange::from_range('B'..='E')
        ]);
        assert_eq!(result, nonempty![BruteRange::from_range('A'..='E')]);
    }
}
