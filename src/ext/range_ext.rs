//! 区間（[`Range`] / [`RangeInclusive`]）の拡張
//!
//! 半開区間 `start..end` と閉区間 `start..=end` を統一的に扱うための拡張トレイト [`RangeExt`] と、
//! 区間の集合をマージする [`interval_merge`] を提供する。
//!
//! 扱う区間の形は 2 種類である。
//!
//! - `start..end`: **半開区間**（`start` を含み、`end` を含まない）。空の区間（`5..5` や `5..3`）も表せる。
//! - `start..=end`: **閉区間**（両端を含む）。
//!
//! どちらの型も `overlaps`（重なっているか）・`touches`（接しているか）・`intersection`（交差）・
//! `merge`（結合）を同じ意味で使えるが、**空区間と「接している」の解釈が型によって異なる**:
//!
//! - 半開区間: `overlaps` / `touches` は**空区間を必ず `false`** にする。隣接する半開区間
//!   （`1..3` と `3..5`）は共通部分を持たないので `overlaps` は `false` だが `touches` は `true` になる。
//! - 閉区間: `1..=3` と `3..=5` は `3` を共有するので `overlaps` が `true`。一方 `1..=3` と `4..=5` は
//!   `end + 1 == start` のときだけ「接している」とみなし `touches` が `true` になる。
//!
//! `merge` は `touches` が `true` のときだけ結合でき、そうでなければ `None` を返す。
//! なお [`RangeExt`] は [`crate::ext::prelude`] に含まれないので、使うときは
//! `use atcoder::ext::range_ext::RangeExt;` とする。
//!
//! # Examples
//! ```
//! use atcoder::ext::range_ext::{interval_merge, RangeExt};
//!
//! let a = 1..3;
//! assert!(!a.overlaps(&(3..5))); // 隣り合っているだけで重ならない
//! assert!(a.touches(&(3..5))); // これが overlaps との違い
//! assert_eq!(a.merge(&(3..5)), Some(1..5));
//!
//! let b = 1..=3;
//! assert!(b.overlaps(&(3..=5))); // 閉区間は端を共有する
//!
//! assert_eq!(interval_merge(vec![1..3, 2..4, 5..6]), vec![1..4, 5..6]);
//! ```

use crate::prelude::IterExt;
use std::ops::{Add, Range, RangeInclusive};

/// [`Range`], [`RangeInclusive`] の拡張。
///
/// 半開区間 `Range<T>` と閉区間 `RangeInclusive<T>` の両方に実装される。
/// 各メソッドは原則としてレシーバと `other` の**同じ型同士**を比べる（半開区間と閉区間は直接は混ぜられない）。
/// 混ぜて扱いたいときは [`Self::into_exclusive`] で両方を半開区間に揃えるか、
/// [`interval_merge`]（`Range<T>`・`RangeInclusive<T>` どちらの列も受け取れる）を使う。
///
/// # 備考
/// - `RangeInclusive<T>` 側の impl には `T: Add<Output = T> + From<u8> + Clone` を要求する
///   （`into_exclusive` で `end + 1` を作るため）。`i32` などのプリミティブ型はそのまま使える。
/// - どのメソッドでも空区間（半開区間では始端 ≧ 終端、閉区間では始端 > 終端）は
///   「何とも重ならず・接しない」ものとして扱われる。
pub trait RangeExt<T> {
    /// ([`RangeInclusive`] の)終端を含まない範囲に変換する。
    ///
    /// [`Range<T>`] ではそのままの値を返し、[`RangeInclusive<T>`] では終端を含まない半開区間
    /// `start..(end + 1)` を返す。半開区間と閉区間を同じコードで扱いたいとき、これで片方に揃える。
    /// 空の閉区間（`5..=4`）は空の半開区間（`5..5`）に変換される。
    ///
    /// # 計算量
    /// 時間 O(1)
    ///
    /// # Examples
    /// ```
    /// use atcoder::ext::range_ext::RangeExt;
    ///
    /// assert_eq!((1..3).into_exclusive(), 1..3);
    /// assert_eq!((1..=3).into_exclusive(), 1..4);
    ///
    /// // 両方を半開区間に揃えてから比較できる
    /// let a = (1..=2).into_exclusive();
    /// let b = (3..=5).into_exclusive();
    /// assert!(!a.overlaps(&b));
    /// assert!(a.touches(&b));
    /// ```
    fn into_exclusive(self) -> Range<T>;

    /// 他の範囲と重なっているかを判定する。
    ///
    /// 両者が**内部の点を共有する**（共通部分が空でない）ときに `true`。
    /// ただし `overlaps` は**空区間を必ず `false`** にする。`5..5` や `5..3` のような始端 ≧ 終端の
    /// 半開区間、`5..=4` のような閉区間は何とも重ならない。
    /// また「隙間なく接しているだけ」の区間（半開区間の `1..3` と `3..5`）も共通部分を持たないので `false`。
    /// 接しているかまで含めたいときは [`Self::touches`] を使う。
    ///
    /// # 計算量
    /// 時間 O(1)
    ///
    /// # Examples
    /// ```
    /// use atcoder::ext::range_ext::RangeExt;
    ///
    /// assert!((1..3).overlaps(&(2..4)));
    /// assert!(!(1..3).overlaps(&(3..5))); // 接しているだけ
    /// assert!(!(5..5).overlaps(&(1..9))); // 空区間
    /// ```
    fn overlaps(&self, other: &Self) -> bool
    where
        T: Ord;

