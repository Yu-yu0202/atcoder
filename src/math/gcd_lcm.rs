//! GCD / LCM 関係

use num_traits::PrimInt;

/// GCD（最大公約数）を求める
pub fn gcd<T: PrimInt>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// LCM（最小公倍数）を求める
pub fn lcm<T: PrimInt>(a: T, b: T) -> T {
    (a / gcd(a, b)) * b
}

/// 拡張 GCD。GCD（最大公約数）と、そのときの a, b の係数を求める
///
/// ax + by = gcd(a, b) を満たす x, y を求めるときなどに利用する
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
