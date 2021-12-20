use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl<T: AsRef<str>> From<T> for Command {
    fn from(f: T) -> Command {
        let s = f.as_ref();
        let count: i32 = s.split(" ").nth(1).unwrap().parse().unwrap();
        match s.split(" ").next().unwrap() {
            "forward" => Command::Forward(count),
            "up" => Command::Up(count),
            "down" => Command::Down(count),
            _ => panic!(),
        }
    }
}

fn two_1_impl(commands: &[Command]) -> i32 {
    let mut depth = 0;
    let mut position = 0;

    for command in commands {
        match command {
            Command::Up(d) => depth -= d,
            Command::Down(d) => depth += d,
            Command::Forward(d) => position += d,
        }
    }

    depth * position
}

fn two_2_impl(commands: &[Command]) -> i32 {
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;

    for command in commands {
        match command {
            Command::Up(d) => aim -= d,
            Command::Down(d) => aim += d,
            Command::Forward(d) => {
                position += d;
                depth += d * aim;
            }
        }
    }

    depth * position
}

pub fn two() -> Result<(), std::io::Error> {
    let file = File::open("2_input")?;
    let reader = BufReader::new(file);
    let lines: Vec<Command> = reader.lines().map(|s| From::from(s.unwrap())).collect();

    let res = two_1_impl(&lines);
    println!("Day 2 part 1: {}", res);
    let res2 = two_2_impl(&lines);
    println!("Day 2 part 2: {}", res2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_2::{two_1_impl, two_2_impl, Command};

    #[test]
    fn it_works() {
        let lines = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let commands: Vec<Command> = lines.iter().map(From::from).collect();

        assert_eq!(150, two_1_impl(&commands));
        assert_eq!(900, two_2_impl(&commands));
    }
}
