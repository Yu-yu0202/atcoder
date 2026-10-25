//! 既存型の拡張 trait(s)

pub mod change_minmax;
pub mod iter_ext;
pub mod range_ext;
pub mod slice_ext;

/// Prelude for [`crate::ext`]
pub mod prelude {
    pub use super::change_minmax::*;
    pub use super::iter_ext::*;
    pub use super::slice_ext::*;
}
