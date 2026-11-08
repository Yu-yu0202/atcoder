//! 誤差なしで有理数を扱うための分数型。
//!
//! [`Fraction`] は分子・分母を [`u64`] で持ち、常に約分した正規形（`gcd(num, den) == 1`）を保つ。
//! 分母を明示的に持つため [`f64`] のような浮動小数点の誤差が生じない。
//! 符号は [`Sign`] として分離しているので、分子・分母はそれぞれ非負で扱える。
//!
//! 演算には 2 系統ある。
//! - `checked_*` 系: オーバーフローを [`None`] で返す安全な演算
//! - `Add` 等の演算子: `checked_*` をもとに、オーバーフロー時は panic する
//!
//! 内部では四則演算のたびに交差約分（[`Fraction::cross_reduce`]）と
//! 公倍数の縮小を行うため、オーバーフローは [`u64`] 同士の乗算まで抑えられる
//! （ただし分子・分母の積が [`u128`] でも収まらない極端な値では `None` を返す）。
//!
//! # Examples
//! ```
//! use atcoder::math::fraction::Fraction;
//!
//! let a = Fraction::new(1, 3);
//! let b = Fraction::new(1, 6);
//!
//! // 誤差なく 1/2 になる（浮動小数点では 0.5 付近の近似になる）。
//! assert_eq!(a + b, Fraction::new(1, 2));
//!
//! // オーバーフロー時は panic せず None が得られる。
//! assert!(a.checked_add(Fraction::new(1, 6)).is_some());
//! ```

use crate::macros::if_none_return;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// [`Fraction`] の符号。
///
/// [`u64`] と組で使うと非負の有理数全域を、[`i64`] と組で使うと符号付き有理数全域を
/// 表せる。`0` は正負の区別がないため [`Sign::Zero`] に正規化される。
///
/// 定数値は [`i8`] から [`repr(i8)`] で読み出せるため、[`Sign::as_i8`] は
/// 単なるキャストではなく値の読み出しになる。
///
/// # Examples
/// ```
/// use atcoder::math::fraction::Sign;
///
/// assert_eq!(Sign::Positive.as_i8(), 1);
/// assert_eq!(Sign::Zero.as_i8(), 0);
/// assert_eq!(Sign::Negative.as_i8(), -1);
/// ```
#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Sign {
    /// ゼロ。
    ///
    /// [`Fraction`] では `0/1` に正規化され、分子・分母は無視される。
    Zero = 0,
    /// 正の（有理）数。
    ///
    /// [`Fraction`] では `num > 0 && den > 0` に対応する。
    Positive = 1,
    /// 負の（有理）数。
    ///
    /// [`Fraction`] では符号が負で、絶対値は `num / den` になる。
    Negative = -1,
}

impl Sign {
    /// 符号を [`i8`] の -1, 0, 1 に変換する。
    ///
    /// [`repr(i8)`] なので実体はキャストである。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Sign;
    ///
    /// assert_eq!(Sign::Positive.as_i8(), 1);
    /// assert_eq!(Sign::Negative.as_i8(), -1);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_i8(self) -> i8 {
        self as _
    }

    /// 符号を [`i32`] の -1, 0, 1 に変換する。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Sign;
    ///
    /// assert_eq!(Sign::Zero.as_i32(), 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self.as_i8() as _
    }

    /// 符号を [`i64`] の -1, 0, 1 に変換する。
    ///
    /// 分子・分母（[`u64`]）と掛け合わせて符号付きの値を復元する用途に向く。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::{Fraction, Sign};
    ///
    /// let f = Fraction::new(-2, 3);
    /// assert_eq!(f.numerator_abs() as i64 * f.sign().as_i64(), -2);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_i64(self) -> i64 {
        self.as_i8() as _
    }

    /// 符号同士を掛ける。
    ///
    /// [`Sign::Zero`] を絡めると必ず [`Sign::Zero`] になる（吸収元）。
    /// 同符号どうしは [`Sign::Positive`]、異符号は [`Sign::Negative`]。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Sign;
    ///
    /// assert_eq!(Sign::Positive * Sign::Negative, Sign::Negative);
    /// assert_eq!(Sign::Negative * Sign::Negative, Sign::Positive);
    /// assert_eq!(Sign::Zero * Sign::Positive, Sign::Zero);
    /// ```
    #[inline]
    #[must_use]
    pub const fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Zero, _) | (_, Self::Zero) => Self::Zero,
            (Self::Positive, Self::Positive) | (Self::Negative, Self::Negative) => Self::Positive,
            _ => Self::Negative,
        }
    }
}

