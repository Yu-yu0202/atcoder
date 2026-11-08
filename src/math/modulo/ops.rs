//! modulo 演算のユーティリティ
//!
//! 任意の整数型（[`num_traits::PrimInt`] を実装する型）に対して、法 `m` 付きの
//! 加算・乗算・冪乗を提供する。`ModInt` のような型を作らなくても生の整数のまま
//! 剰余計算ができるため、法が問題ごとに変わる場合や 1 回だけ計算したい場合に向く。
//!
//! `m` は正の整数であればよく、素数である必要はない。

use num_traits::PrimInt;

#[inline]
fn norm_mod<T: PrimInt>(a: T, m: T) -> T {
    let r = a % m;
    if r < T::zero() { r + m } else { r }
}

/// 剰余付き加算
///
/// `(a + b) mod m` を計算する。`a` と `b` をあらかじめ `[0, m)` に正規化してから、
/// 和が `m` 以上になったときに `m` を引く（または `a - (m - b)`）ことで循環させる。
/// 和をそのまま足さないため、加算での中間値が飽和・桁溢れしない。
///
/// # Panics
/// - `m` が負または0の場合
///
/// # 計算量
/// 時間 O(1)
///
/// # Examples
/// ```
/// use atcoder::math::modulo::ops::mod_add;
///
/// assert_eq!(mod_add::<u32>(1, 2, 3), 0);
/// assert_eq!(mod_add::<u32>(2, 2, 3), 1);
/// assert_eq!(mod_add::<u32>(3, 2, 3), 2); // 5 mod 3
///
/// // 大きな値を足しても法の範囲で循環する。
/// assert_eq!(mod_add::<u64>(u64::MAX, 1, 10), 6); // 5 + 1
/// assert_eq!(mod_add::<i64>(-7, -1, 5), 2); // -8 mod 5
/// ```
#[inline]
#[must_use]
#[track_caller]
pub fn mod_add<T: PrimInt>(a: T, b: T, m: T) -> T {
    assert!(m > T::zero(), "modulus must be positive");

    let a = norm_mod(a, m);
    let b = norm_mod(b, m);

    add_mod_reduced(a, b, m)
}

/// 剰余付き乗算
///
/// `(a * b) mod m` を、ロシアン農夫（バイナリ）法で計算する。`a` と `b` を `[0, m)` に
/// 正規化してから、`b` の各ビットを下から見て、ビットが立っていれば `ans` に `a` を
/// 加算し、`a` を倍々にしていく。積をそのまま取らず `+`・`&`・`>>` だけで構成するため、
/// `u64` 同士の積すら桁溢れしうるような大きな型（`u128` など）でも安全に使える。
///
/// `b` が 0 なら 0 を返す。
///
/// # Panics
/// - `m` が負または0の場合
///
/// # 計算量
/// 時間 O(log b)
///
/// # Examples
/// ```
/// use atcoder::math::modulo::ops::mod_mul;
///
/// assert_eq!(mod_mul::<u32>(2, 2, 3), 1);
/// assert_eq!(mod_mul::<u32>(3, 2, 3), 0);
/// assert_eq!(mod_mul::<u32>(1, 0, 3), 0);
///
/// // u64 の積が桁溢れする値でも正しく求められる。
/// assert_eq!(mod_mul::<u64>(u64::MAX, u64::MAX, 1_000_000_007), 114944269);
/// assert_eq!(mod_mul::<u128>(u128::MAX, u128::MAX, 1_000_000_007), 233580713);
/// ```
#[inline]
#[must_use]
#[track_caller]
pub fn mod_mul<T: PrimInt>(a: T, b: T, m: T) -> T {
    assert!(m > T::zero(), "modulus must be positive");

    let mut a = norm_mod(a, m);
    let mut b = norm_mod(b, m);
    let mut ans = T::zero();
    let one = T::one();

    while b > T::zero() {
        if (b & one) == one {
            ans = add_mod_reduced(ans, a, m);
        }

        a = add_mod_reduced(a, a, m);
        b = b >> 1usize;
    }

    ans
}

#[inline]
fn add_mod_reduced<T: PrimInt>(a: T, b: T, m: T) -> T {
    if a >= m - b { a - (m - b) } else { a + b }
}

/// 剰余付き冪乗
///
/// `a^b mod m` を二乗法で計算する。`b` の各ビットを下から見て、ビットが立っていれば
/// `ans` に `a` を掛け、`a` を二乗していく。途中の積はすべて [`mod_mul`] で
/// `mod m` を取りながら進めるため、大きくても桁溢れしない。
///
/// `m` は正の整数であればよく、素数である必要はない。`b` が 0 なら `1 mod m`
/// （`m = 1` のときは 0）を返す。
///
/// # Panics
/// - `m` が負または0の場合
/// - `b` が負の場合
///
/// # 計算量
/// 時間 O(log b)
///
/// # Examples
/// ```
/// use atcoder::math::modulo::ops::{mod_mul, mod_pow};
///
/// assert_eq!(mod_pow::<u64>(2, 10, 1_000), 24); // 1024 mod 1000
/// assert_eq!(mod_pow::<u64>(3, 0, 7), 1);
/// assert_eq!(mod_pow::<u64>(2, 100, 1_000_000_007), 976371285);
///
/// // a^b * a^c = a^(b+c) が成り立つ。
/// let m = 1_000_000_007_u64;
/// assert_eq!(
///     mod_mul(mod_pow(3_u64, 10, m), mod_pow(3_u64, 20, m), m),
///     mod_pow(3_u64, 30, m),
/// );
/// ```
#[inline]
#[must_use]
#[track_caller]
pub fn mod_pow<T: PrimInt>(mut a: T, mut b: T, m: T) -> T {
    assert!(m > T::zero(), "modulus must be positive");
    assert!(b >= T::zero(), "exponent must be non-negative");

    a = norm_mod(a, m);

    let one = T::one();
    let mut ans = one % m;

    while b > T::zero() {
        if (b & one) == one {
            ans = mod_mul(ans, a, m);
        }

        a = mod_mul(a, a, m);
        b = b >> 1usize;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_add() {
        assert_eq!(mod_add::<u32>(0, 0, 3), 0_u32);
        assert_eq!(mod_add::<u32>(1, 2, 3), 0_u32);
        assert_eq!(mod_add::<u32>(2, 2, 3), 1_u32);
        assert_eq!(mod_add::<u32>(3, 2, 3), 2_u32);
    }

    #[test]
    fn test_mod_mul() {
        assert_eq!(mod_mul::<u32>(1, 0, 3), 0_u32);
        assert_eq!(mod_mul::<u32>(0, 1, 3), 0_u32);
        assert_eq!(mod_mul::<u32>(1, 2, 3), 2_u32);
        assert_eq!(mod_mul::<u32>(2, 2, 3), 1_u32);
        assert_eq!(mod_mul::<u32>(3, 2, 3), 0_u32);
    }

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow::<u32>(0, 0, 3), 1_u32);
        assert_eq!(mod_pow::<u32>(1, 2, 3), 1_u32);
        assert_eq!(mod_pow::<u32>(2, 2, 3), 1_u32);
        assert_eq!(mod_pow::<u32>(3, 2, 3), 0_u32);
    }
}
