//! [`Iterator`] の拡張
//!
//! [`Iterator`] に便利メソッドを足すトレイト [`IterExt`] を提供する。 [`Iterator`] を満たすすべての
//! イテレータに blanket impl されるため、`map` / `filter` などの結果に対してそのまま呼び出せる。
//!
//! - [`collect_vec`](IterExt::collect_vec): イテレータを [`Vec`] に回収する（`self.collect::<Vec<_>>()` の短縮形）。
//! - [`join`](IterExt::join): 表示可能な要素を、区切り文字 `sep` を挟みながら文字列に連結する。
//!
//! # Examples
//! ```
//! use atcoder::ext::prelude::*;
//!
//! assert_eq!((1..=3).collect_vec(), vec![1, 2, 3]);
//! assert_eq!((1..=3).join(", "), "1, 2, 3");
//! ```

use std::fmt::Write;

/// [`Iterator`] の拡張 trait。
///
/// [`Iterator`] を満たすすべての型に blanket impl される（`impl<T: Iterator + Sized> IterExt for T`）。
/// `use atcoder::ext::prelude::*;` してしまえば `map` などの戻り値にも直接メソッドが生える。
pub trait IterExt: Iterator + Sized {
    /// [`Iterator`] を [`Vec`] に変換する。
    ///
    /// `self.collect::<Vec<_>>()` と同等。型注釈を書かなくても推論が働くので、
    /// `let xs: Vec<_> = (0..n).map(|i| f(i)).collect();` を短く書ける。
    ///
    /// # 計算量
    /// 時間・空間ともに O(n)（n は要素数）
    ///
    /// # Examples
    /// ```
    /// use atcoder::ext::prelude::*;
    ///
    /// let v = (1..=3).collect_vec();
    /// assert_eq!(v, vec![1, 2, 3]);
    ///
    /// let w = (1..=3).map(|x| x * x).collect_vec();
    /// assert_eq!(w, vec![1, 4, 9]);
    /// ```
    #[inline]
    fn collect_vec(self) -> Vec<Self::Item> {
        self.collect()
    }

    /// [`Iterator`] の要素を文字列に結合する。
    ///
    /// 各要素を [`Display`](std::fmt::Display) で文字列化し、その**間**に `sep` を挟んで連結する。
    /// したがって `sep` は先頭にも最後にも付かない。要素数が 0 のときは空文字列、1 のときは `sep` なしで
    /// その要素だけになる。 [`size_hint`](Iterator::size_hint) の下限で [`String`] の容量を確保してから
    /// 1 回の `fold` で連結するため、余分な再確保が発生しにくい。
    ///
    /// 標準ライブラリの slice の `join` は配列（slice）向けだが、これはイテレータ向けで
    /// `map` の結果などそのまま渡せるのが違いである。
    ///
    /// # 計算量
    /// 時間 O(全要素の表示文字数の合計 + `sep.len()` × (要素数 - 1)), 空間は同程度
    ///
    /// # Examples
    /// ```
    /// use atcoder::ext::prelude::*;
    ///
    /// assert_eq!((1..=3).join(", "), "1, 2, 3");
    /// assert_eq!((1..=1).join(", "), "1");
    /// assert_eq!((0..0).join(", "), "");
    ///
    /// let chars = ['a', 'b', 'c'].iter();
    /// assert_eq!(chars.join("-"), "a-b-c");
    /// ```
    #[inline]
    fn join(self, sep: &str) -> String
    where
        Self: Sized,
        Self::Item: std::fmt::Display,
    {
        let (lower, _) = self.size_hint();
        let capacity = sep.len() * lower.saturating_sub(1);

        self.fold(
            (String::with_capacity(capacity), false),
            |(mut out, has_item), item| {
                if has_item {
                    out.push_str(sep);
                }
                write!(&mut out, "{item}").unwrap();
                (out, true)
            },
        )
        .0
    }
}

impl<T: Iterator + Sized> IterExt for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_vec() {
        let x = 1..=3;
        assert_eq!(x.collect_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_join() {
        let x = 1..=3;
        assert_eq!(x.join(", "), "1, 2, 3");
    }
}
