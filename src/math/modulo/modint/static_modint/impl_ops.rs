//! [`StaticModInt`] の各種演算子（`std::ops`）の実装
//!
//! 加法・減法・乗法・除法と、それらの複合代入演算子を提供する。右辺は
//! [`StaticModInt<M>`] 自身のほか、[`RemEuclidU32`] を実装する任意の整数型も受け付ける
//! （整数リテラルを直接書けるようにするため）。整数型が来た場合は `[0, M)` に
//! 正規化してから演算する。

use crate::math::modulo::modint::{RemEuclidU32, StaticModInt};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

fn new_rem_euclid<const M: u32, T: RemEuclidU32>(rhs: T) -> StaticModInt<M> {
    unsafe {
        // SAFETY: `rem_euclid_u32` を通すので 0 <= x < MOD を保証できる
        StaticModInt::new_unchecked(rhs.rem_euclid_u32(M))
    }
}

impl<const M: u32, T: RemEuclidU32> Add<T> for StaticModInt<M> {
    type Output = Self;

    /// `mod M` で循環する加算を行う。
    ///
    /// `rhs` は `[0, M)` に正規化してから足すので、負の整数や `M` 以上の整数も
    /// そのまま渡せる。結果は必ず `[0, M)` に入る。
    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        let rhs = new_rem_euclid(rhs);
        self._add(rhs)
    }
}

impl<const M: u32> Add<StaticModInt<M>> for StaticModInt<M> {
    type Output = Self;

    /// `mod M` で循環する加算を行う。
    ///
    /// 両辺とも `[0, M)` なので、和が `M` 以上になるときだけ `M` を引いて戻す。
    #[inline]
    fn add(self, rhs: StaticModInt<M>) -> Self::Output {
        self._add(rhs)
    }
}

impl<const M: u32> AddAssign<StaticModInt<M>> for StaticModInt<M> {
    /// `mod M` で循環する加算を行い、`self` に代入する。
    #[inline]
    fn add_assign(&mut self, rhs: StaticModInt<M>) {
        *self = self._add(rhs);
    }
}

impl<const M: u32, T: RemEuclidU32> AddAssign<T> for StaticModInt<M> {
    /// `mod M` で循環する加算を行い、`self` に代入する。
    ///
    /// `rhs` は `[0, M)` に正規化してから足す。
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        *self = self._add(new_rem_euclid(rhs));
    }
}

impl<const M: u32, T: RemEuclidU32> Sub<T> for StaticModInt<M> {
    type Output = Self;

    /// `mod M` で循環する減算を行う。
    ///
    /// `rhs` は `[0, M)` に正規化してから引くので、負の整数を引く（= 足す）こともできる。
    /// 結果が負になりうるが、`M` を足して `[0, M)` に戻す。
    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        let rhs = new_rem_euclid(rhs);
        self._sub(rhs)
    }
}

impl<const M: u32> Sub<StaticModInt<M>> for StaticModInt<M> {
    type Output = Self;

    /// `mod M` で循環する減算を行う。
    ///
    /// 差が負になるときは `M` を足して `[0, M)` に戻す（`self - rhs mod M`）。
    #[inline]
    fn sub(self, rhs: StaticModInt<M>) -> Self::Output {
        self._sub(rhs)
    }
}

impl<const M: u32, T: RemEuclidU32> SubAssign<T> for StaticModInt<M> {
    /// `mod M` で循環する減算を行い、`self` に代入する。
    ///
    /// `rhs` は `[0, M)` に正規化してから引く。
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        *self = self._sub(new_rem_euclid(rhs));
    }
}

impl<const M: u32> SubAssign<StaticModInt<M>> for StaticModInt<M> {
    /// `mod M` で循環する減算を行い、`self` に代入する。
    #[inline]
    fn sub_assign(&mut self, rhs: StaticModInt<M>) {
        *self = self._sub(rhs);
    }
}

impl<const M: u32, T: RemEuclidU32> Mul<T> for StaticModInt<M> {
    type Output = Self;

    /// `mod M` で循環する乗算を行う。
    ///
    /// `rhs` は `[0, M)` に正規化してから掛ける。`M <= u32::MAX` より積は `u64` に
    /// 収まるので、`u64` で計算してから 1 回 `% M` する。
    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        self._mul(unsafe {
            // SAFETY: `rem_euclid_u32` を通すので 0 <= x < MOD を保証できる
            Self::new_unchecked(rhs.rem_euclid_u32(M))
        })
    }
}

impl<const M: u32> Mul<StaticModInt<M>> for StaticModInt<M> {
    type Output = Self;

    /// `mod M` で循環する乗算を行う。
    ///
    /// `u64` で積を計算してから `% M` するため、`u32` 同士の積では桁溢れしない。
    #[inline]
    fn mul(self, rhs: StaticModInt<M>) -> Self::Output {
        self._mul(rhs)
    }
}

impl<const M: u32, T: RemEuclidU32> MulAssign<T> for StaticModInt<M> {
    /// `mod M` で循環する乗算を行い、`self` に代入する。
    ///
    /// `rhs` は `[0, M)` に正規化してから掛ける。
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        *self = self._mul(new_rem_euclid(rhs));
    }
}

impl<const M: u32> MulAssign<StaticModInt<M>> for StaticModInt<M> {
    /// `mod M` で循環する乗算を行い、`self` に代入する。
    #[inline]
    fn mul_assign(&mut self, rhs: StaticModInt<M>) {
        *self = self._mul(rhs);
    }
}

impl<const M: u32, T: RemEuclidU32> Div<T> for StaticModInt<M> {
    type Output = Self;

    /// `mod M` における除算（`self * rhs^-1`）を行う。
    ///
    /// `rhs` の逆元を掛けるので、`rhs` は `[0, M)` に正規化してから使う。
    ///
    /// # Panics
    /// - `rhs` が 0 と合同な場合（0 除算）
    /// - `rhs` と `M` が互いに素でなく逆元が存在しない場合
    ///   （`M` が素数なら `rhs` が 0 のときに相当する）
    #[inline]
    #[track_caller]
    fn div(self, rhs: T) -> Self::Output {
        self._div(new_rem_euclid(rhs))
    }
}

impl<const M: u32> Div<StaticModInt<M>> for StaticModInt<M> {
    type Output = Self;

    /// `mod M` における除算（`self * rhs^-1`）を行う。
    ///
    /// 除算は逆元を掛けることで実現されるため、`rhs` が 0 だと結果が定まらない。
    ///
    /// # Panics
    /// - `rhs` が 0 と合同な場合（0 除算）
    /// - `rhs` と `M` が互いに素でなく逆元が存在しない場合
    ///   （`M` が素数なら `rhs` が 0 のときに相当する）
    #[inline]
    fn div(self, rhs: StaticModInt<M>) -> Self::Output {
        self._div(rhs)
    }
}

impl<const M: u32, T: RemEuclidU32> DivAssign<T> for StaticModInt<M> {
    /// `mod M` における除算を行い、`self` に代入する。
    ///
    /// 逆元を掛けるので、`rhs` は `[0, M)` に正規化してから使う。
    ///
    /// # Panics
    /// - `rhs` が 0 と合同な場合（0 除算）
    /// - `rhs` と `M` が互いに素でなく逆元が存在しない場合
    ///   （`M` が素数なら `rhs` が 0 のときに相当する）
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        *self = self._div(new_rem_euclid(rhs));
    }
}
