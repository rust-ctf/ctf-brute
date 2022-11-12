use std::ops::{Range, RangeInclusive};

use itertools::Itertools;

use crate::ops::resetiter::ResetIter;

use super::{Pattern, PatternIter};

use rayon::{prelude::*};

impl Pattern
{
    pub fn par_iter(&self) -> rayon::iter::Map<rayon::range::Iter<u128>, impl Fn(u128) -> String + '_>
    {
        let size = self.len().expect("Iterator is too large");
        let iter = self;
        (0..size).into_par_iter().map(move |i|
            {
                let mut buffer = String::new();
                unsafe { _ =  iter.nth_unchecked(i, &mut buffer); }
                buffer
            })
    }

    // pub fn iter(&self) -> std::iter::Map<Range<u128>, impl Fn(u128) -> String + '_>
    // {
    //     let size = self.len().expect("Iterator is too large");
    //     let iter = self;
    //     (0..size).into_iter().map(move |i|
    //         {
    //             let mut buffer = String::new();
    //             unsafe { _ =  iter.nth_unchecked(i, &mut buffer); }
    //             buffer
    //         })
    // }

    unsafe fn nth_unchecked<'a>(&self, index: u128, buffer: &'a mut String) -> &'a str
    {
        match self
        {
            Pattern::Range(range) => buffer.push(range.nth_unchecked(index as u32)),
            Pattern::MRange(range) => buffer.push(range.nth_unchecked(index as u32)),
            Pattern::Group(patterns) =>
            {
                let mut index = index;
                for i in (0..patterns.len())
                {
                    let pattern = &patterns[i];
                    let size = pattern.len().expect("Iterator is too large");
                    let new_index = index % size;
                    pattern.nth_unchecked(new_index, buffer);
                    index /= size;
                }
            }
            Pattern::Length(pattern, range, indexes) =>
            {
                let range_index = match indexes.binary_search(&index) {
                    Ok(index) => index,
                    Err(index) => index - 1,
                };
                let len = *range.start() + (range_index as u32);
                let mut index = index - indexes[range_index];
                for i in 0..len
                {
                    let pattern = pattern;
                    let size = pattern.len().expect("Iterator is too large");
                    let new_index = index % size;
                    pattern.nth_unchecked(new_index, buffer);
                    index /= size;
                }
            }
        }
        buffer.as_str()
    }
}

// impl<'st,'bf> Iterator for &'st PatternIter<'st,'bf> {
//     type Item = &'st str;

//     fn next(&mut self) -> Option<Self::Item> {
//         if !self.has_next() {
//             return None;
//         }
//         Some(self.get_next())
//     }
// }

// impl<'st,'bf> PatternIter<'st,'bf> {
//     //TODO: Return string slice to avoid allocations
//     // type Item<'st> = &'st str where
//     //     Self: 'st;

//     fn has_next(&'st self) -> bool {
//         match self {
//             Self::Base(pattern, _, _) => pattern.has_next(),
//             Self::Range(iter) => iter.has_next(),
//             Self::MRange(iter) => iter.has_next(),
//             Self::Group(iters) => iters[0].has_next(),
//             Self::Length(iters, length, _) => {
//                 iters.len() != 0 && iters[0].has_next() || *length == 0
//             }
//         }
//     }

//     fn move_next(&'st mut self) {
//         match self {
//             Self::Base(pattern, buffer, init) => {
//                 *init = false;
//                 pattern.move_next();
//             }
//             Self::Range(iter) => iter.move_next(),
//             Self::MRange(iter) => iter.move_next(),
//             Self::Group(iters) => {
//                 for i in (0..iters.len()).rev() {
//                     debug_assert!(iters.get(i).is_some());
//                     let iter = &mut iters[i];
//                     debug_assert!(iter.has_next());
//                     iter.move_next();
//                     if iter.has_next() {
//                         break;
//                     }
//                     if i != 0 {
//                         iter.reset()
//                     }
//                 }
//             }
//             Self::Length(iters, length, _) => {
//                 if *length == 0 {
//                     *length = 1;
//                     return;
//                 }
//                 debug_assert!(*length <= iters.len());
//                 let start = iters.len() - *length;
//                 for i in (start..iters.len()).rev() {
//                     debug_assert!(iters.get(i).is_some());
//                     let iter = &mut iters[i];
//                     debug_assert!(iter.has_next());
//                     iter.move_next();
//                     if iter.has_next() {
//                         break;
//                     }
//                     if i != start {
//                         iter.reset()
//                     }
//                 }

//                 debug_assert!(iters.get(start).is_some());

//                 if !iters[start].has_next() {
//                     *length += 1;
//                     if start != 0 {
//                         iters[start].reset()
//                     }
//                 }
//             }
//         }
//     }

