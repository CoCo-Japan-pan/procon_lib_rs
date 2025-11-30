---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc227/editorial/2909
    - https://qiita.com/drken/items/3beb679e54266f20ab63
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.9/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.9/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! [\u30A8\u30E9\u30C8\u30B9\u30C6\u30CD\u30B9\u306E\u7BE9](https://qiita.com/drken/items/3beb679e54266f20ab63)\n\
    \nuse std::ops::{Add, MulAssign, Sub};\n\npub struct Eratosthenes {\n    max_n:\
    \ usize,\n    primes: Vec<usize>,\n    min_factor: Vec<usize>,\n}\n\nimpl Eratosthenes\
    \ {\n    /// `O(NloglogN)` `max_n`\u4EE5\u4E0B\u306E\u7D20\u6570\u3092\u6C42\u3081\
    \u308B\n    pub fn new(max_n: usize) -> Self {\n        if max_n == 0 {\n    \
    \        return Self {\n                max_n: 0,\n                primes: vec![],\n\
    \                min_factor: vec![],\n            };\n        }\n        let mut\
    \ min_factor = vec![0; max_n + 1];\n        let mut primes = vec![];\n       \
    \ min_factor[1] = 1;\n        for num in 2..=max_n {\n            if min_factor[num]\
    \ != 0 {\n                continue;\n            }\n            primes.push(num);\n\
    \            for cur_num in (num..=max_n).step_by(num) {\n                if min_factor[cur_num]\
    \ == 0 {\n                    min_factor[cur_num] = num;\n                }\n\
    \            }\n        }\n        Self {\n            max_n,\n            primes,\n\
    \            min_factor,\n        }\n    }\n\n    pub fn is_prime(&self, n: usize)\
    \ -> bool {\n        assert!(n <= self.max_n);\n        n >= 2 && self.min_factor[n]\
    \ == n\n    }\n\n    pub fn get_primes(&self) -> &[usize] {\n        &self.primes\n\
    \    }\n\n    /// `O(log n)` \u3067n\u3092\u7D20\u56E0\u6570\u5206\u89E3  \n \
    \   /// (\u7D20\u56E0\u6570\u3001\u3079\u304D) \u306E\u914D\u5217\u3092\u8FD4\u3059\
    \n    pub fn factorize(&self, mut n: usize) -> Vec<(usize, usize)> {\n       \
    \ assert!(n <= self.max_n);\n        let mut res = vec![];\n        while n >\
    \ 1 {\n            let p = self.min_factor[n];\n            let mut cnt = 0;\n\
    \            while self.min_factor[n] == p {\n                cnt += 1;\n    \
    \            n /= p;\n            }\n            res.push((p, cnt));\n       \
    \ }\n        res\n    }\n\n    /// `\u221Ar`\u4EE5\u4E0B\u306E\u7D20\u6570\u3092\
    \u69CB\u9020\u4F53\u306E\u30E1\u30F3\u30D0\u3068\u3057\u3066\u6301\u3063\u3066\
    \u3044\u308B\u3053\u3068\u3092\u524D\u63D0\u3068\u3059\u308B  \n    /// \u9589\
    \u533A\u9593`[l, r]`\u306E\u7D20\u56E0\u6570\u5206\u89E3\u3092\u307E\u3068\u3081\
    \u3066\u884C\u3046  \n    /// `M = max(r - l + 1, \u221Ar)` \u3068\u3057\u3066\
    \ `O(M loglog M)`  \n    /// \u7D20\u56E0\u6570\u5206\u89E3\u306E\u7D50\u679C\u3092\
    \u4E8C\u6B21\u5143\u914D\u5217\u3067\u3059\u3079\u3066\u6301\u3064\u306E\u3067\
    \u30E1\u30E2\u30EA\u4F7F\u7528\u91CF\u306B\u6CE8\u610F  \n    /// <https://atcoder.jp/contests/abc227/editorial/2909>\n\
    \    pub fn factorize_range(&self, l: usize, r: usize) -> Vec<Vec<(usize, usize)>>\
    \ {\n        if r < l {\n            return vec![];\n        }\n        assert!(r\
    \ / self.max_n <= self.max_n);\n        let mut ret = vec![vec![]; r - l + 1];\n\
    \        let mut nums = (l..=r).collect::<Vec<_>>();\n        for &p in &self.primes\
    \ {\n            for num in ((l + p - 1) / p * p..=r).step_by(p) {\n         \
    \       if num == 0 {\n                    continue;\n                }\n    \
    \            let mut cnt = 0;\n                let idx = num - l;\n          \
    \      while nums[idx] % p == 0 {\n                    nums[idx] /= p;\n     \
    \               cnt += 1;\n                }\n                ret[idx].push((p,\
    \ cnt));\n            }\n        }\n        for (idx, &num) in nums.iter().enumerate()\
    \ {\n            if num > 1 {\n                ret[idx].push((num, 1));\n    \
    \        }\n        }\n        ret\n    }\n\n    /// `\u221Ar` \u4EE5\u4E0B\u306E\
    \u7D20\u6570\u3092\u69CB\u9020\u4F53\u306E\u30E1\u30F3\u30D0\u3068\u3057\u3066\
    \u6301\u3063\u3066\u3044\u308B\u3053\u3068\u3092\u524D\u63D0\u3068\u3059\u308B\
    \  \n    /// \u9589\u533A\u9593 `[l, r]` \u304C\u7D20\u6570\u304B\u5426\u304B\u3092\
    \u307E\u3068\u3081\u3066\u5224\u5B9A  \n    /// `M = max(r - l + 1, \u221Ar)`\
    \ \u3068\u3057\u3066 `O(M loglog M)`\n    pub fn is_prime_range(&self, l: usize,\
    \ r: usize) -> Vec<bool> {\n        if r < l {\n            return vec![];\n \
    \       }\n        assert!(r / self.max_n <= self.max_n);\n        let mut ret\
    \ = vec![true; r - l + 1];\n        if l == 0 {\n            ret[0] = false;\n\
    \        }\n        if l <= 1 {\n            ret[1 - l] = false;\n        }\n\
    \        for &p in &self.primes {\n            for num in ((l + p - 1) / p * p..=r).step_by(p)\
    \ {\n                if num == p {\n                    continue;\n          \
    \      }\n                let idx = num - l;\n                ret[idx] = false;\n\
    \            }\n        }\n        ret\n    }\n\n    /// \u7D04\u6570\u306E\u500B\
    \u6570\u30AA\u30FC\u30C0\u30FC\u3067\u7D04\u6570\u5217\u6319 \u7279\u306B\u51FA\
    \u529B\u306F\u30BD\u30FC\u30C8\u3057\u3066\u3044\u306A\u3044\u306E\u3067\u6CE8\
    \u610F\n    pub fn enumerate_divisors(&self, n: usize) -> Vec<usize> {\n     \
    \   let pc = self.factorize(n);\n        let size = pc.iter().map(|(_, c)| c +\
    \ 1).product::<usize>();\n        let mut ret = Vec::with_capacity(size);\n  \
    \      ret.push(1);\n        for (p, c) in pc {\n            let cur_size = ret.len();\n\
    \            for i in 0..cur_size {\n                let mut new_num = ret[i];\n\
    \                for _ in 0..c {\n                    new_num *= p;\n        \
    \            ret.push(new_num);\n                }\n            }\n        }\n\
    \        ret\n    }\n\n    /// \u500D\u6570\u95A2\u4FC2\u306B\u95A2\u3059\u308B\
    \u9AD8\u901F\u30BC\u30FC\u30BF\u5909\u63DB  \n    /// `list[i] = func({list[i\u306E\
    \u500D\u6570\u9054]})` \u306B\u5909\u63DB\u3059\u308B  \n    /// \u53EF\u63DB\u306A\
    \u4E8C\u9805\u6F14\u7B97`func`\u3092\u6307\u5B9A\u3059\u308B  \n    /// 0\u756A\
    \u76EE\u306E\u5024\u306B\u3064\u3044\u3066\u306F\u4F55\u3082\u3057\u306A\u3044\
    \u306E\u3067\u6CE8\u610F\n    pub fn multiple_zeta<T: Copy>(&self, mut list: Vec<T>,\
    \ func: impl Fn(T, T) -> T) -> Vec<T> {\n        let n = list.len().saturating_sub(1);\n\
    \        assert!(n <= self.max_n);\n        for p in self.primes.iter().take_while(|&&p|\
    \ p <= n) {\n            for i in (1..=(n / p)).rev() {\n                list[i]\
    \ = func(list[i], list[i * p]);\n            }\n        }\n        list\n    }\n\
    \n    /// \u500D\u6570\u95A2\u4FC2\u306B\u95A2\u3059\u308B\u9AD8\u901F\u30E1\u30D3\
    \u30A6\u30B9\u5909\u63DB(\u52A0\u7B97\u306E\u9006\u6F14\u7B97)  \n    /// 0\u756A\
    \u76EE\u306E\u5024\u306B\u3064\u3044\u3066\u306F\u4F55\u3082\u3057\u306A\u3044\
    \u306E\u3067\u6CE8\u610F\n    pub fn multiple_mobius<T: Sub<Output = T> + Copy>(&self,\
    \ mut list: Vec<T>) -> Vec<T> {\n        let n = list.len().saturating_sub(1);\n\
    \        assert!(n <= self.max_n);\n        for p in self.primes.iter().take_while(|&&p|\
    \ p <= n) {\n            for i in 1..=(n / p) {\n                list[i] = list[i]\
    \ - list[i * p];\n            }\n        }\n        list\n    }\n\n    /// \u6DFB\
    \u3048\u5B57gcd\u7573\u307F\u8FBC\u307F  \n    /// 0\u756A\u76EE\u306E\u5024\u306B\
    \u3064\u3044\u3066\u306F\u4F55\u3082\u3057\u306A\u3044\u306E\u3067\u6CE8\u610F\
    \n    pub fn gcd_convolution<T: Add<Output = T> + Sub<Output = T> + MulAssign\
    \ + Copy>(\n        &self,\n        f: &[T],\n        g: &[T],\n    ) -> Vec<T>\
    \ {\n        assert_eq!(f.len(), g.len());\n        let n = f.len().saturating_sub(1);\n\
    \        assert!(n <= self.max_n);\n        let f = f.to_vec();\n        let mut\
    \ f = self.multiple_zeta(f, |a, b| a + b);\n        let g = g.to_vec();\n    \
    \    let g = self.multiple_zeta(g, |a, b| a + b);\n        for i in 1..=n {\n\
    \            f[i] *= g[i];\n        }\n        self.multiple_mobius(f)\n    }\n\
    \n    /// \u7D04\u6570\u95A2\u4FC2\u306B\u95A2\u3059\u308B\u9AD8\u901F\u30BC\u30FC\
    \u30BF\u5909\u63DB  \n    /// `list[i] = func({list[i\u306E\u7D04\u6570\u9054\
    ]})` \u306B\u5909\u63DB\u3059\u308B  \n    /// \u53EF\u63DB\u306A\u4E8C\u9805\u6F14\
    \u7B97`func`\u3092\u6307\u5B9A\u3059\u308B  \n    /// 0\u756A\u76EE\u306E\u5024\
    \u306B\u3064\u3044\u3066\u306F\u4F55\u3082\u3057\u306A\u3044\u306E\u3067\u6CE8\
    \u610F\n    pub fn divisor_zeta<T: Copy>(&self, mut list: Vec<T>, func: impl Fn(T,\
    \ T) -> T) -> Vec<T> {\n        let n = list.len().saturating_sub(1);\n      \
    \  assert!(n <= self.max_n);\n        for p in self.primes.iter().take_while(|&&p|\
    \ p <= n) {\n            for i in 1..=(n / p) {\n                list[i * p] =\
    \ func(list[i * p], list[i]);\n            }\n        }\n        list\n    }\n\
    \n    /// \u7D04\u6570\u95A2\u4FC2\u306B\u95A2\u3059\u308B\u9AD8\u901F\u30E1\u30D3\
    \u30A6\u30B9\u5909\u63DB(\u52A0\u7B97\u306E\u9006\u6F14\u7B97)  \n    /// 0\u756A\
    \u76EE\u306E\u5024\u306B\u3064\u3044\u3066\u306F\u4F55\u3082\u3057\u306A\u3044\
    \u306E\u3067\u6CE8\u610F\n    pub fn divisor_mobius<T: Sub<Output = T> + Copy>(&self,\
    \ mut list: Vec<T>) -> Vec<T> {\n        let n = list.len().saturating_sub(1);\n\
    \        assert!(n <= self.max_n);\n        for p in self.primes.iter().take_while(|&&p|\
    \ p <= n) {\n            for i in (1..=(n / p)).rev() {\n                list[i\
    \ * p] = list[i * p] - list[i];\n            }\n        }\n        list\n    }\n\
    }\n\n/// \u30AA\u30A4\u30E9\u30FC\u306E\u30C8\u30FC\u30B7\u30A7\u30F3\u30C8\u95A2\
    \u6570 \u03C6(n) (: = n\u3068\u4E92\u3044\u306B\u7D20\u306An\u4EE5\u4E0B\u306E\
    \u81EA\u7136\u6570\u306E\u500B\u6570) \u3092 1\u304B\u3089n\u307E\u3067\u307E\u3068\
    \u3081\u3066\u6C42\u3081\u308B `O(n log log n)`\npub fn euler_totient_function(n:\
    \ usize) -> Vec<usize> {\n    let mut phi = (0..=n).collect::<Vec<usize>>();\n\
    \    for p in 2..=n {\n        if phi[p] != p {\n            continue;\n     \
    \   }\n        for multiple in (p..=n).step_by(p) {\n            phi[multiple]\
    \ -= phi[multiple] / p;\n        }\n    }\n    phi\n}\n\nfn mod_pow(base: u64,\
    \ mut exp: u64, modulus: u64) -> u64 {\n    let mut res = 1;\n    let mut b =\
    \ (base % modulus) as u128;\n    let modulus = modulus as u128;\n    while exp\
    \ > 0 {\n        if exp & 1 == 1 {\n            res = (res * b) % modulus;\n \
    \       }\n        b = (b * b) % modulus;\n        exp >>= 1;\n    }\n    res\
    \ as u64\n}\n\nfn suspect(a: u64, mut t: u64, n: u64) -> bool {\n    let mut x\
    \ = mod_pow(a, t, n);\n    let n1 = n - 1;\n    while t != n1 && x != 1 && x !=\
    \ n1 {\n        x = mod_pow(x, 2, n);\n        t <<= 1;\n    }\n    ((t & 1) ==\
    \ 1) || x == n1\n}\n\n/// `n < 2^64`\u306B\u304A\u3051\u308B\u30DF\u30E9\u30FC\
    \u30FB\u30E9\u30D3\u30F3\u7D20\u6570\u5224\u5B9A\u6CD5 `O(log n)`  \n/// \u30AA\
    \u30FC\u30D0\u30D5\u30ED\u30FC\u5BFE\u7B56\u3067128bit\u6574\u6570\u3092\u4F7F\
    \u7528\u3057\u3066\u3044\u308B\u5206\u5C11\u3057\u9045\u3044\u304B\u3082  \n///\
    \ \u9023\u7D9A\u3059\u308B\u533A\u9593\u306E\u7D20\u6570\u5224\u5B9A\u3092\u884C\
    \u3046\u5834\u5408\u306F\u3001`is_prime_range`\u3092\u4F7F\u7528\u3059\u308B\u306E\
    \u304C\u3088\u3055\u305D\u3046\npub fn miller_rabin(n: u64) -> bool {\n    if\
    \ n < 2 {\n        return false;\n    }\n    const CHECK_LIST: [u64; 12] = [2,\
    \ 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];\n    for p in CHECK_LIST {\n     \
    \   if n % p == 0 {\n            return n == p;\n        }\n    }\n    let mut\
    \ d = (n - 1) >> 1;\n    d >>= d.trailing_zeros();\n    for a in CHECK_LIST.into_iter().take_while(|&a|\
    \ a < n) {\n        if !suspect(a, d, n) {\n            return false;\n      \
    \  }\n    }\n    true\n}\n\n#[cfg(test)]\nmod test {\n    use super::*;\n    use\
    \ rand::prelude::*;\n\n    #[test]\n    fn test_divisors_manual() {\n        let\
    \ era = Eratosthenes::new(60);\n        let mut divisors_60 = era.enumerate_divisors(60);\n\
    \        divisors_60.sort_unstable();\n        assert_eq!(divisors_60, [1, 2,\
    \ 3, 4, 5, 6, 10, 12, 15, 20, 30, 60])\n    }\n\n    #[test]\n    fn test_multiple_zeta_manual()\
    \ {\n        let list = (0..=12).collect::<Vec<usize>>();\n        let era = Eratosthenes::new(12);\n\
    \        let list = era.multiple_zeta(list, |a, b| a + b);\n        assert_eq!(list,\
    \ [0, 78, 42, 30, 24, 15, 18, 7, 8, 9, 10, 11, 12]);\n    }\n\n    #[test]\n \
    \   fn test_divisor_zeta_manual() {\n        let list = (0..=12).collect::<Vec<usize>>();\n\
    \        let era = Eratosthenes::new(12);\n        let list = era.divisor_zeta(list,\
    \ |a, b| a + b);\n        assert_eq!(list, [0, 1, 3, 4, 7, 6, 12, 8, 15, 13, 18,\
    \ 12, 28]);\n    }\n\n    #[test]\n    fn test_zeta_mobius() {\n        fn test(size:\
    \ usize) {\n            let mut rng = thread_rng();\n            let list = (0..=size)\n\
    \                .map(|_| rng.gen_range(-100_000_000..=100_000_000))\n       \
    \         .collect::<Vec<i64>>();\n            let era = Eratosthenes::new(size);\n\
    \            let zeta = era.multiple_zeta(list.clone(), |a, b| a + b);\n     \
    \       let mobius = era.multiple_mobius(zeta);\n            assert_eq!(list,\
    \ mobius);\n            let zeta = era.divisor_zeta(list.clone(), |a, b| a + b);\n\
    \            let mobius = era.divisor_mobius(zeta);\n            assert_eq!(list,\
    \ mobius);\n        }\n        for size in [0, 1, 10, 100, 1000, 10000, 100000,\
    \ 1000000] {\n            test(size);\n        }\n    }\n\n    #[test]\n    fn\
    \ test_gcd_conv() {\n        fn test(size: usize) {\n            let mut rng =\
    \ thread_rng();\n            let f = (0..=size)\n                .map(|_| rng.gen_range(-100..=100))\n\
    \                .collect::<Vec<i64>>();\n            let g = (0..=size)\n   \
    \             .map(|_| rng.gen_range(-100..=100))\n                .collect::<Vec<i64>>();\n\
    \            let era = Eratosthenes::new(size);\n            let conv = era.gcd_convolution(&f,\
    \ &g);\n            let mut ans = vec![0; size + 1];\n            for i in 1..=size\
    \ {\n                for j in 1..=size {\n                    let gcd = num::integer::gcd(i,\
    \ j);\n                    ans[gcd] += f[i] * g[j];\n                }\n     \
    \       }\n            assert!(conv.iter().skip(1).eq(ans.iter().skip(1)));\n\
    \        }\n        for size in [0, 1, 10, 100, 1000] {\n            test(size);\n\
    \        }\n    }\n\n    #[test]\n    fn test_miller_rabin() {\n        const\
    \ SIZE: usize = 1000000;\n        let era = Eratosthenes::new(SIZE);\n       \
    \ for i in 1..=SIZE {\n            assert_eq!(era.is_prime(i), miller_rabin(i\
    \ as u64), \"i = {}\", i);\n        }\n\n        assert!(!miller_rabin(10_u64.pow(18)\
    \ * 2 + 1));\n        assert!(miller_rabin((1_u64 << 61) - 1));\n    }\n\n   \
    \ #[test]\n    fn test_factorize_range() {\n        const SIZE: usize = 1000000;\n\
    \        let era = Eratosthenes::new(SIZE);\n        let fact_range = era.factorize_range(0,\
    \ SIZE);\n        for i in 0..=SIZE {\n            let fact = era.factorize(i);\n\
    \            assert_eq!(fact, fact_range[i]);\n        }\n    }\n\n    #[test]\n\
    \    fn test_is_prime_range() {\n        const SIZE: usize = 1000000;\n      \
    \  let era = Eratosthenes::new(SIZE);\n        let is_prime_range = era.is_prime_range(0,\
    \ SIZE);\n        for i in 0..=SIZE {\n            assert_eq!(era.is_prime(i),\
    \ is_prime_range[i], \"i = {}\", i);\n        }\n    }\n\n    #[test]\n    fn\
    \ test_euler_totient_function() {\n        const SIZE: usize = 10000;\n      \
    \  let phi = euler_totient_function(SIZE);\n        for i in 1..=SIZE {\n    \
    \        let mut cnt = 0;\n            for j in 1..=i {\n                if num::integer::gcd(i,\
    \ j) == 1 {\n                    cnt += 1;\n                }\n            }\n\
    \            assert_eq!(phi[i], cnt, \"i = {}\", i);\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/math/prime_utils/src/lib.rs
  requiredBy: []
  timestamp: '2025-10-19 13:58:01+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/math/prime_utils/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/prime_utils/src/lib.rs
- /library/crates/math/prime_utils/src/lib.rs.html
title: crates/math/prime_utils/src/lib.rs
---
