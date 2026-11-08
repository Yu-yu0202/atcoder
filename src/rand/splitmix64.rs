//! `SplitMix64` の実装

use crate::rand::RandBackend;

/// `SplitMix64` のバックエンド
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct SplitMix64(u64);

impl RandBackend for SplitMix64 {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

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
