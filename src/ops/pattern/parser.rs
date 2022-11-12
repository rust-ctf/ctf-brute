use std::{collections::VecDeque, ops::RangeInclusive};

use crate::ops::{BruteRange, MBruteRange};

use super::Pattern;

use logos::{Lexer, Logos};
use nonempty::NonEmpty;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LColon,

    #[token("]")]
    RColon,

    #[regex(r"\{[\d]+\}", lex_lenght_single)]
    #[regex(r"\{,[\d]+\}", lex_lenght_from_zero)]
    #[regex(r"\{[\d]+,[\d]+\}", lex_lenght)]
    Length((u32, u32)),

    //TODO: Implement other combinations for range
    #[regex(r"[^\\\[\]\{\}\(\)\-]\-[^\\\[\]\{\}\(\)\-]", lex_range)]
    Range((char, char)),

    #[regex(r"\\[wWdUalhHXpnNmb]", lex_escape_char)]
    Escape(char),

    #[regex(r"\\x[0-9A-Za-z][0-9A-Za-z]", lex_escape_ascii)]
    #[regex(r"\\u[0-9A-Za-z][0-9A-Za-z][0-9A-Za-z][0-9A-Za-z]", lex_escape_unicode)]
    #[regex(r"\\[\\\[\]\{\}\(\)\-]", lex_escape_char)]
    #[regex(r"[^\\\[\]\{\}\(\)\-]", lex_char)]
    Char(char),

    #[error]
    Error,
}

fn lex_escape_char(lex: &mut Lexer<Token>) -> Option<char> {
    let slice = lex.slice();
    if slice.len() != 2 {
        return None;
    }
    slice.chars().nth(1)
}

fn lex_char(lex: &mut Lexer<Token>) -> Option<char> {
    let slice = lex.slice();
    if slice.len() != 1 {
        return None;
    }
    slice.chars().nth(0)
}

fn lex_range(lex: &mut Lexer<Token>) -> Option<(char, char)> {
    let slice = lex.slice();
    if slice.len() != 3 {
        return None;
    }
    Some((slice.chars().nth(0)?, slice.chars().nth(2)?))
}

fn lex_escape_ascii(lex: &mut Lexer<Token>) -> Option<char> {
    let slice = lex.slice();
    if slice.len() != 4 {
        return None;
    }
    let byte = u8::from_str_radix(&slice[2..=3], 16);
    Some(byte.ok()? as char)
}

fn lex_escape_unicode(lex: &mut Lexer<Token>) -> Option<char> {
    let slice = lex.slice();
    if slice.len() != 6 {
        return None;
    }
    let value = u16::from_str_radix(&slice[2..=5], 16);
    char::from_u32(value.ok()? as u32)
}

fn lex_lenght_single(lex: &mut Lexer<Token>) -> Option<(u32, u32)> {
    let slice = lex.slice();
    let n: u32 = slice[1..slice.len() - 1].parse().ok()?; // skip '{}'
    Some((n, n))
}

fn lex_lenght_from_zero(lex: &mut Lexer<Token>) -> Option<(u32, u32)> {
    let slice = lex.slice();
    let n: u32 = slice[2..slice.len() - 1].parse().ok()?; // skip '{,}'
    Some((0, n))
}

fn lex_lenght(lex: &mut Lexer<Token>) -> Option<(u32, u32)> {
    let slice = lex.slice();
    let split: Vec<&str> = slice[1..slice.len() - 1].split(',').collect();
    if split.len() != 2 {
        return None;
    }
    let l: u32 = split.first()?.parse().ok()?;
    let r: u32 = split.get(1)?.parse().ok()?;
    if r < l {
        return None;
    }
    Some((l, r))
}

pub fn parse_pattern(pattern: &str) -> Option<Pattern> {
    let mut lex = Token::lexer(pattern);
    parse_group(&mut lex, &None)
}

fn parse_group(lex: &mut Lexer<Token>, end: &Option<Token>) -> Option<Pattern> {
    let mut patterns: VecDeque<Pattern> = VecDeque::new();
    loop {
        let pattern = match lex.next() {
            Some(Token::Char(c)) => Pattern::Range(BruteRange::from_char(c)),
            Some(Token::Range((l, r))) => Pattern::Range(BruteRange::new(l, r)),
            Some(Token::Escape(e)) => pattern_from_escape(e)?,
            Some(Token::LParen) => parse_group(lex, &Some(Token::RParen))?,
            Some(Token::LColon) => parse_range(lex, &Some(Token::RColon))?,
            Some(Token::Length((l, r))) => {
                let last = patterns.pop_back()?;
                let len = last.len()?;
                let mut indexes = Vec::with_capacity((r - l) as usize);
                _ = (l..=r).into_iter().fold(0u128, |s, _| {
                    indexes.push(s);
                    s + len
                });
                Pattern::Length(Box::new(last), RangeInclusive::new(l, r), indexes)
            }
            t => {
                if end.eq(&t) {
                    break;
                }
                return None;
            }
        };
        patterns.push_back(pattern);
    }
    let mut patterns = Vec::from(patterns);
    match patterns.len() {
        0 => None,
        1 => patterns.pop(),
        _ => Some(Pattern::Group(patterns)),
    }
}

