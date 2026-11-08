//! [`crate::math::modulo::modint::StaticModInt`] が値を取り込むときに使う小さなトレイト
//!
//! `StaticModInt::new` は任意の整数型（`u8`〜`u128`, `i8`〜`i128`, `usize`）から
//! 値を受け取り、法の剰余に正規化して格納する。その正規化を型ごとに切り替えるための
//! 依存トレイトが [`RemEuclidU32`] であり、符号付き・符号なしで実装を分けている。

/// ユークリッド剰余を u32 で返す trait
///
/// 符号付き・符号なしを問わず、`m` で割った**非負**の余りを `u32` で返す。
/// [`crate::math::modulo::modint::StaticModInt::new`] が任意の整数型から値を
/// 受け取るために使うトレイトで、負の値を与えた場合も結果は `[0, m)` に正規化される
/// （Rust の組込み `%` は負の値を返しうるため、その代替として用意されている）。
///
/// 実装は `u8`・`u16`・`u32`・`i8`・`i16`・`i32`・`i64`・`isize`・`u64`・
/// `u128`・`i128` に対して提供されている。
pub trait RemEuclidU32 {
    /// ユークリッド剰余を u32 で返す。
    ///
    /// 返り値は `self` を `m` で割った非負の余り、すなわち `[0, m)` に入る。
    /// `self` が `u32` の範囲に収まらない型の場合は、内部で `i64` などへ拡張してから
    /// 剰余を取る。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::modulo::modint::RemEuclidU32;
    ///
    /// assert_eq!((-3_i32).rem_euclid_u32(7), 4);
    /// assert_eq!(10_u32.rem_euclid_u32(7), 3);
    /// ```
    fn rem_euclid_u32(&self, m: u32) -> u32;
}

macro_rules! impl_rem_euclid_u32_for_signed {
    ($($ty: tt), *) => {
        $(
            impl RemEuclidU32 for $ty {
                #[inline]
                fn rem_euclid_u32(&self, m: u32) -> u32 {
                    (*self as i64).rem_euclid(i64::from(m)) as _
                }
            }
        )*
    }
}

macro_rules! impl_rem_euclid_u32_for_unsigned {
    ($($ty: tt), *) => {
        $(
            impl RemEuclidU32 for $ty {
                #[inline]
                fn rem_euclid_u32(&self, m: u32) -> u32 {
                    *self as u32 % m
                }
            }
        )*
    }
}

impl_rem_euclid_u32_for_signed!(i8, i16, i32, i64, isize);

impl RemEuclidU32 for i128 {
    #[inline]
    fn rem_euclid_u32(&self, m: u32) -> u32 {
        (*self).rem_euclid(i128::from(m)) as _
    }
}

impl_rem_euclid_u32_for_unsigned!(u8, u16, u32);

impl RemEuclidU32 for u64 {
    #[inline]
    fn rem_euclid_u32(&self, m: u32) -> u32 {
        (*self % (m as u64)) as _
    }
}

impl RemEuclidU32 for u128 {
    #[inline]
    fn rem_euclid_u32(&self, m: u32) -> u32 {
        (*self % (m as u128)) as _
    }
}

#[cfg(target_pointer_width = "32")]
impl_rem_euclid_u32_for_unsigned!(usize);

#[cfg(target_pointer_width = "64")]
impl RemEuclidU32 for usize {
    #[inline]
    fn rem_euclid_u32(&self, m: u32) -> u32 {
        (*self % (m as usize)) as _
    }
}
