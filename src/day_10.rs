use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::iter::Peekable;

type Parsed = Vec<Vec<char>>;

enum Chunk {
    Paren(Vec<Chunk>),
    Square(Vec<Chunk>),
    Curly(Vec<Chunk>),
    Html(Vec<Chunk>),
}

enum ParseError {
    Incomplete(Vec<char>),
    Corrupted(char),
}

fn parse_tree<I: Iterator<Item = char>>(
    res: &mut Vec<Chunk>,
    input: &mut Peekable<I>,
    expected: char,
) -> Result<(), ParseError> {
    let (_start, end, ty) = match input.peek() {
        Some('(') => ('(', ')', Chunk::Paren as fn(Vec<Chunk>) -> Chunk),
        Some('[') => ('[', ']', Chunk::Square as fn(Vec<Chunk>) -> Chunk),
        Some('{') => ('{', '}', Chunk::Curly as fn(Vec<Chunk>) -> Chunk),
        Some('<') => ('<', '>', Chunk::Html as fn(Vec<Chunk>) -> Chunk),
        Some(c) if *c == expected => {
            input.next();
            return Ok(());
        }
        Some(c) => return Err(ParseError::Corrupted(*c)),
        None => {
            if expected != '#' {
                return Err(ParseError::Incomplete(vec![expected]));
            } else {
                return Ok(());
            }
        }
    };
    input.next();

    let mut inner_res = Vec::new();
    match parse_tree(&mut inner_res, input, end) {
        Ok(_) => {}
        Err(ParseError::Incomplete(mut v)) => {
            if expected != '#' {
                v.push(expected);
            }
            return Err(ParseError::Incomplete(v));
        }
        Err(e) => return Err(e),
    };
    res.push(ty(inner_res));
    parse_tree(res, input, expected)
}

fn ten_impl(input: &Parsed, day_2: bool) -> usize {
    if !day_2 {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for row in input {
            let mut res = Vec::new();
            let parsed = parse_tree(&mut res, &mut row.iter().copied().peekable(), '#');
            match parsed {
                Err(ParseError::Corrupted(c)) => {
                    *counts.entry(c).or_default() += 1;
                }
                _ => {}
            }
        }

        return *counts.get(&')').unwrap_or(&0) * 3
            + *counts.get(&']').unwrap_or(&0) * 57
            + *counts.get(&'}').unwrap_or(&0) * 1197
            + *counts.get(&'>').unwrap_or(&0) * 25137;
    }

    let mut counts: Vec<usize> = Vec::new();
    for row in input {
        let mut res = Vec::new();
        let parsed = parse_tree(&mut res, &mut row.iter().copied().peekable(), '#');
        match parsed {
            Err(ParseError::Incomplete(v)) => {
                let mut res = 0;
                for c in v {
                    res *= 5;
                    res += match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    };
                }
                counts.push(res);
            }
            _ => {}
        }
    }

    counts.sort();

    return counts[counts.len() / 2];
}

fn parse<S: AsRef<str>>(input: &[S]) -> Parsed {
    input
        .iter()
        .map(|s| s.as_ref().chars().collect())
        .collect::<Vec<Vec<_>>>()
}

pub fn ten() -> Result<(), std::io::Error> {
    let file = File::open("10_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = ten_impl(&lines, false);
    println!("Day 10 part 1: {}", res);
    let res_2 = ten_impl(&lines, true);
    println!("Day 10 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_10::{parse, ten_impl};

    #[test]
    fn it_works() {
        let lines = vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];
        assert_eq!(26397, ten_impl(&parse(&lines), false));
        assert_eq!(288957, ten_impl(&parse(&lines), true));
    }
}
