mod impl_ops;

use crate::math::modulo::modint::RemEuclidU32;

/// 法がコンパイル時定数の `ModInt`
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StaticModInt<const M: u32>(u32);

impl<const M: u32> StaticModInt<M> {
    /// 指定した値で [`StaticModInt`] を作成する。
    ///
    /// # Panics
    /// - M が 0 以下の場合
    #[must_use]
    #[track_caller]
    pub fn new<T: RemEuclidU32>(x: T) -> Self {
        assert!(M > 0);
        unsafe {
            // SAFETY: `rem_euclid_u32` を通すので 0 <= x < MOD を保証できる
            Self::new_unchecked(x.rem_euclid_u32(M))
        }
    }

    /// 指定した値から Mod を取らずに [`StaticModInt`] を作成する。
    ///
    /// # Safety
    /// - value は M 未満である必要がある。
    ///   - M 以上の value を入力した場合の動作は、未定義である。計算結果の保証はない。
    #[must_use]
    pub const unsafe fn new_unchecked(value: u32) -> Self {
        Self(value)
    }

    /// 格納されている値を返す。
    #[inline]
    #[must_use]
    pub const fn value(&self) -> u32 {
        self.0
    }

    /// 逆元を返す。
    #[inline]
    #[must_use]
    pub const fn inv(self) -> Option<Self> {
        let (g, x, _) = ext_gcd_u32(self.0, M);
        if g != 1 {
            return None;
        }

        Some(Self(x.rem_euclid(M as i64) as _))
    }

    #[inline(always)]
    const fn _add(self, rhs: Self) -> Self {
        let sum = self.0 as u64 + rhs.0 as u64;
        let m = M as u64;

        Self(if sum >= m { (sum - m) as _ } else { sum as _ })
    }

    #[inline(always)]
    const fn _sub(self, rhs: Self) -> Self {
        let sub = self.0 as i64 - rhs.0 as i64;
        let m = M as i64;

        Self(if sub < 0 { (sub + m) as _ } else { sub as _ })
    }

    #[inline(always)]
    const fn _mul(self, rhs: Self) -> Self {
        let mul = (self.0 as u64 * rhs.0 as u64) % M as u64;
        Self(mul as _)
    }

    #[inline(always)]
    #[track_caller]
    const fn _div(self, rhs: Self) -> Self {
        assert!(rhs.0 != 0, "division by zero");

        let inv = rhs.inv();
        assert!(inv.is_some(), "inverse is indeterminate");

        self._mul(inv.unwrap())
    }
}

/// [`crate::math::gcd_lcm::ext_gcd`] の u32 固定版。
const fn ext_gcd_u32(a: u32, b: u32) -> (u32, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }

    let (g, x, y) = ext_gcd_u32(b, a % b);
    (g, y, x - (a / b) as i64 * y)
}
