//! 自動で mod を取ってくれる数値型
//!
//! [`StaticModInt`] は、法 `M` をコンパイル時定数（const generic）として持ち、
//! 四則演算のたびに自動で `mod M` を取る数値型である。加減乗算・除算は
//! 演算子（`+`, `-`, `*`, `/` と各複合代入）としてオーバーロードされており、
//! 整数リテラルからは [`StaticModInt::new`] で変換できる。
//!
//! 格納値は常に `[0, M)` に正規化されるため、途中結果が負になったり法を超えたり
//! しない。法を実行時に切り替える必要がない（= 1 問 1 法である）競技プログラミングでは、
//! この型をそのまま使うのが最も簡潔である。
//!
//! 法を表す定数が必要な場面（`mod_pow` の引数など）には [`StaticModInt`] から
//! 法を取り出す手段は用意していないので、そのような場合は [`crate::math::modulo::ops`] の
//! 関数か、自分で法の定数を定義すること。
//!
//! prelude（[`prelude`]）が提供するのは以下の 4 つ。
//!
//! - [`StaticModInt`]: 法を const generic で指定する本体
//! - [`prelude::MInt`]: [`StaticModInt`] の別名（短く書くため）
//! - [`ModInt998244353`]: 法 998244353 固定の別名
//! - [`ModInt1000000007`]: 法 1000000007 固定の別名
//!
//! # Examples
//! ```
//! use atcoder::math::modulo::prelude::*;
//!
//! // prelude に入っている別名でそのまま使える。
//! let x = MInt::<1000000007>::new(1_000_000_006);
//! let y = MInt::<1000000007>::new(3);
//! assert_eq!((x + y).value(), 2);
//! assert_eq!((x * y).value(), 1_000_000_004);
//!
//! // 負の値から作ると [0, M) に正規化される。
//! assert_eq!(ModInt998244353::new(-1_i32).value(), 998244352);
//! ```

mod static_modint;
mod traits;

pub use traits::RemEuclidU32;

pub use static_modint::StaticModInt;

/// 法が 998244353 の [`StaticModInt`]
///
/// 998244353 = 119 × 2^23 + 1 は素数で、長さ 2^23 以下の NTT（数論変換）が実行できる
/// 代表的な法である。畳み込みを伴う多項式計算や、法が指定されていない場面での
/// 既定値としてよく使われる。
///
/// `value()` が 0 以外の値なら必ず逆元が存在する（[`StaticModInt::inv`] が
/// `Some` を返す）ので、除算を安心して使える。
///
/// # Examples
/// ```
/// use atcoder::math::modulo::prelude::*;
///
/// let x = ModInt998244353::new(2);
/// assert_eq!((x * x * x).value(), 8);
/// assert_eq!((x / x).value(), 1);
/// ```
pub type ModInt998244353 = StaticModInt<998244353>;
/// 法が 1000000007 の [`StaticModInt`]
///
/// 10^9 + 7 は素数で、2 つの値を掛けても 2 × 10^18 程度に収まるため 64 bit 整数の
/// 積として安全に扱える。NTT 向きではないが、数え上げ・期待値・多項式の係数など
/// 「とりあえず素数 mod」の場面で最もよく使われる法である。
///
/// `value()` が 0 以外の値なら必ず逆元が存在する（[`StaticModInt::inv`] が
/// `Some` を返す）ため、除算・階乗の逆元・組合せの計算にそのまま使える。
///
/// # Examples
/// ```
/// use atcoder::math::modulo::prelude::*;
///
/// // 2 * 500000004 ≡ 500000004 * 2 ≡ 1 (mod 10^9+7)
/// let half = ModInt1000000007::new(500000004);
/// assert_eq!((half * 2).value(), 1);
/// assert_eq!((ModInt1000000007::new(7) * half).value(), 500000007);
/// ```
pub type ModInt1000000007 = StaticModInt<1000000007>;

/// Prelude for [`crate::math::modulo::modint`]
pub mod prelude {
    pub use super::ModInt998244353;
    pub use super::ModInt1000000007;
    pub use super::static_modint::StaticModInt;
    pub use super::static_modint::StaticModInt as MInt;
}
