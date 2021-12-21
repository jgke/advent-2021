use crate::grid::Grid;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Parsed = Grid<usize>;

fn fifteen_impl(input: &Parsed, day_2: bool) -> usize {
    let map = if !day_2 {
        input.clone()
    } else {
        let w = input.row_size();
        let h = input.col_size();
        Grid::new_with(w * 5, h * 5, |x, y| {
            ((input.get(x % w, y % h).unwrap() + x / w + y / h) - 1) % 9 + 1
        })
    };

    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();

    queue.push((usize::MAX, 0, 0));
    while let Some((c, x, y)) = queue.pop() {
        if x == map.row_size() - 1 && y == map.col_size() - 1 {
            return usize::MAX - c;
        }
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        for (nx, ny) in map.nbors(x, y) {
            queue.push((c - map.get(nx, ny).unwrap(), nx, ny));
        }
    }

    unreachable!()
}

fn parse<S: AsRef<str>>(input: &[S]) -> Parsed {
    Grid::new(
        input
            .iter()
            .map(|s| {
                s.as_ref()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect(),
    )
}

pub fn fifteen() -> Result<(), std::io::Error> {
    let file = File::open("15_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = fifteen_impl(&lines, false);
    println!("Day 15 part 1: {}", res);
    let res_2 = fifteen_impl(&lines, true);
    println!("Day 15 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_15::{fifteen_impl, parse};

    #[test]
    fn it_works() {
        let lines = vec![
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581",
        ];
        assert_eq!(40, fifteen_impl(&parse(&lines), false));
        assert_eq!(315, fifteen_impl(&parse(&lines), true));
    }
}
