use rayon::prelude::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Op {
    Num(i64),
    Inp(u8),
    Add(Box<Op>, Box<Op>),
    Mul(Box<Op>, Box<Op>),
    Div(Box<Op>, Box<Op>),
    Mod(Box<Op>, Box<Op>),
    Eql(Box<Op>, Box<Op>),
}

impl Op {
    fn new(from: i64) -> Op {
        Op::Num(from)
    }

    fn new_inp(from: u8) -> Op {
        Op::Inp(from)
    }

    fn is_zero(&self) -> bool {
        match self {
            Op::Num(0) => true,
            _ => false,
        }
    }

    fn is_one(&self) -> bool {
        match self {
            Op::Num(1) => true,
            _ => false,
        }
    }

    fn can_be_neg(&self) -> bool {
        match self {
            Op::Num(n) if *n < 0 => true,
            Op::Num(_) => false,
            Op::Inp(_) => false,
            Op::Eql(_, _) => false,
            Op::Add(left, right) => left.can_be_neg() || right.can_be_neg(),
            Op::Mul(left, right) => left.can_be_neg() || right.can_be_neg(),
            Op::Div(left, right) => left.can_be_neg() || right.can_be_neg(),
            Op::Mod(_, _) => false,
        }
    }

    fn is_digit(&self) -> bool {
        match self {
            Op::Num(n) if *n < 10 => true,
            Op::Num(_) => false,
            Op::Inp(_) => true,
            Op::Eql(_, _) => true,
            Op::Add(left, right) => {
                left.can_be_neg() || right.can_be_neg() || (left.is_digit() && right.is_digit())
            }
            Op::Mul(left, right) => left.is_digit() && right.is_digit(),
            Op::Div(left, right) => left.is_digit() && right.is_digit(),
            Op::Mod(left, right) => left.is_digit() || right.is_digit(),
        }
    }

    fn is_mul(&self, d: i64) -> bool {
        match self {
            Op::Num(n) => n % d == 0,
            Op::Add(left, right) => {
                left.is_mul(d) && right.is_mul(d)
            }
            Op::Mul(left, right) => {
                left.is_mul(d) || right.is_mul(d)
            }
            _ => false
        }
    }

    fn is_div(&self, d: i64) -> bool {
        if let Op::Div(left, right) = self {
            match (&**left, &**right) {
                (_, Op::Num(x)) => *x == d,
                _ => false,
            }
        } else {
            false
        }
    }

    fn do_mul(self, d: i64) -> Op {
        if let Op::Div(left, right) = self {
            match (*left, *right) {
                (op, Op::Num(x)) if x == d => op,
                _ => panic!(),
            }
        } else {
            panic!()
        }
    }

    fn max_value(&self) -> i64 {
        match self {
            Op::Num(n) => *n,
            Op::Inp(_) => 9,
            Op::Add(left, right) => left.max_value() + right.max_value(),
            Op::Mul(left, right) => if left.can_be_neg() || right.can_be_neg() {
                i64::MAX
            } else {
                left.max_value() * right.max_value()
            },
            Op::Div(left, _) => left.max_value(),
            Op::Mod(_, right) => right.max_value(),
            Op::Eql(_, _) => 1,
        }
    }

    fn min_value(&self) -> i64 {
        match self {
            Op::Num(n) => *n,
            Op::Inp(_) => 0,
            Op::Add(left, right) => left.min_value() + right.min_value(),
            Op::Mul(left, right) => if left.can_be_neg() || right.can_be_neg() {
                i64::MIN
            } else {
                left.min_value() * right.min_value()
            },
            Op::Div(left, right) => if left.can_be_neg() || right.can_be_neg() {
                i64::MIN
            } else {
                left.min_value() / right.max_value()
            },
            Op::Mod(left, right) => if left.min_value() < right.min_value() && left.max_value() < right.min_value() {
                left.min_value()
            } else {
                0
            },
            Op::Eql(_, _) => 0,
        }
    }

