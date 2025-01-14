//! ダブリング  

#[derive(Debug, Clone)]
pub struct Doubling {
    n: usize,
    max_pow: u64,
    table: Vec<Vec<u32>>,
}

impl Doubling {
    /// [0, n) -> [0, n) の写像funcを受け取る  
    /// nは32bit以下で収まると仮定している  
    /// 写像はmax_pow乗までのみ考える
    pub fn new(func: &[usize], max_pow: u64) -> Self {
        let n = func.len();
        let mut table = vec![func.iter().map(|&x| x as u32).collect::<Vec<_>>()];
        let mut cur_pow = max_pow >> 1;
        while cur_pow > 0 {
            let mut next = vec![0; n];
            let last_table = table.last().unwrap();
            for i in 0..n {
                next[i] = last_table[last_table[i] as usize];
            }
            table.push(next);
            cur_pow >>= 1;
        }
        Self { n, max_pow, table }
    }

    /// func^x(index)を返す
    pub fn query(&self, mut index: usize, mut x: u64) -> usize {
        assert!(index < self.n);
        assert!(x <= self.max_pow);
        let mut table_index = 0;
        while x > 0 {
            if x & 1 == 1 {
                index = self.table[table_index][index] as usize;
            }
            table_index += 1;
            x >>= 1;
        }
        index
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test() {
        let mut rng = thread_rng();
        const SIZE: usize = 1000;
        let func = (0..SIZE)
            .map(|_| rng.gen_range(0..SIZE))
            .collect::<Vec<_>>();
        let max_pow = 1000;
        let doubling = Doubling::new(&func, max_pow);
        for _ in 0..100 {
            let index = rng.gen_range(0..SIZE);
            let x = rng.gen_range(0..max_pow);
            let expected = (0..x).fold(index, |i, _| func[i]);
            assert_eq!(doubling.query(index, x as u64), expected);
        }
    }

    #[test]
    fn test_two_beki() {
        let mut rng = thread_rng();
        const SIZE: usize = 1024;
        let func = (0..SIZE)
            .map(|_| rng.gen_range(0..SIZE))
            .collect::<Vec<_>>();
        let max_pow = 1024;
        let doubling = Doubling::new(&func, max_pow);
        for bit in 0..=10 {
            let index = rng.gen_range(0..SIZE);
            let x = 1 << bit;
            let expected = (0..x).fold(index, |i, _| func[i]);
            assert_eq!(doubling.query(index, x as u64), expected);
            let x = x - 1;
            let expected = (0..x).fold(index, |i, _| func[i]);
            assert_eq!(doubling.query(index, x as u64), expected);
        }
    }
}
