//! 累積和

use num_traits::Num;

/// Prefix sum を計算する。
///
/// # 計算量
/// 時間 O(n)
#[inline]
pub fn prefix_sum<T: Num + Copy>(arr: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(arr.len() + 1);
    result.push(T::zero());

    for &val in arr {
        let sum = match result.last().copied() {
            Some(x) => x,
            // 直前に `T::zero()` してる
            None => unreachable!(),
        } + val;
        result.push(sum);
    }

    result
}

/// Suffix sum を計算する。
///
/// # 計算量
/// 時間 O(n)
#[inline]
pub fn suffix_sum<T: Num + Copy>(arr: &[T]) -> Vec<T> {
    let n = arr.len();
    let mut result = vec![T::zero(); n + 1];

    for i in (0..n).rev() {
        result[i] = result[i + 1] + arr[i];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_sum() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(prefix_sum(&arr), vec![0, 1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_suffix_sum() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(suffix_sum(&arr), vec![15, 14, 12, 9, 5, 0]);
    }
}