fn parse_range(lex: &mut Lexer<Token>, end: &Option<Token>) -> Option<Pattern> {
    let mut patterns: VecDeque<BruteRange> = VecDeque::new();
    loop {
        match lex.next() {
            Some(Token::Char(c)) => patterns.push_back(BruteRange::from_char(c)),
            Some(Token::Range((l, r))) => patterns.push_back(BruteRange::new(l, r)),
            Some(Token::Escape(e)) => {
                for pattern in ranges_from_escape(e)? {
                    patterns.push_back(*pattern);
                }
            }
            t => {
                if end.eq(&t) {
                    break;
                }
                return None;
            }
        };
    }
    let mut patterns = Vec::from(patterns);
    match patterns.len() {
        0 => None,
        1 => Some(Pattern::Range(patterns.pop()?)),
        _ => Some(Pattern::MRange(MBruteRange::from_ranges(
            NonEmpty::from_vec(patterns)?,
        ))),
    }
}

fn pattern_from_escape(c: char) -> Option<Pattern> {
    match c {
        'w' => Some(Pattern::Range(BruteRange::RANGE_LETTERS_LOWERCASE)),
        'W' => Some(Pattern::Range(BruteRange::RANGE_LETTERS_UPPERCASE)),
        'd' => Some(Pattern::Range(BruteRange::RANGE_NUMBERS)),
        'U' => Some(Pattern::Range(BruteRange::RANGE_UNICODE)),
        'a' => Some(Pattern::Range(BruteRange::RANGE_ASCII)),
        'l' => Some(Pattern::MRange(MBruteRange::letters())),
        'h' => Some(Pattern::MRange(MBruteRange::hex_lower())),
        'H' => Some(Pattern::MRange(MBruteRange::hex_upper())),
        'X' => Some(Pattern::MRange(MBruteRange::hex())),
        'p' => Some(Pattern::MRange(MBruteRange::punct())),
        'n' => Some(Pattern::MRange(MBruteRange::alphanum_lower())),
        'N' => Some(Pattern::MRange(MBruteRange::alphanum_upper())),
        'm' => Some(Pattern::MRange(MBruteRange::alphanum())),
        'b' => Some(Pattern::MRange(MBruteRange::brute())),
        _ => None,
    }
}

