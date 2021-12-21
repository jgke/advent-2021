use crate::grid::Grid;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Parsed = Grid<usize>;

fn increase(grid: &mut Grid<usize>, x: usize, y: usize) {
    grid.set(x, y, grid.get(x, y).unwrap() + 1);
}

fn flash(
    grid: &mut Grid<usize>,
    flashed: &mut Grid<bool>,
    x: usize,
    y: usize,
    flash_count: &mut usize,
) {
    if *grid.get(x, y).unwrap() <= 9 || *flashed.get(x, y).unwrap() {
        return;
    }
    flashed.set(x, y, true);
    *flash_count += 1;
    for (nx, ny) in grid.diag_nbors(x, y) {
        increase(grid, nx, ny);
        flash(grid, flashed, nx, ny, flash_count);
    }
}

fn eleven_impl(input: &Parsed, day_2: bool) -> usize {
    let mut grid = input.clone();
    if !day_2 {
        let steps = 100;
        let mut flash_count = 0;
        for _ in 0..steps {
            let mut flashed = grid.map(|_| false);

            for y in 0..grid.col_size() {
                for x in 0..grid.row_size() {
                    increase(&mut grid, x, y);
                }
            }

            for y in 0..grid.col_size() {
                for x in 0..grid.row_size() {
                    flash(&mut grid, &mut flashed, x, y, &mut flash_count);
                }
            }

            for y in 0..grid.col_size() {
                for x in 0..grid.row_size() {
                    if *grid.get(x, y).unwrap() > 9 {
                        grid.set(x, y, 0);
                    }
                }
            }
        }

        return flash_count;
    }

    let mut step = 1;
    loop {
        let mut flashed = grid.map(|_| false);
        let mut flash_count = 0;

        for y in 0..grid.col_size() {
            for x in 0..grid.row_size() {
                increase(&mut grid, x, y);
            }
        }

        for y in 0..grid.col_size() {
            for x in 0..grid.row_size() {
                flash(&mut grid, &mut flashed, x, y, &mut flash_count);
            }
        }

        if flash_count == grid.row_size() * grid.row_size() {
            return step;
        }

        for y in 0..grid.col_size() {
            for x in 0..grid.row_size() {
                if *grid.get(x, y).unwrap() > 9 {
                    grid.set(x, y, 0);
                }
            }
        }

        step += 1;
    }
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
            .collect::<Vec<Vec<_>>>(),
    )
}

pub fn eleven() -> Result<(), std::io::Error> {
    let file = File::open("11_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = eleven_impl(&lines, false);
    println!("Day 11 part 1: {}", res);
    let res_2 = eleven_impl(&lines, true);
    println!("Day 11 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_11::{eleven_impl, parse};

    #[test]
    fn it_works() {
        let lines = vec![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ];
        assert_eq!(1656, eleven_impl(&parse(&lines), false));
        assert_eq!(195, eleven_impl(&parse(&lines), true));
    }
}
