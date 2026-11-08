//! GCD / LCM（最大公約数・最小公約数）関係。
//!
//! - [`gcd`] — ユークリッドの互除法による最大公約数
//! - [`lcm`] — [`gcd`] を利用した最小公倍数
//! - [`ext_gcd`] — 最大公約数と同時に、ベズーの等式 `a * x + b * y = gcd(a, b)`
//!   を満たす係数 `(x, y)` も求める
//!
//! 3 つとも [`num_traits::PrimInt`] を実装する任意の整数型（符号付き・符号なし、
//! ビット幅も問わない）に対して使える。
//!
//! 符号付き型を渡した場合、[`PrimInt`] の `%` が被除数の符号を保つため
//! [`gcd`] の戻り値の符号は一意に決まらない（例: `gcd(12, -18) == -6`）。
//! 正の最大公約数が必要なときは `gcd(a.abs(), b.abs())` とするか、
//! 戻り値に `.abs()` を適用する。
//!
//! # Examples
//! ```
//! use atcoder::math::gcd_lcm::{ext_gcd, gcd, lcm};
//!
//! assert_eq!(gcd(12, 18), 6);
//! assert_eq!(lcm(4, 6), 12);
//!
//! // ax + by = gcd(a, b) を満たす係数が得られる。
//! let (g, x, y) = ext_gcd(30, 18);
//! assert_eq!(g, 6);
//! assert_eq!(30 * x + 18 * y, 6);
//! ```

use num_traits::PrimInt;

/// GCD（最大公約数）をユークリッドの互除法で求める。
///
/// `a`・`b` の少なくとも一方が 0 の場合は、もう一方の値をそのまま返す
/// （`gcd(a, 0) == a`・`gcd(0, b) == b`）。両方 0 のときは 0 を返す。
///
/// 符号付き型への入力では、途中の剰余が被除数の符号を保つため結果の符号は
/// 入力の符号から決まらない（`gcd(12, -18) == -6`・`gcd(-12, 18) == 6`）。
/// 絶対値の最大公約数が欲しい場合は引数か戻り値に `.abs()` を付ける。
///
/// # 計算量
/// 時間 O(log(min(a, b)))、空間 O(1)
///
/// # Examples
/// ```
/// use atcoder::math::gcd_lcm::gcd;
///
/// assert_eq!(gcd(12, 18), 6);
/// assert_eq!(gcd(18, 12), 6);
///
/// // 片方が 0 ならもう片方。
/// assert_eq!(gcd(12, 0), 12);
/// assert_eq!(gcd(0, 12), 12);
/// assert_eq!(gcd(0, 0), 0);
///
/// // 型は PrimInt を実装するものなら何でもよい。
/// assert_eq!(gcd(1_u64 << 40, 1_u64 << 30), 1 << 30);
/// ```
pub fn gcd<T: PrimInt>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// LCM（最小公倍数）を [`gcd`] を利用して求める。
///
/// `lcm(a, b) = a / gcd(a, b) * b` と計算する。`a`・`b` の少なくとも一方が
/// 0 の場合は必ず 0 を返す（`lcm(a, 0) == 0`）。ただし両方が 0 の場合は
/// `gcd(0, 0) == 0` による 0 除算で panic する。
///
/// 戻り値の絶対値は数学的な最小公倍数と一致するが、[`gcd`] と同じく符号は
/// 保証されない。結果が `T` に収まらない場合のオーバーフローチェックは
/// 行われないため、制約が大きい計算では `u128` などに広げてから使う。
///
/// # Panics
/// - `a`・`b` がともに 0 の場合（`0 / 0` のゼロ除算）。
/// - 演算結果や途中の `a / gcd(a, b) * b` が `T` に収まらない場合（デバッグビルド）。
///   リリースビルドではパニックせず桁あふれた値になる。
///
/// # 計算量
/// 時間 O(log(min(a, b)))、空間 O(1)
///
/// # Examples
/// ```
/// use atcoder::math::gcd_lcm::lcm;
///
/// assert_eq!(lcm(12, 18), 36);
/// assert_eq!(lcm(18, 12), 36);
///
/// // 片方が 0 なら 0。
/// assert_eq!(lcm(12, 0), 0);
///
/// // 2 数周期のそろう周期（足し合わせて割り切れる最小の周期）を求める場面などに使う。
/// assert_eq!(lcm(4, 6), 12);
/// ```
///
/// 両方 0 の場合は panic する。
///
/// ```should_panic
/// use atcoder::math::gcd_lcm::lcm;
///
/// // `gcd(0, 0) == 0` なので `0 / 0` になりゼロ除算。
/// let _ = lcm(0, 0);
/// ```
pub fn lcm<T: PrimInt>(a: T, b: T) -> T {
    (a / gcd(a, b)) * b
}

