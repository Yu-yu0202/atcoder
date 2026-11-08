//! 座標圧縮
//!
//! 値を相対的な順位（rank）へ変換する [`coordinate_compression`] を提供する。

use crate::ext::slice_ext::SliceExt;

/// 座標圧縮を行う。
///
/// 各要素を「何番目に小さい値か」を表す rank（0 以上の整数）に変換する。
/// 同じ値は必ず同じ rank になり、小さい値ほど小さい rank が割り当てられる
/// （最小の値が rank 0）。返り値は元の配列と同じ長さ・同じ順序で、
/// `result[i]` は `arr[i]` の rank を表す。
///
/// # 計算量
/// 時間 O(n log n)（`n = arr.len()`、ソートが律速）
///
/// # Examples
/// ```
/// use atcoder::algorithm::coordinate_compression::coordinate_compression;
///
/// let arr = [50, 10, 50, 30];
/// // 10 -> 0, 30 -> 1, 50 -> 2
/// assert_eq!(coordinate_compression(&arr), vec![2, 0, 2, 1]);
/// ```
pub fn coordinate_compression<T: Ord>(arr: &[T]) -> Vec<usize> {
    let order = arr.argsort();
    let mut result = vec![0; arr.len()];
    let mut rank = 0;

    for (i, &idx) in order.iter().enumerate() {
        if i > 0 && arr[order[i - 1]] != arr[idx] {
            rank += 1;
        }

        result[idx] = rank;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coodinate_compression() {
        let arr = [50, 10, 50, 30];
        assert_eq!(coordinate_compression(&arr), vec![2, 0, 2, 1]);
    }
}
