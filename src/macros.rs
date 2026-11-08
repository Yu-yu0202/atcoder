//! 内部用マクロ
//!
//! crate の内部からしか使わない補助マクロを置く。外部に公開される API ではない。

/// `Option` を早期 return する `?` 演算子の代用。
///
/// `const fn` の中では `?` 演算子が使えないため、その代わりとして使う。
/// `Some(x)` なら `x` を取り出し、`None` ならコンパイルされた関数から即座に `None` を return する。
/// `Option` を「外せるか、さもなくば return」の意味で 1 行にまとめたい場面で使う。
///
/// # Examples
///
/// crate 内部用なので、外部からは `if_none_return!` を直接呼べない（`pub(crate)`）。
/// 以下は同等のマクロを手元に定義して動きを確かめる例である。
///
/// ```
/// macro_rules! if_none_return {
///     ($opt: expr) => {
///         match $opt {
///             ::core::option::Option::Some(x) => x,
///             ::core::option::Option::None => return ::core::option::Option::None,
///         }
///     };
/// }
///
/// const fn add_one(x: Option<u32>) -> Option<u32> {
///     Some(if_none_return!(x) + 1)
/// }
///
/// assert_eq!(add_one(Some(2)), Some(3));
/// assert_eq!(add_one(None), None);
/// ```
macro_rules! if_none_return {
    ($opt: expr) => {
        match $opt {
            ::core::option::Option::Some(x) => x,
            ::core::option::Option::None => return ::core::option::Option::None,
        }
    };
}

pub(crate) use if_none_return;