impl Neg for Sign {
    type Output = Self;

    /// 符号を反転する。
    ///
    /// `Sign::Zero` はそのままで、`Sign::Positive` と `Sign::Negative` が入れ替わる。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Sign;
    ///
    /// assert_eq!(-Sign::Positive, Sign::Negative);
    /// assert_eq!(-Sign::Zero, Sign::Zero);
    /// ```
    fn neg(self) -> Self::Output {
        match self {
            Self::Positive => Self::Negative,
            Self::Negative => Self::Positive,
            Self::Zero => Self::Zero,
        }
    }
}

impl Mul<Self> for Sign {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self.mul(rhs)
    }
}

impl Mul<u64> for Sign {
    type Output = i64;

    /// 符号を `u64` の絶対値に適用して符号付きの `i64` を得る。
    ///
    /// `rhs` が `i64::MAX` を超える場合は `i64` に収まらず、値が反転する。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Sign;
    ///
    /// assert_eq!(Sign::Negative * 3, -3);
    /// assert_eq!(Sign::Positive * 3, 3);
    /// ```
    #[inline]
    #[track_caller]
    fn mul(self, rhs: u64) -> i64 {
        match self {
            Self::Positive => rhs as i64,
            Self::Negative => -(rhs as i64),
            Self::Zero => 0,
        }
    }
}

/// 誤差なしで有理数を表現できる分数型。
///
/// 値は `sign * (num / den)` であり、常に約分済みの正規形（`gcd(num, den) == 1`）を保つ。
/// `0` は `Sign::Zero`・`num = 0`・`den = 1` に、`1` は `den = 1` に正規化される。
///
/// 演算には 2 系統ある。
/// - `checked_add` / `checked_sub` / `checked_mul` / `checked_div`:
///   オーバーフローやゼロ除算を [`None`] で返す安全版
/// - [`Add`] 等の演算子: 上記を呼び、失敗すれば panic する簡便版
///
/// 誤差を出さずに分母を保つため、[`PartialOrd`]・[`PartialEq`]・[`Hash`] も
/// 正規形どうしの比較として実装されている（等しい有理数からは等しいハッシュが得られる）。
///
/// # Examples
/// ```
/// use atcoder::math::fraction::Fraction;
///
/// let a = Fraction::new(1, 3);
/// let b = Fraction::new(1, 6);
///
/// assert_eq!(a + b, Fraction::new(1, 2));
/// assert_eq!(a - b, Fraction::new(1, 6));
/// assert_eq!(a * b, Fraction::new(1, 18));
/// assert!(a > Fraction::new(1, 4));
/// ```
// TODO: PartialEq, Eq, PartialOrd, Ordの実装
#[derive(Clone, Copy, Debug)]
pub struct Fraction {
    sign: Sign,
    num: u64,
    den: u64,
}

impl Fraction {
    /// 0 を表す分数（`0/1`）。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Fraction;
    ///
    /// assert!(Fraction::ZERO.is_zero());
    /// assert_eq!(Fraction::ZERO, Fraction::new(0, 7));
    /// ```
    pub const ZERO: Self = Self {
        sign: Sign::Zero,
        num: 0,
        den: 1,
    };
    /// 1 を表す分数（`1/1`）。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Fraction;
    ///
    /// assert!(Fraction::ONE.is_positive());
    /// assert_eq!(Fraction::ONE, Fraction::new(3, 3));
    /// ```
    pub const ONE: Self = Self {
        sign: Sign::Positive,
        num: 1,
        den: 1,
    };