/// 拡張 GCD。`a`・`b` の最大公約数と、ベズーの等式 `a * x + b * y = gcd(a, b)`
/// を満たす係数 `x`・`y` を求める。
///
/// 返り値は `(g, x, y)` のタプルで、`g = gcd(a, b)`・`a * x + b * y == g` を
/// 常に満たす。`b == 0` のときは `(a, 1, 0)` を返し（`a * 1 + 0 * 0 == a`）、
/// これは [`gcd`] の `gcd(a, 0) == a` と一致する。
///
/// 典型用途は、合同式 `a * x == c (mod m)` を解く場面である。`gcd(a, m)` が `c`
/// を割り切る場合に限り解が存在し、その 1 つがこの `x` から `(c / g) * x (mod m)`
/// として得られる。また `m` が素数で `gcd(a, m) == 1` のとき、`x (mod m)` が
/// `a` の逆元になる。符号付き型では `g` の符号も [`gcd`] と同じ規則になる。
///
/// # 計算量
/// 時間 O(log(min(a, b)))、空間 O(log(min(a, b)))（再帰のスタック分）
///
/// # Examples
/// ```
/// use atcoder::math::gcd_lcm::ext_gcd;
///
/// // ax + by = gcd(a, b) を満たす係数。
/// let (g, x, y) = ext_gcd(30, 18);
/// assert_eq!(g, 6);
/// assert_eq!(30 * x + 18 * y, 6);
///
/// // 素数 998244353 を法とする 3 の逆元。
/// let (g, x, y) = ext_gcd(3_i64, 998_244_353);
/// assert_eq!(g, 1);
/// assert_eq!(3 * x + 998_244_353 * y, 1);
/// assert_eq!((x.rem_euclid(998_244_353) * 3) % 998_244_353, 1);
/// ```
pub fn ext_gcd<T: PrimInt>(a: T, b: T) -> (T, T, T) {
    if b == T::zero() {
        return (a, T::one(), T::zero());
    }

    let (g, x, y) = ext_gcd(b, a % b);
    (g, y, x - (a / b) * y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(18, 12), 6);
        assert_eq!(gcd(12, 0), 12);
        assert_eq!(gcd(0, 12), 12);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(1, 0), 1);
        assert_eq!(gcd(0, 1), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(18, 12), 36);
        assert_eq!(lcm(12, 0), 0);
        assert_eq!(lcm(0, 12), 0);
        assert_eq!(lcm(1, 1), 1);
        assert_eq!(lcm(1, 0), 0);
        assert_eq!(lcm(0, 1), 0);
    }

    #[test]
    fn test_ext_gcd() {
        const CASES: [(i32, i32, i32); 11] = [
            (30, 18, 6),
            (35, 15, 5),
            (17, 12, 1),
            (101, 10, 1),
            (240, 46, 2),
            (0, 5, 5),
            (7, 0, 7),
            (12, 8, 4),
            (99, 78, 3),
            (252, 105, 21),
            (391, 299, 23),
        ];

        for (a, b, expected_g) in CASES {
            let (g, x, y) = ext_gcd(a, b);

            assert_eq!(g, expected_g);
            assert_eq!(a * x + b * y, g);
        }
    }
}
