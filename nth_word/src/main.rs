use std::io;

fn main() {
    let word = "the quick brown fox jumped over the lazy dog";
    let n = 3;

    println!("the nth ({}) word is {} of \"{}\"", n, nth_word(word, n), word);
}


fn nth_word(s: &str, n: i32) -> &str {
    let bytes = s.as_bytes();

    let mut start = 0;
    let mut end = 0;
    let mut counter = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && counter < n {
          start = end;
          end = i;
          counter = counter + 1;
	  println!("start {} end {} counter {}", start, end, counter)
        } else if counter == n {
	  return &s[start as usize..end as usize];
	}
    }

    &s[..]
}
