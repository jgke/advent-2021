use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn three_1_impl<T: AsRef<str>>(report: &[T]) -> i32 {
    let num: Vec<Vec<bool>> = report
        .iter()
        .map(|t| {
            t.as_ref()
                .chars()
                .map(|t| t.to_digit(10).unwrap() != 0)
                .collect()
        })
        .collect();

    let rowlen = report[0].as_ref().chars().count();

    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;

    for i in 0..rowlen {
        let mut count: i32 = 0;
        for nums in &num {
            count += nums[i] as i32;
        }
        let c = count > (num.len() / 2) as i32;
        gamma = (gamma << 1) | ((!c) as i32);
        epsilon = (epsilon << 1) | ((c) as i32);
    }

    gamma * epsilon
}

fn vec_to_num(num: &[bool]) -> i32 {
    let mut res = 0;
    for i in num {
        res = (res << 1) | (*i as i32);
    }
    res
}

fn num_counts_in_index(report: &[Vec<bool>], i: usize) -> (usize, usize) {
    (
        report.iter().map(|v| v[i]).filter(|b| *b).count(),
        report.iter().map(|v| v[i]).filter(|b| !*b).count(),
    )
}

fn is_true_most_common_in_index(report: &[Vec<bool>], i: usize) -> bool {
    let counts = num_counts_in_index(report, i);
    counts.0 >= counts.1
}

fn is_false_most_common_in_index(report: &[Vec<bool>], i: usize) -> bool {
    let counts = num_counts_in_index(report, i);
    counts.0 < counts.1
}

fn gamma(report: &Vec<Vec<bool>>) -> i32 {
    let rowlen = report[0].len();
    let mut current_sets = report.clone();

    for i in 0..rowlen {
        if current_sets.len() == 1 {
            return vec_to_num(&current_sets[0]);
        }
        if is_true_most_common_in_index(&current_sets, i) {
            current_sets = current_sets.into_iter().filter(|v| v[i]).collect();
        } else {
            current_sets = current_sets.into_iter().filter(|v| !v[i]).collect();
        }
    }

    assert_eq!(1, current_sets.len());
    vec_to_num(&current_sets[0])
}

fn epsilon(report: &Vec<Vec<bool>>) -> i32 {
    let rowlen = report[0].len();
    let mut current_sets = report.clone();

    for i in 0..rowlen {
        if current_sets.len() == 1 {
            return vec_to_num(&current_sets[0]);
        }
        if is_false_most_common_in_index(&current_sets, i) {
            current_sets = current_sets.into_iter().filter(|v| v[i]).collect();
        } else {
            current_sets = current_sets.into_iter().filter(|v| !v[i]).collect();
        }
    }

    assert_eq!(1, current_sets.len());
    vec_to_num(&current_sets[0])
}

fn three_2_impl<T: AsRef<str>>(report: &[T]) -> (i32, i32, i32) {
    let num: Vec<Vec<bool>> = report
        .iter()
        .map(|t| {
            t.as_ref()
                .chars()
                .map(|t| t.to_digit(10).unwrap() != 0)
                .collect()
        })
        .collect();

    let _rowlen = report[0].as_ref().chars().count();

    let g: i32 = gamma(&num);
    let e: i32 = epsilon(&num);

    (g, e, g * e)
}

pub fn three() -> Result<(), std::io::Error> {
    let file = File::open("3_input")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();

    let res = three_1_impl(&lines);
    println!("Day 3 part 1: {}", res);
    let res2 = three_2_impl(&lines);
    println!("Day 3 part 2: {:?}", res2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_3::{three_1_impl, three_2_impl};

    #[test]
    fn it_works() {
        let lines = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        assert_eq!(198, three_1_impl(&lines));
        assert_eq!((23, 10, 230), three_2_impl(&lines));
    }
}
