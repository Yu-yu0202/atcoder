//! 乱数生成アルゴリズムの詰め合わせ

use crate::ext::range_ext::RangeExt;

pub mod splitmix64;
pub mod xoshiro256ss;

/// 乱数生成器の trait
pub trait RandBackend: Sized {
    /// 乱数生成器をシードから作成する。
    fn new(seed: u64) -> Self;

    /// 64bit 精度で乱数を生成する。
    fn next_u64(&mut self) -> u64;
}

/// 乱数生成器
///
/// # Example
///
/// ```
/// use atcoder::rand::{Rand, splitmix64::SplitMix64};
///
/// let mut rng: Rand<SplitMix64> = Rand::new(42);
///
/// assert_eq!(rng.next_u64(), 13679457532755275413);
/// assert_eq!(rng.next_bool(), false);
/// assert_eq!(rng.next_f64(), 0.27860113025513866);
/// ```
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Rand<R: RandBackend>(R);

impl<R: RandBackend> Rand<R> {
    /// `seed` から乱数生成器を作成する。
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self(R::new(seed))
    }

    /// 64bit 精度で乱数を生成する。
    pub fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    /// 32bit 精度で乱数を生成する。
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// 乱数から bool を生成する。
    pub fn next_bool(&mut self) -> bool {
        self.next_u64() >> 63 != 0
    }

    /// 乱数から f64 を生成する。[0, 1) の範囲で一様。
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / ((1u64 << 53) as f64))
    }

    /// `0..upper` の範囲で一様な乱数を生成する。
    ///
    /// # Panics
    ///
    /// `upper` が 0 である場合。
    pub fn below(&mut self, upper: u64) -> u64 {
        assert_ne!(upper, 0, "upper must be non-zero");

        let threshold = upper.wrapping_neg() % upper;

        loop {
            let x = self.next_u64();

            if x >= threshold {
                return x % upper;
            }
        }
    }

    /// `range` 内の u64 を生成する。
    ///
    /// # Panics
    ///
    /// `range` が空である場合。
    pub fn range_u64<T: RangeExt<u64>>(&mut self, range: T) -> u64 {
        let range = range.into_exclusive();
        assert!(range.start < range.end, "range must be non-empty");

        range.start + self.below(range.end - range.start)
    }

    /// `range` 内の i64 を生成する。
    ///
    /// # Panics
    ///
    /// `range` が空である場合。
    pub fn range_i64<T: RangeExt<i64>>(&mut self, range: T) -> i64 {
        let range = range.into_exclusive();
        assert!(range.start < range.end, "range must be non-empty");

        ordered_to_i64(
            i64_to_ordered(range.start) + self.below(i64_to_ordered(range.end - range.start)),
        )
    }
}

const SIGN_BIT: u64 = 1 << 63;

#[inline(always)]
const fn i64_to_ordered(x: i64) -> u64 {
    (x as u64) ^ SIGN_BIT
}

#[inline(always)]
const fn ordered_to_i64(x: u64) -> i64 {
    (x ^ SIGN_BIT) as i64
}