    fn do_mod_div(self, m: i64) -> Op {
        if self.max_value() < m {
            return self;
        }
        match self {
            Op::Num(n) => Op::new(n % m),
            Op::Inp(n) if m >= 10 => Op::new_inp(n),
            Op::Inp(_) => Op::Mod(Box::new(self), Box::new(Op::new(m))),
            Op::Add(left, right) => Op::Mod(
                Box::new(Op::Add(
                    Box::new(left.do_mod_div(m)),
                    Box::new(right.do_mod_div(m)),
                )),
                Box::new(Op::new(m)),
            ),
            Op::Mul(left, right) => Op::Mod(
                Box::new(Op::Mul(
                    Box::new(left.do_mod_div(m)),
                    Box::new(right.do_mod_div(m)),
                )),
                Box::new(Op::new(m)),
            ),
            Op::Div(_, _) => Op::Mod(Box::new(self), Box::new(Op::new(m))),
            Op::Mod(left, right) => if right.is_num() && right.get_num() == m {
                Op::Mod(left, right)
            } else {
                Op::Mod(Box::new(Op::Mod(left, right)), Box::new(Op::new(m)))
            },
            Op::Eql(_, _) => self,
        }
    }

    fn do_div(self, d: i64) -> Op {
        match self {
            Op::Num(n) => { assert!(n%d == 0); Op::Num(n/d) },
            Op::Add(left, right) => Op::Add(Box::new(left.do_div(d)), Box::new(right.do_div(d))),
            Op::Mul(left, right) if left.is_mul(d) => *right,
            Op::Mul(left, right) if right.is_mul(d) => *left,
            _ => panic!()
        }
    }

    fn get_num(&self) -> i64 {
        match self {
            Op::Num(n) => *n,
            _ => panic!(),
        }
    }

    fn is_num(&self) -> bool {
        match self {
            Op::Num(_) => true,
            _ => false,
        }
    }

    fn is_inp(&self) -> bool {
        match self {
            Op::Inp(_) => true,
            _ => false,
        }
    }

