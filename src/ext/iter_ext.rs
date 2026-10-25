//! [`Iterator`] の拡張

use std::fmt::Write;

/// [`Iterator`] の拡張 trait
pub trait IterExt: Iterator + Sized {
    /// [`Iterator`] を [`Vec`] に変換する。
    ///
    /// `self.collect::<Vec<_>>()` と同等。
    #[inline]
    fn collect_vec(self) -> Vec<Self::Item> {
        self.collect()
    }

    /// [`Iterator`] の要素を文字列に結合する。
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
