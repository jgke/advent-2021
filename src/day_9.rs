use crate::grid::Grid;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Parsed = Grid<i32>;

fn get_lowest_nbor(
    visited: &mut Grid<(bool, Option<i32>)>,
    grid: &Parsed,
    x: usize,
    y: usize,
) -> i32 {
    let (cell_visited, lowest_nbor) = visited.get(x, y).unwrap();
    let this = *grid.get(x, y).unwrap();
    if *cell_visited {
        lowest_nbor.unwrap_or(*grid.get(x, y).unwrap())
    } else {
        visited.set(x, y, (true, None));
        let mut nbors = Vec::new();
        for (nx, ny) in grid.nbors(x, y) {
            if *grid.get(nx, ny).unwrap() <= this {
                nbors.push(get_lowest_nbor(visited, grid, nx, ny));
            }
        }
        let lowest = nbors
            .into_iter()
            .min()
            .unwrap_or(this)
            .min(*grid.get(x, y).unwrap());
        visited.set(x, y, (true, Some(lowest)));
        lowest
    }
}

fn basin_size(visited: &mut Grid<bool>, grid: &Parsed, x: usize, y: usize) -> Option<usize> {
    if *visited.get(x, y).unwrap() {
        return None;
    }
    visited.set(x, y, true);
    if grid.get(x, y) == Some(&9) {
        return None;
    }
    let mut size = 1;
    for (nx, ny) in grid.nbors(x, y) {
        size += basin_size(visited, grid, nx, ny).unwrap_or(0);
    }
    Some(size)
}

fn nine_impl(input: &Parsed, day_2: bool) -> i32 {
    if !day_2 {
        let mut visited = Grid::new_with(input.row_size(), input.col_size(), |_, _| (false, None));
        for y in 0..input.col_size() {
            for x in 0..input.row_size() {
                get_lowest_nbor(&mut visited, input, x, y);
            }
        }

        let mut risks = 0;
        for y in 0..input.col_size() {
            for x in 0..input.row_size() {
                if visited.get(x, y).unwrap().1.unwrap() == *input.get(x, y).unwrap() {
                    risks += 1 + input.get(x, y).unwrap();
                }
            }
        }
        return risks;
    }

    let mut visited = Grid::new_with(input.row_size(), input.col_size(), |_, _| false);
    let mut basins = Vec::new();
    for y in 0..input.col_size() {
        for x in 0..input.row_size() {
            if let Some(b) = basin_size(&mut visited, input, x, y) {
                basins.push(b as i32);
            }
        }
    }
    basins.sort_unstable();
    basins.reverse();
    basins.into_iter().take(3).product()
}

fn parse<S: AsRef<str>>(input: &[S]) -> Parsed {
    Grid::new(
        input
            .iter()
            .map(|s| {
                s.as_ref()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect()
            })
            .collect::<Vec<Vec<_>>>(),
    )
}

pub fn nine() -> Result<(), std::io::Error> {
    let file = File::open("9_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = nine_impl(&lines, false);
    println!("Day 9 part 1: {}", res);
    let res_2 = nine_impl(&lines, true);
    println!("Day 9 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_9::{nine_impl, parse};

    #[test]
    fn it_works() {
        let lines = vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ];
        assert_eq!(15, nine_impl(&parse(&lines), false));
        assert_eq!(1134, nine_impl(&parse(&lines), true));
    }
}
