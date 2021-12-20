use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn one_impl(input: &[i32]) -> usize {
    let mut prev = None;
    let mut res = 0;
    for item in input {
        if let Some(p) = prev {
            if p < item {
                res += 1;
            }
        }
        prev = Some(item);
    }
    res
}

fn two_window(input: &[i32]) -> Vec<i32> {
    let mut res = Vec::new();
    for i in 2..input.len() {
        res.push(input[(i - 2)..=i].iter().sum())
    }
    res
}

pub fn one() -> Result<(), std::io::Error> {
    let file = File::open("1_input")?;
    let reader = BufReader::new(file);
    let lines: Vec<i32> = reader
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();
    let res = one_impl(&lines);
    println!("Day 1 part 1: {}", res);
    let windowed = two_window(&lines);
    println!("Day 1 part 1: {}", one_impl(&windowed));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_1::{one_impl, two_window};

    #[test]
    fn it_works() {
        let lines = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, one_impl(&lines));

        let windowed_lines = two_window(&lines);
        assert_eq!(5, one_impl(&windowed_lines));
    }
}
