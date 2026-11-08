//! アルゴリズム詰め合わせセット
//!
//! - [`binary_search`]：めぐる式二分探索。
//! - [`coordinate_compression`]：座標圧縮。
//! - [`cumsum`]：累積和（[`cumsum::prefix_sum`] / [`cumsum::suffix_sum`]）。

pub mod binary_search;
pub mod coordinate_compression;
pub mod cumsum;

/// Prelude for [`crate::algorithm`]
pub mod prelude {
    pub use super::binary_search::*;
    pub use super::coordinate_compression::*;
    pub use super::cumsum::*;
}
