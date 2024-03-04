use modint_mersenne::{FromPrimitiveInt, ModIntMersenne};
use std::iter::once;
use std::ops::RangeBounds;
use std::time::SystemTime;

/// 各接頭辞のハッシュ値を事前計算しておき、連続部分列のハッシュ値を`O(1)`で求める
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollingHash {
    /// baseの累乗のテーブル[0..s.len()]
    base_pow_table: Vec<ModIntMersenne>,
    /// sのprefixのhash値のテーブル[0..s.len()]
    prefix_hash_table: Vec<ModIntMersenne>,
}

impl RollingHash {
    /// sのrolling hashを構築 `O(|s|)`
    pub fn new(s: &Vec<char>) -> Self {
        // baseは乱数を生成(randクレートを使えない場合を考慮して時間で)
        let rand_duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let rand_time = rand_duration.subsec_nanos() as u64 + rand_duration.as_secs();
        let base = ModIntMersenne::new(rand_time);
        let base_pow_table: Vec<ModIntMersenne> = once(ModIntMersenne::new(1))
            .chain((0..s.len()).scan(ModIntMersenne::new(1), |acc, _| {
                *acc *= base;
                Some(*acc)
            }))
            .collect();
        let prefix_hash_table: Vec<ModIntMersenne> = once(ModIntMersenne::new(0))
            .chain(s.iter().scan(ModIntMersenne::new(0), |acc, s| {
                *acc = *acc * base + ModIntMersenne::new(*s as u64);
                Some(*acc)
            }))
            .collect();
        Self {
            base_pow_table,
            prefix_hash_table,
        }
    }

    /// 部分列`s[range]`のhash値を返す `O(1)`
    pub fn get_hash<R: RangeBounds<usize>>(&self, range: R) -> ModIntMersenne {
        let l = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.prefix_hash_table.len() - 1,
        };
        assert!(l <= r && r < self.prefix_hash_table.len());
        self.prefix_hash_table[r] - self.prefix_hash_table[l] * self.base_pow_table[r - l]
    }

    /// `base^i`を返す
    pub fn get_base_pow(&self, i: usize) -> ModIntMersenne {
        self.base_pow_table[i]
    }

    /// 接頭辞のhash値を返す(`get_hash(0..i)`と同じ)
    pub fn get_prefix_hash(&self, i: usize) -> ModIntMersenne {
        self.prefix_hash_table[i]
    }
}
