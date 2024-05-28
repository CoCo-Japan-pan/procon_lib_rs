pub fn ceil_log2(n: u32) -> u32 {
    32 - n.saturating_sub(1).leading_zeros()
}
