//! chmin / chmax の拡張

/// chmin / chmax を提供する trait
pub trait ChangeMinMax: PartialOrd + Sized {
    /// chmin をとる。
    #[inline]
    fn chmin(&mut self, other: Self) -> bool {
        if *self > other {
            *self = other;
            true
        } else {
            false
        }
    }

    /// chmax をとる。
    #[inline]
    fn chmax(&mut self, other: Self) -> bool {
        if *self < other {
            *self = other;
            true
        } else {
            false
        }
    }
}

impl<T: PartialOrd + Sized> ChangeMinMax for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chmin() {
        let mut x = 2;
        assert!(x.chmin(1));
        assert_eq!(x, 1);

        assert!(!x.chmin(2));
        assert_eq!(x, 1);
    }

    #[test]
    fn test_chmax() {
        let mut x = 1;
        assert!(x.chmax(2));
        assert_eq!(x, 2);

        assert!(!x.chmax(1));
        assert_eq!(x, 2);
    }
}
