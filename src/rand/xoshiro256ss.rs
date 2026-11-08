//! `Xoshiro256StarStar` のアルゴリズム
//!
//! Blackman & Vigna の xoshiro256** は、256bit（`u64` 4 個）の状態を進める
//! linear PRNG の一種である。状態の撹拌には xorshift と rotate のみを使うため
//! 命令数が少なく、速度と乱数品質のバランスが取れている。BigCrush などの
//! statistical test を通過することが知られているため、本番の Random 用途に使える。
//!
//! 状態の初期化は [`SplitMix64`] に seed を渡して 4 回出力させることで行う
//! （state を直接 seed で埋めると、初期状態が偏ってしまうため）。
//!
//! [`RandBackend`] を実装しているので、普通は
//! [`Rand<Xoshiro256StarStar>`](crate::rand::Rand) で包んで使う。

use crate::rand::{RandBackend, splitmix64::SplitMix64};

/// `Xoshiro256StarStar` 型のバックエンド。
///
/// 4 個の `u64`（計 256bit）を状態として持つ xoshiro256** PRNG。1 回
/// [`next_u64`](RandBackend::next_u64) を呼ぶたびに状態を 1 ステップ進め、
/// xorshift で状態を混ぜてから乗算と回転で出力値を撹拌する。
///
/// [`Xoshiro256StarStar`] は状態が 256bit あるため周期は 2^256 - 1 と極めて長く、
/// 乱数生成器としては実用的な限界を超える。生成速度は
/// [`SplitMix64`] より速い。
///
/// # 計算量
/// 時間 O(1), 空間 O(1)
///
/// # Examples
///
/// ```
/// use atcoder::rand::{Rand, RandBackend, xoshiro256ss::Xoshiro256StarStar};
///
/// // 同じ seed から作った 2 つの生成器は、まったく同じ列を返す（決定性）。
/// let mut a = Xoshiro256StarStar::new(42);
/// let mut b = Xoshiro256StarStar::new(42);
/// assert_eq!(a.next_u64(), b.next_u64());
///
/// // `Rand` に載せると競プロ向けの便利メソッドが使える。
/// let mut rng: Rand<Xoshiro256StarStar> = Rand::new(42);
/// assert!(rng.below(1000) < 1000);
/// let _: f64 = rng.next_f64();
/// ```
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Xoshiro256StarStar([u64; 4]);

impl RandBackend for Xoshiro256StarStar {
    /// `seed` から [`Xoshiro256StarStar`] を作成する。
    ///
    /// 内部では [`SplitMix64`] に seed を渡し、その出力を 4 回読んで 256bit の
    /// 状態を埋める。`SplitMix64` の分布の良さを利用して、seed に極端な偏りがあっても
    /// 初期状態が偏らないようにしている。
    fn new(seed: u64) -> Self {
        let mut sm = SplitMix64::new(seed);
        Self([sm.next_u64(), sm.next_u64(), sm.next_u64(), sm.next_u64()])
    }

    /// 64bit 精度で次の乱数を生成する。
    ///
    /// 状態 4 個を xorshift と rotate で 1 ステップ進め、その結果を
    /// 乗算 2 段（`* 5` と rotate してから `* 9`）で撹拌して出力する。
    fn next_u64(&mut self) -> u64 {
        let [s0, s1, s2, s3] = &mut self.0;
        let result = s1.wrapping_mul(5).rotate_left(7).wrapping_mul(9);

        let t = *s1 << 17;
        *s2 ^= *s0;
        *s3 ^= *s1;
        *s1 ^= *s2;
        *s0 ^= *s3;

        *s2 ^= t;
        *s3 = s3.rotate_left(45);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_sequence() {
        const EXPECTED: [u64; 10] = [
            11520,
            0,
            1509978240,
            1215971899390074240,
            1216172134540287360,
            607988272756665600,
            16172922978634559625,
            8476171486693032832,
            10595114339597558777,
            2904607092377533576,
        ];

        let mut rng = Xoshiro256StarStar([1, 2, 3, 4]);

        for expected in EXPECTED {
            assert_eq!(rng.next_u64(), expected);
        }
    }
}
