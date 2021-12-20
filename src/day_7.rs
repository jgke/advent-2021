
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn seven_cost(input: &Vec<i32>, pos: i32) -> usize {
    input.iter().map(|x| (*x - pos).abs() as usize).sum()
}

fn seven_exp_cost(input: &Vec<i32>, pos: i32) -> usize {
    input
        .iter()
        .map(|x| {
            let n = (*x - pos).abs() as usize;
            (n * (n + 1)) / 2
        })
        .sum()
}

fn seven_impl(input: &Vec<i32>, exp: bool) -> usize {
    let min_pos = *input.iter().min().unwrap();
    let max_pos = *input.iter().max().unwrap();

    if exp {
        (min_pos..=max_pos)
            .map(|pos| seven_exp_cost(input, pos))
            .min()
            .unwrap()
    } else {
        (min_pos..=max_pos)
            .map(|pos| seven_cost(input, pos))
            .min()
            .unwrap()
    }
}

fn parse<S: AsRef<str>>(input: &[S]) -> Vec<i32> {
    input
        .iter()
        .flat_map(|s| s.as_ref().split(","))
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn seven() -> Result<(), std::io::Error> {
    let file = File::open("7_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = seven_impl(&lines, false);
    println!("Day 7 part 1: {}", res);
    let res_2 = seven_impl(&lines, true);
    println!("Day 7 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_7::{parse, seven_cost, seven_impl};

    #[test]
    fn it_works() {
        let lines = vec!["16,1,2,0,4,2,7,1,2,14"];
        assert_eq!(37, seven_cost(&parse(&lines), 2));
        assert_eq!(41, seven_cost(&parse(&lines), 1));
        assert_eq!(39, seven_cost(&parse(&lines), 3));
        assert_eq!(71, seven_cost(&parse(&lines), 10));
        assert_eq!(37, seven_impl(&parse(&lines), false));
        assert_eq!(168, seven_impl(&parse(&lines), true));
    }
}
