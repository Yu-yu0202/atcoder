//! Xoshiro256** の実装

use crate::rand::{RandBackend, splitmix64::SplitMix64};

/// Xoshiro256** の実装
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Xoshiro256StarStar([u64; 4]);

impl RandBackend for Xoshiro256StarStar {
    fn new(seed: u64) -> Self {
        let mut sm = SplitMix64::new(seed);
        Self([sm.next_u64(), sm.next_u64(), sm.next_u64(), sm.next_u64()])
    }

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
