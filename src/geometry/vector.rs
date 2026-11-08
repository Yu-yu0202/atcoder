//! 整数ベクトル
//!
//! 2 次元平面上のベクトルを [`i64`] 成分の組として保持する [`Vector`] を提供する。
//! 成分どうしの積（内積・外積）は [`i64`] の範囲ではオーバーフローしうるため、
//! [`Vector::dot`] / [`Vector::cross`] / [`Vector::norm_sq`] は結果を [`i128`] で返す。
//! 加減算とスカラー倍は [`i64`] のままなので、そちらは符号付きオーバーフローに注意する。

use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

/// 2 次元上の整数ベクトル
///
/// `x` 成分・`y` 成分を [`i64`] で保持するタプル構造体で、`v.0` / `v.1` として直接成分を読み出すこともできる。
/// 通常は [`Vector::new`] で組み立てるか、[`super::point::Point`] どうしの差（`p - q`）として得る。
/// 座標系は右手系（`y` 軸が上向き）を仮定し、反時計回りを正の向きとして扱う。
///
/// # Examples
/// ```
/// use atcoder::geometry::vector::Vector;
///
/// let v = Vector::new(3, 4);
/// assert_eq!((v.0, v.1), (3, 4));
/// assert_eq!((v.x(), v.y()), (3, 4));
/// assert_eq!(v.norm_sq(), 25);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Vector(
    /// x 成分
    pub i64,
    /// y 成分
    pub i64,
);

impl Vector {
    /// `x` 成分・`y` 成分を指定してベクトルを作成する。
    ///
    /// 値の検査は行わず、任意の [`i64`] の値をそのまま保持する。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let v = Vector::new(-3, 4);
    /// assert_eq!(v, Vector::new(-3, 4));
    /// assert_eq!(v.x(), -3);
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn new(x: i64, y: i64) -> Self {
        Self(x, y)
    }

    /// `x` 成分（横方向の成分）を返す。
    #[inline(always)]
    #[must_use]
    pub const fn x(&self) -> i64 {
        self.0
    }

    /// `y` 成分（縦方向の成分）を返す。
    #[inline(always)]
    #[must_use]
    pub const fn y(&self) -> i64 {
        self.1
    }

    /// ベクトル `self` と `rhs` の内積を求める。
    ///
    /// 幾何学的には 2 つのベクトルが「どれだけ同じ向きを向いているか」を表し、値が 0 なら 2 つは直交する
    /// （[`Vector::is_orthogonal`] 参照）。
    /// 成分ごとの積の和を [`i128`] で計算するため、[`i64`] どうしをかける際のオーバーフローを避けられる。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let a = Vector::new(1, 2);
    /// let b = Vector::new(3, 4);
    /// assert_eq!(a.dot(&b), 11);
    ///
    /// let c = Vector::new(-2, 1);
    /// assert_eq!(a.dot(&c), 0);
    /// ```
    #[must_use]
    pub const fn dot(&self, rhs: &Self) -> i128 {
        (self.x() as i128 * rhs.x() as i128) + (self.y() as i128 * rhs.y() as i128)
    }

    /// ベクトル `self` と `rhs` の 2 次元外積を求める。
    ///
    /// 符号が回転方向を表し、正なら `rhs` は `self` から反時計回りに進んだ向き、負なら時計回り、0 なら 2 つは平行
    /// （[`Vector::is_parallel`] 参照）。
    /// 絶対値は 2 つのベクトルが張る平行四辺形の面積（符号付き）に相当する。
    /// 成分ごとの積を [`i128`] で計算するため、[`i64`] どうしをかける際のオーバーフローを避けられる。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let a = Vector::new(1, 0);
    /// let b = Vector::new(1, 1);
    /// assert_eq!(a.cross(&b), 1); // 反時計回り
    ///
    /// let c = Vector::new(0, 1);
    /// assert_eq!(c.cross(&b), -1); // 時計回り
    /// ```
    #[must_use]
    pub const fn cross(&self, rhs: &Self) -> i128 {
        (self.x() as i128 * rhs.x() as i128) - (self.y() as i128 * rhs.y() as i128)
    }

    /// ベクトルの長さの 2 乗を求める。
    ///
    /// 平方根を取らないため誤差のない整数のまま長さの比較ができ、[`super::point::Point::euclid_dist_sq`] と同じく
    /// 距離の大小比較に使う。成分ごとの積を [`i128`] で計算するため、[`i64`] どうしをかける際のオーバーフローを避けられる。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let v = Vector::new(3, 4);
    /// assert_eq!(v.norm_sq(), 25);
    /// assert!(Vector::new(3, 4).norm_sq() > Vector::new(3, 3).norm_sq());
    /// ```
    #[must_use]
    pub const fn norm_sq(&self) -> i128 {
        self.x() as i128 * self.x() as i128 + self.y() as i128 * self.y() as i128
    }

