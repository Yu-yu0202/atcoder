//! 代数学など（Monoid/MapMonoid etc）
//!
//! データ構造が扱う「値の集め方」をトレイトとして抽象化したものである。
//! [`Monoid`] は集合と二値演算と単位元だけを、[`MapMonoid`] はさらに「作用」を、
//! [`Reversible`] は順序反転時の集め方を定める。
//!
//! 利用側はこれらを実装するだけで、遅延評価セグメント木・平衡二分木・トレープなどの
//! 共通のデータ構造に、任意の演算（和・総積・最小値・一次変換など）を乗せられる。
//! つまり「何を」集めるかと「どう」集めるかを分離するための層である。
//!
//! # Examples
//! ```
//! use atcoder::math::algebra::{MapMonoid, Monoid, Reversible};
//!
//! // 要素: i64、集め方: 最小値、作用: 「定数 c を足す」（区間加算・区間最小値）。
//! struct RangeAddMin;
//!
//! impl Monoid for RangeAddMin {
//!     type S = i64;
//!
//!     fn e() -> Self::S {
//!         i64::MAX
//!     }
//!
//!     fn op(a: &Self::S, b: &Self::S) -> Self::S {
//!         (*a).min(*b)
//!     }
//! }
//!
//! impl MapMonoid for RangeAddMin {
//!     type F = i64;
//!
//!     fn id() -> Self::F {
//!         0
//!     }
//!
//!     fn composition(new: &Self::F, old: &Self::F) -> Self::F {
//!         // 加算の合成は足し算。
//!         new + old
//!     }
//!
//!     fn mapping(c: &Self::F, x: &Self::S) -> Self::S {
//!         x + c
//!     }
//! }
//!
//! impl Reversible for RangeAddMin {
//!     fn rev(s: &Self::S) -> Self::S {
//!         // 最小値は順序を入れ替えても変わらない。
//!         *s
//!     }
//! }
//!
//! assert_eq!(RangeAddMin::op(&3, &5), 3);
//! assert_eq!(RangeAddMin::mapping(&10, &4), 14);
//! assert_eq!(RangeAddMin::rev(&RangeAddMin::op(&3, &5)), RangeAddMin::op(&3, &5));
//! ```

/// モノイド構造。
///
/// 集合 `S` と二値演算 `op` が以下の公理を満すとき、組 `(S, op)` をモノイドという。
///
/// - 結合律: `(a op b) op c = a op (b op c)`
/// - 単位元: `e op a = a op e = a`
///
/// 可換性（`a op b = b op a`）は要求しない。非可換な例として
/// 文字列の連接や行列の積などがあり、逆順に集める必要があるなら
/// [`Reversible`] と併用する。
///
/// データ構造は集約値を組み立てる際に [`Self::e`] を初期値、
/// [`Self::op`] を合成規則として使う。公理を破ると結果が依る順序や
/// 区間の分け方によって変わってしまうため、実装時は両方を必ず満たすこと。
///
/// # Examples
/// ```
/// use atcoder::math::algebra::Monoid;
///
/// /// 総和を取るモノイド（可換なので順序を気にしなくてよい）。
/// struct Sum;
///
/// impl Monoid for Sum {
///     type S = i64;
///
///     fn e() -> Self::S {
///         0
///     }
///
///     fn op(a: &Self::S, b: &Self::S) -> Self::S {
///         a + b
///     }
/// }
///
/// // 単位元・結合律の確認。
/// assert_eq!(Sum::e(), 0);
/// assert_eq!(Sum::op(&Sum::e(), &7), 7);
/// assert_eq!(Sum::op(&Sum::op(&1, &2), &3), Sum::op(&1, &Sum::op(&2, &3)));
/// ```
pub trait Monoid {
    /// 対象の型。
    ///
    /// 集約値の型であり、[`Self::op`] の結果もこの型になる。クローンできなければ
    /// ならない（値の保存・復元に使うため）。
    type S: Clone;

