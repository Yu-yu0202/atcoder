use crate::math::modulo::modint::{RemEuclidU32, StaticModInt};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

fn new_rem_euclid<const M: u32, T: RemEuclidU32>(rhs: T) -> StaticModInt<M> {
    unsafe {
        // SAFETY: `rem_euclid_u32` を通すので 0 <= x < MOD を保証できる
        StaticModInt::new_unchecked(rhs.rem_euclid_u32(M))
    }
}

impl<const M: u32, T: RemEuclidU32> Add<T> for StaticModInt<M> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        let rhs = new_rem_euclid(rhs);
        self._add(rhs)
    }
}

impl<const M: u32> Add<StaticModInt<M>> for StaticModInt<M> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: StaticModInt<M>) -> Self::Output {
        self._add(rhs)
    }
}

impl<const M: u32> AddAssign<StaticModInt<M>> for StaticModInt<M> {
    #[inline]
    fn add_assign(&mut self, rhs: StaticModInt<M>) {
        *self = self._add(rhs);
    }
}

impl<const M: u32, T: RemEuclidU32> AddAssign<T> for StaticModInt<M> {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        *self = self._add(new_rem_euclid(rhs));
    }
}

impl<const M: u32, T: RemEuclidU32> Sub<T> for StaticModInt<M> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        let rhs = new_rem_euclid(rhs);
        self._sub(rhs)
    }
}

impl<const M: u32> Sub<StaticModInt<M>> for StaticModInt<M> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: StaticModInt<M>) -> Self::Output {
        self._sub(rhs)
    }
}

impl<const M: u32, T: RemEuclidU32> SubAssign<T> for StaticModInt<M> {
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        *self = self._sub(new_rem_euclid(rhs));
    }
}

impl<const M: u32> SubAssign<StaticModInt<M>> for StaticModInt<M> {
    #[inline]
    fn sub_assign(&mut self, rhs: StaticModInt<M>) {
        *self = self._sub(rhs);
    }
}

impl<const M: u32, T: RemEuclidU32> Mul<T> for StaticModInt<M> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        self._mul(unsafe {
            // SAFETY: `rem_euclid_u32` を通すので 0 <= x < MOD を保証できる
            Self::new_unchecked(rhs.rem_euclid_u32(M))
        })
    }
}

impl<const M: u32> Mul<StaticModInt<M>> for StaticModInt<M> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: StaticModInt<M>) -> Self::Output {
        self._mul(rhs)
    }
}

impl<const M: u32, T: RemEuclidU32> MulAssign<T> for StaticModInt<M> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        *self = self._mul(new_rem_euclid(rhs));
    }
}

impl<const M: u32> MulAssign<StaticModInt<M>> for StaticModInt<M> {
    #[inline]
    fn mul_assign(&mut self, rhs: StaticModInt<M>) {
        *self = self._mul(rhs);
    }
}

impl<const M: u32, T: RemEuclidU32> Div<T> for StaticModInt<M> {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn div(self, rhs: T) -> Self::Output {
        self._div(new_rem_euclid(rhs))
    }
}

impl<const M: u32> Div<StaticModInt<M>> for StaticModInt<M> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: StaticModInt<M>) -> Self::Output {
        self._div(rhs)
    }
}

impl<const M: u32, T: RemEuclidU32> DivAssign<T> for StaticModInt<M> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        *self = self._div(new_rem_euclid(rhs));
    }
}
