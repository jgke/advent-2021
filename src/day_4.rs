use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn get_victory_row(board: &[Vec<usize>], draws: &HashSet<usize>) -> Option<usize> {
    for row in board {
        if row.iter().all(|i| draws.contains(i)) {
            return Some(
                board[0..5]
                    .iter()
                    .flat_map(|row| row.iter().filter(|i| !draws.contains(i)))
                    .sum(),
            );
        }
    }
    None
}

fn parse_board<S: AsRef<str>, I: Iterator<Item = S>>(i: I) -> Vec<Vec<usize>> {
    let rows: Vec<Vec<usize>> = i
        .map(|s| {
            s.as_ref()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let mut board: Vec<Vec<usize>> = rows.clone();
    for i in 0..rows[0].len() {
        board.push(rows.iter().map(|row| row[i]).collect());
    }

    board
}

fn four_impl<S: AsRef<str> + Sized>(input: &[S], wait_last: bool) -> usize {
    let draws: Vec<usize> = input[0]
        .as_ref()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards: Vec<Vec<Vec<usize>>> = Vec::new();
    let mut i = 2;

    while i < input.len() {
        boards.push(parse_board((&input[i..(i + 5)]).iter()));
        i += 6;
    }

    let mut current_draws = HashSet::new();
    let mut winning_boards = HashSet::new();

    for draw in draws {
        current_draws.insert(draw);

        let winner: Vec<(usize, usize)> = boards
            .iter()
            .enumerate()
            .filter(|(i, _b)| !wait_last || !winning_boards.contains(i))
            .filter_map(|(i, b)| get_victory_row(b, &current_draws).map(|w| (i, w)))
            .collect();

        for w in winner {
            if !wait_last || winning_boards.len() == boards.len() - 1 {
                return w.1 * draw;
            }
            winning_boards.insert(w.0);
        }
    }

    unimplemented!()
}

pub fn four() -> Result<(), std::io::Error> {
    let file = File::open("4_input")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();
    let res = four_impl(&lines, false);
    println!("Day 4 part 1: {}", res);
    let res_2 = four_impl(&lines, true);
    println!("Day 4 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_4::four_impl;

    #[test]
    fn it_works() {
        let lines = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ];
        assert_eq!(4512, four_impl(&lines, false));
        assert_eq!(1924, four_impl(&lines, true));
    }
}
