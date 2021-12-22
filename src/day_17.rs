use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Parsed = ((i32, i32), (i32, i32));

#[derive(Debug)]
struct Body {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Body {
    fn step(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        match self.vel.0.cmp(&0) {
            std::cmp::Ordering::Greater => self.vel.0 -= 1,
            std::cmp::Ordering::Less => self.vel.0 += 1,
            std::cmp::Ordering::Equal => {}
        }
        self.vel.1 -= 1;
    }
}

fn simulate(vel: (i32, i32), target: Parsed) -> Option<i32> {
    let mut max_y = 0;
    let mut body = Body { pos: (0, 0), vel };
    while body.pos.1 >= target.1 .0 {
        if target.0 .0 <= body.pos.0
            && body.pos.0 <= target.0 .1
            && target.1 .0 <= body.pos.1
            && body.pos.1 <= target.1 .1
        {
            return Some(max_y);
        }
        body.step();
        max_y = max_y.max(body.pos.1);
    }

    None
}

fn seventeen_impl(input: &Parsed) -> (i32, i32) {
    let mut best = i32::MIN;
    let mut count = 0;
    for xvel in (input.0 .0.min(0) - 1)..(input.0 .1.max(0) + 1) {
        for yvel in (input.1 .0.min(0) - 1)..1000 {
            if let Some(h) = simulate((xvel, yvel), *input) {
                best = best.max(h);
                count += 1;
            }
        }
    }
    (best, count)
}

fn parse<S: AsRef<str>>(input: &[S]) -> Parsed {
    let nums: Vec<i32> = input[0]
        .as_ref()
        .split(|c: char| !c.is_digit(10) && c != '-')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    ((nums[0], nums[1]), (nums[2], nums[3]))
}

pub fn seventeen() -> Result<(), std::io::Error> {
    let file = File::open("17_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = seventeen_impl(&lines);
    println!("Day 15 part (1, 2): {:?}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_17::{parse, seventeen_impl, simulate};

    #[test]
    fn it_works() {
        assert_eq!(Some(45), simulate((6, 9), ((20, 30), (-10, -5))));
        assert_eq!(
            (45, 112),
            seventeen_impl(&parse(&vec!["target area: x=20..30, y=-10..-5"]))
        );
    }
}
