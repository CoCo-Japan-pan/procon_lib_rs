/// From <https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/internal_bit.rs>  
/// n <= 2^(ceil_log2(n)) を満たす最小のnを返す
pub fn ceil_log2(n: u32) -> u32 {
    32 - n.saturating_sub(1).leading_zeros()
}
