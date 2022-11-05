use std::{collections::VecDeque, ops::RangeInclusive};

use crate::ops::{BruteRange, MBruteRange};

use super::Pattern;

fn parse_pattern(pattern: &str) -> Option<Pattern> {
    parse_group_str(pattern)
}

fn parse_group_str(pattern: &str) -> Option<Pattern> {
    let mut queue: VecDeque<char> = pattern.chars().collect();
    parse_group(&mut queue, None)
}

fn parse_group(queue: &mut VecDeque<char>, end: Option<char>) -> Option<Pattern> {
    let mut patterns = vec![];
    loop {
        if queue.len() == 0 {
            break;
        }
        let chr = queue.pop_front()?;
        let next_chr = queue.pop_front();
        let mut pattern = match (chr, next_chr) {
            ('\\', Some(c)) => match c {
                '\\' | '[' | ']' | '{' | '}' | '(' | ')' | '-' => {
                    Pattern::Range(BruteRange::from_char(c))
                }
                'w' => Pattern::Range(BruteRange::RANGE_LETTERS_LOWERCASE),
                'W' => Pattern::Range(BruteRange::RANGE_LETTERS_UPPERCASE),
                'd' => Pattern::Range(BruteRange::RANGE_NUMBERS),
                'u' => Pattern::Range(BruteRange::RANGE_UNICODE),
                'a' => Pattern::Range(BruteRange::RANGE_ASCII),
                'l' => Pattern::MRange(MBruteRange::letters()),
                'h' => Pattern::MRange(MBruteRange::hex_lower()),
                'H' => Pattern::MRange(MBruteRange::hex_upper()),
                'x' => Pattern::MRange(MBruteRange::hex()),
                'p' => Pattern::MRange(MBruteRange::punct()),
                'n' => Pattern::MRange(MBruteRange::alphanum_lower()),
                'N' => Pattern::MRange(MBruteRange::alphanum_upper()),
                'm' => Pattern::MRange(MBruteRange::alphanum()),
                'b' => Pattern::MRange(MBruteRange::brute()),
                _ => return None,
            },
            ('(', Some(c)) => {
                queue.push_front(c);
                parse_group(queue, Some(')'))?
            }
            ('(', _) => return None,
            (_, Some('-')) => {
                let r = queue.pop_front()?;
                Pattern::Range(BruteRange::new(chr, r))
            }
            (_, Some(c)) => {
                queue.push_front(c);
                if end.is_some() && chr == end.unwrap() {
                    break;
                }
                Pattern::Range(BruteRange::from_char(chr))
            }
            _ => {
                if end.is_some() && chr == end.unwrap() {
                    break;
                }
                Pattern::Range(BruteRange::from_char(chr))
            }
        };
        patterns.push(parse_pattern_length(queue, pattern)?);
    }

    parse_pattern_length(queue, Pattern::Group(patterns))
}

fn parse_pattern_length(queue: &mut VecDeque<char>, p: Pattern) -> Option<Pattern> {
    let chr = queue.pop_front();
    if let Some('{') = chr {
        let l1 = parse_digit_length(queue)?;
        if let Some(',') = queue.pop_front() {
        } else {
            return None;
        }
        let l2 = parse_digit_length(queue)?;
        if l2 < l1 {
            return None;
        }
        let ret = Some(Pattern::Length(Box::new(p), RangeInclusive::new(l1, l2)));
        if let Some('}') = queue.pop_front() {
            return ret;
        } else {
            return None;
        }
    } else if chr.is_some() {
        queue.push_front(chr.unwrap())
    }
    Some(p)
}

