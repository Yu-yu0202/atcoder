//! 遅延伝搬・反転可能な乱択平衡二分探索木（implicit treap）
//!
//! キーを持たず、**位置（index）** だけでアクセスする平衡二分探索木である。
//! ノードは配列ベースのノードプール（`Vec<Node<M>>`）で管理し、解放されたノードはフリーリストで再利用する。
//! 各区間は「左から何番目か」という index の半開区間 `[l, h)` で指定し、単独の要素の参照・変更、区間反転、区間への作用、区間の集約値の取得を
//! いずれも O(log n) で処理できる。
//!
//! 要素の型は `M::S`、作用の型は `M::F` で、[`crate::math::algebra::MapMonoid`]（[`crate::math::algebra::Monoid`] に遅延作用を付加したもの）と
//! [`crate::math::algebra::Reversible`]（反転したときに集約値がどう変わるかを表すもの）を実装する必要がある。
//! 反転した部分木の集約値は [`crate::math::algebra::Reversible::rev`] で変換されるので、反転後の [`ImplicitTreap::fold`] も正しい値を返す。
//!
//! 代表的な用途は以下のとおり:
//! - 配列の区間 reverse（[`ImplicitTreap::insert`] / [`ImplicitTreap::remove`] と組み合わせると区間の移動にも使える）
//! - 配列の区間に対する一括の値変換（区間 affine 変換・区間代入など）
//! - 配列の区間に対する集約値のクエリ（区間和・区間最大値など）
//!
//! 乱数は [`crate::rand::xoshiro256ss::Xoshiro256StarStar`] で生成し、優先度によって木の高さを期望 O(log n) に保つ。
//!
//! # 計算量
//! 1回の [`insert`](ImplicitTreap::insert) / [`remove`](ImplicitTreap::remove) / [`get`](ImplicitTreap::get) /
//! [`set`](ImplicitTreap::set) / [`fold`](ImplicitTreap::fold) / [`apply`](ImplicitTreap::apply) /
//! [`reverse`](ImplicitTreap::reverse) は時間 O(log n)。
//! [`into_vec`](ImplicitTreap::into_vec) / [`to_vec`](ImplicitTreap::to_vec) は時間 O(n)。

use std::ops::{Bound, RangeBounds};
use crate::math::algebra::{MapMonoid, Reversible};
use crate::rand::Rand;
use crate::rand::xoshiro256ss::Xoshiro256StarStar;

const NIL: u32 = 0;
const LAZY: u8 = 1;
const REV: u8 = 2;

#[derive(Clone, Copy, Debug)]
struct Node<M: MapMonoid + Reversible> {
    ch: [u32; 2],
    size: u32,
    pri: u32,
    flags: u8,
    val: M::S,
    agg: M::S,
    lazy: M::F,
}

#[derive(Clone)]
// `<T as Monoid>::S`, `<T as MapMonoid>::F` が Debug を実装していないので無理
#[allow(missing_debug_implementations)]
/// 遅延伝搬反転可能乱択平衡二分探索木
///
/// キーを持たず、位置（index）だけで要素へアクセスする。
/// 要素の型は `M::S`、作用の型は `M::F` で、[`crate::math::algebra::MapMonoid`] と
/// [`crate::math::algebra::Reversible`] を実装した `M` で具体化する。
///
/// 詳しい使い方は [モジュールのドキュメント](self) を参照。
pub struct ImplicitTreap<M: MapMonoid + Reversible> {
    nodes: Vec<Node<M>>,
    free: Vec<u32>,
    root: u32,
    rng: Rand<Xoshiro256StarStar>,
}

