use super::MBruteRange;
use super::MBruteRangeIter;

impl Iterator for MBruteRangeIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl MBruteRange {
    pub fn iter(&self) -> MBruteRangeIter {
        let iter = self.ranges.clone().into_iter().flatten();
        MBruteRangeIter { iter }
    }
}

impl IntoIterator for MBruteRange {
    type Item = char;

    type IntoIter = MBruteRangeIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
