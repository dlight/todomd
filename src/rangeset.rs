use std::borrow::Borrow;

/// A range of indices. It's assumed that the range is non-empty. This is normally done using the
/// newtype pattern, but this would lead to some boilerplate (like calling `range.into` or similar)
/// so we just use a type alias. This means that each function that receives a range first has to
/// assert the range is not empty.
pub type Range = std::ops::Range<usize>;

/// I think this won't actually be used, might delete later.
fn intersect_ranges(range1: impl Borrow<Range>, range2: impl Borrow<Range>) -> Option<Range> {
    let range1: &Range = range1.borrow();
    let range2: &Range = range2.borrow();
    assert!(!range1.is_empty());
    assert!(!range2.is_empty());
    let start = range1.start.max(range2.start);
    let end = range1.end.min(range2.end);
    if start < end {
        Some(start..end)
    } else {
        None
    }
}

/// Merge ranges. If ranges are adjacent or overlapping, merge them, If they are not, return the original ranges.
fn merge_ranges(range1: impl Borrow<Range>, range2: impl Borrow<Range>) -> Option<Range> {
    let range1: &Range = range1.borrow();
    let range2: &Range = range2.borrow();
    assert!(!range1.is_empty());
    assert!(!range2.is_empty());
    let start = range1.start.min(range2.start);
    let end = range1.end.max(range2.end);
    if range1.end < range2.start || range2.end < range1.start {
        None
    } else {
        Some(start..end)
    }
}

/// Assumes that the range in the first argument overlaps all ranges in the second argument.
fn merge_ranges_vec(range: impl Borrow<Range>, vec: Vec<impl Borrow<Range>>) -> Option<Range> {
    let range: &Range = range.borrow();
    let mut range = range.clone();
    for r in vec {
        range = merge_ranges(range, r)?;
    }
    Some(range)
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
        use itertools::Itertools;
        use itertools::Position;
        use std::ops::Bound;

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
            new_range = merge_ranges(&new_range, inner_range).unwrap();
        }

        self.contents.splice((start, end), [new_range]);
    }

    /// TODO: make it work
    pub fn remove_range(&mut self, removed_range: impl Borrow<Range>) {
        use itertools::Itertools;
        use itertools::Position;
        use std::ops::Bound;

        let removed_range: &Range = removed_range.borrow();

        if self.contents.is_empty() {
            return;
        }

        // searching for start:
        //  Err(0)       Ok(0)         Err(1)        Ok(1)         Err(2)       Ok(2)
        //  Included(0)  Excluded(0)   Included(1)   Excluded(1)   Included(2)  Excluded(2)
        // [             0,                          1,                         2]

        let start = match self
            .contents
            .binary_search_by(|x| x.end.cmp(&removed_range.start))
        {
            Ok(idx) => Bound::Excluded(idx),
            Err(idx) => Bound::Included(idx),
        };

        // searching for end:
        //  Err(0)       Ok(0)         Err(1)        Ok(1)         Err(2)      Ok(2)
        //  Excluded(0)  Excluded(0)   Excluded(1)   Excluded(1)   Excluded(2) Excluded(2)
        // [             0,                          1,                        2]

        let end = match self
            .contents
            .binary_search_by(|x| x.start.cmp(&removed_range.end))
        {
            Ok(idx) => Bound::Excluded(idx),
            Err(idx) => Bound::Excluded(idx),
        };

        let mut new_ranges = Vec::new();

        // Considering just the endpoints is sufficient
        let endpoint_ranges = self.contents[(start, end)]
            .iter()
            .with_position()
            .filter(|(p, _)| (!matches!(p, Position::Middle)));

        for (position, range) in endpoint_ranges {
            match position {
                Position::First => {
                    let left_range = range.start..removed_range.start;

                    if !left_range.is_empty() {
                        new_ranges.push(left_range);
                    }
                }
                Position::Last => {
                    let right_range = removed_range.end..range.end;

                    if !right_range.is_empty() {
                        new_ranges.push(right_range);
                    }
                }
                Position::Only => {
                    let left_range = range.start..removed_range.start;

                    let right_range = removed_range.end..range.end;

                    if !left_range.is_empty() {
                        new_ranges.push(left_range);
                    }

                    if !right_range.is_empty() {
                        new_ranges.push(right_range);
                    }
                }
                Position::Middle => unreachable!(),
            }
        }

        self.contents.splice((start, end), new_ranges);
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

    /// Test if ranges intersect correctly.
    #[test]
    fn test_intersect_ranges() {
        assert_eq!(intersect_ranges(0..5, 5..10), None);
        assert_eq!(intersect_ranges(0..5, 6..10), None);
        assert_eq!(intersect_ranges(0..5, 4..10), Some(4..5));
        assert_eq!(intersect_ranges(0..5, 0..10), Some(0..5));
        assert_eq!(intersect_ranges(0..5, 0..5), Some(0..5));
        assert_eq!(intersect_ranges(0..3, 1..4), Some(1..3));
        assert_eq!(intersect_ranges(0..3, 2..5), Some(2..3));
        assert_eq!(intersect_ranges(0..1, 1..3), None);
    }

    #[test]
    fn test_merge_ranges() {
        assert_eq!(merge_ranges(1..2, 0..1), Some(0..2));
        assert_eq!(merge_ranges(0..5, 5..10), Some(0..10));
        assert_eq!(merge_ranges(0..5, 6..10), None);
        assert_eq!(merge_ranges(0..5, 5..10), Some(0..10));
        assert_eq!(merge_ranges(0..5, 5..10), Some(0..10));
        assert_eq!(merge_ranges(430..888, 602..835), Some(430..888));
        assert_eq!(
            merge_ranges(usize::MAX - 1..usize::MAX, usize::MAX - 2..usize::MAX - 1),
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
            let merged = merge_ranges(range1.clone(), range2.clone());
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
    fn test_merge_ranges_vec() {
        assert_eq!(
            merge_ranges_vec(0..5, vec![5..10, 1..6, 0..5, 1..2]),
            Some(0..10)
        );
        assert_eq!(
            merge_ranges_vec(0..5, vec![5..10, 1..6, 0..5, 1..2, 11..12]),
            None
        );
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
        let mut ranges = RangeSet::from(vec![1..5, 10..15, 20..25, 2..3, 12..13, 22..23]);
        ranges.remove_range(1..5);
        assert_eq!(ranges.contents, vec![5..10, 10..15, 20..25]);
        ranges.remove_range(10..15);
        assert_eq!(ranges.contents, vec![5..10, 20..25]);
    }
}
