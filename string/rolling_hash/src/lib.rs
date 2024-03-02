use modint_mersenne::ModIntMersenne;

#[derive(Debug, Clone, PartialEq, Eq)]
struct RollingHash {
    prefix_hash_table: Vec<ModIntMersenne>,
}

impl RollingHash {
    pub fn new(s: &Vec<char>) -> Self {
        todo!()
    }
}