//! 代数学など（Monoid/MapMonoid etc）

/// モノイド構造。
pub trait Monoid {
    /// 対象の型。
    type S: Clone;

    /// 単位元を返す。
    ///
    /// 返される値は e op a = a op e = a になる必要がある。
    fn e() -> Self::S;

    /// 二項演算 op(a, b) = a op b を処理する。
    ///
    /// ここで、演算は結合律 (a op b) op c = a op (b op c) を満たす必要がある。
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}

/// 作用付きモノイド。
///
/// [`Monoid`] を満たす必要がある。
pub trait MapMonoid: Monoid {
    /// 作用を表す型。
    type F: Clone;

    /// 作用における単位元を返す。
    fn id() -> Self::F;

    /// `new` を `old` の後に適用する場合の作用を返す。
    fn composition(new: &Self::F, old: &Self::F) -> Self::F;

    /// 作用 `a` を `b` に適用する。
    fn mapping(a: &Self::F, b: &Self::S) -> Self::S;
}

/// 反転によって集約値が変わるモノイド。
///
/// [`Monoid`] を満たす必要がある。
pub trait Reversible: Monoid {
    /// `s` を反転する。
    fn rev(s: &Self::S) -> Self::S;
}
