//! 数学関係の詰め合わせセット

pub mod fraction;
pub mod gcd_lcm;
pub mod modulo;
pub mod prime;

/// Prelude for [`crate::math`]
pub mod prelude {
    pub use super::gcd_lcm::*;
    pub use super::modulo::prelude::*;
    pub use super::prime::*;
}
