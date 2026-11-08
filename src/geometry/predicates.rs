//! 幾何述語など
//!
//! 3 点の位置関係を調べる [`orientation`] と、点が線分上にあるかを調べる [`on_segment`] を提供する。
//! 座標系は右手系・`y` 軸上向きを仮定し、反時計回り（counter-clockwise）を正の向きとして扱う。
//! どちらも [`Point`] の差から得られるベクトルの外積（[`super::vector::Vector::cross`]）で判定するため、
//! 平方根を使わず誤差のない整数演算だけで済む。

use crate::geometry::point::Point;
use std::cmp::{Ordering, max, min};

/// 3 点 `p`, `q`, `r` の位置関係を、反時計回り: [`Ordering::Less`]、時計回り: [`Ordering::Greater`]、共線: [`Ordering::Equal`] で表す。
///
/// `p` → `q` → `r` と進む向きを見る。座標系は右手系で `y` 軸が上向きであることを仮定し、
/// この向きを反時計回りとして扱う。3 点が一直線上に並ぶ場合は共線（[`Ordering::Equal`]）となる。
/// 内部では `q - p` と `r - p` の外積（[`super::vector::Vector::cross`]）の符号を比較している。
///
/// # Examples
/// ```
/// use atcoder::geometry::point::Point;
/// use atcoder::geometry::predicates::orientation;
/// use std::cmp::Ordering;
///
/// let p = Point::new(0, 0);
///
/// assert_eq!(orientation(p, Point::new(1, 1), Point::new(0, 1)), Ordering::Less); // 反時計回り
/// assert_eq!(orientation(p, Point::new(1, 1), Point::new(1, 0)), Ordering::Greater); // 時計回り
/// assert_eq!(orientation(p, Point::new(2, 2), Point::new(3, 3)), Ordering::Equal); // 共線
/// ```
#[must_use]
pub fn orientation(p: Point, q: Point, r: Point) -> Ordering {
    (q - p).cross(&(r - p)).cmp(&0)
}

/// 線分 `ab` 上に点 `p` が存在するかを判定する。
///
/// 端点 `a`・`b` も線分の一部とみなす（境界内）。共線判定（[`orientation`]）だけでは、
/// 線分を挟んで反対側に延びた直線上の点も通ってしまうため、
/// さらに `p` の座標が `a` と `b` の座標の範囲（バウンディングボックス）に収まっているかを確かめる。
///
/// # Examples
/// ```
/// use atcoder::geometry::point::Point;
/// use atcoder::geometry::predicates::on_segment;
///
/// let a = Point::new(0, 0);
/// let b = Point::new(4, 4);
///
/// assert!(on_segment(a, b, Point::new(2, 2)));
/// assert!(on_segment(a, b, a)); // 端点も含む
/// assert!(on_segment(a, b, b));
/// assert!(!on_segment(a, b, Point::new(1, 0)));
/// assert!(!on_segment(a, b, Point::new(5, 5))); // 線分の延長線上
/// ```
#[must_use]
pub fn on_segment(a: Point, b: Point, p: Point) -> bool {
    orientation(a, b, p) == Ordering::Equal
        && min(a.x(), b.x()) <= p.x()
        && p.x() <= max(a.x(), b.x())
        && min(a.y(), b.y()) <= p.y()
        && p.y() <= max(a.y(), b.y())
}
