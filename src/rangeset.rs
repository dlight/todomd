use std::borrow::Borrow;
use std::ops::Bound;

use itertools::{Itertools, Position};

/// A range of indices. It's assumed that the range is non-empty. This is normally done using the
/// newtype pattern, but this would lead to some boilerplate (like calling `range.into` or similar)
/// so we just use a type alias. This means that each function that receives a range first has to
/// assert the range is not empty.
///
/// (Maybe I should work with empty ranges as well)
pub type Range = std::ops::Range<usize>;

trait RangeExt {
    /// Merge ranges. If ranges are adjacent or overlapping, returns a single range that covers them. If they are not, returns None.
    fn merge_range(&self, range: impl Borrow<Self>) -> Option<Self>
    where
        Self: Sized;

    fn range_difference(&self, range: impl Borrow<Self>) -> Vec<Self>
    where
        Self: Sized;
}

impl RangeExt for Range {
    fn merge_range(&self, range: impl Borrow<Self>) -> Option<Self> {
        let range: &Range = range.borrow();
        assert!(!self.is_empty());
        assert!(!range.is_empty());
        let start = self.start.min(range.start);
        let end = self.end.max(range.end);
        if self.end < range.start || range.end < self.start {
            None
        } else {
            Some(start..end)
        }
    }

    fn range_difference(&self, range: impl Borrow<Self>) -> Vec<Self> {
        let range: &Range = range.borrow();
        assert!(!self.is_empty());
        assert!(!range.is_empty());
        if self.end < range.start || self.start > range.end {
            return vec![self.clone()];
        } else if self.start < range.start && self.end > range.end {
            return vec![self.start..range.start, range.end..self.end];
        } else if self.start < range.start {
            return vec![self.start..range.start];
        } else if self.end > range.end {
            return vec![range.end..self.end];
        } else {
            return vec![];
        }
    }
}

/// A set of non-overlapping, non-adjacent ranges. Inserting new ranges will merge them if
/// possible. Ranges are kept sorted.
#[derive(Debug, Default)]
pub struct RangeSet {
    contents: Vec<Range>,
}

impl From<Vec<Range>> for RangeSet {
    fn from(vec: Vec<Range>) -> Self {
        vec.into_iter().collect()
    }
}

impl FromIterator<Range> for RangeSet {
    fn from_iter<I: IntoIterator<Item = Range>>(iter: I) -> Self {
        let mut range = RangeSet::default();

        for v in iter {
            range.insert_range(v);
        }

        range
    }
}

impl RangeSet {
    pub fn insert_range(&mut self, range: impl Borrow<Range>) {
        let q: &str;
        let r: &str;

        let range: &Range = range.borrow();

        if (self.contents.is_empty()) {
            self.contents.push(range.clone());
            return;
        }

        // searching for start:
        //  Err(0)       Ok(0)         Err(1)        Ok(1)         Err(2)       Ok(2)
        //  Included(0)  Included(0)   Included(1)   Included(1)   Included(2)  Included(2)
        // [             0,                          1,                         2]

        let start = match self.contents.binary_search_by(|x| x.end.cmp(&range.start)) {
            Ok(idx) => Bound::Included(idx),
            Err(idx) => Bound::Included(idx),
        };

        // searching for end:
        //  Err(0)       Ok(0)         Err(1)        Ok(1)         Err(2)      Ok(2)
        //  Excluded(0)  Included(0)   Excluded(1)   Included(1)   Excluded(2) Included(2)
        // [             0,                          1,                        2]

        let end = match self.contents.binary_search_by(|x| x.start.cmp(&range.end)) {
            Ok(idx) => Bound::Included(idx),
            Err(idx) => Bound::Excluded(idx),
        };

        let mut new_range = range.clone();

        // Merging with just the endpoints is sufficient, because the middle values are absorbed by
        // the inserted range
        let endpoints = self.contents[(start, end)]
            .iter()
            .with_position()
            .filter_map(|(p, r)| (!matches!(p, Position::Middle)).then_some(r));

        for inner_range in endpoints {
            new_range = new_range.merge_range(inner_range).unwrap();
        }

        self.contents.splice((start, end), [new_range]);
    }

    /// Inneficient but dead simple way to remove ranges. (TODO: revert into the more optimized
    /// implementation, and maybe keep around this one for testingq)
    pub fn remove_range(&mut self, removed_range: impl Borrow<Range>) {
        let removed_range: &Range = removed_range.borrow();

        if (self.contents.is_empty()) {
            return;
        }

        let old_contents = std::mem::take(&mut self.contents);

        self.contents = old_contents
            .into_iter()
            .flat_map(|s| s.range_difference(removed_range))
            .collect();
    }

    pub fn iter(&self) -> impl Iterator<Item = &Range> {
        self.contents.iter()
    }

    pub fn len(&self) -> usize {
        self.contents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    pub fn clear(&mut self) {
        self.contents.clear();
    }
}

impl<'a> IntoIterator for &'a RangeSet {
    type Item = &'a Range;
    type IntoIter = std::slice::Iter<'a, Range>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.iter()
    }
}