    /// ベクトル `self` と `rhs` が平行かどうかを判定する。
    ///
    /// 外積（[`Vector::cross`]）が 0 かどうかで判定する。逆方向を向くベクトルも平行とみなし、
    /// 零ベクトルは任意のベクトルと平行と判定される。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let a = Vector::new(1, 1);
    /// assert!(a.is_parallel(&Vector::new(3, 3)));
    /// assert!(a.is_parallel(&Vector::new(-2, -2)));
    /// assert!(!a.is_parallel(&Vector::new(1, 2)));
    /// assert!(Vector::new(0, 0).is_parallel(&Vector::new(3, 3)));
    /// ```
    #[must_use]
    pub const fn is_parallel(&self, rhs: &Self) -> bool {
        self.cross(rhs) == 0
    }

    /// ベクトル `self` と `rhs` が直交かどうかを判定する。
    ///
    /// 内積（[`Vector::dot`]）が 0 かどうかで判定する。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let a = Vector::new(1, 2);
    /// assert!(a.is_orthogonal(&Vector::new(-2, 1)));
    /// assert!(!a.is_orthogonal(&Vector::new(2, 1)));
    /// ```
    #[must_use]
    pub const fn is_orthogonal(&self, rhs: &Self) -> bool {
        self.dot(rhs) == 0
    }

    /// ベクトル `self` を反時計回りに 90 度回転させたベクトルを求める。
    ///
    /// `(x, y)` は `(-y, x)` に写される。回転後も長さは変わらず、元のベクトルとは直交する
    /// （[`Vector::is_orthogonal`] 参照）。時計回りに 90 度回転したい場合は符号を反転する。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let a = Vector::new(3, 4);
    /// let r = a.rotate90();
    /// assert_eq!(r, Vector::new(-4, 3));
    /// assert_eq!(r.norm_sq(), a.norm_sq());
    /// assert!(a.is_orthogonal(&r));
    /// ```
    #[must_use]
    pub const fn rotate90(self) -> Self {
        Self(-self.y(), self.x())
    }
}

impl Neg for Vector {
    type Output = Self;

    /// ベクトルの向きを反転し、各成分の符号を反転したベクトルを求める。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let v = -Vector::new(3, -4);
    /// assert_eq!(v, Vector::new(-3, 4));
    /// ```
    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl Add<Self> for Vector {
    type Output = Self;

    /// ベクトルを合成し、成分ごとの和を求める。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let a = Vector::new(1, 2) + Vector::new(3, 4);
    /// assert_eq!(a, Vector::new(4, 6));
    /// ```
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Self> for Vector {
    /// 他のベクトルを足し込み、成分ごとの和で `self` を上書きする。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let mut a = Vector::new(1, 2);
    /// a += Vector::new(3, 4);
    /// assert_eq!(a, Vector::new(4, 6));
    /// ```
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub<Self> for Vector {
    type Output = Self;

    /// ベクトルの差を求め、成分ごとの差を返す。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let a = Vector::new(5, 6) - Vector::new(3, 4);
    /// assert_eq!(a, Vector::new(2, 2));
    /// ```
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign<Self> for Vector {
    /// 他のベクトルを引き、成分ごとの差で `self` を上書きする。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let mut a = Vector::new(5, 6);
    /// a -= Vector::new(3, 4);
    /// assert_eq!(a, Vector::new(2, 2));
    /// ```
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Mul<i64> for Vector {
    type Output = Self;

    /// ベクトルをスカラー倍し、各成分を `rhs` 倍したベクトルを求める。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let a = Vector::new(1, -2) * 3;
    /// assert_eq!(a, Vector::new(3, -6));
    /// ```
    #[inline(always)]
    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<Vector> for i64 {
    type Output = Vector;

    /// ベクトルをスカラー倍し、各成分を `self` 倍したベクトルを求める。
    ///
    /// `Mul<i64> for Vector` の実装と同等（スカラー倍なので左右対称）。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let a = 3 * Vector::new(1, -2);
    /// assert_eq!(a, Vector::new(3, -6));
    /// assert_eq!(3 * Vector::new(1, -2), Vector::new(1, -2) * 3);
    /// ```
    #[inline(always)]
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector(rhs.0 * self, rhs.1 * self)
    }
}
