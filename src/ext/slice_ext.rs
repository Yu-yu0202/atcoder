//! Slice の拡張
//!
//! 「ソートしたときの元要素の index」（順位列）を返す [`SliceExt`] トレイトを提供する。
//! 任意の `[T]` に実装されるため、配列・`Vec` のスライス・[`collect_vec`](IterExt::collect_vec) で作った
//! `Vec` のどれに対しても `argsort` 系メソッドをそのまま呼べる。
//!
//! 値そのものではなく index を返すので、元の列を壊さずに「何位かを調べる」「得られた index 列で
//! 別の列を並べ替える」用途に向く。
//!
//! # Examples
//! ```
//! use atcoder::ext::prelude::*;
//!
//! let v = vec!['b', 'a', 'c'];
//! let order = v.argsort(); // 昇順に並べたときの index
//! assert_eq!(order, vec![1, 0, 2]);
//! assert_eq!(order.iter().map(|&i| v[i]).collect_vec(), vec!['a', 'b', 'c']);
//! ```

use crate::ext::iter_ext::IterExt;
use std::cmp::Ordering;

/// Slice の拡張 trait。
///
/// `[T]` に対して実装される。返り値の `Vec<usize>` は
/// 「`self[i]`（の比較結果・キー）の昇順に並べたとき、先頭から i 番目に来る元の index」を表す。
/// たとえば `let sorted: Vec<&T> = v.argsort().iter().map(|&i| &v[i]).collect();` のように
/// 元の列を保ったまま整列結果を取り出せる。
///
/// - [`Self::argsort`]: 自然順（[`Ord`]）で昇順に index を並べる。
/// - [`Self::argsort_by`]: 比較関数で並べたときの index を返す。
/// - [`Self::argsort_by_key`]: キー関数で並べたときの index を返す。
///
/// 返る index 列は、要素の比較結果が同値でも必ず元の index の昇順になる（`argsort_by`・
/// `argsort_by_key` の安定性の扱いについては各メソッドの説明を参照）。
pub trait SliceExt<T> {
    /// ソートされたときの元要素の index を返す。
    ///
    /// 値を昇順にソートしたとき（[`Self::argsort_by`]`(|a, b| a.cmp(b))` と同等）、
    /// 先頭からある元の index が何番目に来るかを返す。同値の要素は元の index の昇順に並ぶ。
    ///
    /// # 計算量
    /// 時間 O(n log n), 空間 O(n)
    ///
    /// # Examples
    /// ```
    /// use atcoder::ext::prelude::*;
    ///
    /// let v = [3, 1, 2];
    /// assert_eq!(v.argsort(), vec![1, 2, 0]);
    ///
    /// let w = ["b", "a", "b"];
    /// assert_eq!(w.argsort(), vec![1, 0, 2]); // 同値なら元の index の昇順
    /// ```
    #[inline]
    fn argsort(&self) -> Vec<usize>
    where
        T: Ord,
    {
        self.argsort_by(|a, b| a.cmp(b))
    }

    /// 比較関数 compare によってソートされたときの元要素の index を返す。
    ///
    /// [`Ordering`] を返す `compare` の順に index を並べ替える。降順にしたければ
    /// `v.argsort_by(|a, b| b.cmp(a))` のように比較を反転させる。
    ///
    /// ソートには `sort_unstable_by`（不安定ソート）を使うが、`compare` が [`Ordering::Equal`] を返した
    /// 要素同士は**元の index の昇順**で比較するため、結果は同値要素についても必ず安定（決定的）に並ぶ。
    /// 言い換えると「結果だけ見れば安定ソートと同じ」だが、`compare` が呼ばれる回数・順は不定なので、
    /// フリーパラメータとして比較関数を組み立てる用途では結果の保証のみを当てにすること。
    ///
    /// # 計算量
    /// 時間 O(n log n), 空間 O(n)
    ///
    /// # Examples
    /// ```
    /// use atcoder::ext::prelude::*;
    ///
    /// let v = [3, 1, 2];
    /// assert_eq!(v.argsort_by(|a, b| b.cmp(a)), vec![0, 2, 1]); // 降順
    ///
    /// let w = [(1, 2), (0, 9), (1, 1)];
    /// assert_eq!(w.argsort_by(|a, b| a.0.cmp(&b.0)), vec![1, 0, 2]); // 0 番目だけ見て比較
    /// ```
    fn argsort_by<F>(&self, compare: F) -> Vec<usize>
    where
        F: FnMut(&T, &T) -> Ordering;

    /// キー関数 f によってソートされたときの元要素の index を返す。
    ///
    /// 各要素からキー `K` を計算し、そのキーの昇順に index を並べ替える（`f` を比較関数に持ち上げたもの）。
    /// 安定ソート（`sort_by_key`）由来で、キーが等しい要素は元の index の昇順に並ぶ。
    /// `f` はソート中に O(n log n) 回程度呼ばれるため、重い計算はあらかじめ列へ写しておくこと。
    ///
    /// # 計算量
    /// 時間 O(n log n), 空間 O(n)
    ///
    /// # Examples
    /// ```
    /// use atcoder::ext::prelude::*;
    ///
    /// let v: [i32; 3] = [-3, 1, -2];
    /// assert_eq!(v.argsort_by_key(|&x| x.abs()), vec![1, 2, 0]);
    ///
    /// let w = ["bb", "a", "ccc"];
    /// assert_eq!(w.argsort_by_key(|s| s.len()), vec![1, 0, 2]); // 短い順
    /// ```
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
