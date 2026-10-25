// mod で置こうと思ったら mod.rs と名前被ったの許してない

//! `ModInt`, Modulo 演算など

pub mod modint;
pub mod ops;

/// Prelude for [`crate::math::modulo::modint`]
pub mod prelude {
    pub use super::modint::prelude::*;
    pub use super::ops::*;
}
