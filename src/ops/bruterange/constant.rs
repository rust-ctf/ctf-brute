use super::BruteRange;

impl BruteRange {
    pub const RANGE_NUMBERS: Self = Self::from_range('0'..='9');
    pub const RANGE_LETTERS_UPPERCASE: Self = Self::from_range('A'..='Z');
    pub const RANGE_LETTERS_LOWERCASE: Self = Self::from_range('a'..='z');

    pub const RANGE_ASCII: Self = Self::from_range('\u{0}'..='\u{FF}');
    pub const RANGE_UNICODE: Self = Self::from_range('\u{0}'..='\u{10FFFF}');

    const RANGE_HEX_LETTERS_UPPERCASE: Self = Self::from_range('A'..='F');
    const RANGE_HEX_LETTERS_LOWERCASE: Self = Self::from_range('a'..='f');

    pub const RANGES_NUMBERS: [Self; 1] = [Self::RANGE_NUMBERS];

    pub const RANGES_ASCII: [Self; 1] = [Self::RANGE_ASCII];
    pub const RANGES_UNICODE: [Self; 1] = [Self::RANGE_UNICODE];

    pub const RANGES_PUNCT: [Self; 4] = [
        Self::from_range('!'..='/'),
        Self::from_range(':'..='@'),
        Self::from_range('['..='`'),
        Self::from_range('{'..='~'),
    ];

    pub const RANGES_BRUTE: [Self; 7] = [
        Self::from_range('!'..='/'),
        Self::RANGE_NUMBERS,
        Self::from_range(':'..='@'),
        Self::RANGE_LETTERS_UPPERCASE,
        Self::from_range('['..='`'),
        Self::RANGE_LETTERS_LOWERCASE,
        Self::from_range('{'..='~'),
    ];

    pub const RANGES_LETTERS_UPPERCASE: [Self; 1] = [Self::RANGE_LETTERS_UPPERCASE];
    pub const RANGES_LETTERS_LOWERCASE: [Self; 1] = [Self::RANGE_LETTERS_LOWERCASE];
    pub const RANGES_LETTERS: [Self; 2] =
        [Self::RANGE_LETTERS_UPPERCASE, Self::RANGE_LETTERS_LOWERCASE];

    pub const RANGES_HEX_LOWERCASE: [Self; 2] =
        [Self::RANGE_NUMBERS, Self::RANGE_HEX_LETTERS_LOWERCASE];
    pub const RANGES_HEX_UPPERCASE: [Self; 2] =
        [Self::RANGE_NUMBERS, Self::RANGE_HEX_LETTERS_UPPERCASE];
    pub const RANGES_HEX: [Self; 3] = [
        Self::RANGE_NUMBERS,
        Self::RANGE_HEX_LETTERS_UPPERCASE,
        Self::RANGE_HEX_LETTERS_LOWERCASE,
    ];

    pub const RANGES_ALPHANUM_LOWERCASE: [Self; 2] =
        [Self::RANGE_NUMBERS, Self::RANGE_LETTERS_LOWERCASE];

    pub const RANGES_ALPHANUM_UPPERCASE: [Self; 2] =
        [Self::RANGE_NUMBERS, Self::RANGE_LETTERS_UPPERCASE];

