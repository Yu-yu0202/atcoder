#![allow(long_running_const_eval)]

//! 素数篩・素数判定・素因数分解。
//!
//! ある上限までの素数判定や素因数分解を繰り返す場面では、毎回試し割りするの
//! ではなく篩（ふるい）を 1 回構築すると全体の計算量が大きく減る。
//!
//! - [`LinearSieve`] — 各整数の最小素因数（SPF）を並べた配列を持つ篩。
//!   [`LinearSieve::factorize`] で任意の数を O(log n) で素因数分解でき、
//!   [`LinearSieve::is_prime`] も O(1) で判定できる。
//! - [`PrimeSieve`] — 上限以下の奇数を 1 個あたり 1 ビットで保持するビット篩。
//!   素数全体を [`PrimeSieve::primes`] で昇順に取り出せるため、素数を列挙して
//!   走査する問題に向く。
//!
//! ## SPF（Smallest Prime Factor, 最小素因数）とは
//! `spf(n)` は `n` を割り切る最小の素数のことである。例えば `12 = 2^2 * 3`
//! なので `spf(12) = 2`、`35 = 5 * 7` なので `spf(35) = 5` になる。
//! 素数 `p` 自身は `spf(p) = p` であり、これが `spf(n) == n` による O(1) の
//! 素数判定にも、`n /= spf(n)` を繰り返す O(log n) の素因数分解にも使える。
//! ただし `0` と `1` は素因数を持たないため、`spf(0) = 0`・`spf(1) = 1` という
//! 特別値を持つ（`1` は素数ではないので注意）。
//!
//! ## どちらを使うか
//! - 素因数分解や約数列挙を大量に行う → [`LinearSieve`]（分解が O(log n)）
//! - 素数を列挙したい / 素数判定だけを大量に行う → [`PrimeSieve`]
//!   （メモリが小さく、素数の一覧が得られる。分解はできない）
//!
//! どちらも `new`（実行時に構築）と `new_const`（コンパイル時に構築）を持つ。
//! `new_const` は実行時のコストをほぼ消せるが、そのぶんコンパイル時間と
//! バイナリサイズが増えるため、上限は数千〜数百万程度にとどめるのが現実的である。
//!
//! # Examples
//! ```
//! use atcoder::math::prime::{LinearSieve, PrimeSieve};
//!
//! // 10 以下の素数を列挙する。
//! let sieve = PrimeSieve::new(10);
//! // assert_eq!(sieve.primes(), &[2_u64, 3, 5, 7][..]);
//! // assert!(sieve.is_prime(7));
//! // assert!(!sieve.is_prime(9));
//!
//! // 素因数分解（上限 1000 まで）。
//! let linear = LinearSieve::new(1000);
//! // assert_eq!(linear.factorize(360), [(2, 3), (3, 2), (5, 1)]);
//! ```

use crate::ext::iter_ext::IterExt;

/// SPF を利用した線形篩。
///
/// `0` から上限 `n` までの各整数について、その SPF（最小素因数）を `u32` で
/// 1 個ずつ並べた配列を内部に持つ。添字はそのまま数に対応し、
/// `self.0[i]` が `i` の SPF である。
///
/// 値の規則は以下の通り。
/// - `spf[0] = 0`・`spf[1] = 1`（特別値）
/// - 素数 `p` については `spf[p] = p`（`spf(n) == n` による素数判定の根拠）
/// - 合成数 `m` については `spf[m] < m`
///
/// 構築時に上限を決める必要があり、それを超える数に対する操作は内部配列の
/// 範囲外アクセスとなりパニックする。上限そのものは公開されないので、
/// 必要な最大値を間違えずに渡すこと。
///
/// 実装は素数 `p <= sqrt(n)` ごとに `p^2` 以上の倍数を巡る方式で、各合成数の
/// SPF は最小素因数によって 1 度だけ書かれる（素数 1 個あたりの走査量の和は
/// `n / p` の総和なので `O(n log log n)`）。厳密な意味では O(n) の線形篩では
/// ないが、実用上は O(n) として扱ってよい。
///
/// # Examples
/// ```
/// use atcoder::math::prime::LinearSieve;
///
/// let sieve = LinearSieve::new(30);
/// assert_eq!(sieve.spf(12), 2);
/// assert_eq!(sieve.factorize(30), [(2, 1), (3, 1), (5, 1)]);
/// assert!(sieve.is_prime(29));
/// ```
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct LinearSieve(Vec<u32>);

