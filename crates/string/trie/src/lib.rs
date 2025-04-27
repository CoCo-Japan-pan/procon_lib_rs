//! トライ木(のベース実装)  
//! 同じprefixをまとめて処理できる  
//! Rolling Hashで代替できることもある

/// アルファベット小文字をBytes形式で受け取ることを仮定している
#[derive(Debug, Clone, Default)]
pub struct Trie {
    chidlren: [Option<Box<Trie>>; 26],
    /// その部分木内に含まれる文字列の個数
    cnt: usize,
    /// 受理するか
    is_end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, s: &[u8]) {
        let mut node = self;
        for &c in s {
            let idx = (c - b'a') as usize;
            if node.chidlren[idx].is_none() {
                node.chidlren[idx] = Some(Box::new(Trie::new()));
            }
            node = node.chidlren[idx].as_mut().unwrap();
            node.cnt += 1;
        }
        node.is_end = true;
    }

    /// sを接頭辞にもつ文字列を全て消す(部分木を削除する)  
    /// 消した文字列の個数を返す
    pub fn delete_prefix(&mut self, s: &[u8]) -> usize {
        if s.is_empty() {
            let ret = self.cnt;
            *self = Trie::new();
            return ret;
        }
        if let Some(child) = &mut self.chidlren[(s[0] - b'a') as usize] {
            let cnt = child.delete_prefix(&s[1..]);
            self.cnt -= cnt;
            cnt
        } else {
            0
        }
    }

    /// sを接頭辞に持つ文字列が存在するか
    pub fn find_prefix(&self, s: &[u8]) -> bool {
        let mut node = self;
        for &c in s {
            let idx = (c - b'a') as usize;
            if let Some(child) = &node.chidlren[idx] {
                node = child;
                if node.is_end {
                    return true;
                }
            } else {
                break;
            }
        }
        false
    }

    /// sが存在するか
    pub fn find(&self, s: &[u8]) -> bool {
        let mut node = self;
        for &c in s {
            let idx = (c - b'a') as usize;
            if let Some(child) = &node.chidlren[idx] {
                node = child;
            } else {
                return false;
            }
        }
        node.is_end
    }
}
