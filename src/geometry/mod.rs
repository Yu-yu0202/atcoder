// geometry は math に含まれるべきだろって思ったそこのあなた。そのとおりです。
// でも階層増やすの違うなって思って。許して。

//! 幾何学詰め合わせセット
//!
//! 2 次元平面上の整数座標を扱うための型と述語を提供する。
//!
//! - [`point`]: 2 次元整数点 [`point::Point`]（座標と点どうしの距離）
//! - [`vector`]: 2 次元整数ベクトル [`vector::Vector`]（ベクトル演算）
//! - [`predicates`]: 幾何述語（[`predicates::orientation`] / [`predicates::on_segment`]）
//!
//! 点・ベクトル・述語は互いに密接に結びついており、**点の差を引くとベクトル**（[`point::Point`] - [`point::Point`] で [`vector::Vector`]）、
//! **点にベクトルを足すと点**（[`point::Point`] + [`vector::Vector`] で [`point::Point`]）という関係になっている。
//! 点と点の距離や位置関係は、差から得られるベクトルの内積・外積として計算する。
//!
//! 座標は [`i64`] で保持し、距離の比較や共線判定は平方根を取らず誤差のない整数演算（[`i128`]）で行う。
//!
//! prelude は存在しない（[`crate::geometry::prelude`] は空）ので、必要な型・関数は各モジュールから直接 `use` する。

pub mod point;
pub mod predicates;
pub mod vector;

/// Prelude for [`crate::geometry`]
pub mod prelude {}