    /// 単位元を返す。
    ///
    /// 返される値は e op a = a op e = a になる必要がある。
    ///
    /// 区間が空のときの集約値としても使われる。`e()` の呼び出しごとに新しい値を
    /// 返してよく、内部状態を持たない（`static` な）実装が期待される。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::algebra::Monoid;
    ///
    /// /// 最小値を取るモノイド。
    /// struct Min;
    ///
    /// impl Monoid for Min {
    ///     type S = i64;
    ///
    ///     fn e() -> Self::S {
    ///         i64::MAX
    ///     }
    ///
    ///     fn op(a: &Self::S, b: &Self::S) -> Self::S {
    ///         (*a).min(*b)
    ///     }
    /// }
    ///
    /// // 単位元はどんな値と演算しても変わらない。
    /// assert_eq!(Min::e(), i64::MAX);
    /// assert_eq!(Min::op(&Min::e(), &42), 42);
    /// assert_eq!(Min::op(&42, &Min::e()), 42);
    /// ```
    fn e() -> Self::S;

    /// 二値演算 op(a, b) = a op b を処理する。
    ///
    /// ここで、演算は結合律 (a op b) op c = a op (b op c) を満たす必要がある。
    ///
    /// 可換性は不要で、非可換演算の場合は `a` と `b` の前後関係が意味を持つ。
    /// 例えば文字列連接なら `op("a", "b") = "ab"` となり、`op("b", "a")` とは異なる。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::algebra::Monoid;
    ///
    /// /// 非可換なモノイド（文字列の連接）。
    /// struct Concat;
    ///
    /// impl Monoid for Concat {
    ///     type S = String;
    ///
    ///     fn e() -> Self::S {
    ///         String::new()
    ///     }
    ///
    ///     fn op(a: &Self::S, b: &Self::S) -> Self::S {
    ///         format!("{a}{b}")
    ///     }
    /// }
    ///
    /// // 順序を変えると結果が変わる（非可換）。
    /// assert_eq!(Concat::op(&"a".to_owned(), &"b".to_owned()), "ab");
    /// assert_eq!(Concat::op(&"b".to_owned(), &"a".to_owned()), "ba");
    /// ```
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}

/// 作用付きモノイド。
///
/// [`Monoid`] を満たす必要がある。
///
/// モノイド `(S, op)` に加えて、作用の型 `F` と
/// - 作用の合成 [`Self::composition`]
/// - 作用の適用 [`Self::mapping`]
///
/// が定義される。`F` は [`Self::id`] を単位元として [`Self::composition`] に関して
/// モノイドをなす必要があり、さらに以下の公理を満たすこと。
///
/// - 作用の単位元: `mapping(id(), a) = a`
/// - 合成の整合性: `mapping(composition(new, old), a) = mapping(new, mapping(old, a))`
///   （[`Self::composition`] の第 1 引数が「後に適用する側」なので、合成写像を適用すると
///   `old` → `new` の順に作用する）
/// - 作用と集約の整合性（遅延伝播のために必要）:
///   `mapping(f, op(a, b)) = op(mapping(f, a), mapping(f, b))`
///
/// 典型的な用途は、区間全体へ同じ写像を一括でかけたい場面である。作用を要素一つずつに
/// 適用するのではなく、集約値に「まだ適用していない作用」を溜め込む形で使う。
///
/// 上の「作用と集約の整合性」のため、作用は集約の変形ではなく準同型写像でなければ
/// ならない。例えば「和」へ「+ c」を作用させるには要素数が必要になるため、区間加算を
/// 総和に載せるだけならこのトレイトだけでは足りず、別途要素数を持たせるか
/// 最小値・最大値のように準同型になる演算を選ぶとよい。
///
/// # Examples
/// ```
/// use atcoder::math::algebra::{MapMonoid, Monoid};
///
/// /// 集め方は最小値、作用は「定数 c を足す」（区間加算・区間最小値）。
/// struct RangeAddMin;
///
/// impl Monoid for RangeAddMin {
///     type S = i64;
///
///     fn e() -> Self::S {
///         i64::MAX
///     }
///
///     fn op(a: &Self::S, b: &Self::S) -> Self::S {
///         (*a).min(*b)
///     }
/// }
///
/// impl MapMonoid for RangeAddMin {
///     type F = i64;
///
///     fn id() -> Self::F {
///         0
///     }
///
///     fn composition(new: &Self::F, old: &Self::F) -> Self::F {
///         new + old
///     }
///
///     fn mapping(c: &Self::F, x: &Self::S) -> Self::S {
///         x + c
///     }
/// }
///
/// // 作用の単位元は何もしない。
/// assert_eq!(RangeAddMin::mapping(&RangeAddMin::id(), &7), 7);
///
/// // 合成は「old を適用してから new」。加算なので順序を入れ替えても同じ。
/// assert_eq!(RangeAddMin::composition(&2, &3), 5);
///
/// // 集計値への作用と、要素への作用の分配（最小値と加算は両立する）。
/// assert_eq!(
///     RangeAddMin::mapping(&10, &RangeAddMin::op(&3, &5)),
///     RangeAddMin::op(&RangeAddMin::mapping(&10, &3), &RangeAddMin::mapping(&10, &5)),
/// );
/// ```
pub trait MapMonoid: Monoid {
    /// 作用を表す型。
    ///
    /// `S` への作用の集合であり、[`Self::composition`] と [`Self::id`] に関して
    /// モノイドをなす。クローンできなければならない。
    type F: Clone;

