use crate::grid::Grid;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}
type Parsed = (Vec<(usize, usize)>, Vec<Fold>);

fn thirteen_impl(input: &Parsed, day2: bool) -> usize {
    let mut points: HashSet<(usize, usize)> = input.0.iter().copied().collect();
    for fold in &input.1 {
        let mut next_points = HashSet::new();

        for point in points {
            match fold {
                Fold::X(x) => {
                    if point.0 <= *x {
                        next_points.insert(point);
                    } else {
                        next_points.insert((point.0 - 2 * (point.0 - x), point.1));
                    }
                }
                Fold::Y(y) => {
                    if point.1 <= *y {
                        next_points.insert(point);
                    } else {
                        next_points.insert((point.0, point.1 - 2 * (point.1 - y)));
                    }
                }
            }
        }

        points = next_points;
        if !day2 {
            return points.len();
        }
    }
    let grid = Grid::new_with(
        40,
        6,
        |x, y| {
            if points.contains(&(x, y)) {
                '#'
            } else {
                '.'
            }
        },
    );
    println!("{}", grid);
    points.len()
}

fn parse<S: AsRef<str>>(input: &[S]) -> Parsed {
    let mut iter = input.iter();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for row in &mut iter {
        if row.as_ref() == "" {
            break;
        }
        let pair = row
            .as_ref()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>();
        left.push((pair[0], pair[1]))
    }

    for row in &mut iter {
        let fold = row
            .as_ref()
            .split("fold along ")
            .nth(1)
            .unwrap()
            .split('=')
            .collect::<Vec<_>>();
        right.push(match fold[0] {
            "x" => Fold::X(fold[1].parse().unwrap()),
            "y" => Fold::Y(fold[1].parse().unwrap()),
            _ => panic!(),
        })
    }

    (left, right)
}

pub fn thirteen() -> Result<(), std::io::Error> {
    let file = File::open("13_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = thirteen_impl(&lines, false);
    println!("Day 13 part 1: {}", res);
    let res_2 = thirteen_impl(&lines, true);
    println!("Day 13 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_13::{parse, thirteen_impl};

    #[test]
    fn it_works() {
        let lines = vec![
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5",
        ];
        assert_eq!(17, thirteen_impl(&parse(&lines), false));
    }
}
