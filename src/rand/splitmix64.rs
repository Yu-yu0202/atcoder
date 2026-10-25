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
