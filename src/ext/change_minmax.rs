//! chmin / chmax の拡張
//!
//! 最小値・最大値への更新（chmin / chmax）を任意の型に対して使えるようにするトレイト
//! [`ChangeMinMax`] を提供する。 [`PartialOrd`] を満たすすべての型に blanket impl されるため、
//! `i32` や `f64`、`usize` などに何もせずそのままメソッドを呼び出せる（`use` は [`crate::ext::prelude`] 経由でも可）。
//!
//! `chmin` / `chmax` は**更新が起きたら `true`、起きなければ `false`** を返す。
//! この返り値のおかげで「この `chmin` で値が更新されたときだけ後続の処理をする」という書き方ができ、
//! DP やダイクストラの内側ループでよく使う。
//!
//! # Examples
//! ```
//! use atcoder::ext::prelude::*;
//!
//! let mut a = 2;
//! assert!(a.chmin(1)); // 更新された
//! assert!(!a.chmin(2)); // 更新されなかった
//! assert_eq!(a, 1);
//!
//! let mut b = 2;
//! assert!(b.chmax(3));
//! assert!(!b.chmax(1));
//! assert_eq!(b, 3);
//! ```

/// chmin / chmax を提供する trait。
///
/// [`PartialOrd`] を満たすすべての型に対して blanket impl される。
/// 比較だけできればよいので `Ord` は要求しない（`f64` のように `Ord` を持たない型でも使える）。
///
/// - [`Self::chmin`]: `self` が `other` より大きいとき `other` で更新する。
/// - [`Self::chmax`]: `self` が `other` より小さいとき `other` で更新する。
///
/// どちらも**更新が起きたら `true`、起きなければ `false`** を返し、同値のときは更新せず `false` になる。
///
/// # Examples
/// ```
/// use atcoder::ext::prelude::*;
///
/// let mut x = 5;
/// assert!(!x.chmin(5)); // 同値なら更新しない
/// assert!(x.chmin(4));
/// assert_eq!(x, 4);
/// ```
pub trait ChangeMinMax: PartialOrd + Sized {
    /// chmin をとる。
    ///
    /// `self` が `other` より**大きい**ときに限り `*self = other` として最小値を更新し `true` を返す。
    /// `self` が `other` 以下のとき（同値のときを含む）は何もせず `false` を返す。
    /// 返り値が「この呼び出しで値が更新されたか」を表すので、更新されたときだけキューに積む・再計算する、
    /// といった使い方ができる。
    ///
    /// # 計算量
    /// 時間 O(1)
    ///
    /// # Examples
    /// ```
    /// use atcoder::ext::prelude::*;
    ///
    /// let mut x = 2;
    /// assert!(x.chmin(1));
    /// assert_eq!(x, 1);
    ///
    /// assert!(!x.chmin(2));
    /// assert_eq!(x, 1);
    /// ```
    #[inline]
    fn chmin(&mut self, other: Self) -> bool {
        if *self > other {
            *self = other;
            true
        } else {
            false
        }
    }

    /// chmax をとる。
    ///
    /// `self` が `other` より**小さい**ときに限り `*self = other` として最大値を更新し `true` を返す。
    /// `self` が `other` 以上のとき（同値のときを含む）は何もせず `false` を返す。
    /// [`Self::chmin`] の逆で、返り値の意味も同じ（更新されたかどうか）である。
    ///
    /// # 計算量
    /// 時間 O(1)
    ///
    /// # Examples
    /// ```
    /// use atcoder::ext::prelude::*;
    ///
    /// let mut x = 1;
    /// assert!(x.chmax(3));
    /// assert_eq!(x, 3);
    ///
    /// assert!(!x.chmax(2));
    /// assert_eq!(x, 3);
    /// ```
    #[inline]
    fn chmax(&mut self, other: Self) -> bool {
        if *self < other {
            *self = other;
            true
        } else {
            false
        }
    }
}

impl<T: PartialOrd + Sized> ChangeMinMax for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chmin() {
        let mut x = 2;
        assert!(x.chmin(1));
        assert_eq!(x, 1);

        assert!(!x.chmin(2));
        assert_eq!(x, 1);
    }

    #[test]
    fn test_chmax() {
        let mut x = 1;
        assert!(x.chmax(2));
        assert_eq!(x, 2);

        assert!(!x.chmax(1));
        assert_eq!(x, 2);
    }
}