    /// `num`, `den` から分数 `num / den` を生成する。
    ///
    /// 構築時に `gcd(num, den)` で約分し、符号は [`Sign`] に分離して保持する。
    /// したがって `num` が負でも `den` は正のままで、値は常に正規形になる。
    /// `num == 0` の場合は [`Fraction::ZERO`] に正規化される。
    ///
    /// # 計算量
    /// 時間 O(log(min{num, den}))
    ///
    /// # Panics
    /// - `den` が 0 の場合。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Fraction;
    ///
    /// // 自動で約分される。
    /// let a = Fraction::new(2, 6);
    /// assert_eq!(a.numerator_abs(), 1);
    /// assert_eq!(a.denominator(), 3);
    ///
    /// // 負の分子は符号として保持される。
    /// let b = Fraction::new(-1, 3);
    /// assert_eq!(b.sign().as_i8(), -1);
    /// assert_eq!(b.numerator_abs(), 1);
    /// assert_eq!(b.denominator(), 3);
    ///
    /// // 0 は 0/1 に正規化される。
    /// assert_eq!(Fraction::new(0, 5), Fraction::ZERO);
    /// ```
    #[track_caller]
    #[must_use]
    pub const fn new(num: i64, den: u64) -> Self {
        assert!(den != 0);

        if num == 0 {
            return Self::ZERO;
        }

        let g = gcd_u64(num.unsigned_abs(), den);

        Self {
            sign: if num >= 0 {
                Sign::Positive
            } else {
                Sign::Negative
            },
            num: num.unsigned_abs() / g,
            den: den / g,
        }
    }

    /// 符号を返す。
    ///
    /// [`Fraction`] は符号をここに、絶対値を [`Fraction::numerator_abs`] /
    /// [`Fraction::denominator`] に分けて持つ。負の値の絶対値を得るには
    /// 符号と組み合わせる。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Fraction;
    ///
    /// assert_eq!(Fraction::new(-2, 3).sign().as_i8(), -1);
    /// assert_eq!(Fraction::ONE.sign().as_i8(), 1);
    /// assert_eq!(Fraction::ZERO.sign().as_i8(), 0);
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn sign(&self) -> Sign {
        self.sign
    }

    /// 分子の絶対値を返す。
    ///
    /// 符号付きの分子が必要な場合は [`Fraction::sign`] と組み合わせる
    /// （[`Sign::as_i64`] を掛けると元の分子が得られる）。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Fraction;
    ///
    /// let a = Fraction::new(-2, 3);
    /// let num_abs = a.numerator_abs();
    /// let num = num_abs as i64 * a.sign().as_i64();
    ///
    /// assert_eq!(num_abs, 2);
    /// assert_eq!(num, -2);
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn numerator_abs(&self) -> u64 {
        self.num
    }

    /// 分母を返す。
    ///
    /// 常に正で、正規形では `gcd(numerator_abs, denominator) == 1` を満たす。
    /// [`Fraction::ZERO`] に対しては 1 を返す（0 の分母を 1 に正規化している）。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Fraction;
    ///
    /// assert_eq!(Fraction::new(2, 6).denominator(), 3);
    /// assert_eq!(Fraction::ZERO.denominator(), 1);
    /// assert_eq!(Fraction::new(-5, 7).denominator(), 7);
    /// ```
    #[inline]
    #[must_use]
    pub const fn denominator(&self) -> u64 {
        match self.sign {
            Sign::Zero => 1,
            Sign::Positive | Sign::Negative => self.den,
        }
    }

