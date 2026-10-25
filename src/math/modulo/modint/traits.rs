/// ユークリッド剰余を u32 で返す trait
pub trait RemEuclidU32 {
    /// ユークリッド剰余を u32 で返す。
    fn rem_euclid_u32(&self, m: u32) -> u32;
}

macro_rules! impl_rem_euclid_u32_for_signed {
    ($($ty: tt), *) => {
        $(
            impl RemEuclidU32 for $ty {
                #[inline]
                fn rem_euclid_u32(&self, m: u32) -> u32 {
                    (*self as i64).rem_euclid(i64::from(m)) as _
                }
            }
        )*
    }
}

macro_rules! impl_rem_euclid_u32_for_unsigned {
    ($($ty: tt), *) => {
        $(
            impl RemEuclidU32 for $ty {
                #[inline]
                fn rem_euclid_u32(&self, m: u32) -> u32 {
                    *self as u32 % m
                }
            }
        )*
    }
}

impl_rem_euclid_u32_for_signed!(i8, i16, i32, i64, isize);

impl RemEuclidU32 for i128 {
    #[inline]
    fn rem_euclid_u32(&self, m: u32) -> u32 {
        (*self).rem_euclid(i128::from(m)) as _
    }
}

impl_rem_euclid_u32_for_unsigned!(u8, u16, u32);

impl RemEuclidU32 for u64 {
    #[inline]
    fn rem_euclid_u32(&self, m: u32) -> u32 {
        (*self % (m as u64)) as _
    }
}

impl RemEuclidU32 for u128 {
    #[inline]
    fn rem_euclid_u32(&self, m: u32) -> u32 {
        (*self % (m as u128)) as _
    }
}

#[cfg(target_pointer_width = "32")]
impl_rem_euclid_u32_for_unsigned!(usize);

#[cfg(target_pointer_width = "64")]
impl RemEuclidU32 for usize {
    #[inline]
    fn rem_euclid_u32(&self, m: u32) -> u32 {
        (*self % (m as usize)) as _
    }
}
