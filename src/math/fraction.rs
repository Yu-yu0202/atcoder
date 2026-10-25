use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use crate::macros::if_none_return;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// [`Fraction`] の符号。
#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Sign {
    /// ゼロ
    Zero = 0,
    /// 正の（有理）数
    Positive = 1,
    /// 負の（有理）数
    Negative = -1,
}

impl Sign {
    /// 符号を -1, 0, 1 の i8 に変換する。
    #[inline]
    #[must_use]
    pub const fn as_i8(self) -> i8 {
        self as _
    }

    /// 符号を -1, 0, 1 の i32 に変換する。
    #[inline]
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self.as_i8() as _
    }

    /// 符号を -1, 0, 1 の i64 に変換する。
    #[inline]
    #[must_use]
    pub const fn as_i64(self) -> i64 {
        self.as_i8() as _
    }

    /// 符号を掛ける
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

/// 分数を誤差なく表現できる型
// TODO: PartialEq, Eq, PartialOrd, Ordの実装
#[derive(Clone, Copy, Debug)]
pub struct Fraction {
    sign: Sign,
    num: u64,
    den: u64,
}

impl Fraction {
    /// ゼロ（0/1）
    pub const ZERO: Self = Self {
        sign: Sign::Zero,
        num: 0,
        den: 1,
    };
    /// 1（1/1）
    pub const ONE: Self = Self {
        sign: Sign::Positive,
        num: 1,
        den: 1,
    };

    /// `num`, `den` から分数を生成する（num/den）。
    ///
    /// # 計算量
    /// 時間 O(log(min{num, den}))
    ///
    /// # Panics
    /// - `den` が 0 の場合。
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
    #[inline(always)]
    #[must_use]
    pub const fn sign(&self) -> Sign {
        self.sign
    }

    /// 分子の絶対値を返す。
    ///
    /// 符号付きにしたい場合、`self.sign` と組み合わせる。
    ///
    /// ```
    /// use atcoder::math::fraction::Fraction;
    /// let a = Fraction::new(-2, 3);
    /// let num_abs = a.numerator_abs();
    /// let num = num_abs as i64 * a.sign().as_i64();
    ///
    /// assert_eq!(num, -2);
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn numerator_abs(&self) -> u64 {
        self.num
    }

    /// 分母を返す。
    #[inline]
    #[must_use]
    pub const fn denominator(&self) -> u64 {
        match self.sign {
            Sign::Zero => 1,
            Sign::Positive | Sign::Negative => self.den,
        }
    }

    /// 値が 0 かどうかを返す。
    #[inline(always)]
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        matches!(self.sign, Sign::Zero)
    }

    /// 値が正かどうかを返す。
    #[inline(always)]
    #[must_use]
    pub const fn is_positive(&self) -> bool {
        matches!(self.sign, Sign::Positive)
    }

    /// 値が負かどうかを返す。
    #[inline(always)]
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        matches!(self.sign, Sign::Negative)
    }

    /// 値を `f64` に変換する。
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
    /// `self.num` と `rhs.den`、`self.den` と `rhs.num` をそれぞれ約分する。
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
    /// オーバーフローが発生した場合、[`None`] を返す。
    ///
    /// # 計算量
    /// 時間 O(log(min{self.num, rhs.den}))
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
    /// オーバーフローが発生した場合、[`None`] を返す。
    #[inline]
    #[must_use]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.checked_add(rhs.negated())
    }

    /// オーバーフローチェック付きの乗算を行う。
    ///
    /// オーバーフローが発生した場合、[`None`] を返す。
    ///
    /// # 計算量
    /// 時間 O(log(min{self.num, rhs.den}))
    #[inline]
    #[must_use]
    pub const fn checked_mul(mut self, mut rhs: Self) -> Option<Self> {
        self.cross_reduce(&mut rhs);

        let num = if_none_return!(self.num.checked_mul(rhs.num));
        let den = if_none_return!(self.den.checked_mul(rhs.den));

        let sign = self.sign.mul(rhs.sign);

        Some(Self { sign, num, den })
    }

    /// オーバーフロー、ゼロ除算チェック付きの除算を行う。
    ///
    /// オーバーフローやゼロ除算が発生した場合、[`None`] を返す。
    ///
    /// # 計算量
    /// 時間 O(log(min{self.num, rhs.den}))
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
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.negated()
    }
}

impl Add<Self> for Fraction {
    type Output = Self;

    #[inline(always)]
    #[track_caller]
    fn add(self, rhs: Self) -> Self {
        self.checked_add(rhs)
            .expect("Overflow detected in Fraction addition")
    }
}

impl AddAssign<Self> for Fraction {
    #[inline(always)]
    #[track_caller]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.checked_add(rhs)
            .expect("Overflow detected in Fraction addition")
    }
}

impl Sub<Self> for Fraction {
    type Output = Self;

    #[inline(always)]
    #[track_caller]
    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
            .expect("Overflow detected in Fraction subtraction")
    }
}

impl SubAssign<Self> for Fraction {
    #[inline(always)]
    #[track_caller]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.checked_sub(rhs)
            .expect("Overflow detected in Fraction subtraction")
    }
}

impl Mul<Self> for Fraction {
    type Output = Self;

    #[inline(always)]
    #[track_caller]
    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(rhs)
            .expect("Overflow detected in Fraction multiplication")
    }
}

impl MulAssign<Self> for Fraction {
    #[inline(always)]
    #[track_caller]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.checked_mul(rhs)
            .expect("Overflow detected in Fraction multiplication")
    }
}

impl Div<Self> for Fraction {
    type Output = Self;

    #[inline(always)]
    #[track_caller]
    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(rhs)
            .expect("Overflow detected in Fraction division")
    }
}

impl DivAssign<Self> for Fraction {
    #[inline(always)]
    #[track_caller]
    fn div_assign(&mut self, rhs: Self) {
        *self = self.checked_div(rhs)
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