    /// 値が 0 かどうかを返す。
    ///
    /// 符号を見るだけなので、分母・分子の値には依存しない。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Fraction;
    ///
    /// assert!(Fraction::ZERO.is_zero());
    /// assert!(Fraction::new(0, 9).is_zero());
    /// assert!(!Fraction::new(1, 9).is_zero());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        matches!(self.sign, Sign::Zero)
    }

    /// 値が正かどうかを返す。
    ///
    /// `0` は正ではない（[`Fraction::is_zero`]）ので、こちらは `false` になる。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Fraction;
    ///
    /// assert!(Fraction::new(1, 3).is_positive());
    /// assert!(!Fraction::ZERO.is_positive());
    /// assert!(!Fraction::new(-1, 3).is_positive());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_positive(&self) -> bool {
        matches!(self.sign, Sign::Positive)
    }

    /// 値が負かどうかを返す。
    ///
    /// `0` は負ではないので `false` になる。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Fraction;
    ///
    /// assert!(Fraction::new(-1, 3).is_negative());
    /// assert!(!Fraction::ZERO.is_negative());
    /// assert!(!Fraction::new(1, 3).is_negative());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        matches!(self.sign, Sign::Negative)
    }

    /// 値を `f64` に変換する。
    ///
    /// `num as f64 / den as f64` で求めるため、ここで初めて丸め誤差が入る。
    /// 誤差を避けたい比較には [`PartialOrd`] を使うこと。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Fraction;
    ///
    /// assert_eq!(Fraction::new(1, 2).to_f64(), 0.5);
    /// assert_eq!(Fraction::new(-1, 4).to_f64(), -0.25);
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_f64(self) -> f64 {
        match self.sign {
            Sign::Zero => 0.0f64,
            Sign::Positive => self.num as f64 / self.den as f64,
            Sign::Negative => -(self.num as f64 / self.den as f64),
        }
    }

    /// 符号を反転させる。
    ///
    /// 新しい [`Fraction`] を返し、`self` は変更されない。`0` は `0` のまま。
    /// 演算子版は [`Neg`]（`-a`）。
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Fraction;
    ///
    /// let a = Fraction::new(1, 3);
    /// assert_eq!(a.negated(), Fraction::new(-1, 3));
    /// assert_eq!(Fraction::ZERO.negated(), Fraction::ZERO);
    /// assert_eq!(-a, Fraction::new(-1, 3));
    /// ```
    #[inline]
    #[must_use]
    pub const fn negated(self) -> Self {
        Self {
            sign: match self.sign {
                Sign::Zero => Sign::Zero,
                Sign::Positive => Sign::Negative,
                Sign::Negative => Sign::Positive,
            },
            num: self.num,
            den: self.den,
        }
    }

    /// 交差約分を行う。
    ///
    /// `self.num` と `rhs.den`、`self.den` と `rhs.num` をそれぞれ約分する。
    /// 乗除算の前に噛ませると、積が [`u64`] に収まる可能性が高まる。
    ///
    /// # 計算量
    /// 時間 O(log(min{self.num, rhs.den}))
    #[inline(always)]
    const fn cross_reduce(&mut self, rhs: &mut Self) {
        let g1 = gcd_u64(self.num, rhs.den);
        let g2 = gcd_u64(rhs.num, self.den);

        self.num /= g1;
        rhs.den /= g1;

        rhs.num /= g2;
        self.den /= g2;
    }

    /// オーバーフローチェック付きの加算を行う。
    ///
    /// 結果が [`u64`] の分子・分母で表現できない場合（主に中間の
    /// [`u128`] 積や分母の積が溢れる場合）は [`None`] を返す。
    /// 演算子版は [`Add`]（`a + b`）で、こちらは失敗時に panic する。
    ///
    /// # 計算量
    /// 時間 O(log(min{self.den, rhs.den}))
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Fraction;
    ///
    /// let a = Fraction::new(1, 3);
    /// let b = Fraction::new(1, 6);
    ///
    /// assert_eq!(a.checked_add(b), Some(Fraction::new(1, 2)));
    ///
    /// // 0 を足しても値は変わらない。
    /// assert_eq!(a.checked_add(Fraction::ZERO), Some(a));
    /// ```
    #[inline]
    #[must_use]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        if self.is_zero() {
            return Some(rhs);
        }
        if rhs.is_zero() {
            return Some(self);
        }

        let g = gcd_u64(self.den, rhs.den);

        let l = self.num as u128 * (rhs.den / g) as u128;
        let r = rhs.num as u128 * (self.den / g) as u128;

        let (num, sign);
        match (self.sign, rhs.sign) {
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => {
                (num, sign) = (if_none_return!(l.checked_add(r)), self.sign);
            }
            (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) if l == r => {
                return Some(Self::ZERO);
            }
            (Sign::Positive, Sign::Negative) => {
                (num, sign) = (if_none_return!(l.checked_sub(r)), self.sign);
            }
            (Sign::Negative, Sign::Positive) => {
                (num, sign) = (if_none_return!(r.checked_sub(l)), rhs.sign);
            }
            _ => return None,
        }

        let h = gcd_u64((num % g as u128) as u64, g);

        let num = num / h as u128;
        if num > u64::MAX as u128 {
            return None;
        }
        let num = num as u64;

        let den = if_none_return!((self.den / g).checked_mul(rhs.den / h));

        Some(Self { sign, num, den })
    }

    /// オーバーフローチェック付きの減算を行う。
    ///
    /// 内部的には `rhs` の符号を反転させて加算する（[`Self::checked_add`] に委譲）。
    /// 結果が表現できない場合は [`None`] を返す。演算子版は [`Sub`]（`a - b`）。
    ///
    /// # 計算量
    /// 時間 O(log(min{self.den, rhs.den}))
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Fraction;
    ///
    /// let a = Fraction::new(1, 3);
    /// let b = Fraction::new(1, 6);
    ///
    /// assert_eq!(a.checked_sub(b), Some(Fraction::new(1, 6)));
    /// assert_eq!(a.checked_sub(a), Some(Fraction::ZERO));
    /// ```
    #[inline]
    #[must_use]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.checked_add(rhs.negated())
    }

    /// オーバーフローチェック付きの乗算を行う。
    ///
    /// 先に交差約分（[`Self::cross_reduce`]）を行うため、表現できる範囲が広い。
    /// 分子同士・分母同士の積が [`u64`] に収まらない場合は [`None`] を返す。
    /// 演算子版は [`Mul`]（`a * b`）。
    ///
    /// # 計算量
    /// 時間 O(log(min{self.den, rhs.den}))
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Fraction;
    ///
    /// let a = Fraction::new(2, 3);
    /// let b = Fraction::new(3, 4);
    ///
    /// assert_eq!(a.checked_mul(b), Some(Fraction::new(1, 2)));
    ///
    /// // 0 を掛けると 0 になる。
    /// assert_eq!(a.checked_mul(Fraction::ZERO), Some(Fraction::ZERO));
    /// ```
    #[inline]
    #[must_use]
    pub const fn checked_mul(mut self, mut rhs: Self) -> Option<Self> {
        self.cross_reduce(&mut rhs);

        let num = if_none_return!(self.num.checked_mul(rhs.num));
        let den = if_none_return!(self.den.checked_mul(rhs.den));

        let sign = self.sign.mul(rhs.sign);

        Some(Self { sign, num, den })
    }

    /// オーバーフロー・ゼロ除算チェック付きの除算を行う。
    ///
    /// `rhs` が 0 の場合は [`None`] を返す。それ以外では `rhs` の分子と分母を
    /// 入れ替えて（逆数にして）乗算する。演算子版は [`Div`]（`a / b`）で、
    /// 失敗時は panic する。
    ///
    /// # 計算量
    /// 時間 O(log(min{self.den, rhs.den}))
    ///
    /// # Examples
    /// ```
    /// use atcoder::math::fraction::Fraction;
    ///
    /// let a = Fraction::new(1, 2);
    /// let b = Fraction::new(3, 4);
    ///
    /// assert_eq!(a.checked_div(b), Some(Fraction::new(2, 3)));
    ///
    /// // 0 で割ることはできない。
    /// assert_eq!(a.checked_div(Fraction::ZERO), None);
    /// ```
    #[inline]
    #[must_use]
    pub const fn checked_div(self, mut rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            return None;
        }

        std::mem::swap(&mut rhs.num, &mut rhs.den);
        self.checked_mul(rhs)
    }
}

