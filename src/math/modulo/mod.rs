// mod で置こうと思ったら mod.rs と名前被ったの許してない

//! `ModInt`, Modulo 演算など
//!
//! 合同式（`mod`）の計算に必要なものをまとめたモジュールである。
//!
//! - [`modint`]: 自動で `mod` を取ってくれる数値型 [`modint::StaticModInt`] と、
//!   よく使う法の別名 [`modint::ModInt998244353`]・[`modint::ModInt1000000007`]
//! - [`ops`]: `mod_add`・`mod_mul`・`mod_pow` といった、任意の整数型に直接使える
//!   剰余演算ユーティリティ
//!
//! `mod` を取りたいだけの場面では [`ops`] の関数を、四則演算を繰り返す場面では
//! 演算子がオーバーロードされた [`modint::StaticModInt`] を使うと読みやすくなる。
//!
//! [`prelude`] は `ModInt998244353`・`ModInt1000000007`・`StaticModInt`・
//! `MInt`（`StaticModInt` の別名）と、[`ops`] の剰余演算関数を一括で導入する。
//!
//! # Examples
//! ```
//! use atcoder::math::modulo::prelude::*;
//!
//! // 型として mod を持たせる方法（法は型で区別される）。
//! let a = ModInt998244353::new(2);
//! assert_eq!((a * a * a).value(), 8); // 2^3 = 8
//! assert_eq!((a - MInt::<998244353>::new(5)).value(), 998244350); // 2 - 5 = -3
//!
//! // 別の法を使う場合。
//! let b = ModInt1000000007::new(3);
//! assert_eq!((b * b).value(), 9);
//!
//! // 生の整数に直接適用する方法。
//! assert_eq!(mod_pow(2_u32, 10, 1_000), 24);
//! ```

pub mod modint;
pub mod ops;

/// Prelude for [`crate::math::modulo::modint`]
pub mod prelude {
    pub use super::modint::prelude::*;
    pub use super::ops::*;
}
