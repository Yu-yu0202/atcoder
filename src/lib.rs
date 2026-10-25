// - 小さい関数には基本的に #[inline] を付与する
// - Panicable な関数は #[track_caller] を付与する、Panic doc を書く
//   → `.unwrap()` / `.expect(..)` など
// - Doc は日本語のほうが絶対わかりやすい
//   慣習なんて知るか

//! 競技プログラミング（主に AtCoder）向けの関数・構造体アソートパック

pub mod algorithm;
pub mod data_structure;
pub mod ext;
pub mod math;
pub mod rand;

pub(crate) mod macros;
pub mod geometry;

#[allow(missing_docs)]
pub mod prelude {
    pub use super::algorithm::prelude::*;
    pub use super::ext::prelude::*;
    pub use super::math::prelude::*;
}