    /// 作用における単位元を返す。
    ///
    /// `mapping(id(), a) = a` を満たす必要がある（何もしない作用）。
    /// 例えば一次変換なら恒等写像 `(1, 0)`、区間加算なら「0 足す」である。
    fn id() -> Self::F;

    /// `new` を `old` の後に適用する場合の作用を返す。
    ///
    /// 結果 `f = composition(new, old)` は `mapping(f, a) = mapping(new, mapping(old, a))`
    /// を満たす必要がある。引数の並びが「新しい作用が先」である点に注意すること。
    ///
    /// また `composition` は結合律を満たす必要があり、[`Self::id`] がその単位元になる。
    fn composition(new: &Self::F, old: &Self::F) -> Self::F;

    /// 作用 `a` を `b` に適用する。
    ///
    /// `a` は作用側 (`F`)、`b` は値側 (`S`) であり、順序に注意。
    /// 結果は `Self::S` に丸められる（`mod` を取る・律速する・丸めるなどは
    /// `S` 側の定義に委ねる）。
    fn mapping(a: &Self::F, b: &Self::S) -> Self::S;
}

/// 反転によって集約値が変わるモノイド。
///
/// [`Monoid`] を満たす必要がある。
///
/// [`Self::rev`] は「要素列を前後反転してから集約した値」を返す。
/// 反転操作を伴うデータ構造（トレープの split / merge など）では、部分木の左右を
/// 入れ替えるタイミングで集計値を組み直す必要があり、その規則を
/// このトレイトで与える。
///
/// 満たすことが期待される性質は以下の通り。
///
/// - `rev(e()) = e()`（単位元は反転しても単位元）
/// - `rev(op(a, b)) = op(rev(b), rev(a))`（左右が入れ替わる）
///
/// `op` が可換なら `rev(s) = *s`（恒等写像）の実装でこの性質を満たす。
///
/// # Examples
/// ```
/// use atcoder::math::algebra::{Monoid, Reversible};
///
/// /// 非可換なモノイド（文字列の連接）。反転は左右を入れ替える。
/// struct Concat;
///
/// impl Monoid for Concat {
///     type S = String;
///
///     fn e() -> Self::S {
///         String::new()
///     }
///
///     fn op(a: &Self::S, b: &Self::S) -> Self::S {
///         format!("{a}{b}")
///     }
/// }
///
/// impl Reversible for Concat {
///     fn rev(s: &Self::S) -> Self::S {
///         s.chars().rev().collect()
///     }
/// }
///
/// // つなげてから反転 = それぞれ反転してから逆順につなげる。
/// let a = "ab".to_owned();
/// let b = "cd".to_owned();
/// assert_eq!(Concat::rev(&Concat::op(&a, &b)), Concat::op(&Concat::rev(&b), &Concat::rev(&a)));
/// ```
pub trait Reversible: Monoid {
    /// `s` を反転する。
    ///
    /// `s` の要素順を逆に見たときの集約値を返す。`op` が可換なモノイドでは
    /// 恒等写像でよい。
    fn rev(s: &Self::S) -> Self::S;
}
