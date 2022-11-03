use itertools::Itertools;
use std::{cmp::max, ops::RangeInclusive};

macro_rules! range {
    ( $var:expr ) => {
        gen_ranges($var)
    };
}

fn gen_ranges(pattern: &str) -> Vec<RangeInclusive<char>> {
    let ranges = pattern.chars().map(|c| RangeInclusive::new(c, c));
    merge_and_order_ranges(ranges)
}

const RANGE_LOWERCASE_LETTERS: RangeInclusive<char> = 'a'..='z';

const RANGE_UPPERCASE_LETTERS: RangeInclusive<char> = 'A'..='Z';

const RANGE_LETTERS: [RangeInclusive<char>; 2] = [RANGE_UPPERCASE_LETTERS, RANGE_LOWERCASE_LETTERS];

const RANGE_NUMBERS: RangeInclusive<char> = '0'..='9';

const RANGE_HEX_LOWERCASE: [RangeInclusive<char>; 2] = [RANGE_NUMBERS, 'a'..='f'];

const RANGE_HEX_UPPERCASE: [RangeInclusive<char>; 2] = [RANGE_NUMBERS, 'A'..='F'];

const RANGE_HEX: [RangeInclusive<char>; 3] = [RANGE_NUMBERS, 'A'..='F', 'a'..='f'];

const RANGE_ALPHANUM_LOWERCASE: [RangeInclusive<char>; 2] =
    [RANGE_NUMBERS, RANGE_LOWERCASE_LETTERS];

const RANGE_ALPHANUM_UPPERCASE: [RangeInclusive<char>; 2] =
    [RANGE_NUMBERS, RANGE_UPPERCASE_LETTERS];

const RANGE_ALPHANUM: [RangeInclusive<char>; 3] = [
    RANGE_NUMBERS,
    RANGE_UPPERCASE_LETTERS,
    RANGE_LOWERCASE_LETTERS,
];

const RANGE_PUNCT: [RangeInclusive<char>; 4] = ['!'..='/', ':'..='@', '['..='`', '{'..='~'];

fn ranges_flatten<I, T>(ranges: I) -> T
where
    I: IntoIterator<Item = RangeInclusive<char>>,
    T: FromIterator<char>,
{
    let merged: Vec<RangeInclusive<char>> = merge_and_order_ranges(ranges);
    merged.into_iter().flatten().collect()
}

fn merge_and_order_ranges<I, T>(ranges: I) -> T
where
    I: IntoIterator<Item = RangeInclusive<char>>,
    T: FromIterator<RangeInclusive<char>>,
{
    let mut sorted = ranges
        .into_iter()
        .filter(|r| !r.is_empty())
        .sorted_by(|r1, r2| r1.start().cmp(r2.start()));
    let mut result: Vec<RangeInclusive<char>> = vec![];
    if let Some(mut range) = sorted.next() {
        for next_range in sorted {
            let start_next: u32 = (*next_range.start()).into();
            let end: u32 = (*range.end()).into();
            let start_next = start_next.checked_sub(1).unwrap_or(start_next);

            // if ranges colide or are allognside ('a'..='b', 'c'..='d') we will merge them
            if next_range.start() >= range.start() && start_next <= end {
                range = RangeInclusive::new(*range.start(), max(*range.end(), *next_range.end()));
                continue;
            }
            //otherwise we add old range to result and continue checking with new range
            result.push(range);
            range = next_range;
        }
        result.push(range);
    }
    result.into_iter().collect()
}

