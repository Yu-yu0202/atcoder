//! 二分探索
//!
//! [`binary_search`] は「めぐる式二分探索」を提供する。
//! 境界のどちら側に解があるかを判定する述語 `f` の単調性を利用し、解の候補を高速に絞り込む。

use num_traits::{PrimInt, Signed};

/// めぐる式二分探索法。
///
/// 述語 `f` に対して `f(ok)` が `true`・`f(ng)` が `false` であることを仮定し、
/// その間で `f(x)` の真偽が単調になる（`true` 側から `false` 側へ 1 回だけ切り替わる）
/// 範囲を二分探索して、`f` を `true` にする最大の `x`（解の候補）を返す。
/// 典型的には「ある条件を満たす最大値」を求めたいときに使う。
///
/// `ok`・`ng` は「現在の答え候補」と「明らかに答えでない値」を表し、`f` 自体は
/// この 2 点に対しては呼ばれない。ループは両者の差が 1 になるまで続き、
/// 最後に `ok` を返す。`f` の真偽が単調でない場合、戻り値に意味は保証されない。
///
/// # 計算量
/// 時間 O(log |ok - ng|)。`f` の呼び出し回数もこのオーダーである。
///
/// # Examples
/// ```
/// use atcoder::algorithm::binary_search::binary_search;
///
/// // x < 5 を満たす最大の x（[0, 10) で探索）
/// assert_eq!(binary_search(0, 10, |x| x < 5), 4);
/// // x <= 5 を満たす最大の x
/// assert_eq!(binary_search(0, 10, |x| x <= 5), 5);
/// ```
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