    pub const RANGES_ALPHANUM: [Self; 3] = [
        Self::RANGE_NUMBERS,
        Self::RANGE_LETTERS_UPPERCASE,
        Self::RANGE_LETTERS_LOWERCASE,
    ];
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_iter_RANGE_NUMBERSS() {
        let result: Vec<char> = BruteRange::RANGE_NUMBERS.iter().collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        );
    }

    #[test]
    fn test_iter_RANGE_LETTERS_LOWERCASE() {
        let result: Vec<char> = BruteRange::RANGE_LETTERS_LOWERCASE.iter().collect();
        assert_eq!(
            result,
            vec![
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
            ]
        );
    }

    #[test]
    fn test_iter_RANGE_LETTERS_UPPERCASE() {
        let result: Vec<char> = BruteRange::RANGE_LETTERS_UPPERCASE.iter().collect();
        assert_eq!(
            result,
            vec![
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
            ]
        );
    }

    #[test]
    fn test_iter_RANGE_HEX_LETTERS_LOWERCASE() {
        let result: Vec<char> = BruteRange::RANGE_HEX_LETTERS_LOWERCASE.iter().collect();
        assert_eq!(result, vec!['a', 'b', 'c', 'd', 'e', 'f']);
    }

    #[test]
    fn test_iter_RANGE_HEX_LETTERS_UPPERCASE() {
        let result: Vec<char> = BruteRange::RANGE_HEX_LETTERS_UPPERCASE.iter().collect();
        assert_eq!(result, vec!['A', 'B', 'C', 'D', 'E', 'F']);
    }

    #[test]
    fn test_iter_RANGES_PUNCT() {
        let result: Vec<char> = BruteRange::RANGES_PUNCT
            .iter_mut()
            .map(|r| r.iter())
            .flatten()
            .collect();
        assert_eq!(
            result,
            vec![
                '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':',
                ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'
            ]
        );
    }

    #[test]
    fn test_iter_RANGES_NUMBERS() {
        let result: Vec<char> = BruteRange::RANGES_NUMBERS
            .iter_mut()
            .map(|r| r.iter())
            .flatten()
            .collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        );
    }

    #[test]
    fn test_iter_RANGES_LETTERS_LOWERCASE() {
        let result: Vec<char> = BruteRange::RANGES_LETTERS_LOWERCASE
            .iter_mut()
            .map(|r| r.iter())
            .flatten()
            .collect();
        assert_eq!(
            result,
            vec![
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
            ]
        );
    }

    #[test]
    fn test_iter_RANGES_LETTERS_UPPERCASE() {
        let result: Vec<char> = BruteRange::RANGES_LETTERS_UPPERCASE
            .iter_mut()
            .map(|r| r.iter())
            .flatten()
            .collect();
        assert_eq!(
            result,
            vec![
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
            ]
        );
    }

    #[test]
    fn test_iter_RANGES_LETTERS() {
        let result: Vec<char> = BruteRange::RANGES_LETTERS
            .iter_mut()
            .map(|r| r.iter())
            .flatten()
            .collect();
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
    fn test_iter_RANGES_BRUTE() {
        let result: Vec<char> = BruteRange::RANGES_BRUTE
            .iter_mut()
            .map(|r| r.iter())
            .flatten()
            .collect();
        assert_eq!(
            result,
            vec![
                '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0',
                '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@',
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', '`',
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~'
            ]
        );
    }

    #[test]
    fn test_iter_RANGES_HEX_LOWERCASE() {
        let result: Vec<char> = BruteRange::RANGES_HEX_LOWERCASE
            .iter_mut()
            .map(|r| r.iter())
            .flatten()
            .collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f']
        );
    }
    #[test]
    fn test_iter_RANGES_HEX_UPPERCASE() {
        let result: Vec<char> = BruteRange::RANGES_HEX_UPPERCASE
            .iter_mut()
            .map(|r| r.iter())
            .flatten()
            .collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F']
        );
    }

    #[test]
    fn test_iter_RANGES_HEX() {
        let result: Vec<char> = BruteRange::RANGES_HEX
            .iter_mut()
            .map(|r| r.iter())
            .flatten()
            .collect();
        assert_eq!(
            result,
            vec![
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
                'a', 'b', 'c', 'd', 'e', 'f'
            ]
        );
    }

    #[test]
    fn test_iter_RANGES_ALPHANUM_LOWERCASE() {
        let result: Vec<char> = BruteRange::RANGES_ALPHANUM_LOWERCASE
            .iter_mut()
            .map(|r| r.iter())
            .flatten()
            .collect();
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
    fn test_iter_RANGES_ALPHANUM_UPPERCASE() {
        let result: Vec<char> = BruteRange::RANGES_ALPHANUM_UPPERCASE
            .iter_mut()
            .map(|r| r.iter())
            .flatten()
            .collect();
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
    fn test_iter_RANGES_ALPHANUM() {
        let result: Vec<char> = BruteRange::RANGES_ALPHANUM
            .iter_mut()
            .map(|r| r.iter())
            .flatten()
            .collect();
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
        let result: Vec<char> = BruteRange::RANGE_ASCII.iter().collect();
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
    fn test_iter_RANGES_ASCII() {
        let result: Vec<char> = BruteRange::RANGES_ASCII
            .iter_mut()
            .map(|r| r.iter())
            .flatten()
            .collect();
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
        let result: Vec<char> = BruteRange::RANGE_UNICODE.iter().collect();
        let expected: Vec<char> = (0..=0x10ffff)
            .into_iter()
            .map(|x| char::from_u32(x))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        assert_eq!(result.len(), expected.len());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iter_RANGES_UNICODE() {
        let result: Vec<char> = BruteRange::RANGES_UNICODE
            .iter_mut()
            .map(|r| r.iter())
            .flatten()
            .collect();
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
