use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Parsed = (usize, usize);

type GameState = (usize, usize, usize, usize, bool, u8);

fn step(pos: usize, n: usize) -> usize {
    (((pos - 1) + n) % 10) + 1
}

fn roll(state: GameState, roll: usize) -> GameState {
    let (p1, p1_score, p2, p2_score, p1_turn, throws) = state;
    if p1_turn {
        (
            step(p1, roll),
            if throws == 0 {
                p1_score + step(p1, roll)
            } else {
                p1_score
            },
            p2,
            p2_score,
            throws > 0,
            if throws == 0 { 2 } else { throws - 1 },
        )
    } else {
        (
            p1,
            p1_score,
            step(p2, roll),
            if throws == 0 {
                p2_score + step(p2, roll)
            } else {
                p2_score
            },
            throws == 0,
            if throws == 0 { 2 } else { throws - 1 },
        )
    }
}

fn plus(a: (usize, usize), b: (usize, usize), c: (usize, usize)) -> (usize, usize) {
    (a.0 + b.0 + c.0, a.1 + b.1 + c.1)
}

fn quantum_dirac(
    history: &mut HashMap<GameState, (usize, usize)>,
    state: GameState,
) -> (usize, usize) {
    if history.contains_key(&state) {
        return history[&state];
    }
    let wins = if state.1 >= 21 {
        (1, 0)
    } else if state.3 >= 21 {
        (0, 1)
    } else {
        plus(
            quantum_dirac(history, roll(state, 1)),
            quantum_dirac(history, roll(state, 2)),
            quantum_dirac(history, roll(state, 3)),
        )
    };
    history.insert(state, wins);
    wins
}

fn twentyone_impl(input: &Parsed, day_2: bool) -> usize {
    if !day_2 {
        let mut state = (input.0, 0, input.1, 0, true, 2);
        let mut n = 0;

        while state.1 < 1000 && state.3 < 1000 {
            state = roll(state, n % 100 + 1);
            n += 1;
        }
        let loser_score = state.1.min(state.3);
        return loser_score * n;
    }

    let (p1_wins, p2_wins) = quantum_dirac(&mut HashMap::new(), (input.0, 0, input.1, 0, true, 2));
    p1_wins.max(p2_wins)
}

fn parse<S: AsRef<str>>(input: &[S]) -> Parsed {
    let plrs: Vec<usize> = input
        .iter()
        .map(|t| t.as_ref().split(' ').last().unwrap().parse().unwrap())
        .collect();
    (plrs[0], plrs[1])
}

pub fn twentyone() -> Result<(), std::io::Error> {
    let file = File::open("21_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = twentyone_impl(&lines, false);
    println!("Day 21 part 1: {}", res);
    let res_2 = twentyone_impl(&lines, true);
    println!("Day 21 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_21::{parse, twentyone_impl};

    #[test]
    fn it_works() {
        assert_eq!(
            739785,
            twentyone_impl(
                &parse(&vec![
                    "Player 1 starting position: 4",
                    "Player 2 starting position: 8",
                ]),
                false
            )
        );
        assert_eq!(
            444356092776315,
            twentyone_impl(
                &parse(&vec![
                    "Player 1 starting position: 4",
                    "Player 2 starting position: 8",
                ]),
                true
            )
        );
    }
}
