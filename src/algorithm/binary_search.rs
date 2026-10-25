//! 二分探索

use num_traits::{PrimInt, Signed};

/// めぐる式二分探索法。
#[inline]
pub fn binary_search<F, T>(mut ok: T, mut ng: T, mut f: F) -> T
where
    T: PrimInt + Signed,
    F: FnMut(T) -> bool,
{
    let two = T::one() + T::one();

    while (ok - ng).abs() > T::one() {
        let mid = ok + (ng - ok) / two;

        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    ok
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search(0, 10, |x| x < 5), 4);
        assert_eq!(binary_search(0, 10, |x| x <= 5), 5);
    }
}
