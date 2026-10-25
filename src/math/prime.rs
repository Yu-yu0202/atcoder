#![allow(long_running_const_eval)]

//! 素数篩・素数判定 など

use crate::ext::iter_ext::IterExt;

/// SPF を利用した線形篩
///
/// `self.0` は SPF の配列である。
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct LinearSieve(Vec<u32>);

impl LinearSieve {
    /// 線形篩を与えられた上限 `n` で構築する。
    ///
    /// # 計算量
    /// 時間 O(n), 空間 O(n)
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
    /// # 計算量
    /// [`Self::new`] の doc を参照
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
                    j += 1;
                }
            }
            i += 1;
        }
    }

    /// `n` の SPF を返す。
    #[inline]
    #[must_use]
    pub fn spf(&self, n: usize) -> u32 {
        self.0[n]
    }

    /// `n` が素数かどうかを返す。
    ///
    /// # 計算量
    /// 時間 O(1)
    #[inline]
    #[must_use]
    pub fn is_prime(&self, n: u32) -> bool {
        n >= 2 && self.0[n as usize] == n
    }

    /// `n` が合成数かどうかを返す。
    ///
    /// # 計算量
    /// 時間 O(1)
    #[inline]
    #[must_use]
    pub fn is_composite(&self, n: u32) -> bool {
        n >= 2 && self.0[n as usize] != n
    }

    /// `n` の素因数分解を行い、結果を `(素因数: u32, 指数: usize)` の [`Vec`] で返す。
    ///
    /// # Panics
    /// `n` が 0 の場合にパニックする。
    ///
    /// # 計算量
    /// 時間 O(log n)
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

/// エラトステネスの篩
#[derive(Clone, Debug)]
pub struct PrimeSieve {
    limit: usize,
    odd: Vec<u64>,
    primes: Vec<u64>,
}

impl PrimeSieve {
    /// エラトステネスの篩を与えられた上限 `limit` で構築する。
    ///
    /// # 計算量
    /// 時間 O(N log log N)、空間 O(N)
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

    /// エラトステネスの篩をコンパイル時定数評価で構築する。
    ///
    /// # 計算量
    /// 時間 O(N log log N)、空間 O(N)
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
    /// # Safety
    /// `n` は `self.limit` 以下である必要がある。
    ///
    /// そうでない場合、内部の `self.odd` のインデックスアクセスによって panic する。
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
    /// # 計算量
    /// 時間 O(1)
    ///
    /// # Panics
    /// `n` が `self.limit` 以上の場合、 panic する。
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
    #[inline]
    #[must_use]
    pub const fn limit(&self) -> usize {
        self.limit
    }

    /// 素数の配列を返す。
    #[inline]
    #[must_use]
    pub fn primes(&self) -> &[u64] {
        &self.primes
    }

    /// 素数の個数を返す。
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.primes.len()
    }

    /// 素数の配列が空かどうかを返す。
    ///
    /// `self.len() == 0` と同義である。
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.primes.is_empty()
    }
}
