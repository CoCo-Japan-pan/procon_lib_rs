/// From <https://github.com/ngtkana/ac-adapter-rs/blob/main/libs/next_permutation/src/lib.rs>  
/// Returns the next permutation of `a` in lexicographic order.
pub fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    let Some(i) = a.windows(2).rposition(|w| w[0] < w[1]) else {
        return false;
    };
    let j = a.iter().rposition(|x| x > &a[i]).unwrap();
    a.swap(i, j);
    a[i + 1..].reverse();
    true
}

pub fn permutations(n: usize) -> Permutations<usize> {
    Permutations::new((0..n).collect())
}

pub struct Permutations<T: Ord + Clone> {
    data: Vec<T>,
    first: bool,
}

impl<T: Ord + Clone> Permutations<T> {
    pub fn new(data: Vec<T>) -> Self {
        Permutations { data, first: true }
    }
}

impl<T: Ord + Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.data.clone());
        }
        if next_permutation(&mut self.data) {
            Some(self.data.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        let mut perms = permutations(3);
        assert_eq!(perms.next().unwrap(), vec![0, 1, 2]);
        assert_eq!(perms.next().unwrap(), vec![0, 2, 1]);
        assert_eq!(perms.next().unwrap(), vec![1, 0, 2]);
        assert_eq!(perms.next().unwrap(), vec![1, 2, 0]);
        assert_eq!(perms.next().unwrap(), vec![2, 0, 1]);
        assert_eq!(perms.next().unwrap(), vec![2, 1, 0]);
        assert!(perms.next().is_none());
    }
}