impl LinearSieve {
    /// 線形篩を与えられた上限 `n` で構築する。
    ///
    /// 構築後は `0` 以上 `n` 以下のすべての整数について [`Self::spf`]・
    /// [`Self::is_prime`]・[`Self::is_composite`]・[`Self::factorize`] が使える。
    /// `n` は上限として**含む**ので、`n` 自身も判定・分解の対象になる。
    ///
    /// # 計算量
    /// 時間 O(n log log n)、空間 O(n)（`Vec<u32>` で 4(n+1) バイト程度）
    ///
    /// # 備考
    /// この関数はSPFを実行中に計算する。
    /// コンパイル時定数評価を行う場合は [`Self::new_const`] を利用すること。
    #[inline]
    #[must_use]
    pub fn new(n: usize) -> Self {
        let mut spf = (0..=n as u32).collect_vec();
        Self::calc_spf(&mut spf);

        Self(spf)
    }

    /// 線形篩をコンパイル時定数評価で構築する。
    ///
    /// SPF の配列を `const` ブロックで埋めてから `Vec` に移すため、実行時の
    /// コストは配列のコピーだけになる。そのぶんバイナリサイズが増えるため、
    /// 上限が巨大的な場合には [`Self::new`] のほうが向く。
    ///
    /// 型引数 `N` は内部配列の**長さ**であり、扱える上限は `N - 1` になる
    /// （[`Self::new`]`(N - 1)` と同値）。なお [`PrimeSieve::new_const`] では
    /// `N` が上限そのものを表す点と異なるので注意。
    ///
    /// # 計算量
    /// [`Self::new`] の doc を参照
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::prime::LinearSieve;
    ///
    /// // 長さ 11 = 上限 10（`LinearSieve::new(10)` と同値）。
    /// let sieve = LinearSieve::new_const::<11>();
    /// assert_eq!(sieve.spf(10), 2);
    /// assert!(sieve.is_prime(7));
    /// assert_eq!(sieve.factorize(10), [(2, 1), (5, 1)]);
    /// ```
    #[inline]
    #[must_use]
    pub fn new_const<const N: usize>() -> Self {
        let spf = const {
            let mut spf = [0; N];
            Self::calc_spf(&mut spf);

            spf
        };

        Self(spf.to_vec())
    }

    // がしゃーん
    // 　　　　　がしゃーん
    //
    // 　　　△ ￥ ▲
    // 　　( ㊤ 皿 ㊤ )
    // 　　(　　　　　)
    // 　／│　 肉　 │＼
    // ＜　 ＼＿＿＿／　 ＞
    // 　　　┃　　┃
    // 　　　＝　　＝
    //
    // SPF定数計算ロボだよ
    // 自動でSPFの計算をcompile-timeに落としてくれるすごいやつだよ
    const fn calc_spf(spf: &mut [u32]) {
        let len = spf.len();
        let mut k = 0;

        while k < len {
            spf[k] = k as u32;
            k += 1;
        }

        if len <= 1 {
            return;
        }
        spf[1] = 1;

        let n = len - 1;
        let mut i = 2;

        while i <= n / i {
            if spf[i] == i as u32 {
                let mut j = i * i;

                while j <= n {
                    if spf[j] == j as u32 {
                        spf[j] = i as u32;
                    }
                    j += i;
                }
            }
            i += 1;
        }
    }

    /// `n` の SPF（最小素因数）を返す。
    ///
    /// `spf(n)` は `n` を割り切る最小の素数である。ただし `n = 0` では 0・
    /// `n = 1` では 1 という特別値を返し、素数 `p` に対しては `spf(p) == p`
    /// になる。
    ///
    /// # Panics
    /// - `n` が構築時の上限を超える場合（内部配列の範囲外アクセス）。
    ///
    /// # 計算量
    /// 時間 O(1)
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::prime::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(100);
    /// assert_eq!(sieve.spf(12), 2); // 12 = 2^2 * 3
    /// assert_eq!(sieve.spf(7), 7); // 素数は自分自身
    /// assert_eq!(sieve.spf(1), 1);
    /// ```
    #[inline]
    #[must_use]
    pub fn spf(&self, n: usize) -> u32 {
        self.0[n]
    }

