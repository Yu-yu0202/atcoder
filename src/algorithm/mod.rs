//! アルゴリズム詰め合わせセット

pub mod binary_search;
pub mod coordinate_compression;
pub mod cumsum;

/// Prelude for [`crate::algorithm`]
pub mod prelude {
    pub use super::binary_search::*;
    pub use super::coordinate_compression::*;
    pub use super::cumsum::*;
}
