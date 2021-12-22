use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Peekable;

#[derive(Clone, Debug, PartialEq)]
enum Number {
    Num(u32),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    fn add(self, other: Box<Number>) -> Number {
        Number::Pair(Box::new(self), other)
    }
    fn num(&self) -> Option<u32> {
        match self {
            Number::Num(e) => Some(*e),
            Number::Pair(_, _) => None,
        }
    }
    fn add_left(&mut self, num: u32) {
        match self {
            Number::Num(e) => *e += num,
            Number::Pair(l, _) => l.add_left(num),
        }
    }
    fn add_right(&mut self, num: u32) {
        match self {
            Number::Num(e) => *e += num,
            Number::Pair(_, r) => r.add_right(num),
        }
    }
    fn explode(&mut self, depth: usize) -> Option<(Option<u32>, Option<u32>)> {
        match self {
            Number::Num(_) => None,
            Number::Pair(l, r) => {
                if let (Some(left), Some(right)) = (l.num(), r.num()) {
                    if depth >= 4 {
                        *self = Number::Num(0);
                        Some((Some(left), Some(right)))
                    } else {
                        None
                    }
                } else if let Some((left, right)) = l.explode(depth + 1) {
                    if let Some(right) = right {
                        r.add_left(right);
                        Some((left, None))
                    } else {
                        Some((left, right))
                    }
                } else if let Some((left, right)) = r.explode(depth + 1) {
                    if let Some(left) = left {
                        l.add_right(left);
                        Some((None, right))
                    } else {
                        Some((left, right))
                    }
                } else {
                    None
                }
            }
        }
    }
    fn split(&mut self) -> bool {
        match self {
            Number::Num(e) => {
                if *e >= 10 {
                    *self = Number::Pair(
                        Box::new(Number::Num(*e / 2)),
                        Box::new(Number::Num((*e + 1) / 2)),
                    );
                    true
                } else {
                    false
                }
            }
            Number::Pair(l, r) => l.split() || r.split(),
        }
    }
    fn reduce(&mut self) {
        let mut cont = true;
        while cont {
            while self.explode(0).is_some() {}
            cont = self.split();
        }
    }
    fn magnitude(&self) -> u32 {
        match self {
            Number::Num(e) => *e,
            Number::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

type Parsed = Vec<Vec<char>>;

fn parse_nums<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> Box<Number> {
    match iter.next() {
        Some('[') => {
            let left = parse_nums(iter);
            assert_eq!(iter.next(), Some(','));
            let right = parse_nums(iter);
            assert_eq!(iter.next(), Some(']'));
            Box::new(Number::Pair(left, right))
        }
        Some(c) => Box::new(Number::Num(c.to_digit(10).unwrap())),
        None => unreachable!(),
    }
}

fn eighteen_impl(input: &[Vec<char>], day_2: bool) -> u32 {
    let mut nums = Vec::new();
    for row in input {
        nums.push(parse_nums(&mut row.iter().copied().peekable()));
    }

    if !day_2 {
        let mut res: Number = *nums[0].clone();
        for num in &nums[1..] {
            res = res.add(num.clone());
            res.reduce();
        }
        return res.magnitude();
    }

    let mut best = 0;

    for i in 0..nums.len() {
        for h in 0..nums.len() {
            if i == h {
                continue;
            }

            let mut res = nums[i].clone().add(nums[h].clone());
            res.reduce();
            best = best.max(res.magnitude());
        }
    }

    best
}

fn parse<S: AsRef<str>>(input: &[S]) -> Parsed {
    input.iter().map(|s| s.as_ref().chars().collect()).collect()
}

pub fn eighteen() -> Result<(), std::io::Error> {
    let file = File::open("18_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = eighteen_impl(&lines, false);
    println!("Day 15 part 1: {}", res);
    let res_2 = eighteen_impl(&lines, true);
    println!("Day 15 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_18::{eighteen_impl, parse, parse_nums, Number};

    fn pn(s: &str) -> Box<Number> {
        parse_nums(&mut s.chars().peekable())
    }

    fn p(l: Number, r: Number) -> Number {
        Number::Pair(Box::new(l), Box::new(r))
    }

    #[test]
    fn nums() {
        use Number::*;
        assert_eq!(
            p(p(Num(1), Num(2)), p(p(Num(3), Num(4)), Num(5))),
            pn("[1,2]").add(pn("[[3,4],5]"))
        );

        let mut r = pn("[[[[[9,8],1],2],3],4]");
        assert_eq!(Some((Some(9), None)), r.explode(0));
        assert_eq!(p(p(p(p(Num(0), Num(9)), Num(2)), Num(3)), Num(4)), *r);

        assert_eq!(
            3488,
            pn("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude()
        );

        let mut r2 = pn("[[[[4,3],4],4],[7,[[8,4],9]]]").add(pn("[1,1]"));
        r2.reduce();
        assert_eq!(*pn("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), r2);

        println!();
        let mut r3 = pn("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")
            .add(pn("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"));
        r3.reduce();
        assert_eq!(
            *pn("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"),
            r3
        );
    }

    #[test]
    fn it_works() {
        assert_eq!(
            4140,
            eighteen_impl(
                &parse(&vec![
                    "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
                    "[[[5,[2,8]],4],[5,[[9,9],0]]]",
                    "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
                    "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
                    "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
                    "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
                    "[[[[5,4],[7,7]],8],[[8,3],8]]",
                    "[[9,3],[[9,9],[6,[4,9]]]]",
                    "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
                    "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
                ]),
                false
            )
        );
        assert_eq!(
            3993,
            eighteen_impl(
                &parse(&vec![
                    "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
                    "[[[5,[2,8]],4],[5,[[9,9],0]]]",
                    "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
                    "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
                    "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
                    "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
                    "[[[[5,4],[7,7]],8],[[8,3],8]]",
                    "[[9,3],[[9,9],[6,[4,9]]]]",
                    "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
                    "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
                ]),
                true
            )
        );
    }
}