    fn reduce(self) -> Box<Op> {
        //// X_11 == X_10
        //if &self == &Op::Eql(Box::new(Op::Inp(11)), Box::new(Op::Inp(10))) {
        //    return Box::new(Op::new(1))
        //}
        //// X_7 - 1 = X_6
        //if &self == &Op::Eql(
        //    Box::new(Op::Add(Box::new(Op::Num(-1)), Box::new(Op::Inp(7)))),
        //    Box::new(Op::Inp(6))) {
        //        return Box::new(Op::new(1))
        //}
        //// 2 + X_5 = X_4
        //if &self == &Op::Eql(
        //    Box::new(Op::Add(Box::new(Op::Num(2)), Box::new(Op::Inp(5)))),
        //    Box::new(Op::Inp(4))) {
        //        return Box::new(Op::new(1))
        //}
        //// ((((X_7 + 10) + (X_5 + 3)) % 26) + -1) != X_4
        //if &self == &Op::Eql(
        //    Box::new(
        //        Op::Add(
        //            Box::new(Op::Mod(
        //                    Box::new(Op::Add(
        //                            Box::new(Op::Add(Box::new(Op::Inp(7)), Box::new(Op::Num(10)))),
        //                            Box::new(Op::Add(Box::new(Op::Inp(5)), Box::new(Op::Num(3)))),
        //                    )),
        //                    Box::new(Op::Num(26))
        //            )),
        //            Box::new(Op::Num(-1)))),
        //        Box::new(Op::Inp(4))) {
        //        return Box::new(Op::new(0))
        //}
        match self {
            Op::Num(_) => Box::new(self),
            Op::Inp(_) => Box::new(self),
            Op::Add(left, right) => {
                let left = left.reduce();
                let right = right.reduce();
                match (*left, *right) {
                    (left, right) if left.is_zero() => Box::new(right),
                    (left, right) if right.is_zero() => Box::new(left),
                    (left, right) if left.is_num() && right.is_num() =>
                        Box::new(Op::new(left.get_num() + right.get_num())),
                    (Op::Add(n, x), m)
                    | (Op::Add(x, n), m)
                    | (m, Op::Add(n, x))
                    | (m, Op::Add(x, n))
                        if n.is_num() && m.is_num() =>
                        Op::Add(Box::new(Op::new(n.get_num() + m.get_num())), x).reduce(),
                    (Op::Add(n, x), op) | (Op::Add(x, n), op)
                    | (op, Op::Add(n, x)) | (op, Op::Add(x, n)) if !n.is_inp() && x.is_inp() =>
                        Op::Add(Box::new(Op::Add(Box::new(op), n)), x).reduce(),
                    (left, right) => Box::new(Op::Add(Box::new(left), Box::new(right))),
                }
            }
            Op::Mul(left, right) => {
                let left = left.reduce();
                let right = right.reduce();
                match (*left, *right) {
                    (Op::Mul(n, x), m)
                    | (Op::Mul(x, n), m)
                    | (m, Op::Mul(n, x))
                    | (m, Op::Mul(x, n))
                        if n.is_num() && m.is_num() =>
                        Op::Mul(Box::new(Op::new(n.get_num() * m.get_num())), x).reduce(),
                   (Op::Add(x, n), m) | (m, Op::Add(x, n)) if m.is_num() =>
                        Box::new(Op::Add(
                            Op::Mul(n, Box::new(m.clone())).reduce(),
                            Box::new(Op::Mul(Box::new(m), x)),
                        )),
                   (Op::Div(x, n), m) | (m, Op::Div(x, n)) if m.is_num() && n.is_num() && m.get_num() == n.get_num() => x,
                    (left, right) if left.is_num() && right.is_num() => Box::new(Op::new(left.get_num() * right.get_num())),
                    (left, right) if left.is_zero() || right.is_zero() => Box::new(Op::new(0)),
                    (left, right) if left.is_one() => Box::new(right), 
                    (left, right) if right.is_one() => Box::new(left),
                    (left, right) => Box::new(Op::Mul(Box::new(left), Box::new(right)))
                }
            }
            Op::Div(left, right) => {
                let left = left.reduce();
                let right = right.reduce();
                match (*left, *right) {
                    (left, _) if left.is_zero() => Box::new(Op::new(0)),
                    (_, right) if right.is_zero() => panic!(),
                    (left, right) if right.is_one() => Box::new(left),
                    (left, right) if left.is_num() && right.is_num() => {
                        let (a, b) = (left.get_num(), right.get_num());
                        Box::new(Op::Num(a / b))
                    }
                    (Op::Add(a, b), m) if m.is_num() =>
                        Box::new(Op::Add(
                            Op::Div(a, Box::new(m.clone())).reduce(),
                            Op::Div(b, Box::new(m.clone())).reduce(),
                        )),
                   (Op::Mul(x, n), m) | (Op::Mul(n, x), m) if m.is_num() && n.is_num() && m.get_num() == n.get_num() => x,
                   (Op::Mul(x, n), m) | (Op::Mul(n, x), m) if m.is_num() && n.is_num() && n.get_num() % m.get_num() == 0 =>
                       Op::Mul(Box::new(Op::Num(n.get_num() / m.get_num())), x).reduce(),
                    (left, right) if left.is_inp() && right.min_value() >= 10 => Box::new(Op::new(0)),
                    (left, right) => Box::new(Op::Div(Box::new(left), Box::new(right)))
                }
            }
            Op::Mod(left, right) => {
                let left = left.reduce();
                let right = right.reduce();
                match (*left, right) {
                    (left, right) if left.max_value() < right.max_value() => Box::new(left),
                    (Op::Mod(lleft, lright), right) if lright.is_num() && right.is_num() && lright.get_num() == right.get_num()
                        => Op::Mod(lleft, lright).reduce(),
                    (left, right) if right.is_num() => Box::new(left.do_mod_div(right.get_num())),
                    (left, right) => Box::new(Op::Mod(Box::new(left), right))
                }
            }
            Op::Eql(left, right) => {
                let left = left.reduce();
                let right = right.reduce();
                match (*left, *right) {
                    (left, right) if left.is_num() && right.is_num() => Box::new(Op::new((left.get_num() == right.get_num()) as i64)),
                    (left, right) if left.is_zero() && right.is_zero() => Box::new(Op::new(1)),
                    (left, right) if left.min_value() > right.max_value() => Box::new(Op::new(0)),
                    (left, right) if left.max_value() < right.min_value() => Box::new(Op::new(0)),
                    (left, right) => Box::new(Op::Eql(Box::new(left), Box::new(right)))
                }
            }
        }
    }

