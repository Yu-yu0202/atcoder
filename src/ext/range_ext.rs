use crate::prelude::IterExt;
use std::ops::{Add, Range, RangeInclusive};

/// [`Range`], [`RangeInclusive`] の拡張
pub trait RangeExt<T> {
    /// ([`RangeInclusive`] の)終端を含まない範囲に変換する。
    fn into_exclusive(self) -> Range<T>;

    /// 他の範囲と重なっているかを判定する。
    fn overlaps(&self, other: &Self) -> bool
    where
        T: Ord;

    /// 他の範囲と接しているかを判定する。
    fn touches(&self, other: &Self) -> bool
    where
        T: Ord;

    /// 他の範囲と交差する範囲を返す。
    fn intersection(&self, other: &Self) -> Option<Self>
    where
        Self: Sized,
        T: Ord + Clone;

    /// 他の範囲とマージする。
    fn merge(&self, other: &Self) -> Option<Self>
    where
        Self: Sized,
        T: Ord + Clone;
}

impl<T> RangeExt<T> for Range<T> {
    #[inline]
    fn into_exclusive(self) -> Range<T> {
        self
    }

    fn overlaps(&self, other: &Self) -> bool
    where
        T: Ord,
    {
        !self.is_empty() && !other.is_empty() && self.start < other.end && other.start < self.end
    }

    fn touches(&self, other: &Self) -> bool
    where
        T: Ord,
    {
        !self.is_empty()
            && !other.is_empty()
            && (self.start <= other.end && other.start <= self.end)
    }

    fn intersection(&self, other: &Self) -> Option<Self>
    where
        T: Ord + Clone,
    {
        let start = self.start.clone().max(other.start.clone());
        let end = self.end.clone().min(other.end.clone());

        (start < end).then_some(start..end)
    }

    fn merge(&self, other: &Self) -> Option<Self>
    where
        T: Ord + Clone,
    {
        if !self.touches(other) {
            return None;
        }

        Some(self.start.clone().min(other.start.clone())..self.end.clone().max(other.end.clone()))
    }
}

impl<T> RangeExt<T> for RangeInclusive<T>
where
    T: Add<Output = T> + From<u8> + Clone,
{
    fn into_exclusive(self) -> Range<T> {
        let (start, end) = self.into_inner();
        start..end + T::from(1)
    }

    fn overlaps(&self, other: &Self) -> bool
    where
        T: Ord,
    {
        !self.is_empty()
            && !other.is_empty()
            && self.start() <= other.end()
            && other.start() <= self.end()
    }

    fn touches(&self, other: &Self) -> bool
    where
        T: Ord,
    {
        if self.is_empty() || other.is_empty() {
            return false;
        }

        self.overlaps(other)
            || self.end().clone() + T::from(1) == *other.start()
            || other.end().clone() + T::from(1) == *self.start()
    }

    fn intersection(&self, other: &Self) -> Option<Self>
    where
        T: Ord,
    {
        let start = self.start().clone().max(other.start().clone());
        let end = self.end().clone().min(other.end().clone());

        (start <= end).then_some(start..=end)
    }

    fn merge(&self, other: &Self) -> Option<Self>
    where
        T: Ord,
    {
        if !self.touches(other) {
            return None;
        }

        Some(
            self.start().clone().min(other.start().clone())
                ..=self.end().clone().max(other.end().clone()),
        )
    }
}

/// 区間マージを行う。
///
/// # 計算量
/// 時間 O(N log N)
pub fn interval_merge<T, R>(intervals: impl IntoIterator<Item = R>) -> Vec<Range<T>>
where
    T: Ord + Clone,
    R: RangeExt<T>,
{
    let mut intervals = intervals
        .into_iter()
        .map(RangeExt::into_exclusive)
        .filter(|r| !r.is_empty())
        .collect_vec();

    intervals.sort_unstable_by(|a, b| a.start.cmp(&b.start).then_with(|| a.end.cmp(&b.end)));

    let mut merged = Vec::<Range<T>>::with_capacity(intervals.len());

    for range in intervals {
        match merged.last_mut() {
            Some(last) if range.start <= last.end => {
                if range.end > last.end {
                    last.end = range.end;
                }
            }
            _ => merged.push(range),
        }
    }

    merged
}
