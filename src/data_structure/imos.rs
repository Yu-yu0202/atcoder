//! いもす法

use crate::ext::range_ext::RangeExt;
use std::ops::{AddAssign, Deref, DerefMut, SubAssign};

/// いもす法
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Imos<T: Copy + Default + AddAssign + SubAssign>(Vec<T>);

impl<T: Copy + Default + AddAssign + SubAssign> Imos<T> {
    /// 指定された長さ `n` の配列を持ついもす法を初期化する。
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self(vec![T::default(); n + 1])
    }

    /// 指定された範囲 `range` に `value` を加える。
    pub fn add<R>(&mut self, range: R, value: T)
    where
        R: RangeExt<usize>,
    {
        let range = range.into_exclusive();

        self.0[range.start] += value;
        self.0[range.end] -= value;
    }

    /// 差分配列を確定し、累積配列を返す。
    #[must_use]
    pub fn build(mut self) -> Vec<T> {
        for i in 1..self.0.len() {
            let prev = self.0[i - 1];
            self.0[i] += prev;
        }

        let len = self.0.len() - 1;
        self.0.truncate(len);

        self.0
    }
}

impl<T: Copy + Default + AddAssign + SubAssign> Deref for Imos<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Copy + Default + AddAssign + SubAssign> DerefMut for Imos<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Copy + Default + AddAssign + SubAssign> IntoIterator for Imos<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T: Copy + Default + AddAssign + SubAssign> IntoIterator for &'a Imos<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T: Copy + Default + AddAssign + SubAssign> IntoIterator for &'a mut Imos<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T: Copy + Default + AddAssign + SubAssign> AsRef<[T]> for Imos<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

impl<T: Copy + Default + AddAssign + SubAssign> AsMut<[T]> for Imos<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let imos = Imos::<i64>::new(5);

        assert_eq!(imos.build(), vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn add_exclusive() {
        let mut imos = Imos::<i64>::new(5);

        imos.add(1..4, 3);

        // [1, 4) += 3
        assert_eq!(imos.build(), vec![0, 3, 3, 3, 0]);
    }

    #[test]
    fn add_inclusive() {
        let mut imos = Imos::<i64>::new(5);

        imos.add(1..=3, 3);

        // 1..=3 == 1..4
        assert_eq!(imos.build(), vec![0, 3, 3, 3, 0]);
    }

    #[test]
    fn exclusive_and_inclusive_are_equivalent() {
        let mut a = Imos::<i64>::new(10);
        let mut b = Imos::<i64>::new(10);

        a.add(2..7, 5);
        b.add(2..=6, 5);

        assert_eq!(a.build(), b.build());
    }

    #[test]
    fn build_single_interval() {
        let mut imos = Imos::<i64>::new(5);

        imos.add(1..4, 3);

        let imos = imos.build();

        assert_eq!(imos, vec![0, 3, 3, 3, 0]);
    }

    #[test]
    fn build_multiple_overlapping_intervals() {
        let mut imos = Imos::<i64>::new(6);

        imos.add(0..3, 2);
        imos.add(2..=4, 1);
        imos.add(4..6, -3);

        let imos = imos.build();

        assert_eq!(
            imos,
            vec![
                2,  // +2
                2,  // +2
                3,  // +2 +1
                1,  // +1
                -2, // +1 -3
                -3, // -3
            ]
        );
    }

    #[test]
    fn full_range() {
        let mut imos = Imos::<i64>::new(5);

        imos.add(0..5, 42);

        let imos = imos.build();

        assert_eq!(imos, vec![42, 42, 42, 42, 42]);
    }

    #[test]
    fn full_range_inclusive() {
        let mut imos = Imos::<i64>::new(5);

        imos.add(0..=4, 42);

        let imos = imos.build();

        assert_eq!(imos, vec![42, 42, 42, 42, 42]);
    }

    #[test]
    fn empty_range_does_nothing() {
        let mut imos = Imos::<i64>::new(5);

        imos.add(2..2, 100);

        assert_eq!(imos.clone().build(), vec![0, 0, 0, 0, 0]);

        let imos = imos.build();

        assert_eq!(imos, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn negative_values() {
        let mut imos = Imos::<i64>::new(5);

        imos.add(1..4, -10);

        let imos = imos.build();

        assert_eq!(imos, vec![0, -10, -10, -10, 0]);
    }

    #[test]
    fn single_element() {
        let mut imos = Imos::<i64>::new(5);

        imos.add(2..3, 7);

        let imos = imos.build();

        assert_eq!(imos, vec![0, 0, 7, 0, 0]);
    }

    #[test]
    fn single_element_inclusive() {
        let mut imos = Imos::<i64>::new(5);

        imos.add(2..=2, 7);

        let imos = imos.build();

        assert_eq!(imos, vec![0, 0, 7, 0, 0]);
    }

    #[test]
    fn cancellation() {
        let mut imos = Imos::<i64>::new(5);

        imos.add(1..4, 10);
        imos.add(1..4, -10);

        let imos = imos.build();

        assert_eq!(imos, vec![0, 0, 0, 0, 0]);
    }
}
