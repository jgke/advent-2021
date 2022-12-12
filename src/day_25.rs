use crate::grid::Grid;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Point = (usize, usize);

type Parsed = (HashSet<Point>, HashSet<Point>, (usize, usize));

fn plus(a: Point, b: Point, m: Point) -> Point {
    ((a.0 + b.0) % m.0, (a.1 + b.1) % m.1)
}

fn twentyfive_impl(input: &Parsed, day_2: bool) -> usize {
    if day_2 {
        panic!();
    }

    let mut right = input.0.clone();
    let mut down = input.1.clone();

    let mut count = 0;

    let mut moved = true;
    while moved {
        println!(
            "{}",
            Grid::new_with(input.2 .0, input.2 .1, |x, y| if right.contains(&(x, y)) {
                '>'
            } else if down.contains(&(x, y)) {
                'v'
            } else {
                '.'
            })
        );

        let mut new_right = HashSet::new();
        for cucumber in &right {
            let new_pos = plus(*cucumber, (1, 0), input.2);
            if right.contains(&new_pos) || down.contains(&new_pos) {
                new_right.insert(*cucumber);
            } else {
                new_right.insert(new_pos);
            }
        }
        moved = new_right != right;
        right = new_right;

        let mut new_down = HashSet::new();
        for cucumber in &down {
            let new_pos = plus(*cucumber, (0, 1), input.2);
            if right.contains(&new_pos) || down.contains(&new_pos) {
                new_down.insert(*cucumber);
            } else {
                new_down.insert(new_pos);
            }
        }

        moved |= new_down != down;
        down = new_down;

        count += 1;
    }

    count
}

fn parse<S: AsRef<str>>(input: &[S]) -> Parsed {
    let mut right = HashSet::new();
    let mut down = HashSet::new();

    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.as_ref().chars().enumerate() {
            match c {
                '.' => {}
                '>' => {
                    right.insert((x, y));
                }
                'v' => {
                    down.insert((x, y));
                }
                _ => panic!(),
            }
        }
    }

    (
        right,
        down,
        (input[0].as_ref().chars().count(), input.len()),
    )
}

pub fn twentyfive() -> Result<(), std::io::Error> {
    let file = File::open("25_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = twentyfive_impl(&lines, false);
    println!("Day 25 part 1: {}", res);
    let res_2 = twentyfive_impl(&lines, true);
    println!("Day 25 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_25::{parse, twentyfive_impl};

    #[test]
    fn it_works() {
        assert_eq!(
            58,
            twentyfive_impl(
                &parse(&vec![
                    "v...>>.vv>",
                    ".vv>>.vv..",
                    ">>.>v>...v",
                    ">>v>>.>.v.",
                    "v>v.vv.v..",
                    ">.>>..v...",
                    ".vv..>.>v.",
                    "v.v..>>v.v",
                    "....v..v.>",
                ]),
                false
            )
        );
    }
}
