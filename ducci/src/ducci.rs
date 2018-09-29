use std::vec::Vec;

pub struct Ducci {
    tup: Vec<i64>,
}

impl Ducci {
    pub fn new(vec: Vec<i64>) -> Ducci {
        Ducci { tup: vec }
    }
}

impl std::fmt::Display for Ducci {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut str = String::new();

        for (i, item) in self.tup.iter().enumerate() {
            if i == self.tup.len() {

            } else {
                str.push_str(item.to_string());
                str.push(", ");
            }
        }

        write!(f, "[{}]", str)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
