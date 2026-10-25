//! 座標圧縮

use crate::ext::slice_ext::SliceExt;

/// 座標圧縮を行う。
///
/// # 計算量
/// 時間 O(n log n)
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
