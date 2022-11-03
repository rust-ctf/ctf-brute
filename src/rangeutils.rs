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

fn range_lowercase_letters() -> RangeInclusive<char> {
    'a'..='z'
}

fn range_uppercase_letters() -> RangeInclusive<char> {
    'A'..='Z'
}

fn range_letters() -> Vec<RangeInclusive<char>> {
    vec![range_uppercase_letters(), range_lowercase_letters()]
}

fn merge_and_order_ranges_vec(ranges: Vec<RangeInclusive<char>>) -> Vec<RangeInclusive<char>> {
    merge_and_order_ranges(ranges.into_iter())
}

fn range_numbers() -> RangeInclusive<char> {
    '0'..='9'
}

fn range_hex_lowercase() -> Vec<RangeInclusive<char>> {
    vec![range_numbers(), 'a'..='f']
}

fn range_hex_uppercase() -> Vec<RangeInclusive<char>> {
    vec![range_numbers(), 'A'..='F']
}

fn range_hex() -> Vec<RangeInclusive<char>> {
    vec![range_numbers(), 'A'..='F', 'a'..='f']
}

fn range_alphanum_lowercase() -> Vec<RangeInclusive<char>> {
    vec![range_numbers(), range_lowercase_letters()]
}

fn range_alphanum_uppercase() -> Vec<RangeInclusive<char>> {
    vec![range_numbers(), range_uppercase_letters()]
}

fn range_alphanum() -> Vec<RangeInclusive<char>> {
    vec![
        range_numbers(),
        range_uppercase_letters(),
        range_lowercase_letters(),
    ]
}

fn range_punct() -> Vec<RangeInclusive<char>> {
    range!("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~")
}

fn range_flatten_vec(ranges: Vec<RangeInclusive<char>>) -> impl Iterator<Item = char> {
    range_flatten(ranges.into_iter())
}

fn range_flatten<I>(ranges: I) -> impl Iterator<Item = char>
where
    I: Iterator<Item = RangeInclusive<char>>,
{
    let sorted = merge_and_order_ranges(ranges);
    let res = sorted.into_iter().flatten();
    res.into_iter()
}

