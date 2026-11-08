//! `SplitMix64` のアルゴリズム
//!
//! Steele の `SplitMix64` は、64bit のカウンタを黄金比の定数
//! `0x9e3779b97f4a7c15`（√5 - 1 を 64bit に近似したもの）だけ進める。
//! 生成されるビット列を変形する。
//!
//! - 状態は `u64` 1 個だけで、seed をそのまま state として使い回す。
//! - 速度より品質を優先しており、低次元（少数の乱数を取り出すだけ）の用途では
//!   分布が非常に良い。
//! - 巨大な配列を作る用途では [`Xoshiro256StarStar`](crate::rand::xoshiro256ss::Xoshiro256StarStar)
//!   のほうが向く（速度の差があるため）。
//!
//! [`RandBackend`] を実装しているので、普通は
//! [`Rand<SplitMix64>`](crate::rand::Rand) で包んで使う。

use crate::rand::RandBackend;

/// `SplitMix64` のバックエンド。
///
/// 64bit の状態を保持する `SplitMix64` PRNG。seed をそのまま内部状態の初期値として使い、
/// 1 回 [`next_u64`](RandBackend::next_u64) を呼ぶたびに state を
/// `0x9e3779b97f4a7c15` だけ進めてから、xorshift と乗算で撹拌した 64bit 値を出力する。
/// （上の撹拌は最終出力だけにかかる）
///
/// # 計算量
/// 時間 O(1), 空間 O(1)
///
/// # Examples
///
/// ```
/// use atcoder::rand::{Rand, splitmix64::SplitMix64};
///
/// let mut rng: Rand<SplitMix64> = Rand::new(42);
/// assert_eq!(rng.below(100), 13);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct SplitMix64(u64);

impl RandBackend for SplitMix64 {
    /// `seed` をそのまま state として [`SplitMix64`] を作成する。
    ///
    /// `seed` を内部状態の初期値として使い回すため、下書きのエントロピーは
    /// seed の決め方に依存する。必要なら seed 自体を撹拌してから渡す。
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// 64bit 精度で次の乱数を生成する。
    ///
    /// state を黄金比の定数 `0x9e3779b97f4a7c15` だけ進め、xorshift 2 段と乗算 2 段で
    /// 撹拌して出力する。この撹拌は最終出力だけにかかる。
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_seed_0() {
        const EXPECTED: [u64; 8] = [
            0xe220_a839_7b1d_cdaf,
            0x6e78_9e6a_a1b9_65f4,
            0x06c4_5d18_8009_454f,
            0xf88b_b8a8_724c_81ec,
            0x1b39_896a_51a8_749b,
            0x53cb_9f0c_747e_a2ea,
            0x2c82_9abe_1f45_32e1,
            0xc584_133a_c916_ab3c,
        ];

        let mut rng = SplitMix64(0);

        for expected in EXPECTED {
            assert_eq!(rng.next_u64(), expected);
        }
    }

    #[test]
    fn reference_seed_1() {
        const EXPECTED: [u64; 5] = [
            0x910a_2dec_8902_5cc1,
            0xbeeb_8da1_658e_ec67,
            0xf893_a2ee_fb32_555e,
            0x71c1_8690_ee42_c90b,
            0x71bb_54d8_d101_b5b9,
        ];

        let mut rng = SplitMix64(1);

        for expected in EXPECTED {
            assert_eq!(rng.next_u64(), expected);
        }
    }
}
