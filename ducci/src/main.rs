use ducci::ducci::Ducci;

pub fn run_challenge(ducci: Ducci) -> i64 {
    let mut next = ducci;

    let mut steps = 1;

    while !next.is_terminal() {
        next = next.next_seq();
        steps = steps + 1;
    }

    return steps;
}

fn main() {
    let c1 = Ducci::new(vec![1, 5, 7, 9, 9]);
    let c2 = Ducci::new(vec![1, 2, 1, 2, 1, 0]);
    let c3 = Ducci::new(vec![10, 12, 41, 62, 31, 50]);
    let c4 = Ducci::new(vec![10, 12, 41, 62, 31]);

    println!("1: steps: {}, should equal: 23", run_challenge(c1));
    println!("2: steps: {}, should equal: 3", run_challenge(c2));
    println!("3: steps: {}, should equal: 22", run_challenge(c3));
    println!("4: steps: {}, should equal: 30", run_challenge(c4));
}
