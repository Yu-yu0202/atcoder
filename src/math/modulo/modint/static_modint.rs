//! コンパイル時に決まった法を持つ `ModInt`
//!
//! [`StaticModInt<M>`] は `M` を const generic で受ける数値型で、加減乗算・除算のたびに
//! 自動で `mod M` を取る。法が実行時に変わらない問題（競技プログラミングではほとんど）では、
//! [`StaticModInt::new`] で値を取り込み、あとは通常の整数と同じように四則演算すればよい。
//!
//! 実装上の特徴は以下の通り。
//!
//! - 格納値は常に `[0, M)` に正規化される
//! - `M <= u32::MAX` なので、積は `u64` で計算してから 1 回 `% M` する
//! - 除算は逆元を掛けるので、`M` が素数なら `value() != 0` のときに必ず成功する
//! - 値の取り込み時の検査を省く [`StaticModInt::new_unchecked`] も用意している

mod impl_ops;

use crate::math::modulo::modint::RemEuclidU32;

/// 法がコンパイル時定数の `ModInt`
///
/// `M` は `0 < M <= u32::MAX` を満たす必要がある（実用上は素数を選ぶことが多い）。
/// 加減乗算の結果は自動で `[0, M)` に丸められ、除算は逆元を掛けることで実現される。
///
/// 1 引数のメソッド（[`Self::new`] など）は `T: RemEuclidU32` を受け取るので、
/// `u8`〜`u128`・`i8`〜`i128`・`usize` のどの整数型からも変換できる。
///
/// # Examples
/// ```
/// use atcoder::math::modulo::prelude::*;
///
/// // 法は型引数で指定する。
/// let a = MInt::<7>::new(5);
/// let b = MInt::<7>::new(5);
/// assert_eq!((a * b).value(), 4); // 25 mod 7 = 4
/// assert_eq!((a + b).value(), 3); // 10 mod 7 = 3
/// assert_eq!((MInt::<7>::new(1) - a).value(), 3); // 1 - 5 = -4 ≡ 3 (mod 7)
/// ```
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StaticModInt<const M: u32>(u32);

impl<const M: u32> StaticModInt<M> {
    /// 指定した値で [`StaticModInt`] を作成する。
    ///
    /// `x` を `M` で割ったユークリッド剰余を格納する。したがって負の `x` を渡しても
    /// 結果は `[0, M)` に正規化され、`M` 以上の値を渡しても `mod M` が取られる。
    /// `T` は [`RemEuclidU32`] を実装する任意の整数型でよい（`i64` や `u128` など）。
    ///
    /// # Panics
    /// - M が 0 以下の場合
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::modulo::prelude::*;
    ///
    /// // 法を超える値は mod M される。
    /// assert_eq!(MInt::<7>::new(10).value(), 3);
    ///
    /// // 負の値も [0, M) に正規化される。
    /// assert_eq!(MInt::<7>::new(-3_i32).value(), 4);
    /// assert_eq!(ModInt1000000007::new(-1_i64).value(), 1_000_000_006);
    /// ```
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
    /// `value` をそのまま格納する（剰余演算を行わないため [`Self::new`] より高速）。
    /// 呼び出し側が `value` が `M` 未満であることを保証する必要があり、
    /// 保証を破った場合の結果は全く予測できない。
    ///
    /// 安全な利用例としては、`[0, M)` に収まることを自分で確認済みの値を使う場合や、
    /// 内部実装で `new_unchecked` を通した直後の値を使い回す場合である。
    ///
    /// # Safety
    /// - value は M 未満である必要がある。
    ///   - M 以上の value を入力した場合の動作は、未定義である。計算結果の保証はない。
    #[must_use]
    pub const unsafe fn new_unchecked(value: u32) -> Self {
        Self(value)
    }

    /// 格納されている値を返す。
    ///
    /// 返り値は `[0, M)` の範囲にある（`M` 未満に正規化された生の `u32`）。
    /// `mod M` の世界から整数の世界へ戻すときに使い、表示や法の比較にも使える。
    #[inline]
    #[must_use]
    pub const fn value(&self) -> u32 {
        self.0
    }

    /// 逆元を返す。
    ///
    /// `self * x ≡ 1 (mod M)` となる `x` を `[0, M)` の範囲で求めて `Some(x)` として返す。
    /// 拡張ユークリッドの互除法により求めている。
    ///
    /// `self.0` と `M` が互いに素でない場合は逆元が存在せず `None` を返す。
    /// `M` が素数なら、`self.value() == 0` のときだけ `None` になる
    /// （`998244353`・`1000000007` はいずれも素数なので、0 以外の除算は常に成功する）。
    /// `M` が合成数の場合は、`M` と共通因数を持つ値（例えば `M = 4` のとき 2）でも
    /// `None` になる。
    ///
    /// # 計算量
    /// 時間 O(log M)
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::modulo::prelude::*;
    ///
    /// let x = MInt::<1000000007>::new(3);
    /// let inv = x.inv().unwrap();
    /// assert_eq!(inv.value(), 333333336);
    /// assert_eq!((x * inv).value(), 1);
    ///
    /// // M が素数なら 0 のときだけ逆元が存在しない。
    /// assert_eq!(MInt::<1000000007>::new(0).inv(), None);
    /// ```
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
