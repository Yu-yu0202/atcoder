//! 乱数生成アルゴリズムの詰め合わせ
//!
//! 乱数生成器はフロントエンドとバックエンドの 2 層に分かれている。
//!
//! - [`RandBackend`] — 生の 64bit 乱数を供給するトレイト。実際の PRNG はここに実装する。
//! - [`Rand`] — `RandBackend` を 1 つ包むラッパーで、`below` や `range_i64` など
//!   競プロでよく使う乱数を提供する。ロジックはバックエンドに依存しない。
//!
//! バックエンドを差し替えるだけで、同じ乱数インターフェースのまま速度・品質を使い分けられる。
//! 既定で [`SplitMix64`]（軽量・高品質）と [`Xoshiro256StarStar`]（高速）を用意しているが、
//! [`RandBackend`] を実装すれば自作の PRNG も [`Rand`] のバックエンドとして使える。
//!
//! # Examples
//! ```
//! use atcoder::rand::{Rand, splitmix64::SplitMix64};
//!
//! let mut rng: Rand<SplitMix64> = Rand::new(42);
//! assert_eq!(rng.next_u64(), 13679457532755275413);
//!
//! // 0..100 の一様な整数（2 回目の draw）
//! assert_eq!(rng.below(100), 91);
//! ```

use crate::ext::range_ext::RangeExt;

pub mod splitmix64;
pub mod xoshiro256ss;

/// 乱数生成器のバックエンドとなる trait。
///
/// [`Rand`] はこの trait を介してのみ乱数を引く。実装側は `new`（`u64` のシードから生成する）と
/// `next_u64`（64bit 精度の乱数を 1 ステップ進めて返す）の 2 つを提供すればよい。
/// それ以外の便利メソッド（区間付き乱数など）は [`Rand`] 側が担当する。
///
/// # 実装者への要件
///
/// - **決定性**。同じ `seed` から作った生成器は、常に同じ乱数列を返すこと。時刻・アドレス・
///   ハードウェア由来のエントロピーを mix してはいけない（再現できないとデバッグもテストもできない）。
/// - **全 bit の乱雑さ**。`next_u64` の戻り値のどの bit を見ても偏りがないこと。
///   [`Rand`] のメソッドは、上位 bit（[`Rand::next_bool`]）・仮数（[`Rand::next_f64`]）・
///   剰余（[`Rand::below`]）と値のあらゆる部分を使う。
/// - **十分な周期**。実用で使い切る長さの乱列を、重複なく生成できること。
///
/// # Examples
///
/// 線形合同法（LCG）をバックエンドとして実装し、[`Rand`] に載せる例。
///
/// ```
/// use atcoder::rand::{Rand, RandBackend};
///
/// /// 最小限の線形合同法。品質は高くないが、実装例として載せる。
/// struct Lcg(u64);
///
/// impl RandBackend for Lcg {
///     fn new(seed: u64) -> Self {
///         Self(seed)
///     }
///
///     fn next_u64(&mut self) -> u64 {
///         self.0 = self
///             .0
///             .wrapping_mul(6364136223846793005)
///             .wrapping_add(1442695040888963407);
///         self.0
///     }
/// }
///
/// let mut rng: Rand<Lcg> = Rand::new(42);
///
/// assert_eq!(rng.next_u64(), 10481999410520546993);
/// assert_eq!(rng.below(100), 20);
/// ```
pub trait RandBackend: Sized {
    /// 乱数生成器をシードから作成する。
    fn new(seed: u64) -> Self;

    /// 64bit 精度で乱数を生成する。
    fn next_u64(&mut self) -> u64;
}

