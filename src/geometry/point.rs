//! 整数点
//!
//! 2 次元平面上の座標を [`i64`] の組として保持する [`Point`] を提供する。
//! 点どうしの差はベクトル（`p - q` で [`Vector`]）、点とベクトルの和・差は再び点
//! （`p + v` / `p - v` で [`Point`]）になる。
//! 点と点の距離を誤差なく比較したい場合は [`Point::euclid_dist_sq`]（平方根を取らない）を使う。

use crate::geometry::vector::Vector;
use std::ops::{Add, Sub};

/// 2 次元上の整数点
///
/// `x` 座標・`y` 座標を [`i64`] で保持するタプル構造体で、`p.0` / `p.1` として直接座標を読み出すこともできる。
/// 座標系は右手系（`y` 軸が上向き）を仮定する。
/// 点と点の差（[`Point`] - [`Point`]）は [`Vector`] になり、距離や位置関係はそのベクトルの内積・外積で計算する。
///
/// # Examples
/// ```
/// use atcoder::geometry::point::Point;
///
/// let p = Point::new(1, 2);
/// assert_eq!((p.0, p.1), (1, 2));
/// assert_eq!((p.x(), p.y()), (1, 2));
/// assert_eq!(p.euclid_dist_sq(&Point::new(1, 2)), 0);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point(
    /// x 座標
    pub i64,
    /// y 座標
    pub i64,
);

impl Point {
    /// `x` 座標・`y` 座標を指定して点を作成する。
    ///
    /// 値の検査は行わず、任意の [`i64`] の値をそのまま保持する。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::point::Point;
    ///
    /// let p = Point::new(-1, 2);
    /// assert_eq!(p, Point::new(-1, 2));
    /// assert_eq!(p.x(), -1);
    /// ```
    #[must_use]
    pub const fn new(x: i64, y: i64) -> Self {
        Self(x, y)
    }

    /// `x` 座標（横方向の座標）を返す。
    #[inline(always)]
    #[must_use]
    pub const fn x(&self) -> i64 {
        self.0
    }

    /// `y` 座標（縦方向の座標）を返す。
    #[inline(always)]
    #[must_use]
    pub const fn y(&self) -> i64 {
        self.1
    }

    /// 2 点間のユークリッド距離の 2 乗を求める。
    ///
    /// 平方根を取らずに整数のまま比較できるため、距離の大小比較（最も近い点を求める等）を誤差なしで行える。
    /// 座標の差は [`i64`] で求めてから 2 乗し、その和を [`i128`] で保持する。
    ///
    /// # 備考
    /// - 座標差は [`i64`] の減算で計算するため、座標の絶対値が極端に大きい場合はデバッグビルドでオーバーフローしうる。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::point::Point;
    ///
    /// let p = Point::new(1, 2);
    /// let q = Point::new(4, 6);
    /// assert_eq!(p.euclid_dist_sq(&q), 25); // 3^2 + 4^2
    /// assert_eq!(p.euclid_dist_sq(&p), 0);
    /// ```
    #[must_use]
    pub const fn euclid_dist_sq(&self, rhs: &Self) -> i128 {
        (self.x() - rhs.x()) as i128 * (self.x() - rhs.x()) as i128
            + (self.y() - rhs.y()) as i128 * (self.y() - rhs.y()) as i128
    }

    /// 2 点間のマンハッタン距離（`|dx| + |dy|`）を求める。
    ///
    /// 上下左右の 4 方向にしか移動できない場合の最短距離。座標の差は [`i64`] で求めてから絶対値を取り、[`i128`] で保持する。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::point::Point;
    ///
    /// let p = Point::new(1, 2);
    /// let q = Point::new(-2, 3);
    /// assert_eq!(p.manhattan_dist(&q), 4); // |1 - (-2)| + |2 - 3|
    /// assert_eq!(p.manhattan_dist(&p), 0);
    /// ```
    #[must_use]
    pub const fn manhattan_dist(&self, rhs: &Self) -> i128 {
        (self.x() as i128 - rhs.x() as i128).abs() + (self.y() as i128 - rhs.y() as i128).abs()
    }
}

impl Sub<Self> for Point {
    type Output = Vector;

    /// 2 点の差からベクトルを求める。`self - rhs` は `rhs` から `self` へ向かうベクトル。
    ///
    /// 点の差を引くとベクトルが得られ、逆向きの演算として [`Point`] + [`Vector`] は点になる。
    /// 得られたベクトルに対して [`Vector::norm_sq`] や幾何述語（[`super::predicates::orientation`]）を使うことで、
    /// 距離や位置関係を誤差のない整数演算で計算できる。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::point::Point;
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let p = Point::new(4, 6);
    /// let q = Point::new(1, 2);
    /// let v = p - q;
    /// assert_eq!(v, Vector::new(3, 4));
    /// assert_eq!(v.norm_sq(), 25);
    /// assert_eq!(q + v, p);
    /// ```
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    /// 点 `self` をベクトル `rhs` だけ平行移動させた点を求める。
    ///
    /// 点にベクトルを足すと点になり、[`Point`] - [`Vector`] で逆向きの移動ができる。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::point::Point;
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let p = Point::new(1, 2);
    /// assert_eq!(p + Vector::new(3, 4), Point::new(4, 6));
    /// ```
    #[inline(always)]
    fn add(self, rhs: Vector) -> Self::Output {
        Self(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    /// 点 `self` をベクトル `rhs` の逆向きに平行移動させた点を求める。
    ///
    /// 点にベクトルを足すと点になり、[`Point`] + [`Vector`] で元の向きの移動ができる。
    ///
    /// # Examples
    /// ```
    /// use atcoder::geometry::point::Point;
    /// use atcoder::geometry::vector::Vector;
    ///
    /// let p = Point::new(1, 2);
    /// assert_eq!(p - Vector::new(3, 4), Point::new(-2, -2));
    /// ```
    #[inline(always)]
    fn sub(self, rhs: Vector) -> Self::Output {
        Self(self.x() - rhs.x(), self.y() - rhs.y())
    }
}