    /// `n` が素数かどうかを返す。
    ///
    /// `n >= 2 && spf(n) == n` で判定する。`0`・`1` は素数ではないので
    /// `false` を返す。`n` の型は `u32` なので、上限も `u32` の範囲に収まる
    /// 必要がある（実用上はメモリの都合でそれよりずっと小さい上限になる）。
    ///
    /// # Panics
    /// - `n` が構築時の上限を超える場合（内部配列の範囲外アクセス）。
    ///
    /// # 計算量
    /// 時間 O(1)
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::prime::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(100);
    /// assert!(sieve.is_prime(97));
    /// assert!(!sieve.is_prime(91)); // 91 = 7 * 13
    /// assert!(!sieve.is_prime(1));
    /// ```
    #[inline]
    #[must_use]
    pub fn is_prime(&self, n: u32) -> bool {
        n >= 2 && self.0[n as usize] == n
    }

    /// `n` が合成数かどうかを返す。
    ///
    /// `n >= 2 && spf(n) != n` で判定する。`0` と `1` は合成数ではない
    /// （素数でもない）ので `false` を返す。
    ///
    /// # Panics
    /// - `n` が構築時の上限を超える場合（内部配列の範囲外アクセス）。
    ///
    /// # 計算量
    /// 時間 O(1)
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::prime::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(100);
    /// assert!(sieve.is_composite(91));
    /// assert!(!sieve.is_composite(97));
    /// // 0 と 1 は合成数ではない。
    /// assert!(!sieve.is_composite(0));
    /// assert!(!sieve.is_composite(1));
    /// ```
    #[inline]
    #[must_use]
    pub fn is_composite(&self, n: u32) -> bool {
        n >= 2 && self.0[n as usize] != n
    }

    /// `n` の素因数分解を行い、結果を `(素因数: u32, 指数: usize)` の [`Vec`] で返す。
    ///
    /// 素因数の昇順に並び、`n == Π p_i^e_i` を満たすタプル列になる。
    /// `n == 1` のときは何の素因数も持たないので空の [`Vec`] を返す。
    /// 約数列挙や約数の個数の計算などの前処理として使う。
    ///
    /// # Panics
    /// - `n` が 0 の場合。
    /// - `n` が構築時の上限を超える場合（内部配列の範囲外アクセス）。
    ///
    /// # 計算量
    /// 時間 O(log n)、空間 O(log n)（`n` の素因数の個数＝相異なる素因数の個数まで）
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::prime::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(1000);
    /// assert_eq!(sieve.factorize(360), [(2, 3), (3, 2), (5, 1)]);
    ///
    /// // 1 は素因数を持たない。
    /// assert!(sieve.factorize(1).is_empty());
    ///
    /// // 素数はその自分自身 1 個だけ。
    /// assert_eq!(sieve.factorize(997), [(997, 1)]);
    /// ```
    #[track_caller]
    #[must_use]
    pub fn factorize(&self, mut n: u32) -> Vec<(u32, usize)> {
        assert!(n > 0, "0 cannot be factorized");
        let mut result = Vec::new();

        while n > 1 {
            let p = self.0[n as usize];

            let mut cnt = 0;
            while n.is_multiple_of(p) {
                n /= p;
                cnt += 1;
            }

            result.push((p, cnt));
        }

        result
    }
}

