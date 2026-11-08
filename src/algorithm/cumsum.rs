//! 累積和
//!
//! 配列の区間和を O(1) で求めるための前置計算を提供する。
//! [`prefix_sum`]・[`suffix_sum`] はどちらも元の配列より 1 要素長い
//! （`arr.len() + 1` 長の）配列を返し、先頭（[`prefix_sum`]）または
//! 末尾（[`suffix_sum`]）に 0 を置く。

use num_traits::Num;

/// 先頭からの累積和（prefix sum）を計算する。
///
/// 長さ `arr.len() + 1` の配列を返す。先頭要素 `sum[0]` は 0 であり、
/// `sum[i]` は元の配列の先頭 `i` 要素（`arr[0..i]`）の和を表す。
/// 半開区間 `[i, j)` の和は `sum[j] - sum[i]` で O(1) に求まる。
/// 空区間 `[i, i)` の和は 0、配列全体の和は `sum[arr.len()]` となる。
///
/// # 計算量
/// 時間 O(n), 空間 O(n)（`n = arr.len()`）
///
/// # Examples
/// ```
/// use atcoder::algorithm::cumsum::prefix_sum;
///
/// let arr = vec![1, 2, 3, 4, 5];
/// let sum = prefix_sum(&arr);
///
/// assert_eq!(sum, vec![0, 1, 3, 6, 10, 15]);
/// // 半開区間 [1, 4) の和は 2 + 3 + 4 = 9
/// assert_eq!(sum[4] - sum[1], 9);
/// ```
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

/// 末尾からの累積和（suffix sum）を計算する。
///
/// 長さ `arr.len() + 1` の配列を返す。末尾要素 `sum[arr.len()]` は 0 であり、
/// `sum[i]` は元の配列の `arr[i..]`（`i` から末尾まで）の和を表す。
/// 半開区間 `[i, j)` の和は `sum[i] - sum[j]` で O(1) に求まる。
/// 空区間 `[i, i)` の和は 0、配列全体の和は `sum[0]` となる。
///
/// # 計算量
/// 時間 O(n), 空間 O(n)（`n = arr.len()`）
///
/// # Examples
/// ```
/// use atcoder::algorithm::cumsum::suffix_sum;
///
/// let arr = vec![1, 2, 3, 4, 5];
/// let sum = suffix_sum(&arr);
///
/// assert_eq!(sum, vec![15, 14, 12, 9, 5, 0]);
/// // 半開区間 [1, 4) の和は 2 + 3 + 4 = 9
/// assert_eq!(sum[1] - sum[4], 9);
/// ```
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