impl<'a> IntoIterator for RangeSet {
    type Item = Range;
    type IntoIter = std::vec::IntoIter<Range>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_merge_ranges() {
        assert_eq!((1..2).merge_range(0..1), Some(0..2));
        assert_eq!((0..5).merge_range(5..10), Some(0..10));
        assert_eq!((0..5).merge_range(&(6..10)), None);
        assert_eq!((0..5).merge_range(5..10), Some(0..10));
        assert_eq!((0..5).merge_range(&(5..10)), Some(0..10));
        assert_eq!((430..888).merge_range(602..835), Some(430..888));
        assert_eq!(
            (usize::MAX - 1..usize::MAX).merge_range(&(usize::MAX - 2..usize::MAX - 1)),
            Some(usize::MAX - 2..usize::MAX)
        );
    }

    #[test]
    fn random_merge_ranges() {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
        for _ in 0..1000 {
            let start1 = rng.random_range(0..1000);
            let start2 = rng.random_range(0..1000);
            let end1 = rng.random_range(0..1000);
            let end2 = rng.random_range(0..1000);
            let range1 = start1..end1;
            let range2 = start2..end2;
            if range1.is_empty() || range2.is_empty() {
                continue;
            }
            dbg!(&range1, &range2);
            let merged = range1.merge_range(range2);
            if start1 < start2 {
                if end1 >= start2 {
                    if end2 >= end1 {
                        assert_eq!(merged, Some(start1..end2));
                    } else {
                        assert_eq!(merged, Some(start1..end1));
                    }
                } else {
                    assert_eq!(merged, None);
                }
            } else {
                if end2 >= start1 {
                    if end1 >= end2 {
                        assert_eq!(merged, Some(start2..end1));
                    } else {
                        assert_eq!(merged, Some(start2..end2));
                    }
                } else {
                    assert_eq!(merged, None);
                }
            }
        }
    }

    #[test]
    fn range_difference() {
        assert_eq!((1..5).range_difference(0..1), vec![1..5]);
        assert_eq!((1..5).range_difference(0..2), vec![2..5]);
        assert_eq!((1..5).range_difference(2..3), vec![1..2, 3..5]);
        assert_eq!((1..5).range_difference(3..5), vec![1..3]);
        assert_eq!((1..5).range_difference(4..5), vec![1..4]);
        assert_eq!((1..5).range_difference(5..6), vec![1..5]);
        assert_eq!((1..5).range_difference(0..6), vec![]);
        assert_eq!((1..5).range_difference(2..4), vec![1..2, 4..5]);
        assert_eq!((1..5).range_difference(1..5), vec![]);
        assert_eq!((1..5).range_difference(0..5), vec![]);
        assert_eq!((1..5).range_difference(0..6), vec![]);
        assert_eq!((1..5).range_difference(1..2), vec![2..5]);
        assert_eq!((1..5).range_difference(4..5), vec![1..4]);
        assert_eq!(
            (usize::MAX - 1..usize::MAX).range_difference(0..1),
            vec![usize::MAX - 1..usize::MAX]
        );
        assert_eq!((1..2).range_difference(1..2), vec![]);
        assert_eq!((0..usize::MAX).range_difference(0..usize::MAX), vec![]);
    }

    #[test]
    fn test_range_set() {
        assert_eq!(RangeSet::from(vec![0..1, 1..2]).contents, vec![0..2]);
        assert_eq!(RangeSet::from(vec![1..2, 0..1]).contents, vec![0..2]);
        assert_eq!(RangeSet::from(vec![0..1, 2..3]).contents, vec![0..1, 2..3]);
        assert_eq!(RangeSet::from(vec![]).contents, vec![]);
        assert_eq!(
            RangeSet::from(vec![5..10, 0..3, 8..15]).contents,
            vec![0..3, 5..15]
        );

        assert_eq!(
            RangeSet::from(vec![
                usize::MAX - 1..usize::MAX,
                usize::MAX - 2..usize::MAX - 1
            ])
            .contents,
            vec![usize::MAX - 2..usize::MAX]
        );

        assert_eq!(
            RangeSet::from(vec![1..3, 5..7, 9..11, 13..15, 6..10]).contents,
            vec![1..3, 5..11, 13..15]
        );

        assert_eq!(
            RangeSet::from(vec![14..21, 0..5, 10..13, 20..25, 4..11]).contents,
            vec![0..13, 14..25]
        );

        assert_eq!(
            RangeSet::from(vec![1..4, 3..6, 5..8, 7..10, 9..12]).contents,
            vec![1..12]
        );

        assert_eq!(
            RangeSet::from(vec![100..200, 300..400, 500..600, 150..350, 450..550]).contents,
            vec![100..400, 450..600]
        );

        assert_eq!(
            RangeSet::from(vec![10..20, 1..2, 2..3, 3..4, 1..6, 4..5, 5..6]).contents,
            vec![1..6, 10..20]
        );
        assert_eq!(
            RangeSet::from(vec![1000..2000, 3000..4000, 0..5000]).contents,
            vec![0..5000]
        );

        assert_eq!(
            RangeSet::from(vec![1..5, 10..15, 20..25, 2..3, 12..13, 22..23]).contents,
            vec![1..5, 10..15, 20..25]
        );

        assert_eq!(
            RangeSet::from(vec![1..10, 20..30, 40..50, 5..45]).contents,
            vec![1..50]
        );
    }

    #[test]
    fn test_remove_rangeset() {
        let mut ranges = RangeSet::from(vec![1..5, 10..15, 20..25]);
        ranges.remove_range(4..22);
        assert_eq!(ranges.contents, vec![1..4, 22..25]);
    }
}