impl PartialEq for Fraction {
    /// 有理数として等しいかを判定する。
    ///
    /// 正規形どうしを比較するため、約分前の形が異なっても等しいと判定される。
    /// `0` 同士は分子・分母に関係なく等しい。
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.sign != other.sign {
            false
        } else {
            self.sign == Sign::Zero || (self.num == other.num && self.den == other.den)
        }
    }
}

impl Eq for Fraction {}

impl Hash for Fraction {
    /// 正規形からハッシュを計算する。
    ///
    /// 等しい有理数からは等しいハッシュが得られる（`HashMap` 等で正しく使える）。
    /// `0` は分子・分母をハッシュに含めないため、`0/1` と `0/7` は同一のハッシュになる。
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sign.hash(state);

        if self.sign != Sign::Zero {
            self.num.hash(state);
            self.den.hash(state);
        }
    }
}

impl Ord for Fraction {
    /// 有理数として大小を比較する。
    ///
    /// 符号が異なる場合は符号で決まり、同符号なら分母をそろえた外積（`u128`）で
    /// 比較する。負の場合は大小を反転する。誤差のない整数演算のみで行う。
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let sign_order = self.sign.as_i8().cmp(&other.sign.as_i8());

        if sign_order != Ordering::Equal {
            return sign_order;
        }