/// エラトステネスの篦。
///
/// 上限 `limit` 以下の奇数を 1 ビットずつ詰めたビット集合（`odd`）と、上限以下の
/// 素数列（`primes`）を保持する。奇数のビット集合のみを持つことで
/// 偶数を間違って素数と判定する心配がなく、2 だけを特別扱いしている。
///
/// [`LinearSieve`] と違って各数の最小素因数（SPF）は持たないため
/// [`LinearSieve::factorize`] のような O(log n) の素因数分解はできないが、
/// 素数そのものの一覧を [`Self::primes`] で取り出せるため、
/// 「素数全体を舐めて何かする」問題ではこちらが向く。
///
/// # Examples
/// ```
/// use atcoder::math::prime::PrimeSieve;
///
/// let sieve = PrimeSieve::new(100);
/// assert_eq!(sieve.limit(), 100);
/// assert_eq!(sieve.primes()[..4], [2, 3, 5, 7]);
/// assert!(sieve.is_prime(97));
/// assert!(!sieve.is_prime(91)); // 91 = 7 * 13
/// ```
#[derive(Clone, Debug)]
pub struct PrimeSieve {
    limit: usize,
    odd: Vec<u64>,
    primes: Vec<u64>,
}

impl PrimeSieve {
    /// エラトステネスの篦を与えられた上限 `limit` で構築する。
    ///
    /// 対象は `2` 以上 `limit` 以下の整数で、`limit` 自身も含む。
    /// 0 と 1 は素数ではない（篦の外）。
    ///
    /// # 計算量
    /// 時間 O(N log log N)、空間 O(N)
    /// （`N = limit`。奇数 1 個あたり 1 ビット、素数 1 個あたり 8 バイト）
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::prime::PrimeSieve;
    ///
    /// let sieve = PrimeSieve::new(50);
    /// assert_eq!(sieve.limit(), 50);
    /// assert_eq!(sieve.primes()[..5], [2, 3, 5, 7, 11]);
    /// assert_eq!(sieve.len(), 15);
    ///
    /// // limit 未満は必ず判定でき、limit ちょうども含む。
    /// assert!(sieve.is_prime(47));
    /// assert!(!sieve.is_prime(49));
    /// ```
    #[must_use]
    pub fn new(limit: usize) -> Self {
        let len = limit / 2 + limit % 2;
        let word_len = len.div_ceil(64);

        let mut odd = vec![0u64; word_len];
        Self::calc_odd(&mut odd, limit);

        let mut primes = Vec::new();

        if limit >= 2 {
            primes.push(2);
        }

        for i in 1..len {
            if odd[i / 64] & (1u64 << (i % 64)) != 0 {
                primes.push((2 * i + 1) as u64);
            }
        }

        Self { limit, odd, primes }
    }

    /// エラトステネスの篦をコンパイル時定数評価で構築する。
    ///
    /// 奇数ビット集合と素数列の両方をコンパイル時に埋めてしまうため、
    /// 実行時にかかるのは `Vec` へのコピーだけになる。制約が小さく
    /// 固定で、素数判定を大量に行う場面では [`Self::new`] より速い。
    ///
    /// 型引数 `N` が上限そのものになる。配列サイズが `N` 語なので
    /// `N` を大きくしすぎるとコンパイル時間とバイナリサイズが膨らむ。
    ///
    /// # 計算量
    /// 時間 O(N log log N)、空間 O(N)
    /// （コンパイル時。実行時はコピーの O(N)）
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::prime::PrimeSieve;
    ///
    /// // コンパイル時に 100 以下の篦が埋まる。
    /// let sieve = PrimeSieve::new_const::<100>();
    /// assert_eq!(sieve.limit(), 100);
    /// assert_eq!(sieve.primes()[..5], [2, 3, 5, 7, 11]);
    /// assert!(sieve.is_prime(89));
    /// ```
    #[must_use]
    pub fn new_const<const N: usize>() -> Self {
        let (data, word_len, prime_len) = const {
            let mut data = [0u64; N];

            Self::calc_odd(&mut data, N);

            let len = N / 2 + N % 2;

            let mut word_len = len / 64;
            if !len.is_multiple_of(64) {
                word_len += 1;
            }

            let mut prime_len = 0;

            if N >= 2 {
                data[word_len + prime_len] = 2;
                prime_len += 1;
            }

            let mut i = 1;

            while i < len {
                if data[i / 64] & (1u64 << (i % 64)) != 0 {
                    data[word_len + prime_len] = (2 * i + 1) as u64;
                    prime_len += 1;
                }

                i += 1;
            }

            (data, word_len, prime_len)
        };

        Self {
            limit: N,
            odd: data[..word_len].to_vec(),
            primes: data[word_len..word_len + prime_len].to_vec(),
        }
    }

