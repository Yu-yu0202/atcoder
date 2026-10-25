//! 乱数生成アルゴリズムの詰め合わせ

pub mod splitmix64;

/// 乱数生成器の trait
pub trait RandBackend: Sized {
    /// 乱数生成器をシードから作成する。
    fn new(seed: u64) -> Self;

    /// 64bit 精度で乱数を生成する。
    fn next_u64(&mut self) -> u64;
}
