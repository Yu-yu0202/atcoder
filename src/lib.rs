// - 小さい関数には基本的に #[inline] を付与する
// - Panicable な関数は #[track_caller] を付与する、Panic doc を書く
//   → `.unwrap()` / `.expect(..)` など
// - Doc は日本語のほうが絶対わかりやすい
//   慣習なんて知るか

//! 競技プログラミング（主に AtCoder）向けの関数・構造体アソートパック
//!
//! 二分探索・累積和・座標圧縮・mod 計算・幾何・乱数など、AtCoder の本番で繰り返し必要になる
//! 要素を、トレイト・関数・構造体の形で提供する。外部クレートへの依存は `num-traits` のみで、
//! 使う機能はモジュール単位で `use` して持ち込む。
//!
//! # モジュール構成
//!
//! - [`algorithm`] — 二分探索（`binary_search`）、累積和（`cumsum`）、座標圧縮（`coordinate_compression`）
//! - [`data_structure`] — いもす法（`imos`）、配列の分割・結合（`implicit_treap`）
//! - [`ext`] — 標準ライブラリの型を拡張するトレイト群（`ChangeMinMax` / `IterExt` / `RangeExt` / `SliceExt`）
//! - [`math`] — 最大公約数・最小公倍数（`gcd_lcm`）、mod 計算（`modulo`）、素数（`prime`）、
//!   代数構造（`algebra`）、有理数（`fraction`）
//! - [`geometry`] — 点（`point`）、ベクトル（`vector`）、幾何判定（`predicates`）
//! - [`rand`] — 乱数生成。フロントエンドの [`rand::Rand`] と、バックエンドの
//!   [`rand::splitmix64::SplitMix64`] / [`rand::xoshiro256ss::Xoshiro256StarStar`]
//!
//! # prelude
//!
//! `use atcoder::prelude::*;` と書けば、[`algorithm`]・[`ext`]・[`math`] の主要な関数・トレイト・型が
//! まとめてスコープに入る。 [`rand`] は prelude に含めていないので、乱数を使うときは
//! `use atcoder::rand::{Rand, xoshiro256ss::Xoshiro256StarStar};` のように明示的に `use` する。
//!
//! # Examples
//!
//! ```
//! use atcoder::prelude::*;
//! use atcoder::rand::{Rand, xoshiro256ss::Xoshiro256StarStar};
//!
//! let mut rng: Rand<Xoshiro256StarStar> = Rand::new(42);
//!
//! // 0..100 の一様乱数を 3 つ
//! let a = [rng.below(100), rng.below(100), rng.below(100)];
//!
//! // prelude の chmin で最小値を取る
//! let mut min = u64::MAX;
//! for &x in &a {
//!     min.chmin(x);
//! }
//! assert_eq!(min, a.iter().copied().min().unwrap());
//!
//! // prelude の累積和（先頭に 0 を置いた長さ n+1 の配列）
//! let s = prefix_sum(&a);
//! assert_eq!(s.len(), a.len() + 1);
//! assert_eq!(s[2], a[0] + a[1]);
//!
//! // prelude の最大公約数
//! assert_eq!(gcd(12, 18), 6);
//! assert_eq!(lcm(4, 6), 12);
//! ```

pub mod algorithm;
pub mod data_structure;
pub mod ext;
pub mod math;
pub mod rand;

pub mod geometry;
pub(crate) mod macros;

#[allow(missing_docs)]
pub mod prelude {
    pub use super::algorithm::prelude::*;
    pub use super::ext::prelude::*;
    pub use super::math::prelude::*;
}
