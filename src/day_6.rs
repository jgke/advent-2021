use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn six_impl(input: &HashMap<i32, usize>, days: i32) -> usize {
    let mut fish = input.clone();

    for _ in 0..days {
        let mut new_input = HashMap::new();

        for (k, v) in fish {
            if k == 0 {
                new_input.entry(8).and_modify(|e| *e += v).or_insert(v);
                new_input.entry(6).and_modify(|e| *e += v).or_insert(v);
            } else {
                new_input.entry(k - 1).and_modify(|e| *e += v).or_insert(v);
            }
        }

        fish = new_input;
    }

    fish.values().copied().sum()
}

fn parse<S: AsRef<str>>(input: &[S]) -> HashMap<i32, usize> {
    let nums: Vec<i32> = input
        .iter()
        .flat_map(|s| s.as_ref().split(','))
        .map(|s| s.parse().unwrap())
        .collect();
    let mut map = HashMap::new();
    for num in nums {
        map.entry(num).and_modify(|e| *e += 1).or_insert(1);
    }
    map
}

pub fn six() -> Result<(), std::io::Error> {
    let file = File::open("6_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = six_impl(&lines, 80);
    println!("Day 6 part 1: {}", res);
    let res_2 = six_impl(&lines, 256);
    println!("Day 6 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_6::{parse, six_impl};

    #[test]
    fn it_works() {
        let lines = vec!["3,4,3,1,2"];
        assert_eq!(5934, six_impl(&parse(&lines), 80));
        assert_eq!(26984457539, six_impl(&parse(&lines), 256));
    }
}
