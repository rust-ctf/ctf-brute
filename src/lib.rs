pub mod ops;

// use std::{iter::Map, ops::RangeInclusive, result};

// use itertools::Itertools;

// mod rangeutils;

// pub struct BruteIterator {
//     pattern: Pattern,
// }

// pub enum Pattern {
//     Range(RangeInclusive<char>),
//     Ranges(Vec<RangeInclusive<char>>),
//     Repeater(Box<Pattern>, RangeInclusive<u32>),
//     Patterns(Vec<Pattern>),
// }

// impl IntoIterator for Pattern
// {
//     type Item = char;

//     type IntoIter = std::ops::Range<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

// impl Pattern {
//     fn into_iter(&self) -> Option<i32>
//     {
//         match &self
//         {
//             Self::Range(range) => {
//                 let range = RangeInclusive::new(*range.start(), *range.end());
//                 let a = range.into_iter();
//                 None
//             }
//             _ => None
//         }
//     }
//     fn size(&self) -> Option<u128> {
//         match &self {
//             Self::Range(range) => rangeutils::range_size(range),
//             Self::Ranges(ranges) => ranges.iter().fold(Some(0u128), |buf, r| {
//                 buf?.checked_mul(rangeutils::range_size(r)?)
//             }),
//             Self::Repeater(pattern, range) => {
//                 let len = pattern.size()?;
//                 if range.is_empty() {
//                     return Some(0);
//                 }
//                 let combo = len.checked_pow(*range.start())?;
//                 Some(
//                     RangeInclusive::new(*range.start() + 1, *range.end())
//                         .into_iter()
//                         .fold(Some((combo, combo)), |buf, _| {
//                             let (total_size, last_size) = buf?;
//                             let size = last_size.checked_mul(len)?;
//                             Some((total_size.checked_mul(size)?, size))
//                         })?
//                         .0,
//                 )
//             }
//             Self::Patterns(patterns) => patterns.iter().fold(Some(1u128), |buf, p| {
//                 let size = p.size()?;
//                 if size == 0 {
//                     buf
//                 } else {
//                     buf?.checked_mul(size)
//                 }
//             }),
//         }
//     }
// }

// // impl Iterator for Pattern {
// //     type Item = String;

// //     fn next(&mut self) -> Option<Self::Item> {
// //         match self {
// //             Pattern::Range(range) => Some(format!("{}", range.next()?)),
// //             Pattern::Ranges(ranges) => {
// //                 for range in ranges {
// //                     let result = range.next();
// //                     if result.is_none() {
// //                         continue;
// //                     }
// //                     return Some(format!("{}", result.unwrap()));
// //                 }
// //                 None
// //             }
// //             Pattern::Repeater(pattern, lenght_range) =>
// //             {
// //                 let next = pattern.next();
// //                 if next.is_some()
// //                 {
// //                     return next.unwrap();
// //                 }

// //                 None
// //             }
// //             Pattern::Patterns(_) => todo!(),
// //         }
// //     }
// // }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
//     // fn test_pattern_iterator_range() {
//     //     let pattern = Pattern::Range('a'..='c');
//     //     let result:Vec<String> = pattern.collect();
//     //     assert_eq!(result, vec!["a", "b", "c"]);
//     // }

//     // #[test]
//     // fn test_pattern_iterator_ranges() {
//     //     let pattern = Pattern::Ranges(vec!['a'..='c', '1'..='4','B'..='E']);
//     //     let result:Vec<String> = pattern.collect();
//     //     assert_eq!(result, vec!["a", "b", "c", "1","2","3","4","B", "C", "D", "E",]);
//     // }
// }mod ops;