    // がしゃーん
    // 　　　　　がしゃーん
    //
    // 　　　△ ￥ ▲
    // 　　( ㊤ 皿 ㊤ )
    // 　　(　　　　　)
    // 　／│　 肉　 │＼
    // ＜　 ＼＿＿＿／　 ＞
    // 　　　┃　　┃
    // 　　　＝　　＝
    //
    // 奇数集合初期化ロボだよ
    // 自動で奇数集合の初期化をcompile-timeに落としてくれるすごいやつだよ
    const fn calc_odd(odd: &mut [u64], limit: usize) {
        let len = limit / 2 + limit % 2;

        let mut word_len = len / 64;
        if !len.is_multiple_of(64) {
            word_len += 1;
        }

        let mut i = 0;

        while i < word_len {
            odd[i] = u64::MAX;
            i += 1;
        }

        if !len.is_multiple_of(64) {
            odd[word_len - 1] &= (1u64 << (len % 64)) - 1;
        }

        if len > 0 {
            odd[0] &= !1u64;
        }

        let mut p = 3;

        while p <= limit / p {
            let idx = p / 2;

            if odd[idx / 64] & (1u64 << (idx % 64)) != 0 {
                let mut j = p * p / 2;

                while j < len {
                    odd[j / 64] &= !(1u64 << (j % 64));
                    j += p;
                }
            }

            p += 2;
        }
    }
    #[inline]
    fn get(&self, i: usize) -> bool {
        self.odd[i / 64] & (1u64 << (i % 64)) != 0
    }

    /// 素数かどうかを境界チェック無しで判定する。
    ///
    /// `self.limit` 以下であることが呼び出し側で保証できるなら、境界チェックの
    /// 1 回分だけ [`Self::is_prime`] より速い。判定内容は
    /// [`Self::is_prime`] と完全に同じである。
    ///
    /// 0 と 1 は素数ではない。2 は唯一の偶数の素数なので特別に真を返し、
    /// それ以外の偶数は偽、奇数は `self.odd` のビット集合を引く。
    ///
    /// # Safety
    /// `n` は `self.limit` 以下である必要がある。
    ///
    /// そうでない場合、内部の `self.odd` のインデックスアクセスによって panic する。
    ///
    /// # 計算量
    /// 時間 O(1)
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::prime::PrimeSieve;
    ///
    /// let sieve = PrimeSieve::new(100);
    /// let n = 97;
    ///
    /// // SAFETY: `n` は `self.limit`（= 100）以下である。
    /// unsafe {
    ///     assert!(sieve.is_prime_unchecked(n));
    ///     assert!(!sieve.is_prime_unchecked(n + 1));
    /// }
    /// ```
    #[inline]
    #[track_caller]
    #[must_use]
    pub unsafe fn is_prime_unchecked(&self, n: usize) -> bool {
        match n {
            0 | 1 => false,
            2 => true,
            _ if n.is_multiple_of(2) => false,
            _ => self.get(n / 2),
        }
    }

    /// 素数かどうかを判定する。
    ///
    /// 0 と 1 は素数ではなく、2 は唯一の偶数の素数である。`self.limit` を
    /// 超える値は判定できないので、範囲内かのチェックを入れた安全版である
    /// （判定の中身は [`Self::is_prime_unchecked`] と共通）。
    ///
    /// # 計算量
    /// 時間 O(1)
    ///
    /// # Panics
    /// `n` が `self.limit` より大きい場合にパニックする。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::prime::PrimeSieve;
    ///
    /// let sieve = PrimeSieve::new(100);
    /// assert!(sieve.is_prime(2));
    /// assert!(sieve.is_prime(97));
    /// assert!(!sieve.is_prime(91)); // 91 = 7 * 13
    /// assert!(!sieve.is_prime(0));
    /// assert!(!sieve.is_prime(1));
    ///
    /// // limit ちょうどまでは含む。
    /// assert!(sieve.is_prime(100) == false);
    /// ```
    #[inline]
    #[track_caller]
    #[must_use]
    pub fn is_prime(&self, n: usize) -> bool {
        assert!(n <= self.limit, "n exceeds `self.limit`");
        unsafe {
            // SAFETY: `assert` 済み
            self.is_prime_unchecked(n)
        }
    }