/// 乱数生成器。
///
/// バックエンド `R`（[`RandBackend`] を実装した任意の PRNG）の薄いラッパー。
/// `next_u64` を中継しつつ、`below` や `range_i64` など「競プロで実際に欲しい乱数」を
/// 揃えて提供する。ロジックはバックエンドに依存しないので、`R` を差し替えても同じ乱数が取れる。
///
/// `R` ごとに型が付く（`Rand<SplitMix64>` と `Rand<Xoshiro256StarStar>` は別の型）なので、
/// 必要な箇所でバックエンドを選んで使う。
///
/// # Examples
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
    ///
    /// `seed` は `R: RandBackend` にそのまま渡される。同じ `seed` から作った生成器は
    /// 以降まったく同じ乱数列を返す（決定性）。逆に `seed` が違えば列も変わる。
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self(R::new(seed))
    }

    /// 64bit 精度で乱数を生成する。
    ///
    /// バックエンドの `next_u64` をそのまま 1 ステップ進めて返す。他のメソッドはすべて
    /// これの戻り値を加工しているので、ビット単位で乱数が要るときはこちらを直接使う。
    pub fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    /// 32bit 精度で乱数を生成する。
    ///
    /// `next_u64` を 1 回呼び、その戻り値の**上位 32bit** を `u32` に切り出して返す。
    /// 32bit 幅で足りる用途（配列の要素数や小さい添字）ではこちらを使う。
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// 乱数から bool を生成する。
    ///
    /// `next_u64` を 1 回呼び、最上位 bit を見て `true` / `false` を返す。
    /// どちらも確率ちょうど 1/2 で、独立な列として受け取れる。
    pub fn next_bool(&mut self) -> bool {
        self.next_u64() >> 63 != 0
    }

    /// 乱数から f64 を生成する。[0, 1) の範囲で一様。
    ///
    /// [0, 1) に一様分布する小数を返す。53bit 精度（倍精度浮動小数点の仮数が持つ全 bit）で
    /// 分解能を持ち、`1.0` は返らない。小数を絡めた乱数（重み付き選択など）が必要なときに使う。
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / ((1u64 << 53) as f64))
    }

    /// `0..upper` の範囲で一様な乱数を生成する。
    ///
    /// 単純に `next_u64() % upper` では、`upper` が `2^64` を割り切らない場合に値が偏る。
    /// そこで `threshold = 2^64 mod upper` 未満（割り切れない端数ぶん）の値は捨てて引き直す
    /// rejection sampling を使う。これにより `upper` が `2^64` を割り切れなくても
    /// `0..upper` のどの値も等確率（ちょうど 1/`upper`）で返る。
    ///
    /// 却下される確率が最大になるのは `upper ≈ 2^63 + 1` のときで、それでも約 1/2 で済む。
    ///
    /// # Panics
    ///
    /// - `upper` が 0 である場合。
    ///
    /// # Examples
    ///
    /// ```
    /// use atcoder::rand::{Rand, splitmix64::SplitMix64};
    ///
    /// let mut rng: Rand<SplitMix64> = Rand::new(42);
    ///
    /// assert_eq!(rng.below(100), 13);
    /// assert_eq!(rng.below(1), 0);
    /// ```
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
    /// `range` は半開区間 `start..end`（`end` を含まない）と閉区間 `start..=end`（`end` を含む）の
    /// 両方が使える。返り値は半開区間なら `start` 以上 `end` 未満、閉区間なら `start` 以上 `end` 以下で
    /// 一様に分布し、偏りは [`Self::below`] と同じ仕組みで除かれている。
    ///
    /// # Panics
    ///
    /// - `range` が空である場合（半開区間で `start >= end`、閉区間で `start > end`）。
    ///
    /// # Examples
    ///
    /// ```
    /// use atcoder::rand::{Rand, splitmix64::SplitMix64};
    ///
    /// let mut rng: Rand<SplitMix64> = Rand::new(42);
    ///
    /// let x = rng.range_u64(1..10); // 1..=9
    /// assert!((1..10).contains(&x));
    /// ```
    pub fn range_u64<T: RangeExt<u64>>(&mut self, range: T) -> u64 {
        let range = range.into_exclusive();
        assert!(range.start < range.end, "range must be non-empty");

        range.start + self.below(range.end - range.start)
    }

    /// `range` 内の i64 を生成する。
    ///
    /// `range` は半開区間 `start..end`（`end` を含まない）と閉区間 `start..=end`（`end` を含む）の
    /// 両方が使える。負の範囲もそのまま渡せる。内部的には符号を避けるため「最上位 bit を反転した
    /// 順序付きの値」に変換してから [`Self::below`] と同じ仕組みで 1 つ引き、最後に元に戻す。
    ///
    /// # Panics
    ///
    /// - `range` が空である場合（半開区間で `start >= end`、閉区間で `start > end`）。
    ///
    /// # Examples
    ///
    /// ```
    /// use atcoder::rand::{Rand, splitmix64::SplitMix64};
    ///
    /// let mut rng: Rand<SplitMix64> = Rand::new(42);
    ///
    /// let _: i64 = rng.range_i64(-5..5); // -5..=4
    /// let _: i64 = rng.range_i64(-5..=5); // -5..=5
    /// ```
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
