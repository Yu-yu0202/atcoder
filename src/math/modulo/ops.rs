//! modulo 演算のユーティリティ

use num_traits::PrimInt;

#[inline]
fn norm_mod<T: PrimInt>(a: T, m: T) -> T {
    let r = a % m;
    if r < T::zero() { r + m } else { r }
}

/// 剰余付き加算
///
/// # Panics
/// - `m` が負または0の場合
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
/// # Panics
/// - `m` が負または0の場合
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
/// # Panics
/// - `m` が負または0の場合
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