fn parse_digit_length(queue: &mut VecDeque<char>) -> Option<u32> {
    let mut str = String::new();
    loop {
        let chr = queue.pop_front();
        if let Some('0'..='9') = chr {
            str.push(chr.unwrap())
        } else if let Some(c) = chr {
            queue.push_front(c);
            break;
        } else {
            break;
        }
    }
    if str.is_empty() {
        None
    } else {
        Some(str.parse().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::ops::BruteRange;

    use super::*;
    use nonempty::{nonempty, NonEmpty};

    #[test]
    fn test_no_pattern() {
        let pattern = parse_pattern(r"abcDEF123").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["abcDEF123"]);
    }

    #[test]
    fn test_no_pattern_group() {
        let pattern = parse_pattern(r"(abcDEF123)").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["abcDEF123"]);
    }

    #[test]
    fn test_no_pattern_groups() {
        let pattern = parse_pattern(r"(a)(b)(c)(D)(E)(F)(1)(2)(3)").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["abcDEF123"]);
    }

    #[test]
    fn test_no_pattern_nested_groups() {
        let pattern = parse_pattern(r"(a(b)c(D(E)F)1(23))").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["abcDEF123"]);
    }

    #[test]
    fn test_pattern_repeat() {
        let pattern = parse_pattern(r"a{1,4}").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["a", "aa", "aaa", "aaaa"]);
    }

    #[test]
    fn test_pattern_repeat_zero_start() {
        let pattern = parse_pattern(r"c{0,4}").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["", "c", "cc", "ccc", "cccc"]);
    }

    #[test]
    fn test_pattern_repeat_group() {
        let pattern = parse_pattern(r"a{1,4}").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec![r"a", "aa", "aaa", "aaaa"]);
    }

    #[test]
    fn test_pattern_repeat_single_lenght() {
        let pattern = parse_pattern(r"a{1,4}").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec![r"a", "aa", "aaa", "aaaa"]);
    }

    #[test]
    fn test_escape_unsupported() {
        let pattern = parse_pattern(r"\\\[\]\{\}\(\)\-").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec![r"\[]{}()-"]);
    }

    #[test]
    fn test_escape_special_letter() {
        let pattern = parse_pattern(r"\\\[\]\{\}\(\)\-").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec![r"\[]{}()-"]);
    }

    #[test]
    fn test_escape_digit() {
        let pattern = parse_pattern(r"\d").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
        );
    }

    #[test]
    fn test_escape_letter() {
        let pattern = parse_pattern(r"\l").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec![
                "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P",
                "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f",
                "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v",
                "w", "x", "y", "z"
            ]
        );
    }

    #[test]
    fn test_escape_letter_lowercase() {
        let pattern = parse_pattern(r"\w").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec![
                "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
                "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"
            ]
        );
    }

    #[test]
    fn test_escape_letter_uppercase() {
        let pattern = parse_pattern(r"\W").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec![
                "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P",
                "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"
            ]
        );
    }

    #[test]
    fn test_escape_hex() {
        let pattern = parse_pattern(r"\x").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec![
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
                "a", "b", "c", "d", "e", "f"
            ]
        );
    }

    #[test]
    fn test_escape_hex_lowercase() {
        let pattern = parse_pattern(r"\h").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"]
        );
    }

    #[test]
    fn test_escape_hex_uppercase() {
        let pattern = parse_pattern(r"\H").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"]
        );
    }

    #[test]
    fn test_escape_punct() {
        let pattern = parse_pattern(r"\p").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec![
                "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/", ":",
                ";", "<", "=", ">", "?", "@", "[", "\\", "]", "^", "_", "`", "{", "|", "}", "~"
            ]
        );
    }

    #[test]
    fn test_escape_alphanum() {
        let pattern = parse_pattern(r"\m").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec![
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
                "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V",
                "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l",
                "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"
            ]
        );
    }

    #[test]
    fn test_escape_alphanum_lowercase() {
        let pattern = parse_pattern(r"\n").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec![
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f",
                "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v",
                "w", "x", "y", "z"
            ]
        );
    }

    #[test]
    fn test_escape_alphanum_uppercase() {
        let pattern = parse_pattern(r"\N").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec![
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
                "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V",
                "W", "X", "Y", "Z"
            ]
        );
    }

    #[test]
    fn test_escape_brute() {
        let pattern = parse_pattern(r"\b").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec![
                "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/", "0",
                "1", "2", "3", "4", "5", "6", "7", "8", "9", ":", ";", "<", "=", ">", "?", "@",
                "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P",
                "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "[", "\\", "]", "^", "_", "`",
                "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
                "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "{", "|", "}", "~"
            ]
        );
    }

    #[test]
    fn test_escape_ascii() {
        let pattern = parse_pattern(r"\a").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        let expected: Vec<String> = (0..=0xff)
            .into_iter()
            .map(|x| char::from_u32(x))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap().to_string())
            .collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_escape_unicode() {
        let pattern = parse_pattern(r"\u").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        let expected: Vec<String> = (0..=0x10ffff)
            .into_iter()
            .map(|x| char::from_u32(x))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap().to_string())
            .collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_escape_invalid() {
        let allowed_chars = HashSet::from([
            '\\', '[', ']', '{', '}', '(', ')', '-', 'w', 'W', 'd', 'u', 'a', 'l', 'h', 'H', 'x',
            'p', 'n', 'N', 'm', 'b',
        ]);

        let not_allowed_escapes: Vec<char> = (0..=0x10ffff)
            .into_iter()
            .map(|x| char::from_u32(x))
            .filter(|x| x.is_some() && !allowed_chars.contains(&x.unwrap()))
            .map(|x| x.unwrap())
            .collect();
        for escape in not_allowed_escapes {
            let pattern = parse_pattern(format!("\\{escape}").as_str());
            assert!(pattern.is_none());
        }
    }
}
