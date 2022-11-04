use std::{collections::VecDeque, ops::RangeInclusive};

use crate::ops::{BruteRange, MBruteRange};

use super::Pattern;

macro_rules! pattern {
    ( $var:expr ) => {
        parse_group_str($var).unwrap()
    };
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
        if l2 < l1 || l2 < 1 || l1 < 1 {
            return None;
        }
        let ret = Some(Pattern::Length(
            Box::new(p),
            RangeInclusive::new(l1 - 1, l2 - 1),
        ));
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
        let num = str.parse().unwrap();
        if num == 0 {
            return None;
        }
        Some(num)
    }
}

#[cfg(test)]
mod tests {
    use crate::ops::BruteRange;

    use super::*;
    use nonempty::{nonempty, NonEmpty};

    #[test]
    fn test_group_digit() {
        let pattern = pattern!(r"\d");
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(
            result,
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
        );
    }

    #[test]
    fn test_group_range() {
        let pattern = pattern!(r"A-C0");
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(result, vec!["A0", "B0", "C0"]);
    }

    #[test]
    fn test_char_lenght() {
        let pattern = pattern!(r"a{1,3}");
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(result, vec!["a", "aa", "aaa"]);
    }

    #[test]
    fn test_group_lenght() {
        let pattern = pattern!(r"(a){1,3}");
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(result, vec!["a", "aa", "aaa"]);
    }

    #[test]
    fn test_group_lenght2() {
        let pattern = pattern!(r"ctf\{(ab{1,2}){1,3}\}");
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(
            result,
            vec![
                "ctf{ab}",
                "ctf{abb}",
                "ctf{abab}",
                "ctf{ababb}",
                "ctf{abbab}",
                "ctf{abbabb}",
                "ctf{ababab}",
                "ctf{abababb}",
                "ctf{ababbab}",
                "ctf{ababbabb}",
                "ctf{abbabab}",
                "ctf{abbababb}",
                "ctf{abbabbab}",
                "ctf{abbabbabb}"
            ]
        );
    }

    #[test]
    fn test_group_digits() {
        let pattern = pattern!(r"\d\d");
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(
            result,
            vec![
                "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13",
                "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
                "28", "29", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41",
                "42", "43", "44", "45", "46", "47", "48", "49", "50", "51", "52", "53", "54", "55",
                "56", "57", "58", "59", "60", "61", "62", "63", "64", "65", "66", "67", "68", "69",
                "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "80", "81", "82", "83",
                "84", "85", "86", "87", "88", "89", "90", "91", "92", "93", "94", "95", "96", "97",
                "98", "99"
            ]
        );
    }
}
