//! 既存型の拡張 trait(s)
//!
//! 標準ライブラリの型・トレイトにメソッドを生やすトレイトを集めたモジュール。
//! どれも blanket impl されるので、対象の型に対して `use` するだけでそのまま呼べる。
//!
//! - [`change_minmax`]: `chmin` / `chmax`（[`change_minmax::ChangeMinMax`]）。 `PartialOrd` な全型に最小値・最大値の更新を追加する。
//! - [`iter_ext`]: [`collect_vec`](iter_ext::IterExt::collect_vec) / [`join`](iter_ext::IterExt::join)（[`iter_ext::IterExt`]）。
//!   [`Iterator`] を便利に扱う。
//! - [`range_ext`]: `into_exclusive` / `overlaps` / `touches` / `intersection` / `merge`（[`range_ext::RangeExt`]）と
//!   [`interval_merge`](range_ext::interval_merge)。 半開区間・閉区間を統一的に扱う。
//! - [`slice_ext`]: `argsort` / `argsort_by` / `argsort_by_key`（[`slice_ext::SliceExt`]）。 ソート結果ではなく元の index を返す。
//!
//! このうち [`change_minmax::ChangeMinMax`], [`iter_ext::IterExt`], [`slice_ext::SliceExt`] は [`prelude`] 経由で
//! 一括 import できる（[`range_ext::RangeExt`] は [`crate::ext::range_ext`] から直接 import する）。
//!
//! # Examples
//! ```
//! use atcoder::ext::prelude::*;
//!
//! let mut x = 3;
//! assert!(x.chmin(1));
//!
//! let v = (1..=3).map(|i| i * i).collect_vec();
//! assert_eq!(v, vec![1, 4, 9]);
//! assert_eq!((&v).iter().copied().join(", "), "1, 4, 9");
//!
//! let a = [3, 1, 2];
//! assert_eq!(a.argsort(), vec![1, 2, 0]);
//! ```

pub mod change_minmax;
pub mod iter_ext;
pub mod range_ext;
pub mod slice_ext;

/// Prelude for [`crate::ext`]
///
/// [`change_minmax::ChangeMinMax`], [`iter_ext::IterExt`], [`slice_ext::SliceExt`] を再 export する。
pub mod prelude {
    pub use super::change_minmax::*;
    pub use super::iter_ext::*;
    pub use super::slice_ext::*;
}