        let l = u128::from(self.num) * u128::from(other.den);
        let r = u128::from(other.num) * u128::from(self.den);

        let order = l.cmp(&r);

        if self.sign == Sign::Negative {
            order.reverse()
        } else {
            order
        }
    }
}

impl PartialOrd for Fraction {
    /// 有理数として大小を比較する（[`Ord`] に委譲）。
    ///
    /// [`f64`] への変換を挟まないため、誤差なしで比較できる。
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Neg for Fraction {
    type Output = Self;

    /// 単項マイナス。符号を反転した [`Fraction`] を返す（[`Fraction::negated`] と同じ）。
    fn neg(self) -> Self::Output {
        self.negated()
    }
}

impl Add<Self> for Fraction {
    type Output = Self;

    /// 加算。オーバーフロー時は panic する（安全版は [`Fraction::checked_add`]）。
    ///
    /// # Panics
    /// - 結果が `u64` の分子・分母で表現できない場合。
    #[inline(always)]
    #[track_caller]
    fn add(self, rhs: Self) -> Self {
        self.checked_add(rhs)
            .expect("Overflow detected in Fraction addition")
    }
}

impl AddAssign<Self> for Fraction {
    /// 加算の複合代入。オーバーフロー時は panic する。
    ///
    /// # Panics
    /// - 結果が `u64` の分子・分母で表現できない場合。
    #[inline(always)]
    #[track_caller]
    fn add_assign(&mut self, rhs: Self) {
        *self = self
            .checked_add(rhs)
            .expect("Overflow detected in Fraction addition")
    }
}

impl Sub<Self> for Fraction {
    type Output = Self;

    /// 減算。オーバーフロー時は panic する（安全版は [`Fraction::checked_sub`]）。
    ///
    /// # Panics
    /// - 結果が `u64` の分子・分母で表現できない場合。
    #[inline(always)]
    #[track_caller]
    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
            .expect("Overflow detected in Fraction subtraction")
    }
}

impl SubAssign<Self> for Fraction {
    /// 減算の複合代入。オーバーフロー時は panic する。
    ///
    /// # Panics
    /// - 結果が `u64` の分子・分母で表現できない場合。
    #[inline(always)]
    #[track_caller]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self
            .checked_sub(rhs)
            .expect("Overflow detected in Fraction subtraction")
    }
}

impl Mul<Self> for Fraction {
    type Output = Self;

    /// 乗算。オーバーフロー時は panic する（安全版は [`Fraction::checked_mul`]）。
    ///
    /// # Panics
    /// - 分子同士・分母同士の積が `u64` に収まらない場合。
    #[inline(always)]
    #[track_caller]
    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(rhs)
            .expect("Overflow detected in Fraction multiplication")
    }
}

impl MulAssign<Self> for Fraction {
    /// 乗算の複合代入。オーバーフロー時は panic する。
    ///
    /// # Panics
    /// - 分子同士・分母同士の積が `u64` に収まらない場合。
    #[inline(always)]
    #[track_caller]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self
            .checked_mul(rhs)
            .expect("Overflow detected in Fraction multiplication")
    }
}

impl Div<Self> for Fraction {
    type Output = Self;

    /// 除算。`rhs` が 0、またはオーバーフロー時に panic する
    /// （安全版は [`Fraction::checked_div`]）。
    ///
    /// # Panics
    /// - `rhs` が 0 の場合。
    /// - 結果が `u64` の分子・分母で表現できない場合。
    #[inline(always)]
    #[track_caller]
    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(rhs)
            .expect("Overflow detected in Fraction division")
    }
}

impl DivAssign<Self> for Fraction {
    /// 除算の複合代入。`rhs` が 0、またはオーバーフロー時に panic する。
    ///
    /// # Panics
    /// - `rhs` が 0 の場合。
    /// - 結果が `u64` の分子・分母で表現できない場合。
    #[inline(always)]
    #[track_caller]
    fn div_assign(&mut self, rhs: Self) {
        *self = self
            .checked_div(rhs)
            .expect("Overflow detected in Fraction division")
    }
}

/// [`gcd`](crate::math::gcd_lcm::gcd) の u64 版。
const fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    a
}