impl<M: MapMonoid + Reversible> ImplicitTreap<M> {
    /// 要素数 `capacity` 分のノードを事前に確保して、空の [`ImplicitTreap`] を作成する。
    ///
    /// 最終的な要素数があらかじめ分かっている場合に、ノードプールの再確保を避けて構築できる。
    /// `capacity` を超えて要素を追加する場合は自動的にノードプールが伸長し、`remove` で解放したノードは再利用される。
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity + 1);
        nodes.push(Node {
            ch: [NIL; 2], size: 0, pri: 0, flags: 0,
            val: M::e(), agg: M::e(), lazy: M::id(),
        });

        Self { nodes, free: Vec::new(), root: NIL, rng: Rand::new(0x9E37_79B9_7F4A_7C15) }
    }

    /// 空の [`ImplicitTreap`] を作成する。
    ///
    /// ノードは [`ImplicitTreap::insert`] の呼び出しに応じて自動的に確保される。
    /// 要素数が見積もれる場合は [`ImplicitTreap::with_capacity`] を使うと効率的である。
    #[must_use]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// 要素数を返す。
    #[must_use]
    pub fn len(&self) -> usize {
        self.size(self.root) as usize
    }

    /// 要素数が 0 であるかを判定する。
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// 位置 `i` に要素 `x` を挿入する。
    ///
    /// `i` は 0 始まりの index で、`i == len()` のときは末尾に追加される。
    /// 挿入地点以降の要素の position は 1 つ後ろにずれる。
    ///
    /// # Panics
    /// - `i > len()` の場合。
    ///
    /// # 計算量
    /// 時間 O(log n)
    ///
    /// # Examples
    /// ```
    /// use atcoder::data_structure::implicit_treap::ImplicitTreap;
    /// use atcoder::math::algebra::{MapMonoid, Monoid, Reversible};
    ///
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
    /// impl Reversible for Sum {
    ///     fn rev(s: &Self::S) -> Self::S {
    ///         *s
    ///     }
    /// }
    ///
    /// impl MapMonoid for Sum {
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
    ///     fn mapping(a: &Self::F, b: &Self::S) -> Self::S {
    ///         b + a
    ///     }
    /// }
    ///
    /// let mut t = ImplicitTreap::<Sum>::new();
    /// for x in [1, 2, 3, 4, 5] {
    ///     t.insert(t.len(), x); // 末尾に追加
    /// }
    /// assert_eq!(t.len(), 5);
    ///
    /// t.insert(2, 30); // 3 と 4 の間に 30 を挿入
    /// assert_eq!(t.to_vec(), vec![1, 2, 30, 3, 4, 5]);
    ///
    /// assert_eq!(t.fold(1..4), 35); // 2 + 30 + 3
    /// t.apply(1..4, &10); // その区間に 10 を足す
    /// assert_eq!(t.to_vec(), vec![1, 12, 40, 13, 4, 5]);
    ///
    /// t.reverse(1..4);
    /// assert_eq!(t.to_vec(), vec![1, 13, 40, 12, 4, 5]);
    /// assert_eq!(t.fold(1..4), 65);
    ///
    /// t.set(0, 100); // 先頭を 100 に置き換える
    /// assert_eq!(t.get(0), 100);
    /// assert_eq!(t.fold(..), 100 + 13 + 40 + 12 + 4 + 5);
    ///
    /// let v = t.into_vec();
    /// assert_eq!(v, vec![100, 13, 40, 12, 4, 5]);
    /// ```
    pub fn insert(&mut self, i: usize, x: M::S) {
        assert!(i <= self.len());
        let (a, b) = self.split(self.root, i as u32);
        let n = self.alloc(x);
        let an = self.merge(a, n);
        self.root = self.merge(an, b);
    }

    /// 位置 `i` の要素を削除して、その値を返す。
    ///
    /// 削除地点以降の要素の position は 1 つ前にずれる。
    /// 削除されたノードは内部のフリーリストに戻され、次の [`ImplicitTreap::insert`] で再利用される。
    ///
    /// # Panics
    /// - `i >= len()` の場合。
    ///
    /// # 計算量
    /// 時間 O(log n)
    pub fn remove(&mut self, i: usize) -> M::S {
        assert!(i < self.len());
        let (a, bc) = self.split(self.root, i as u32);
        let (b, c) = self.split(bc, 1);
        self.root = self.merge(a, c);
        self.free.push(b);
        self.nodes[b as usize].val.clone()
    }

    /// 位置 `i` の要素を clone して返す。
    ///
    /// 要素を直接書き換える `&mut` を返すメソッドは用意していない。値の変更は [`ImplicitTreap::set`] を使う。
    ///
    /// # Panics
    /// - `i >= len()` の場合。
    ///
    /// # 計算量
    /// 時間 O(log n)
    pub fn get(&mut self, i: usize) -> M::S {
        assert!(i < self.len());
        let (mut t, mut i) = (self.root, i as u32);
        loop {
            self.push(t);
            let [l, r] = self.nodes[t as usize].ch;
            let ls = self.size(l);
            if i < ls { t = l; }
            else if i == ls { return self.nodes[t as usize].val.clone(); }
            else { i -= ls + 1; t = r; }
        }
    }

    /// 位置 `i` の要素を `x` に置き換える。
    ///
    /// たどり着くまでのノードに溜まっていた遅延作用は先に子へ伝搬されるので、新しい値 `x` に作用が適用されることはない。
    /// `x` にも作用を適用したい場合は [`crate::math::algebra::MapMonoid::mapping`] で変換してから設定する。
    ///
    /// # Panics
    /// - `i >= len()` の場合。
    ///
    /// # 計算量
    /// 時間 O(log n)
    pub fn set(&mut self, i: usize, x: M::S) {
        assert!(i < self.len());
        self.set_rec(self.root, i as u32, x);
    }

    /// 位置の区間 `r` の要素を [`crate::math::algebra::Monoid::op`] で畳み込んだ集約値を返す。
    ///
    /// `r` は [`std::ops::RangeBounds`] なので `a..b`・`a..=b`・`a..`・`..b`・`..` などが使える。
    /// 開始境界が `Excluded(x)` の場合は `x + 1` から、終了境界が `Included(x)` の場合は `x + 1` までを対象とする（内部的には半開区間 `[l, h)`）。
    /// 区間が空の場合は [`crate::math::algebra::Monoid::e`] を返す。
    ///
    /// # Panics
    /// - 区間の終端が `len()` を超える場合。
    /// - 開始境界が終了境界を超える場合（例: `4..2`）。
    ///
    /// # 計算量
    /// 時間 O(log n)
    pub fn fold(&mut self, r: impl RangeBounds<usize>) -> M::S {
        let (l, h) = self.bounds(r);
        self.fold_rec(self.root, l, h)
    }

    /// 位置の区間 `r` に作用 `f` を遅延適用する。
    ///
    /// 作用はノードに[`crate::math::algebra::MapMonoid::composition`] で合成して保持され、必要なときだけ子へ伝搬される。
    /// 区間がノードの担当範囲とちょうど一致する場合はそのノードに作用を置くだけで済む。
    /// `r` の指定方法は [`ImplicitTreap::fold`] と同じ。
    ///
    /// # Panics
    /// - 区間の終端が `len()` を超える場合。
    /// - 開始境界が終了境界を超える場合。
    ///
    /// # 計算量
    /// 時間 O(log n)
    pub fn apply(&mut self, r: impl RangeBounds<usize>, f: &M::F) {
        let (l, h) = self.bounds(r);
        self.apply_rec(self.root, l, h, f);
    }

    /// 位置の区間 `r` を反転する。
    ///
    /// 区間の要素数が 1 以下の場合は何もしない。
    /// 反転した部分木の集約値は [`crate::math::algebra::Reversible::rev`] で変換されるので、反転後の [`ImplicitTreap::fold`] も正しい値を返す。
    /// `r` の指定方法は [`ImplicitTreap::fold`] と同じ。
    ///
    /// # Panics
    /// - 区間の終端が `len()` を超える場合。
    /// - 開始境界が終了境界を超える場合。
    ///
    /// # 計算量
    /// 時間 O(log n)
    pub fn reverse(&mut self, r: impl RangeBounds<usize>) {
        let (l, h) = self.bounds(r);
        if h - l <= 1 { return; }
        let (a, bc) = self.split(self.root, l);
        let (b, c) = self.split(bc, h - l);
        self.toggle(b);
        let ab = self.merge(a, b);
        self.root = self.merge(ab, c);
    }

    /// `self` を消費して、要素を左から右へ [`Vec`] に移す。
    ///
    /// 行きがけ順にノードをたどりながら遅延作用を子へ伝搬させるので、未適用の [`ImplicitTreap::apply`] や
    /// [`ImplicitTreap::reverse`] も解決された結果が得られる。
    ///
    /// # Panics
    /// - 内部の `unwrap()` はスタックが空のときには到達しないため、実際にはパニックしない。
    ///
    /// # 計算量
    /// 時間 O(n), 空間 O(n)
    #[must_use]
    pub fn into_vec(mut self) -> Vec<M::S> {
        let mut out = Vec::with_capacity(self.len());
        let (mut st, mut t) = (Vec::new(), self.root);
        while t != NIL || !st.is_empty() {
            while t != NIL {
                self.push(t);
                st.push(t);
                t = self.nodes[t as usize].ch[0];
            }
            let u = st.pop().unwrap();
            out.push(self.nodes[u as usize].val.clone());
            t = self.nodes[u as usize].ch[1];
        }
        out
    }

    /// `self` を消費せずに、要素を左から右へ [`Vec`] に移す。
    ///
    /// `&self` なので [`ImplicitTreap`] の借用を保持したまま使える。
    /// 未適用の遅延作用・反転も [`ImplicitTreap::into_vec`] と同じく解決した結果を返す。
    ///
    /// # 計算量
    /// 時間 O(n), 空間 O(n)
    #[must_use]
    pub fn to_vec(&self) -> Vec<M::S> {
        let mut out = Vec::with_capacity(self.len());
        self.collect(self.root, false, None, &mut out);
        out
    }

    fn collect(&self, t: u32, rev: bool, f: Option<&M::F>, out: &mut Vec<M::S>) {
        if t == NIL { return; }
        let n = &self.nodes[t as usize];

        let cf: Option<M::F> = match (f, n.flags & LAZY != 0) {
            (Some(f), true) => Some(M::composition(f, &n.lazy)),
            (Some(f), false) => Some(f.clone()),
            (None, true) => Some(n.lazy.clone()),
            (None, false) => None,
        };

        let crev = rev ^ (n.flags & REV != 0);

        let [mut a, mut b] = n.ch;
        if rev { std::mem::swap(&mut a, &mut b); }

        self.collect(a, crev, cf.as_ref(), out);
        out.push(match f {
            Some(f) => M::mapping(f, &n.val),
            None => n.val.clone(),
        });
        self.collect(b, crev, cf.as_ref(), out);
    }

    fn alloc(&mut self, x: M::S) -> u32 {
        let node = Node {
            ch: [NIL; 2], size: 1, pri: self.rng.next_u32(),
            flags: 0, val: x.clone(), agg: x, lazy: M::id(),
        };

        if let Some(i) = self.free.pop() {
            self.nodes[i as usize] = node;
            i
        } else {
            self.nodes.push(node);
            self.nodes.len() as u32 - 1
        }
    }

    fn all_apply(&mut self, t: u32, f: &M::F) {
        if t == NIL { return; }
        let n = &mut self.nodes[t as usize];
        n.val = M::mapping(f, &n.val);
        n.agg = M::mapping(f, &n.agg);
        n.lazy = if n.flags & LAZY != 0 { M::composition(f, &n.lazy) } else { f.clone() };
        n.flags |= LAZY;
    }

    fn toggle(&mut self, t: u32) {
        if t == NIL { return; }
        let n = &mut self.nodes[t as usize];
        n.ch.swap(0, 1);
        n.agg = M::rev(&n.agg);
        n.flags ^= REV;
    }

    fn push(&mut self, t: u32) {
        let fl = self.nodes[t as usize].flags;
        if fl == 0 { return; }
        let [l, r] = self.nodes[t as usize].ch;
        if fl & LAZY != 0 {
            let f = self.nodes[t as usize].lazy.clone();
            self.all_apply(l, &f);
            self.all_apply(r, &f);
        }
        if fl & REV != 0 {
            self.toggle(l);
            self.toggle(r);
        }
        self.nodes[t as usize].flags = 0;
    }

    fn pull(&mut self, t: u32) {
        let [l, r] = self.nodes[t as usize].ch;
        let (nl, nt, nr) = (&self.nodes[l as usize], &self.nodes[t as usize], &self.nodes[r as usize]);
        let size = nl.size + 1 + nr.size;
        let agg = M::op(&M::op(&nl.agg, &nt.val), &nr.agg);
        let n = &mut self.nodes[t as usize];
        n.size = size;
        n.agg = agg;
    }

    fn split(&mut self, t: u32, k: u32) -> (u32, u32) {
        if t == NIL { return (NIL, NIL); }
        if k == 0 { return (NIL, t); }
        if k == self.size(t) { return (t, NIL); }
        self.push(t);
        let [l, r] = self.nodes[t as usize].ch;
        let ls = self.size(l);
        if k <= ls {
            let (a, b) = self.split(l, k);
            self.nodes[t as usize].ch[0] = b;
            self.pull(t);
            (a, t)
        } else {
            let (a, b) = self.split(r, k - ls - 1);
            self.nodes[t as usize].ch[1] = a;
            self.pull(t);
            (t, b)
        }
    }

    fn merge(&mut self, a: u32, b: u32) -> u32 {
        if a == NIL { return b; }
        if b == NIL { return a; }
        if self.nodes[a as usize].pri > self.nodes[b as usize].pri {
            self.push(a);
            let m = self.merge(self.nodes[a as usize].ch[1], b);
            self.nodes[a as usize].ch[1] = m;
            self.pull(a);
            a
        } else {
            self.push(b);
            let m = self.merge(a, self.nodes[b as usize].ch[0]);
            self.nodes[b as usize].ch[0] = m;
            self.pull(b);
            b
        }
    }

    fn fold_rec(&mut self, t: u32, l: u32, r: u32) -> M::S {
        if t == NIL || l >= r { return M::e(); }
        if l == 0 && r == self.size(t) { return self.nodes[t as usize].agg.clone(); }
        self.push(t);
        let [lc, rc] = self.nodes[t as usize].ch;
        let ls = self.size(lc);
        let mut ans = M::e();
        if l < ls { ans = self.fold_rec(lc, l, r.min(ls)); }
        if l <= ls && ls < r { ans = M::op(&ans, &self.nodes[t as usize].val); }
        if ls + 1 < r {
            let x = self.fold_rec(rc, l.saturating_sub(ls + 1), r - ls - 1);
            ans = M::op(&ans, &x);
        }
        ans
    }

    fn apply_rec(&mut self, t: u32, l: u32, r: u32, f: &M::F) {
        if t == NIL || l >= r { return; }
        if l == 0 && r == self.size(t) { self.all_apply(t, f); return; }
        self.push(t);
        let [lc, rc] = self.nodes[t as usize].ch;
        let ls = self.size(lc);
        if l < ls { self.apply_rec(lc, l, r.min(ls), f); }
        if l <= ls && ls < r {
            let n = &mut self.nodes[t as usize];
            n.val = M::mapping(f, &n.val);
        }
        if ls + 1 < r { self.apply_rec(rc, l.saturating_sub(ls + 1), r - ls - 1, f); }
        self.pull(t);
    }

    fn set_rec(&mut self, t: u32, i: u32, x: M::S) {
        self.push(t);
        let [l, r] = self.nodes[t as usize].ch;
        let ls = self.size(l);
        if i < ls { self.set_rec(l, i, x); }
        else if i == ls { self.nodes[t as usize].val = x; }
        else { self.set_rec(r, i - ls - 1, x); }
        self.pull(t);
    }

    fn bounds(&self, r: impl RangeBounds<usize>) -> (u32, u32) {
        let n = self.len();
        let l = match r.start_bound() { Bound::Included(&x) => x, Bound::Excluded(&x) => x + 1, Bound::Unbounded => 0 };
        let h = match r.end_bound() { Bound::Included(&x) => x + 1, Bound::Excluded(&x) => x, Bound::Unbounded => n };
        assert!(l <= h && h <= n);
        (l as u32, h as u32)
    }

    #[inline(always)]
    fn size(&self, t: u32) -> u32 { self.nodes[t as usize].size }
}

impl<M: MapMonoid + Reversible> Default for ImplicitTreap<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: MapMonoid + Reversible> FromIterator<M::S> for ImplicitTreap<M> {
    fn from_iter<T: IntoIterator<Item=M::S>>(it: T) -> Self {
        let it = it.into_iter();
        let mut s = Self::with_capacity(it.size_hint().0);
        let mut stack: Vec<u32> = Vec::new();
        for x in it {
            let t = s.alloc(x);
            let mut last = NIL;
            while let Some(&top) = stack.last() {
                if s.nodes[top as usize].pri >= s.nodes[t as usize].pri { break; }
                s.pull(top);
                last = top;
                _ = stack.pop();
            }
            s.nodes[t as usize].ch[0] = last;
            if let Some(&top) = stack.last() { s.nodes[top as usize].ch[1] = t; }
            stack.push(t);
        }
        if let Some(&r) = stack.first() { s.root = r; }
        while let Some(t) = stack.pop() { s.pull(t); } // 深い方から確定させる
        s
    }
}