    /// 他の範囲と接しているかを判定する。
    ///
    /// **重なっているか、隙間なく隣接しているか**のどちらかを満たすときに `true`。
    /// つまり `touches` は `overlaps` を含み、さらに `1..3` と `3..5` のような
    /// 「接しているが重ならない」区間（共通部分を持たない）でも `true` になる。
    /// 閉区間では `1..=3` と `4..=5` のように `end + 1 == start` のときだけ隣接とみなす。
    /// 空区間は何とも重ならず・接しないので `false`。
    ///
    /// [`Self::merge`] はこの `touches` が `true` のときだけ 2 区間を結合できる（そうでなければ `None`）。
    ///
    /// # 計算量
    /// 時間 O(1)
    ///
    /// # Examples
    /// ```
    /// use atcoder::ext::range_ext::RangeExt;
    ///
    /// assert!((1..3).touches(&(3..5))); // 隣接
    /// assert!((1..3).touches(&(2..4))); // 重なる
    /// assert!(!(1..3).touches(&(4..6))); // 2 だけ離れている
    ///
    /// assert!((1..=3).touches(&(4..=5))); // 閉区間で end + 1 == start
    /// assert!(!(1..=3).touches(&(5..=6)));
    /// ```
    fn touches(&self, other: &Self) -> bool
    where
        T: Ord;

    /// 他の範囲と交差する範囲を返す。
    ///
    /// 「始端の大きいほう」から「終端の小さいほう」までの共通部分を `Some` で返す
    /// （型はレシーバと同じ。半開区間同士なら半開区間、閉区間同士なら閉区間）。
    /// 交差しない、または交差部分が空になるときは `None` を返す。
    ///
    /// # 計算量
    /// 時間 O(1)
    ///
    /// # Examples
    /// ```
    /// use atcoder::ext::range_ext::RangeExt;
    ///
    /// assert_eq!((1..4).intersection(&(2..6)), Some(2..4));
    /// assert_eq!((1..=4).intersection(&(2..=6)), Some(2..=4));
    /// assert_eq!((1..3).intersection(&(3..5)), None); // 接しているだけ
    /// ```
    fn intersection(&self, other: &Self) -> Option<Self>
    where
        Self: Sized,
        T: Ord + Clone;

    /// 他の範囲とマージする。
    ///
    /// `touches` が `true` のとき（重なるか、隙間なく隣接しているとき）2 区間を覆う最小の区間を `Some` で返す。
    /// `touches` が `false` のとき（間に隙間があるか、片方が空のとき）は結合できないので `None`。
    /// 返り値の型はレシーバと同じ型で、結果の区間は両入力を含む。
    ///
    /// # 計算量
    /// 時間 O(1)
    ///
    /// # Examples
    /// ```
    /// use atcoder::ext::range_ext::RangeExt;
    ///
    /// assert_eq!((1..3).merge(&(3..5)), Some(1..5));
    /// assert_eq!((1..3).merge(&(2..4)), Some(1..4));
    /// assert_eq!((1..3).merge(&(4..6)), None); // 隙間がある
    ///
    /// assert_eq!((1..=3).merge(&(4..=6)), Some(1..=6));
    /// ```
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
/// 空でない区間たちをマージし、互いに重ならず・接したりしない区間の集合を返す。
/// 入力は `Range<T>`（半開区間）でも `RangeInclusive<T>`（閉区間）でもよく、両方を混ぜてもよい
/// （内部で [`RangeExt::into_exclusive`] により半開区間に揃えてから処理する）。
/// 返り値は**始端の昇順に並んだ半開区間**（`Range<T>`）の列で、入力に含まれる空区間は無視される。
/// 「隙間なく接している」区間も 1 つにまとめる（`1..3` と `3..5` は `1..5` になる）。
/// 典型用途は、与えられた区間をまとめて「覆われている集合が連続になるか」を調べること、
/// あるいは逆に残った隙間を求めることである。
///
/// # 計算量
/// 時間 O(N log N)
///
/// # Examples
/// ```
/// use atcoder::ext::range_ext::interval_merge;
///
/// let intervals = vec![1..3, 2..4, 5..6, 5..5];
/// assert_eq!(interval_merge(intervals), vec![1..4, 5..6]);
///
/// let intervals: Vec<std::ops::RangeInclusive<i32>> = vec![1..=3, 4..=6];
/// assert_eq!(interval_merge(intervals), vec![1..7]);
/// ```
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
