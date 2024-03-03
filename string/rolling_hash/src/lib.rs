use modint_mersenne::{FromPrimitiveInt, ModIntMersenne};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollingHash {
    /// baseの累乗のテーブル[0..s.len()]
    base_pow_table: Vec<ModIntMersenne>,
    /// sのprefixのhash値のテーブル[0..s.len()]
    prefix_hash_table: Vec<ModIntMersenne>,
}

impl RollingHash {
    pub fn new(s: &Vec<char>) -> Self {
        // [2, MOD -2]の範囲で乱数を生成
        let rand_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        let base = ModIntMersenne::new(rand_time);
        let base_pow_table: Vec<ModIntMersenne> = (0..s.len())
            .scan(ModIntMersenne::new(1), |acc, _| {
                *acc = *acc * base;
                Some(*acc)
            })
            .collect();
        let prefix_hash_table: Vec<ModIntMersenne> = (0..s.len())
            .scan(ModIntMersenne::new(0), |acc, i| {
                *acc = *acc * base + ModIntMersenne::new(s[i] as u64);
                Some(*acc)
            })
            .collect();
        Self {
            base_pow_table,
            prefix_hash_table,
        }
    }

    /// 部分列[l, r)のhash値を返す(0-base)
    pub fn get_hash(&self, l: usize, r: usize) -> ModIntMersenne {
        assert!(l <= r);
        self.prefix_hash_table[r] - self.prefix_hash_table[l] * self.base_pow_table[r - l]
    }

    /// base^iを返す
    pub fn get_base_pow(&self, i: usize) -> ModIntMersenne {
        self.base_pow_table[i]
    }

    /// 接頭辞のhash値を返す
    pub fn get_prefix_hash(&self, i: usize) -> ModIntMersenne {
        self.prefix_hash_table[i]
    }
}
