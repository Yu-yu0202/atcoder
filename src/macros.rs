//! 内部用マクロ

/// `?` 演算子の代用。
///
/// `const fn` では `?` が使えないので、これで代用する
macro_rules! if_none_return {
    ($opt: expr) => {
        match $opt {
            ::core::option::Option::Some(x) => x,
            ::core::option::Option::None => return ::core::option::Option::None,
        }
    };
}

pub(crate) use if_none_return;
