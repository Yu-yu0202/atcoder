//! Slice の拡張

use crate::ext::iter_ext::IterExt;
use std::cmp::Ordering;

/// Slice の拡張 trait
pub trait SliceExt<T> {
    /// ソートされたときの元要素の index を返す。
    ///
    /// [`Self::argsort_by`]`(|a, b| a.cmp(b))` と同等。
    #[inline]
    fn argsort(&self) -> Vec<usize>
    where
        T: Ord,
    {
        self.argsort_by(|a, b| a.cmp(b))
    }

    /// 比較関数 compare によってソートされたときの元要素の index を返す。
    fn argsort_by<F>(&self, compare: F) -> Vec<usize>
    where
        F: FnMut(&T, &T) -> Ordering;

    /// キー関数 f によってソートされたときの元要素の index を返す。
    fn argsort_by_key<K, F>(&self, f: F) -> Vec<usize>
    where
        K: Ord,
        F: FnMut(&T) -> K;
}

impl<T> SliceExt<T> for [T] {
    #[inline]
    fn argsort_by<F>(&self, mut compare: F) -> Vec<usize>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut incides = (0..self.len()).collect_vec();
        incides.sort_unstable_by(|&a, &b| compare(&self[a], &self[b]).then_with(|| a.cmp(&b)));
        incides
    }

    #[inline]
    fn argsort_by_key<K, F>(&self, mut f: F) -> Vec<usize>
    where
        K: Ord,
        F: FnMut(&T) -> K,
    {
        let mut incides = (0..self.len()).collect_vec();
        incides.sort_by_key(|&i| f(&self[i]));
        incides
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argsort() {
        let v = [3, 1, 2];
        assert_eq!(v.argsort(), vec![1, 2, 0]);
    }

    #[test]
    fn test_argsort_by() {
        let v = [3, 1, 2];
        assert_eq!(v.argsort_by(|a, b| b.cmp(a)), vec![0, 2, 1]);
    }

    #[test]
    fn test_argsort_by_key() {
        let v: [i32; 3] = [-3, 1, -2];
        assert_eq!(v.argsort_by_key(|&x| x.abs()), vec![1, 2, 0]);
    }
}
