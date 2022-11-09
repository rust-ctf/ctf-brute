# ctf-brute
[![crate](https://img.shields.io/crates/v/ctf-brute.svg)](https://crates.io/crates/ctf-brute)

Brute-force utilities for Rust.

## About
This crate is created as helper utility for capture the flag competitions where you have to brute force / guess text with some known patterns and/or limitations


## Pattern
Pattern supports regex-like syntax for generating text that 

### Syntax
|Name|Example|Description
|---|---|---|
|letter| 'a'  | single letter |
|escaped letter| '\{' | letter that cannot be used standalone due to syntax use. Escaped characters: "[]\(\)\{\}-" |
|ascii letter | '\x4F' | letter in ascii notation |
|unicode letter| '\u1F92F' | letter in unicode notation |
|range| 'A-Z' | all letters from smaller to larger letter (unicode value) |
|advanced range| [abC-Ef-g] | multiple ranges and/or letters |
|group| (pattern) | groups multiple patterns |
|repeater| pattern{n} | repeats pattern n times |
|advanced repeater| pattern{n,m} | repeats pattern for all numbers between n and m |
|zero repeater| pattern{,n} | repeats pattern for all numbers between 0 and n |

### Special Symbols
| Symbol | Description | Character set |
| --- | --- | --- |
|\w| lowercase letters | "abcdefghijklmnopqrstuvwxyz" |
|\W| uppercase letters | "ABCDEFGHIJKLMNOPQRSTUVWXYZ" |
|\l| letters | "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz" |
|\d| numbers | "0123456789" |
|\h| lowercase hexadecimal letters | "0123456789abcdef" |
|\H| uppercase hexadecimal letters | "0123456789ABCDEF" |
|\X| hexadecimal letters | "0123456789ABCDEFabcdef" |
|\p| punctuations | "!\"#$%&'\(\)*+,-./:;<=>?@[\\]^_`\{\|\}~" |
|\n| lowercase alphanumeric letters | "0123456789abcdefghijklmnopqrstuvwxyz" |
|\N| uppercase alphanumeric letters | "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ" |
|\m| alphanumeric letters | "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz" |
|\b| alpha numeric and punctuations | "!\"#$%&'\(\)*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz\{\|\}~" |
|\a| ascii | All ascii characters |
|\U| unicode | All unicode characters |

### Examples
```rs
use ctf_brute::ops::Pattern;

fn main() {
    let pattern = Pattern::from_pattern("(abc){1,4} End").expect("Failed to parse pattern");
    for result in pattern.iter() {
        println!("{result}");
    }
}
```
Output:
```
abc
abcabc
abcabcabc
abcabcabcabc
```


```rs
use ctf_brute::ops::Pattern;

fn main() {
    let pattern = Pattern::from_pattern(r"\d{3}").expect("Failed to parse pattern");
    for result in pattern.iter() {
        println!("{result}");
    }
}
```
Output:
```
000
001
002
.
.
.
997
998
999
```


```rs
use ctf_brute::ops::Pattern;

fn main() {
    let pattern = Pattern::from_pattern(r"[A-C5-7x-z\]\{]").expect("Failed to parse pattern");
    for result in pattern.iter() {
        println!("{result}");
    }
}
```
Output:
```
5
6
7
A
B
C
]
x
y
z
{
```

```rs
use ctf_brute::ops::Pattern;

fn main() {
    let pattern = Pattern::from_pattern(r"Text: [ab][C-E]a{0,4}").expect("Failed to parse pattern");
    for result in pattern.iter() {
        println!("{result}");
    }
}
```
Outputs
```
Text: aC
Text: aCa   
Text: aCaa  
Text: aCaaa 
Text: aCaaaa
Text: aD    
Text: aDa   
Text: aDaa  
Text: aDaaa
Text: aDaaaa
Text: aE
Text: aEa
Text: aEaa
Text: aEaaa
Text: aEaaaa
Text: bC
Text: bCa
Text: bCaa
Text: bCaaa
Text: bCaaaa
Text: bD
Text: bDa
Text: bDaa
Text: bDaaa
Text: bDaaaa
Text: bE
Text: bEa
Text: bEaa
Text: bEaaa
Text: bEaaaa
```


```rs
use ctf_brute::ops::Pattern;

fn main() {
    let pattern = Pattern::from_pattern(r"\b").expect("Failed to parse pattern");
    let results: Vec<String> = pattern.iter().collect();
    println!("{results:?}")
}
```
Outputs
```
["!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ":", ";", "<", "=", ">", "?", "@", 
"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "[", "\\", "]", "^", "_", "`", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "{", "|", "}", "~"]
```

## Contribution
Because this is my first crate and im farley new to rust, and due to nature of this crate help with optimizations is needed. Feel free to open issue if you have any suggestion or even better make pull request with fix. I have made a lot of unit tests so you don't have to worry about breaking functionality.