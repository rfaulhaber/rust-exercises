use std::vec::Vec;

#[derive(Debug)]
pub struct Ducci {
    tup: Vec<i64>,
    seq_list: Vec<Vec<i64>>,
}

impl Ducci {
    pub fn new(vec: Vec<i64>) -> Ducci {
        Ducci {
            tup: vec,
            seq_list: Vec::new(),
        }
    }

    pub fn next_seq(&self) -> Ducci {
        let mut new_vec = Vec::new();

        for (i, item) in self.tup.iter().enumerate() {
            if i < self.tup.len() - 1 {
                new_vec.push((self.tup[i] - self.tup[i + 1]).abs());
            } else {
                new_vec.push((self.tup[self.tup.len() - 1] - self.tup[0]).abs());
            }
        }

        let mut new_seq_list = self.seq_list.clone();
        new_seq_list.push(self.tup.clone());

        Ducci {
            tup: new_vec,
            seq_list: new_seq_list,
        }
    }

    pub fn is_terminal(&self) -> bool {
        if !self.is_repeat() {
            for item in self.tup.iter() {
                if item != &0 {
                    return false;
                }
            }
        }

        return true;
    }

    fn is_repeat(&self) -> bool {
        return self.seq_list.contains(&self.tup);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_seq_calculates_correctly() {
        let d = Ducci::new(vec![1, 2, 1, 2, 1, 0]);
        let new_d = d.next_seq();

        assert_eq!(vec![1, 1, 1, 1, 1, 1], new_d.tup);
    }

    #[test]
    fn next_seq_calculates_correctly_non_trivial() {
        let d = Ducci::new(vec![0, 653, 1854, 4063]);
        let new_d = d.next_seq();

        assert_eq!(vec![653, 1201, 2209, 4063], new_d.tup);
    }

    #[test]
    fn is_terminal_finds_terminal_seq() {
        let d = Ducci::new(vec![0, 0, 0]);

        assert!(d.is_terminal());
    }

    #[test]
    fn is_terminal_finds_nonterminal_seq() {
        let d = Ducci::new(vec![0, 1, 0]);

        assert!(!d.is_terminal());
    }

    #[test]
    fn is_repeat_notices_repeat() {
        let d = Ducci::new(vec![0, 0, 0]);
        let new_d = d.next_seq();

        assert!(new_d.is_repeat());
    }
}