    fn rewrite(&mut self, terms: [u8; 14]) {
        match self {
            Op::Num(_) => {},
            Op::Inp(i) => *self = Op::Num(terms[*i as usize] as i64),
            Op::Add(left, right)
            | Op::Mul(left, right)
            | Op::Div(left, right)
            | Op::Mod(left, right)
            | Op::Eql(left, right) => {
                left.rewrite(terms);
                right.rewrite(terms);
            },
        }
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (left, op, right) = match self {
            Op::Num(e) => return write!(f, "{}", e),
            Op::Inp(x) => return write!(f, "X_{}", x),
            Op::Add(left, right) => (left, "+", right),
            Op::Mul(left, right) => (left, "*", right),
            Op::Div(left, right) => (left, "/", right),
            Op::Mod(left, right) => (left, "%", right),
            Op::Eql(left, right) => (left, "=", right),
        };
        write!(f, "(")?;
        write!(f, "{} {} {}", left, op, right)?;
        write!(f, ")")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NumOrAddr {
    Num(i64),
    Addr(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Instr {
    Inp(u8),
    Add(u8, NumOrAddr),
    Mul(u8, NumOrAddr),
    Div(u8, NumOrAddr),
    Mod(u8, NumOrAddr),
    Eql(u8, NumOrAddr),
}

type Parsed = Vec<Instr>;
type Memory = [Op; 4];

struct State {
    mem: Memory,
}

impl State {
    fn new() -> State {
        State {
            mem: [Op::Num(0), Op::Num(0), Op::Num(0), Op::Num(0)],
        }
    }

    fn read(&self, num: u8) -> Op {
        self.mem[num as usize].clone()
    }

    fn write(&mut self, num: u8, val: Op) {
        self.mem[num as usize] = val;
    }

    fn read_val(&self, num: NumOrAddr) -> Op {
        match num {
            NumOrAddr::Num(e) => Op::new(e),
            NumOrAddr::Addr(a) => self.read(a),
        }
    }

    fn op_add(&self, addr: u8, val: NumOrAddr) -> Op {
        let mem_val = self.read(addr);
        let op_val = self.read_val(val);
        let res = *match (mem_val, op_val) {
            (n, op) | (op, n) if n.is_zero() => op,
            (Op::Num(a), Op::Num(b)) => Op::new(a + b),
            (left, right) => Op::Add(Box::new(left), Box::new(right)),
        }.reduce();
        res
    }

    fn op_mul(&self, addr: u8, val: NumOrAddr) -> Op {
        let mem_val = self.read(addr);
        let op_val = self.read_val(val);
        let res = match (mem_val, op_val) {
            (n, _) | (_, n) if n.is_zero() => Op::Num(0),
            (n, op) | (op, n) if n.is_one() => op,
            (Op::Num(b), op) | (op, Op::Num(b)) if op.is_div(b) => op.do_mul(b),
            (Op::Num(a), Op::Num(b)) => Op::new(a * b),
            (left, right) => Op::Mul(Box::new(left), Box::new(right)),
        };
        res
    }

    fn op_div(&self, addr: u8, val: NumOrAddr) -> Op {
        let mem_val = self.read(addr);
        let op_val = self.read_val(val);
        let res = match (mem_val, op_val) {
            (n, _) if n.is_zero() => Op::Num(0),
            (_, n) if n.is_zero() => panic!(),
            (op, n) if n.is_one() => op,
            (op, Op::Num(b)) if op.is_mul(b) => op.do_div(b),
            (Op::Num(a), Op::Num(b)) => Op::new(a / b),
            (left, right) => Op::Div(Box::new(left), Box::new(right)),
        };
        res
    }

    fn op_mod(&self, addr: u8, val: NumOrAddr) -> Op {
        let mem_val = self.read(addr);
        let op_val = self.read_val(val);
        let res = *match (mem_val, op_val) {
            (n, _) if n.is_zero() => Op::Num(0),
            (_, n) if n.is_zero() => panic!(),
            (Op::Inp(a), n) if !n.is_digit() => Op::new_inp(a),
            (Op::Num(a), Op::Num(b)) => Op::new(a % b),
            (op, Op::Num(b)) => op.do_mod_div(b),
            (left, right) => Op::Mod(Box::new(left), Box::new(right)),
        }.reduce();
        res
    }

    fn op_eql(&self, addr: u8, val: NumOrAddr) -> Op {
        let mem_val = self.read(addr);
        let op_val = self.read_val(val);
        let res = match (mem_val, op_val) {
            (n, Op::Inp(_)) | (Op::Inp(_), n) if n.is_zero() || !n.is_digit() => Op::Num(0),
            (Op::Num(a), Op::Num(b)) => Op::new((a == b) as i64),
            (left, right) => Op::Eql(Box::new(left), Box::new(right)),
        };
        res
    }

    fn run_program(mut self, program: &[Instr], mut input: Vec<i64>) -> Memory {
        input.reverse();

        for instr in program {
            match *instr {
                Instr::Inp(addr) => self.write(
                    addr,
                    Op::new_inp({
                        input.pop().unwrap();
                        input.len() as u8
                    }),
                ),
                Instr::Add(addr, val) => self.write(addr, self.op_add(addr, val)),
                Instr::Mul(addr, val) => self.write(addr, self.op_mul(addr, val)),
                Instr::Div(addr, val) => self.write(addr, self.op_div(addr, val)),
                Instr::Mod(addr, val) => self.write(addr, self.op_mod(addr, val)),
                Instr::Eql(addr, val) => self.write(addr, self.op_eql(addr, val)),
            }
        }

        self.mem
    }
}

fn twentyfour_impl(input: &Parsed, day_2: bool) -> usize {
    if day_2 {
        unimplemented!();
    }
    let stmt = State::new().run_program(input, [0; 14].to_vec());

    for x_13 in (1..=9).rev() {
        for x_12 in (1..=9).rev() {
            for x_11 in (1..=9).rev() {
                let x_10 = x_11;
                for x_9 in (1..=9).rev() {
                    for x_8 in (1..=9).rev() {
                        (2..=9).into_par_iter().rev().for_each(|x_7| {
                            let x_6 = x_7 - 1;
                            for x_5 in (1..=9).rev() {
                                for x_4 in (1..=9).rev() {
                                    for x_3 in (1..=9).rev() {
                                        for x_2 in (1..=9).rev() {
                                            for x_1 in (1..=9).rev() {
                                                for x_0 in (1..=9).rev() {
                                                    let mut new_stmt = stmt[3].clone();
                                                    let params = [
                                                         x_0, x_1, x_2, x_3,
                                                         x_4, x_5, x_6, x_7,
                                                         x_8, x_9, x_10, x_11,
                                                         x_12, x_13,
                                                    ];
                                                    new_stmt.rewrite(params);

                                                    if x_3 == 9 && x_2 == 9 && x_1 == 9 && x_0 == 9 {
                                                        println!("{:?}", params);
                                                    }
                                                    if *new_stmt.reduce() == Op::Num(0) {
                                                        println!("Found solution: {:?}", params);
                                                        panic!();
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        });
                    }
                }
            }
        }
    }

    unimplemented!();
}

fn parse<S: AsRef<str>>(input: &[S]) -> Parsed {
    fn parse_addr(s: &str) -> u8 {
        let c = s.chars().next().unwrap();
        match c {
            'x' | 'y' | 'z' | 'w' => (c as u8) - b'w',
            _ => panic!(),
        }
    }
    fn parse_num_or_addr(s: &str) -> NumOrAddr {
        let c = s.chars().next().unwrap();
        match c {
            'x' | 'y' | 'z' | 'w' => NumOrAddr::Addr((c as u8) - b'w'),
            _ => NumOrAddr::Num(s.parse().unwrap()),
        }
    }

    input
        .iter()
        .map(|s| {
            let parts = s.as_ref().split(' ').collect::<Vec<_>>();
            let addr = parse_addr(parts[1]);
            if parts[0] == "inp" {
                return Instr::Inp(addr);
            }
            let operand = parse_num_or_addr(parts[2]);
            match parts[0] {
                "add" => Instr::Add(addr, operand),
                "mul" => Instr::Mul(addr, operand),
                "div" => Instr::Div(addr, operand),
                "mod" => Instr::Mod(addr, operand),
                "eql" => Instr::Eql(addr, operand),
                op => unimplemented!("{}", op),
            }
        })
        .collect()
}

pub fn twentyfour() -> Result<(), std::io::Error> {
    let file = File::open("24_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = twentyfour_impl(&lines, false);
    println!("Day 24 part 1: {}", res);
    let res_2 = twentyfour_impl(&lines, true);
    println!("Day 24 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_24::{parse, Memory, Parsed, State};

    fn run_program(program: &Parsed, input: Vec<i64>) -> Memory {
        State::new().run_program(program, input)
    }

    #[test]
    fn it_works() {
        assert_eq!(
            [0, -2, 0, 0],
            run_program(&parse(&vec!["inp x", "mul x -1",]), vec![2],)
        );
        assert_eq!(
            [0, 4, 0, 0],
            run_program(
                &parse(&vec!["inp z", "inp x", "mul z 3", "eql z x",]),
                vec![2, 4],
            )
        );
        assert_eq!(
            [0, 6, 0, 1],
            run_program(
                &parse(&vec!["inp z", "inp x", "mul z 3", "eql z x",]),
                vec![2, 6],
            )
        );
        assert_eq!(
            [0, 1, 0, 1],
            run_program(
                &parse(&vec![
                    "inp w", "add z w", "mod z 2", "div w 2", "add y w", "mod y 2", "div w 2",
                    "add x w", "mod x 2", "div w 2", "mod w 2",
                ]),
                vec![5],
            )
        );
    }
}