//     fn get_next(&mut self) -> &'bf str {
//         let value = self.peek();
//         self.move_next();
//         value
//     }

//     fn peek(&'st mut self) -> &'bf str {
//         match self {
//             Self::Base(pattern, buffer, init) =>
//             {
//                 if !*init
//                 {
//                     buffer.clear();
//                     pattern.peek_buffered(buffer);
//                     *init = true;
//                 }
                
//                 buffer.as_str()
//             }
//            _ => panic!("Calling peek from non base pattern is not allowed")
//         }
//     }

//     fn reset(&'st mut self) {
//         match self {
//             Self::Base(pattern, buffer , init) => {
//                 pattern.reset();
//                 *init = false;
//                 buffer.clear();
//             }
//             Self::Range(iter) => {
//                 iter.reset();
//             }
//             Self::MRange(iter) => {
//                 iter.reset();
//             }
//             Self::Group(iters) => {
//                 iters.iter_mut().for_each(|i| i.reset());
//             }
//             Self::Length(iters, length, start_length) => {
//                 iters.iter_mut().for_each(|i| i.reset());
//                 *length = *start_length;
//             }
//         }
//     }
// }

// impl<'st,'bf> PatternIter<'st,'bf>
// {
//     fn peek_buffered(&'st mut self, buffer: &'bf mut String)
//     {
//         match self {
//             Self::Base(pattern, _, _) =>
//             {
//                 pattern.peek_buffered(buffer);
//             }
//             Self::Range(iter) => {
//                 debug_assert!(iter.has_next());
//                 buffer.push(iter.peek())
//             }
//             Self::MRange(iter) => {
//                 debug_assert!(iter.has_next());
//                 buffer.push(iter.peek())
//             }
//             Self::Group(iters) => {
//                 for iter in iters.iter_mut() {
//                     iter.peek_buffered(buffer);
//                 }
//             }
//             Self::Length(iters, length, _) => {
//                 debug_assert!(*length <= iters.len());
//                 let start = iters.len() - *length;
//                 for i in start..iters.len() {
//                     debug_assert!(iters.get(i as usize).is_some());
//                     let iter = &mut iters[i as usize];
//                     debug_assert!(iter.has_next());
//                     iter.peek_buffered(buffer);
//                 }
//             }
//         }
//     }
// }

// impl Pattern {
//     pub fn iter<'st, 'bf>(&'st self, buffer: &'bf mut String) -> PatternIter<'st,'bf> {
//         let iter = match self {
//             Self::Range(range) => PatternIter::Range(range.iter()),
//             Self::MRange(range) => PatternIter::MRange(range.iter()),
//             Self::Group(patterns) => {
//                 PatternIter::Group(patterns.iter().map(move |p| p.iter(buffer)).collect())
//             }
//             Self::Length(pattern, range) => {
//                 let max_len = *range.end();
//                 let start = *range.start();
//                 let iters = (0..max_len).into_iter().map(move |_| pattern.iter(buffer)).collect();
//                 PatternIter::Length(iters, start as usize, start as usize)
//             }
//         };
//         PatternIter::Base(Box::new(iter), buffer, false)

//     }
// }

// impl<'st> IntoIterator for &'st Pattern {
//     type Item = String;

//     type IntoIter = PatternIter<'st>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }

#[cfg(test)]
mod tests {
    use crate::ops::BruteRange;

    use super::*;
    use nonempty::{nonempty, NonEmpty};

    #[test]
    fn test_group() {
        let pattern = Pattern::Group(vec![
            Pattern::Range(BruteRange::from_range('a'..='b')),
            Pattern::Range(BruteRange::from_range('a'..='c')),
        ]);
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(result, vec!["aa", "ab", "ac", "ba", "bb", "bc"]);
    }

    #[test]
    fn test_lenght() {
        let pattern = Pattern::Length(
            Box::new(Pattern::Range(BruteRange::from_range('a'..='c'))),
            1..=2,
        );
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(
            result,
            vec!["a", "b", "c", "aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc"]
        );
    }

    #[test]
    fn test_lenght2() {
        let pattern = Pattern::Length(
            Box::new(Pattern::Range(BruteRange::from_range('a'..='c'))),
            0..=3,
        );
        let result: Vec<String> = pattern.iter().collect();
        assert_eq!(
            result,
            vec![
                "", "a", "b", "c", "aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc", "aaa",
                "aab", "aac", "aba", "abb", "abc", "aca", "acb", "acc", "baa", "bab", "bac", "bba",
                "bbb", "bbc", "bca", "bcb", "bcc", "caa", "cab", "cac", "cba", "cbb", "cbc", "cca",
                "ccb", "ccc"
            ]
        );
    }
}
