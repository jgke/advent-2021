use crate::grid::Grid;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Parsed = (Vec<bool>, Grid<bool>);

fn to_num<I: IntoIterator<Item = bool>>(bits: I) -> usize {
    let mut res = 0;
    for bit in bits {
        res = (res << 1) | (bit as usize);
    }
    res
}

fn run_round(algo: &[bool], img: &Grid<bool>, def: bool) -> Grid<bool> {
    Grid::new_with(img.row_size() + 2, img.col_size() + 2, |x, y| {
        algo[to_num(
            (-1..=1)
                .flat_map(|y| (-1..=1).map(move |x| (x, y)))
                .map(|(xx, yy)| {
                    let x = x as i32;
                    let y = y as i32;
                    if img.legal(x + xx - 1, y + yy - 1) {
                        *img.get((x + xx - 1) as usize, (y + yy - 1) as usize)
                            .unwrap_or(&def)
                    } else {
                        def
                    }
                }),
        )]
    })
}

fn twenty_impl(input: &Parsed, day_2: bool) -> u32 {
    let rounds = if !day_2 { 2 } else { 50 };
    let mut grid = run_round(&input.0, &input.1, false);
    for i in 1..rounds {
        grid = run_round(&input.0, &grid, input.0[0] && (i % 2) != 0);
    }
    grid.iter().map(|c| *c as u32).sum()
}

fn parse<S: AsRef<str>>(input: &[S]) -> Parsed {
    let algo = input[0].as_ref().chars().map(|c| c == '#').collect();

    let grid_rows = input[2..]
        .iter()
        .map(|s| s.as_ref().chars().map(|c| c == '#').collect())
        .collect();

    (algo, Grid::new(grid_rows))
}

pub fn twenty() -> Result<(), std::io::Error> {
    let file = File::open("20_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = twenty_impl(&lines, false);
    println!("Day 15 part 1: {}", res);
    let res_2 = twenty_impl(&lines, true);
    println!("Day 15 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_20::{parse, twenty_impl};

    #[test]
    fn it_works() {
        assert_eq!(35, twenty_impl(&parse(&vec![
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
            "",
            "#..#.",
            "#....",
            "##..#",
            "..#..",
            "..###",
        ]), false));
        assert_eq!(3351, twenty_impl(&parse(&vec![
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
            "",
            "#..#.",
            "#....",
            "##..#",
            "..#..",
            "..###",
        ]), true));
    }
}
