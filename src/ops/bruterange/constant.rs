use super::BruteRange;

impl BruteRange {
    pub const RANGE_NUMBERS: BruteRange = BruteRange::from_range('0'..='9');
    pub const RANGE_LETTERS_UPPERCASE: BruteRange = BruteRange::from_range('A'..='Z');
    pub const RANGE_LETTERS_LOWERCASE: BruteRange = BruteRange::from_range('a'..='z');

    pub const RANGE_ASCII: BruteRange = BruteRange::from_range('\u{0}'..='\u{FF}');
    pub const RANGE_UNICODE: BruteRange = BruteRange::from_range('\u{0}'..='\u{10FFFF}');

    const RANGE_HEX_LETTERS_UPPERCASE: BruteRange = BruteRange::from_range('A'..='F');
    const RANGE_HEX_LETTERS_LOWERCASE: BruteRange = BruteRange::from_range('a'..='f');

    pub const RANGES_NUMBERS: [BruteRange; 1] = [BruteRange::RANGE_NUMBERS];

    pub const RANGES_ASCII: [BruteRange; 1] = [BruteRange::RANGE_ASCII];
    pub const RANGES_UNICODE: [BruteRange; 1] = [BruteRange::RANGE_UNICODE];

    pub const RANGES_PUNCT: [BruteRange; 4] = [
        BruteRange::from_range('!'..='/'),
        BruteRange::from_range(':'..='@'),
        BruteRange::from_range('['..='`'),
        BruteRange::from_range('{'..='~'),
    ];

    pub const RANGES_LETTERS_UPPERCASE: [BruteRange; 1] = [BruteRange::RANGE_LETTERS_UPPERCASE];
    pub const RANGES_LETTERS_LOWERCASE: [BruteRange; 1] = [BruteRange::RANGE_LETTERS_LOWERCASE];
    pub const RANGES_LETTERS: [BruteRange; 2] = [
        BruteRange::RANGE_LETTERS_UPPERCASE,
        BruteRange::RANGE_LETTERS_LOWERCASE,
    ];

    pub const RANGES_HEX_LOWERCASE: [BruteRange; 2] = [
        BruteRange::RANGE_NUMBERS,
        BruteRange::RANGE_HEX_LETTERS_LOWERCASE,
    ];
    pub const RANGES_HEX_UPPERCASE: [BruteRange; 2] = [
        BruteRange::RANGE_NUMBERS,
        BruteRange::RANGE_HEX_LETTERS_UPPERCASE,
    ];
    pub const RANGES_HEX: [BruteRange; 3] = [
        BruteRange::RANGE_NUMBERS,
        BruteRange::RANGE_HEX_LETTERS_UPPERCASE,
        BruteRange::RANGE_HEX_LETTERS_LOWERCASE,
    ];

    pub const RANGES_ALPHANUM_LOWERCASE: [BruteRange; 2] = [
        BruteRange::RANGE_NUMBERS,
        BruteRange::RANGE_LETTERS_LOWERCASE,
    ];

    pub const RANGES_ALPHANUM_UPPERCASE: [BruteRange; 2] = [
        BruteRange::RANGE_NUMBERS,
        BruteRange::RANGE_LETTERS_UPPERCASE,
    ];

    pub const RANGES_ALPHANUM: [BruteRange; 3] = [
        BruteRange::RANGE_NUMBERS,
        BruteRange::RANGE_LETTERS_UPPERCASE,
        BruteRange::RANGE_LETTERS_LOWERCASE,
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
        let result: Vec<char> = BruteRange::RANGES_PUNCT.into_iter().flatten().collect();
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
        let result: Vec<char> = BruteRange::RANGES_NUMBERS.into_iter().flatten().collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        );
    }

    #[test]
    fn test_iter_RANGES_LETTERS_LOWERCASE() {
        let result: Vec<char> = BruteRange::RANGES_LETTERS_LOWERCASE
            .into_iter()
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
            .into_iter()
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
        let result: Vec<char> = BruteRange::RANGES_LETTERS.into_iter().flatten().collect();
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
    fn test_iter_RANGES_HEX_LOWERCASE() {
        let result: Vec<char> = BruteRange::RANGES_HEX_LOWERCASE
            .into_iter()
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
            .into_iter()
            .flatten()
            .collect();
        assert_eq!(
            result,
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F']
        );
    }

    #[test]
    fn test_iter_RANGES_HEX() {
        let result: Vec<char> = BruteRange::RANGES_HEX.into_iter().flatten().collect();
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
            .into_iter()
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
            .into_iter()
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
        let result: Vec<char> = BruteRange::RANGES_ALPHANUM.into_iter().flatten().collect();
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
        let result: Vec<char> = BruteRange::RANGES_ASCII.into_iter().flatten().collect();
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
        let result: Vec<char> = BruteRange::RANGES_UNICODE.into_iter().flatten().collect();
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
