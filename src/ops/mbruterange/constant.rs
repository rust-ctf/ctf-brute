use nonempty::NonEmpty;

use super::BruteRange;
use super::MBruteRange;

impl MBruteRange {
    pub fn numbers() -> MBruteRange {
        let pattern = BruteRange::RANGES_NUMBERS;
        MBruteRange::from_ranges(NonEmpty::from_slice(&pattern).unwrap())
    }

    pub fn punct() -> MBruteRange {
        let pattern = BruteRange::RANGES_PUNCT;
        MBruteRange::from_ranges(NonEmpty::from_slice(&pattern).unwrap())
    }

    pub fn letters_upper() -> MBruteRange {
        let pattern = BruteRange::RANGES_LETTERS_UPPERCASE;
        MBruteRange::from_ranges(NonEmpty::from_slice(&pattern).unwrap())
    }

    pub fn letters_lower() -> MBruteRange {
        let pattern = BruteRange::RANGES_LETTERS_LOWERCASE;
        MBruteRange::from_ranges(NonEmpty::from_slice(&pattern).unwrap())
    }

    pub fn letters() -> MBruteRange {
        let pattern = BruteRange::RANGES_LETTERS;
        MBruteRange::from_ranges(NonEmpty::from_slice(&pattern).unwrap())
    }

    pub fn hex_lower() -> MBruteRange {
        let pattern = BruteRange::RANGES_HEX_LOWERCASE;
        MBruteRange::from_ranges(NonEmpty::from_slice(&pattern).unwrap())
    }

    pub fn hex_upper() -> MBruteRange {
        let pattern = BruteRange::RANGES_HEX_UPPERCASE;
        MBruteRange::from_ranges(NonEmpty::from_slice(&pattern).unwrap())
    }

    pub fn hex() -> MBruteRange {
        let pattern = BruteRange::RANGES_HEX;
        MBruteRange::from_ranges(NonEmpty::from_slice(&pattern).unwrap())
    }

    pub fn alphanum_lower() -> MBruteRange {
        let pattern = BruteRange::RANGES_ALPHANUM_LOWERCASE;
        MBruteRange::from_ranges(NonEmpty::from_slice(&pattern).unwrap())
    }

    pub fn alphanum_upper() -> MBruteRange {
        let pattern = BruteRange::RANGES_ALPHANUM_UPPERCASE;
        MBruteRange::from_ranges(NonEmpty::from_slice(&pattern).unwrap())
    }

    pub fn alphanum() -> MBruteRange {
        let pattern = BruteRange::RANGES_ALPHANUM;
        MBruteRange::from_ranges(NonEmpty::from_slice(&pattern).unwrap())
    }

    pub fn ascii() -> MBruteRange {
        let pattern = BruteRange::RANGES_ASCII;
        MBruteRange::from_ranges(NonEmpty::from_slice(&pattern).unwrap())
    }

    pub fn unicode() -> MBruteRange {
        let pattern = BruteRange::RANGES_UNICODE;
        MBruteRange::from_ranges(NonEmpty::from_slice(&pattern).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_numbers() {
        let result: Vec<char> = MBruteRange::numbers().iter().collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        );
    }

    #[test]
    fn test_iter_letters_lower() {
        let result: Vec<char> = MBruteRange::letters_lower().iter().collect();
        assert_eq!(
            result,
            vec![
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
            ]
        );
    }

    #[test]
    fn test_iter_letters_upper() {
        let result: Vec<char> = MBruteRange::letters_upper().iter().collect();
        assert_eq!(
            result,
            vec![
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
            ]
        );
    }

    #[test]
    fn test_iter_punct() {
        let result: Vec<char> = MBruteRange::punct().iter().collect();
        assert_eq!(
            result,
            vec![
                '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':',
                ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'
            ]
        );
    }

    #[test]
    fn test_iter_letters() {
        let result: Vec<char> = MBruteRange::letters().iter().collect();
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
    fn test_iter_hex_lower() {
        let result: Vec<char> = MBruteRange::hex_lower().iter().collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f']
        );
    }
    #[test]
    fn test_iter_hex_upper() {
        let result: Vec<char> = MBruteRange::hex_upper().iter().collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F']
        );
    }

    #[test]
    fn test_iter_hex() {
        let result: Vec<char> = MBruteRange::hex().iter().collect();
        assert_eq!(
            result,
            vec![
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
                'a', 'b', 'c', 'd', 'e', 'f'
            ]
        );
    }

    #[test]
    fn test_iter_alphanum_lower() {
        let result: Vec<char> = MBruteRange::alphanum_lower().iter().collect();
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
    fn test_iter_alphanum_upper() {
        let result: Vec<char> = MBruteRange::alphanum_upper().iter().collect();
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
    fn test_iter_alphanum() {
        let result: Vec<char> = MBruteRange::alphanum().iter().collect();
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
    fn test_iter_RANGE_ASCII() {
        let result: Vec<char> = MBruteRange::ascii().iter().collect();
        let expected: Vec<char> = (0..=0xff)
            .into_iter()
            .map(|x| char::from_u32(x))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        assert_eq!(result.len(), expected.len());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iter_RANGE_UNICODE() {
        let result: Vec<char> = MBruteRange::unicode().iter().collect();
        let expected: Vec<char> = (0..=0x10ffff)
            .into_iter()
            .map(|x| char::from_u32(x))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        assert_eq!(result.len(), expected.len());
        assert_eq!(result, expected);
    }
}