pub fn range_size(range: &RangeInclusive<char>) -> Option<u128> {
    match range.size_hint().1?.try_into() {
        Ok(size) => Some(size),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_ranges_emtpy() {
        let result: Vec<RangeInclusive<char>> = merge_and_order_ranges([]);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_merge_ranges_single() {
        let result: Vec<RangeInclusive<char>> = merge_and_order_ranges(['a'..='z']);
        assert_eq!(result, vec!['a'..='z']);
    }

    #[test]
    fn test_merge_ranges_multi() {
        let result: Vec<RangeInclusive<char>> =
            merge_and_order_ranges(['0'..='9', 'A'..='Z', 'a'..='z']);
        assert_eq!(result, vec!['0'..='9', 'A'..='Z', 'a'..='z']);
    }

    #[test]
    fn test_merge_ranges_multi_sort() {
        let result: Vec<RangeInclusive<char>> =
            merge_and_order_ranges(['a'..='z', 'A'..='Z', '0'..='9']);
        assert_eq!(result, vec!['0'..='9', 'A'..='Z', 'a'..='z']);
    }

    #[test]
    fn test_merge_ranges_single_invalid() {
        let result: Vec<RangeInclusive<char>> = merge_and_order_ranges(['z'..='a']);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_merge_ranges_multi_invalid() {
        let result: Vec<RangeInclusive<char>> =
            merge_and_order_ranges(['z'..='a', 'Z'..='A', '9'..='0']);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_merge_ranges_one_invalid1() {
        let result: Vec<RangeInclusive<char>> =
            merge_and_order_ranges(['9'..='0', 'A'..='Z', 'a'..='z']);
        assert_eq!(result, vec!['A'..='Z', 'a'..='z']);
    }

    #[test]
    fn test_merge_ranges_one_invalid2() {
        let result: Vec<RangeInclusive<char>> =
            merge_and_order_ranges(['0'..='9', 'Z'..='A', 'a'..='z']);
        assert_eq!(result, vec!['0'..='9', 'a'..='z']);
    }

    #[test]
    fn test_merge_ranges_one_invalid3() {
        let result: Vec<RangeInclusive<char>> =
            merge_and_order_ranges(['0'..='9', 'A'..='Z', 'z'..='a']);
        assert_eq!(result, vec!['0'..='9', 'A'..='Z']);
    }

    #[test]
    fn test_merge_ranges_single_overlap() {
        let result: Vec<RangeInclusive<char>> = merge_and_order_ranges(['A'..='a', 'Z'..='z']);
        assert_eq!(result, vec!['A'..='z']);
    }

    #[test]
    fn test_merge_ranges_under_overlap() {
        let result: Vec<RangeInclusive<char>> =
            merge_and_order_ranges(['A'..='a', 'Z'..='z', 'B'..='b']);
        assert_eq!(result, vec!['A'..='z']);
    }

    #[test]
    fn test_merge_ranges_multi_overlap() {
        let result: Vec<RangeInclusive<char>> =
            merge_and_order_ranges(['A'..='D', 'B'..='E', 'a'..='d', 'b'..='e']);
        assert_eq!(result, vec!['A'..='E', 'a'..='e']);
    }

    #[test]
    fn test_merge_ranges_overlap_start() {
        let result: Vec<RangeInclusive<char>> = merge_and_order_ranges(['A'..='C', 'A'..='E']);
        assert_eq!(result, vec!['A'..='E']);
    }

    #[test]
    fn test_merge_ranges_overlap_end() {
        let result: Vec<RangeInclusive<char>> = merge_and_order_ranges(['A'..='C', 'C'..='E']);
        assert_eq!(result, vec!['A'..='E']);
    }

    #[test]
    fn test_merge_ranges_overlap_end2() {
        let result: Vec<RangeInclusive<char>> =
            merge_and_order_ranges(['A'..='C', 'C'..='E', 'E'..='G']);
        assert_eq!(result, vec!['A'..='G']);
    }

    #[test]
    fn test_merge_ranges_allongside() {
        let result: Vec<RangeInclusive<char>> = merge_and_order_ranges(['A'..='C', 'D'..='E']);
        assert_eq!(result, vec!['A'..='E']);
    }

    #[test]
    fn test_merge_ranges_multi_allongside() {
        let result: Vec<RangeInclusive<char>> =
            merge_and_order_ranges(['A'..='C', 'D'..='E', 'F'..='H']);
        assert_eq!(result, vec!['A'..='H']);
    }

    #[test]
    fn test_merge_ranges_not_allongside() {
        let result: Vec<RangeInclusive<char>> = merge_and_order_ranges(['A'..='B', 'D'..='E']);
        assert_eq!(result, vec!['A'..='B', 'D'..='E']);
    }

    #[test]
    fn test_merge_ranges_char() {
        let result: Vec<RangeInclusive<char>> = merge_and_order_ranges(['1'..='1']);
        assert_eq!(result, vec!['1'..='1']);
    }

    #[test]
    fn test_merge_ranges_chars() {
        let result: Vec<RangeInclusive<char>> = merge_and_order_ranges(['A'..='A', 'a'..='a']);
        assert_eq!(result, vec!['A'..='A', 'a'..='a']);
    }

    #[test]
    fn test_merge_ranges_chars_allongside() {
        let result: Vec<RangeInclusive<char>> = merge_and_order_ranges(['A'..='A', 'B'..='B']);
        assert_eq!(result, vec!['A'..='B']);
    }

    #[test]
    fn test_merge_ranges_chars_overlap() {
        let result: Vec<RangeInclusive<char>> = merge_and_order_ranges(['A'..='A', 'A'..='A']);
        assert_eq!(result, vec!['A'..='A']);
    }

    #[test]
    fn test_merge_ranges_range_char_overlap() {
        let result: Vec<RangeInclusive<char>> = merge_and_order_ranges(['A'..='D', 'D'..='D']);
        assert_eq!(result, vec!['A'..='D']);
    }

    #[test]
    fn test_merge_ranges_range_char_allongside() {
        let result: Vec<RangeInclusive<char>> = merge_and_order_ranges(['A'..='D', 'E'..='E']);
        assert_eq!(result, vec!['A'..='E']);
    }

    #[test]
    fn test_merge_ranges_char_range_overlap() {
        let result: Vec<RangeInclusive<char>> = merge_and_order_ranges(['A'..='A', 'A'..='D']);
        assert_eq!(result, vec!['A'..='D']);
    }

    #[test]
    fn test_merge_ranges_char_range_allongside() {
        let result: Vec<RangeInclusive<char>> = merge_and_order_ranges(['A'..='A', 'B'..='E']);
        assert_eq!(result, vec!['A'..='E']);
    }

    #[test]
    fn test_flatten_RANGE_LOWERCASE_LETTERS() {
        let result: Vec<char> = ranges_flatten([RANGE_LOWERCASE_LETTERS]);
        assert_eq!(
            result,
            vec![
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
            ]
        );
    }

    #[test]
    fn test_flatten_RANGE_UPPERCASE_LETTERS() {
        let result: Vec<char> = ranges_flatten([RANGE_UPPERCASE_LETTERS]);
        assert_eq!(
            result,
            vec![
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
            ]
        );
    }

    #[test]
    fn test_flatten_RANGE_LETTERS() {
        let result: Vec<char> = ranges_flatten(RANGE_LETTERS);
        assert_eq!(
            result,
            vec![
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
                'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                'w', 'x', 'y', 'z'
            ]
        );
    }

    #[test]
    fn test_flatten_RANGE_NUMBERS() {
        let result: Vec<char> = ranges_flatten([RANGE_NUMBERS]);
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        );
    }

    #[test]
    fn test_flatten_RANGE_HEX_LOWERCASE() {
        let result: Vec<char> = ranges_flatten(RANGE_HEX_LOWERCASE);
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f']
        );
    }

    #[test]
    fn test_flatten_RANGE_HEX_UPPERCASE() {
        let result: Vec<char> = ranges_flatten(RANGE_HEX_UPPERCASE);
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F']
        );
    }

    #[test]
    fn test_flatten_RANGE_HEX() {
        let result: Vec<char> = ranges_flatten(RANGE_HEX);
        assert_eq!(
            result,
            vec![
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
                'a', 'b', 'c', 'd', 'e', 'f'
            ]
        );
    }

    #[test]
    fn test_flatten_RANGE_ALPHANUM_LOWERCASE() {
        let result: Vec<char> = ranges_flatten(RANGE_ALPHANUM_LOWERCASE);
        assert_eq!(
            result,
            vec![
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
                'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                'w', 'x', 'y', 'z'
            ]
        );
    }

    #[test]
    fn test_flatten_RANGE_ALPHANUM_UPPERCASE() {
        let result: Vec<char> = ranges_flatten(RANGE_ALPHANUM_UPPERCASE);
        assert_eq!(
            result,
            vec![
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
                'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
                'W', 'X', 'Y', 'Z'
            ]
        );
    }

    #[test]
    fn test_flatten_RANGE_ALPHANUM() {
        let result: Vec<char> = ranges_flatten(RANGE_ALPHANUM);
        assert_eq!(
            result,
            vec![
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
                'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
                'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
                'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
            ]
        );
    }

    #[test]
    fn test_flatten_range_punct() {
        let result: Vec<char> = ranges_flatten(RANGE_PUNCT);
        assert_eq!(
            result,
            vec![
                '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':',
                ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'
            ]
        );
    }

    #[test]
    fn test_range_macro() {
        let result = range!("abcABCdef321980");
        assert_eq!(result, vec!['0'..='3', '8'..='9', 'A'..='C', 'a'..='f'])
    }
}
