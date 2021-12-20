use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone, Copy)]
struct Line {
    pub start: (i32, i32),
    pub end: (i32, i32),
}

impl Line {
    fn new(point: ((i32, i32), (i32, i32))) -> Line {
        if point.0 .0 < point.1 .0 {
            Line {
                start: point.0,
                end: point.1,
            }
        } else {
            Line {
                start: point.1,
                end: point.0,
            }
        }
    }

    fn part_1_line(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    fn neg_k(&self) -> bool {
        self.start.1 > self.end.1
    }

    fn points(&self) -> HashSet<(i32, i32)> {
        if self.part_1_line() {
            if self.start.0 != self.end.0 {
                (self.start.0..=self.end.0)
                    .map(|x| (x, self.start.1))
                    .collect()
            } else {
                if self.neg_k() {
                    (self.end.1..=self.start.1)
                        .map(|y| (self.start.0, y))
                        .collect()
                } else {
                    (self.start.1..=self.end.1)
                        .map(|y| (self.start.0, y))
                        .collect()
                }
            }
        } else {
            if self.neg_k() {
                (self.start.0..=self.end.0)
                    .map(|x| (x, self.start.1 - (x - self.start.0)))
                    .collect()
            } else {
                (self.start.0..=self.end.0)
                    .map(|x| (x, self.start.1 + (x - self.start.0)))
                    .collect()
            }
        }
    }
}

fn get_coll(a: Line, b: Line) -> Vec<(i32, i32)> {
    a.points().intersection(&b.points()).copied().collect()
}

fn five_impl(input: &[Line], part1: bool) -> usize {
    let mut points: HashSet<(i32, i32)> = HashSet::new();

    for a in 0..input.len() {
        for b in 0..input.len() {
            if a >= b {
                continue;
            }

            if part1 && (!input[a].part_1_line() || !input[b].part_1_line()) {
                continue;
            }

            for coll in get_coll(input[a], input[b]) {
                if coll == (1, 3) {
                    panic!()
                }
                points.insert(coll);
            }
        }
    }

    points.len()
}

fn parse<S: AsRef<str>>(input: &[S]) -> Vec<Line> {
    input
        .iter()
        .map(|s| {
            let pairs: Vec<Vec<i32>> = s
                .as_ref()
                .split(" -> ")
                .map(|s| s.split(",").map(|s| s.parse().unwrap()).collect())
                .collect();

            Line::new(((pairs[0][0], pairs[0][1]), (pairs[1][0], pairs[1][1])))
        })
        .collect()
}

pub fn five() -> Result<(), std::io::Error> {
    let file = File::open("5_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = five_impl(&lines, true);
    println!("Day 5 part 1: {}", res);
    let res_2 = five_impl(&lines, false);
    println!("Day 5 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_5::{five_impl, parse};

    #[test]
    fn it_works() {
        let lines = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];
        assert_eq!(5, five_impl(&parse(&lines), true));
        assert_eq!(12, five_impl(&parse(&lines), false));
    }
}
