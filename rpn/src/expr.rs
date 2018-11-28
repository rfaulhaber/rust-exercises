use num::Num;
use std::ops::{Add, Div, Mul, Sub};

enum Operation {
    ADD,
    SUB,
    MULT,
    DIV,
    IDIV,
    EXP,
    FACT,
}

// fn map_str_to_op(s: String) -> Option<Operation> {
//     return match s.as_ref() {
//         "+" => Some(Operation::ADD),
//         "-" => Some(Operation::SUB),
//         "*" => Some(Operation::MULT),
//         "/" => Some(Operation::DIV),
//         "//" => Some(Operation::IDIV),
//         "^" => Some(Operation::EXP),
//         "FACT" => Some(Operation::FACT),
//         _ => None,
//     };
// }

// fn map_op_to_str(o: Operation) -> Option<String> {
//     return match o {
//         Operation::ADD => Some("+"),
//         "+" => Some(Operation::ADD),
//         "-" => Some(Operation::SUB),
//         "*" => Some(Operation::MULT),
//         "/" => Some(Operation::DIV),
//         "//" => Some(Operation::IDIV),
//         "^" => Some(Operation::EXP),
//         "FACT" => Some(Operation::FACT),
//     };
// }

trait Evaluable<T: Num> {
    fn evaluate(&self, Operation) -> Option<T>;
}

trait Printable {
    fn to_infix_string(&self) -> String;
    fn to_string(&self) -> String;
}

struct Expr<T: Num> {
    l: T,
    r: Box<Option<Expr<T>>>,
}

impl<T: Num> Evaluable<T> for Expr<T> {
    fn evaluate(&self, o: Operation) -> Option<T> {
        panic!("not implemented yet!");
    }
}