    /// 篩の上限を返す。
    ///
    /// 構築時に渡した `limit` をそのまま返す。この値は [`Self::is_prime`] が
    /// 判定できる最大値と一致する（`limit` 自身も含む）。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::prime::PrimeSieve;
    ///
    /// assert_eq!(PrimeSieve::new(10).limit(), 10);
    /// assert_eq!(PrimeSieve::new_const::<64>().limit(), 64);
    /// ```
    #[inline]
    #[must_use]
    pub const fn limit(&self) -> usize {
        self.limit
    }

    /// 素数の配列を返す。
    ///
    /// `2` から始まり昇順に並んだスライスで、要素数は [`Self::len`] と一致する。
    /// `limit < 2` のときは空スライスになる。
    ///
    /// # 計算量
    /// 時間 O(1)（スライスの取得のみ。構築時の列挙済み配列を返す）
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::prime::PrimeSieve;
    ///
    /// let sieve = PrimeSieve::new(20);
    /// assert_eq!(sieve.primes(), &[2, 3, 5, 7, 11, 13, 17, 19][..]);
    ///
    /// // 上限が 2 未満なら 1 個もない。
    /// assert!(PrimeSieve::new(1).primes().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn primes(&self) -> &[u64] {
        &self.primes
    }

    /// 素数の個数を返す。
    ///
    /// `self.limit` 以下の素数の個数（素数計数関数）であり、
    /// [`Self::primes`] の長さと一致する。`limit < 2` なら 0。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::prime::PrimeSieve;
    ///
    /// assert_eq!(PrimeSieve::new(10).len(), 4); // 2, 3, 5, 7
    /// assert_eq!(PrimeSieve::new(1).len(), 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.primes.len()
    }

    /// 素数の配列が空かどうかを返す。
    ///
    /// `self.len() == 0` と同義である。`limit < 2` のときに限り真になる。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::prime::PrimeSieve;
    ///
    /// assert!(!PrimeSieve::new(10).is_empty());
    /// assert!(PrimeSieve::new(1).is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.primes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_sieve_new_small() {
        let sieve = LinearSieve::new(30);

        assert_eq!(sieve.spf(1), 1);
        assert_eq!(sieve.spf(2), 2);
        assert_eq!(sieve.spf(4), 2);
        assert_eq!(sieve.spf(5), 5);
        assert_eq!(sieve.spf(12), 2);
        assert_eq!(sieve.spf(30), 2);
        assert_eq!(sieve.spf(29), 29);

        assert!(sieve.is_prime(29));
        assert!(!sieve.is_prime(1));
        assert!(sieve.is_composite(30));
        assert!(!sieve.is_composite(29));

        assert_eq!(sieve.factorize(30), [(2, 1), (3, 1), (5, 1)]);
        assert_eq!(sieve.factorize(12), [(2, 2), (3, 1)]);
        assert!(sieve.factorize(1).is_empty());
    }

    #[test]
    fn linear_sieve_new_1000() {
        let sieve = LinearSieve::new(1000);

        assert_eq!(sieve.spf(360), 2);
        assert_eq!(sieve.factorize(360), [(2, 3), (3, 2), (5, 1)]);
        assert_eq!(sieve.factorize(997), [(997, 1)]);
        assert_eq!(sieve.factorize(1).len(), 0);
    }

    #[test]
    fn prime_sieve_basic() {
        let sieve = PrimeSieve::new(100);

        assert_eq!(sieve.limit(), 100);
        assert_eq!(sieve.primes()[..5], [2, 3, 5, 7, 11]);
        assert_eq!(sieve.len(), 25);
        assert!(!sieve.is_empty());

        assert!(sieve.is_prime(97));
        assert!(!sieve.is_prime(91));
        assert!(!sieve.is_prime(1));
    }

    #[test]
    fn prime_sieve_new_const() {
        let sieve = PrimeSieve::new_const::<100>();

        assert_eq!(sieve.limit(), 100);
        assert_eq!(sieve.primes()[..5], [2, 3, 5, 7, 11]);
        assert!(sieve.is_prime(89));
    }
}