fn merge_and_order_ranges<I>(ranges: I) -> Vec<RangeInclusive<char>>
where
    I: Iterator<Item = RangeInclusive<char>>,
{
    let mut sorted = ranges
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
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_ranges_emtpy() {
        let result = merge_and_order_ranges_vec(vec![]);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_merge_ranges_single() {
        let result = merge_and_order_ranges_vec(vec!['a'..='z']);
        assert_eq!(result, vec!['a'..='z']);
    }

    #[test]
    fn test_merge_ranges_multi() {
        let result = merge_and_order_ranges_vec(vec!['0'..='9', 'A'..='Z', 'a'..='z']);
        assert_eq!(result, vec!['0'..='9', 'A'..='Z', 'a'..='z']);
    }

    #[test]
    fn test_merge_ranges_multi_sort() {
        let result = merge_and_order_ranges_vec(vec!['a'..='z', 'A'..='Z', '0'..='9']);
        assert_eq!(result, vec!['0'..='9', 'A'..='Z', 'a'..='z',]);
    }

    #[test]
    fn test_merge_ranges_single_invalid() {
        let result = merge_and_order_ranges_vec(vec!['z'..='a']);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_merge_ranges_multi_invalid() {
        let result = merge_and_order_ranges_vec(vec!['z'..='a', 'Z'..='A', '9'..='0']);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_merge_ranges_one_invalid1() {
        let result = merge_and_order_ranges_vec(vec!['9'..='0', 'A'..='Z', 'a'..='z']);
        assert_eq!(result, vec!['A'..='Z', 'a'..='z']);
    }

    #[test]
    fn test_merge_ranges_one_invalid2() {
        let result = merge_and_order_ranges_vec(vec!['0'..='9', 'Z'..='A', 'a'..='z']);
        assert_eq!(result, vec!['0'..='9', 'a'..='z']);
    }

    #[test]
    fn test_merge_ranges_one_invalid3() {
        let result = merge_and_order_ranges_vec(vec!['0'..='9', 'A'..='Z', 'z'..='a']);
        assert_eq!(result, vec!['0'..='9', 'A'..='Z']);
    }

    #[test]
    fn test_merge_ranges_single_overlap() {
        let result = merge_and_order_ranges_vec(vec!['A'..='a', 'Z'..='z']);
        assert_eq!(result, vec!['A'..='z']);
    }

    #[test]
    fn test_merge_ranges_under_overlap() {
        let result = merge_and_order_ranges_vec(vec!['A'..='a', 'Z'..='z', 'B'..='b']);
        assert_eq!(result, vec!['A'..='z']);
    }

    #[test]
    fn test_merge_ranges_multi_overlap() {
        let result = merge_and_order_ranges_vec(vec!['A'..='D', 'B'..='E', 'a'..='d', 'b'..='e']);
        assert_eq!(result, vec!['A'..='E', 'a'..='e']);
    }

    #[test]
    fn test_merge_ranges_overlap_start() {
        let result = merge_and_order_ranges_vec(vec!['A'..='C', 'A'..='E']);
        assert_eq!(result, vec!['A'..='E']);
    }

    #[test]
    fn test_merge_ranges_overlap_end() {
        let result = merge_and_order_ranges_vec(vec!['A'..='C', 'C'..='E']);
        assert_eq!(result, vec!['A'..='E']);
    }

    #[test]
    fn test_merge_ranges_overlap_end2() {
        let result = merge_and_order_ranges_vec(vec!['A'..='C', 'C'..='E', 'E'..='G']);
        assert_eq!(result, vec!['A'..='G']);
    }

    #[test]
    fn test_merge_ranges_allongside() {
        let result = merge_and_order_ranges_vec(vec!['A'..='C', 'D'..='E']);
        assert_eq!(result, vec!['A'..='E']);
    }

    #[test]
    fn test_merge_ranges_multi_allongside() {
        let result = merge_and_order_ranges_vec(vec!['A'..='C', 'D'..='E', 'F'..='H']);
        assert_eq!(result, vec!['A'..='H']);
    }

    #[test]
    fn test_merge_ranges_not_allongside() {
        let result = merge_and_order_ranges_vec(vec!['A'..='B', 'D'..='E']);
        assert_eq!(result, vec!['A'..='B', 'D'..='E']);
    }

    #[test]
    fn test_merge_ranges_char() {
        let result = merge_and_order_ranges_vec(vec!['1'..='1']);
        assert_eq!(result, vec!['1'..='1']);
    }

    #[test]
    fn test_merge_ranges_chars() {
        let result = merge_and_order_ranges_vec(vec!['A'..='A', 'a'..='a']);
        assert_eq!(result, vec!['A'..='A', 'a'..='a']);
    }

    #[test]
    fn test_merge_ranges_chars_allongside() {
        let result = merge_and_order_ranges_vec(vec!['A'..='A', 'B'..='B']);
        assert_eq!(result, vec!['A'..='B']);
    }

    #[test]
    fn test_merge_ranges_chars_overlap() {
        let result = merge_and_order_ranges_vec(vec!['A'..='A', 'A'..='A']);
        assert_eq!(result, vec!['A'..='A']);
    }

    #[test]
    fn test_merge_ranges_range_char_overlap() {
        let result = merge_and_order_ranges_vec(vec!['A'..='D', 'D'..='D']);
        assert_eq!(result, vec!['A'..='D']);
    }

    #[test]
    fn test_merge_ranges_range_char_allongside() {
        let result = merge_and_order_ranges_vec(vec!['A'..='D', 'E'..='E']);
        assert_eq!(result, vec!['A'..='E']);
    }

    #[test]
    fn test_merge_ranges_char_range_overlap() {
        let result = merge_and_order_ranges_vec(vec!['A'..='A', 'A'..='D']);
        assert_eq!(result, vec!['A'..='D']);
    }

    #[test]
    fn test_merge_ranges_char_range_allongside() {
        let result = merge_and_order_ranges_vec(vec!['A'..='A', 'B'..='E']);
        assert_eq!(result, vec!['A'..='E']);
    }

    #[test]
    fn test_flatten_range_lowercase_letters() {
        let result = range_flatten_vec(vec![range_lowercase_letters()]).collect_vec();
        assert_eq!(
            result,
            vec![
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
            ]
        );
    }

    #[test]
    fn test_flatten_range_uppercase_letters() {
        let result = range_flatten_vec(vec![range_uppercase_letters()]).collect_vec();
        assert_eq!(
            result,
            vec![
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
            ]
        );
    }

    #[test]
    fn test_flatten_range_letters() {
        let result = range_flatten_vec(range_letters()).collect_vec();
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
    fn test_flatten_range_numbers() {
        let result = range_flatten_vec(vec![range_numbers()]).collect_vec();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        );
    }

    #[test]
    fn test_flatten_range_hex_lowercase() {
        let result = range_flatten_vec(range_hex_lowercase()).collect_vec();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f']
        );
    }

    #[test]
    fn test_flatten_range_hex_uppercase() {
        let result = range_flatten_vec(range_hex_uppercase()).collect_vec();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F']
        );
    }

    #[test]
    fn test_flatten_range_hex() {
        let result = range_flatten_vec(range_hex()).collect_vec();
        assert_eq!(
            result,
            vec![
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
                'a', 'b', 'c', 'd', 'e', 'f'
            ]
        );
    }

    #[test]
    fn test_flatten_range_alphanum_lowercase() {
        let result = range_flatten_vec(range_alphanum_lowercase()).collect_vec();
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
    fn test_flatten_range_alphanum_uppercase() {
        let result = range_flatten_vec(range_alphanum_uppercase()).collect_vec();
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
    fn test_flatten_range_alphanum() {
        let result = range_flatten_vec(range_alphanum()).collect_vec();
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
        let result = range_flatten_vec(range_punct()).collect_vec();
        assert_eq!(
            result,
            vec![
                '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':',
                ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'
            ]
        );
    }

    fn test_range_macro() {
        let result = range!("abcABCdef321980");
        assert_eq!(result, vec!['0'..='3', '8'..='9', 'A'..='C', 'a'..='f'])
    }
}
