//! 自動で mod を取ってくれる数値型

mod static_modint;
mod traits;

pub use traits::RemEuclidU32;

pub use static_modint::StaticModInt;

/// 法が 998244353 の [`StaticModInt`]
pub type ModInt998244353 = StaticModInt<998244353>;
/// 法が 1000000007 の [`StaticModInt`]
pub type ModInt1000000007 = StaticModInt<1000000007>;

/// Prelude for [`crate::math::modulo::modint`]
pub mod prelude {
    pub use super::ModInt998244353;
    pub use super::ModInt1000000007;
    pub use super::static_modint::StaticModInt;
    pub use super::static_modint::StaticModInt as MInt;
}