const fn ranges_from_escape(c: char) -> Option<&'static [BruteRange]> {
    match c {
        'w' => Some(&BruteRange::RANGES_LETTERS_LOWERCASE),
        'W' => Some(&BruteRange::RANGES_LETTERS_UPPERCASE),
        'd' => Some(&BruteRange::RANGES_NUMBERS),
        'U' => Some(&BruteRange::RANGES_UNICODE),
        'a' => Some(&BruteRange::RANGES_ASCII),
        'l' => Some(&BruteRange::RANGES_LETTERS),
        'h' => Some(&BruteRange::RANGES_HEX_LOWERCASE),
        'H' => Some(&BruteRange::RANGES_HEX_UPPERCASE),
        'X' => Some(&BruteRange::RANGES_HEX),
        'p' => Some(&BruteRange::RANGES_PUNCT),
        'n' => Some(&BruteRange::RANGES_ALPHANUM_LOWERCASE),
        'N' => Some(&BruteRange::RANGES_ALPHANUM_UPPERCASE),
        'm' => Some(&BruteRange::RANGES_ALPHANUM),
        'b' => Some(&BruteRange::RANGES_BRUTE),
        _ => None,
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
        let pattern = parse_pattern(r"(a){1,4}").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["a", "aa", "aaa", "aaaa"]);
    }

    #[test]
    fn test_pattern_repeat_group2() {
        let pattern = parse_pattern(r"(ab){0,3}").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["", "ab", "abab", "ababab"]);
    }

    #[test]
    fn test_pattern_repeat_from_zero() {
        let pattern = parse_pattern(r"(ab){,3}").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["", "ab", "abab", "ababab"]);
    }

    #[test]
    fn test_pattern_repeat_same_lenght() {
        let pattern = parse_pattern(r"c{1,1}").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["c"]);
    }

    #[test]
    fn test_pattern_repeat_same_lenght_zero() {
        let pattern = parse_pattern(r"c{0,0}").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec![""]);
    }

    #[test]
    fn test_pattern_repeat_same_lenght2() {
        let pattern = parse_pattern(r"A{2}").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["AA"]);
    }

    #[test]
    fn test_pattern_repeat_same_lenght_zero2() {
        let pattern = parse_pattern(r"c{0}").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec![""]);
    }

    #[test]
    fn test_pattern_repeat_same_lenght_zero_from_zero() {
        let pattern = parse_pattern(r"c{,0}").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec![""]);
    }

    #[test]
    fn test_pattern_repeat_nested() {
        let pattern = parse_pattern(r"(cb{1,2}){1,2}").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec!["cb", "cbb", "cbcb", "cbcbb", "cbbcb", "cbbcbb"]
        );
    }

    #[test]
    fn test_pattern_repeat_nested_no_group() {
        let pattern = parse_pattern(r"cb{1,2}{1,2}").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["cb", "cbb", "cbb", "cbbb", "cbbb", "cbbbb"]);
    }
    #[test]
    fn test_pattern_range() {
        let pattern = parse_pattern(r"a-c").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_pattern_range_reversed() {
        let pattern = parse_pattern(r"9-3").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["3", "4", "5", "6", "7", "8", "9"]);
    }

    #[test]
    fn test_pattern_range_group() {
        let pattern = parse_pattern(r"(a-c)").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_pattern_range_inbetween_letters() {
        let pattern = parse_pattern(r"Xa-cX").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["XaX", "XbX", "XcX"]);
    }

    #[test]
    fn test_pattern_ranges() {
        let pattern = parse_pattern(r"a-cA-C").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec!["aA", "aB", "aC", "bA", "bB", "bC", "cA", "cB", "cC"]
        );
    }

    #[test]
    fn test_pattern_ranges_group1() {
        let pattern = parse_pattern(r"(a-c)(A-C)").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec!["aA", "aB", "aC", "bA", "bB", "bC", "cA", "cB", "cC"]
        );
    }

    #[test]
    fn test_pattern_ranges_group2() {
        let pattern = parse_pattern(r"X(a-c)X(A-C)X").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec!["XaXAX", "XaXBX", "XaXCX", "XbXAX", "XbXBX", "XbXCX", "XcXAX", "XcXBX", "XcXCX"]
        );
    }

    #[test]
    fn test_pattern_ranges_nostart() {
        let pattern = parse_pattern(r"-C");
        assert!(pattern.is_none());
    }

    #[test]
    fn test_pattern_ranges_noend() {
        let pattern = parse_pattern(r"A-");
        assert!(pattern.is_none());
    }

    #[test]
    fn test_pattern_ranges_nostartend() {
        let pattern = parse_pattern(r"-");
        assert!(pattern.is_none());
    }

    #[test]
    fn test_pattern_ranges_nostart_group() {
        let pattern = parse_pattern(r"(-C)");
        assert!(pattern.is_none());
    }

    #[test]
    fn test_pattern_ranges_noend_group() {
        let pattern = parse_pattern(r"(A-)");
        assert!(pattern.is_none());
    }

    #[test]
    fn test_pattern_ranges_nostartend_group() {
        let pattern = parse_pattern(r"(-)");
        assert!(pattern.is_none());
    }

    #[test]
    fn test_pattern_multirange_empty() {
        let pattern = parse_pattern(r"[]");
        assert!(pattern.is_none());
    }

    #[test]
    fn test_pattern_multirange_letter() {
        let pattern = parse_pattern(r"[a]").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["a"]);
    }

    #[test]
    fn test_pattern_multirange_letters() {
        let pattern = parse_pattern(r"[abc]").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_pattern_multirange_letters_unordered() {
        let pattern = parse_pattern(r"[cAb]").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["A", "b", "c"]);
    }

    #[test]
    fn test_pattern_multirange_range() {
        let pattern = parse_pattern(r"[A-C]").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["A", "B", "C"]);
    }

    #[test]
    fn test_pattern_multirange_ranges() {
        let pattern = parse_pattern(r"[A-Cb-c]").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["A", "B", "C", "b", "c"]);
    }

    #[test]
    fn test_pattern_multirange_ranges_overlap() {
        let pattern = parse_pattern(r"[A-CB-D]").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["A", "B", "C", "D"]);
    }

    #[test]
    fn test_pattern_multirange_ranges_unordered() {
        let pattern = parse_pattern(r"[C-AD-B]").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["A", "B", "C", "D"]);
    }

    #[test]
    fn test_pattern_multirange_ranges_letters() {
        let pattern = parse_pattern(r"[AB-CDE-GH]").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["A", "B", "C", "D", "E", "F", "G", "H"]);
    }

    #[test]
    fn test_pattern_multirange_inside_group() {
        let pattern = parse_pattern(r"[(a)]");
        assert!(pattern.is_none())
    }

    #[test]
    fn test_pattern_multirange_inside_lenght() {
        let pattern = parse_pattern(r"[a{1,2}]");
        assert!(pattern.is_none())
    }

    #[test]
    fn test_pattern_multirange_inside_multirange() {
        let pattern = parse_pattern(r"[[a]]");
        assert!(pattern.is_none())
    }

    #[test]
    fn test_pattern_multirange_group() {
        let pattern = parse_pattern(r"([Ab-cDe-gH])").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["A", "D", "H", "b", "c", "e", "f", "g"]);
    }

    #[test]
    fn test_pattern_multirange_group_length() {
        let pattern = parse_pattern(r"([ab]){1,2}").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["a", "b", "aa", "ab", "ba", "bb"]);
    }

    #[test]
    fn test_pattern_multirange_length() {
        let pattern = parse_pattern(r"[0-1]{0,2}").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["", "0", "1", "00", "01", "10", "11"]);
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
        let pattern = parse_pattern(r"\X").unwrap();
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
    fn test_escape_ascii_format() {
        let pattern = parse_pattern(r"\x41\x73\x43\x69\x69").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["AsCii"]);
    }

    #[test]
    fn test_escape_unicode_format() {
        let pattern = parse_pattern(r"\u0055\u004E\u0031\u0043\u006F\u0064\u0065").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["UN1Code"]);
    }

    #[test]
    fn test_escape_invalid() {
        let allowed_chars = HashSet::from([
            '\\', '[', ']', '{', '}', '(', ')', '-', 'w', 'W', 'd', 'u', 'U', 'a', 'l', 'h', 'H',
            'x', 'X', 'p', 'n', 'N', 'm', 'b',
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

    ////////////////////////////////////
    ///
    ///

    #[test]
    fn test_escape_letter_in_range() {
        let pattern = parse_pattern(r"[\l]").unwrap();
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
    fn test_escape_letter_lowercase_in_range() {
        let pattern = parse_pattern(r"[\w]").unwrap();
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
    fn test_escape_letter_uppercase_in_range() {
        let pattern = parse_pattern(r"[\W]").unwrap();
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
    fn test_escape_hex_in_range() {
        let pattern = parse_pattern(r"[\X]").unwrap();
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
    fn test_escape_hex_lowercase_in_range() {
        let pattern = parse_pattern(r"[\h]").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"]
        );
    }

    #[test]
    fn test_escape_hex_uppercase_in_range() {
        let pattern = parse_pattern(r"[\H]").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(
            result,
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"]
        );
    }

    #[test]
    fn test_escape_punct_in_range() {
        let pattern = parse_pattern(r"[\p]").unwrap();
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
    fn test_escape_alphanum_in_range() {
        let pattern = parse_pattern(r"[\m]").unwrap();
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
    fn test_escape_alphanum_lowercase_in_range() {
        let pattern = parse_pattern(r"[\n]").unwrap();
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
    fn test_escape_alphanum_uppercase_in_range() {
        let pattern = parse_pattern(r"[\N]").unwrap();
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
    fn test_escape_brute_in_range() {
        let pattern = parse_pattern(r"[\b]").unwrap();
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
    fn test_escape_ascii_in_range() {
        let pattern = parse_pattern(r"[\a]").unwrap();
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
    fn test_escape_ascii_format_in_range() {
        let pattern = parse_pattern(r"[\x41\x73\x43\x69\x69]").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["A", "C", "i", "s"]);
    }

    #[test]
    fn test_escape_unicode_format_in_range() {
        let pattern = parse_pattern(r"[\u0055\u004E\u0031\u0043\u006F\u0064\u0065]").unwrap();
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(pattern.len().unwrap(), result.len() as u128);
        assert_eq!(result, vec!["1", "C", "N", "U", "d", "e", "o"]);
    }

    #[test]
    fn test_escape_invalid_in_range() {
        let allowed_chars = HashSet::from([
            '\\', '[', ']', '{', '}', '(', ')', '-', 'w', 'W', 'd', 'u', 'U', 'a', 'l', 'h', 'H',
            'x', 'X', 'p', 'n', 'N', 'm', 'b',
        ]);

        let not_allowed_escapes: Vec<char> = (0..=0x10ffff)
            .into_iter()
            .map(|x| char::from_u32(x))
            .filter(|x| x.is_some() && !allowed_chars.contains(&x.unwrap()))
            .map(|x| x.unwrap())
            .collect();
        for escape in not_allowed_escapes {
            let pattern = parse_pattern(format!("[\\{escape}]").as_str());
            assert!(pattern.is_none());
        }
    }
}
