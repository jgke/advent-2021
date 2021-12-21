use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const ONE_BITS: u32 = 2;
const FOUR_BITS: u32 = 4;
const SEVEN_BITS: u32 = 3;
const EIGHT_BITS: u32 = 7;

fn calc_map(numbers: &[u8]) -> HashMap<u8, u8> {
    let mut segments: HashMap<u8, HashSet<u8>> = HashMap::new();
    segments.insert(0, [0, 2, 3, 5, 6, 7, 8, 9].iter().copied().collect());
    segments.insert(1, [0, 4, 5, 6, 8, 9].iter().copied().collect());
    segments.insert(2, [0, 1, 2, 3, 4, 7, 8, 9].iter().copied().collect());
    segments.insert(3, [2, 3, 4, 5, 6, 8, 9].iter().copied().collect());
    segments.insert(4, [0, 2, 6, 8].iter().copied().collect());
    segments.insert(5, [0, 1, 3, 4, 5, 6, 7, 8, 9].iter().copied().collect());
    segments.insert(6, [0, 2, 3, 5, 6, 8, 9].iter().copied().collect());

    let mut rev_segments: HashMap<u32, HashSet<u8>> = HashMap::new();
    rev_segments.insert(ONE_BITS, [2, 5].iter().copied().collect());
    rev_segments.insert(FOUR_BITS, [1, 2, 3, 5].iter().copied().collect());
    rev_segments.insert(SEVEN_BITS, [0, 2, 5].iter().copied().collect());
    rev_segments.insert(EIGHT_BITS, [0, 1, 2, 3, 4, 5, 6].iter().copied().collect());

    let mut translated_numbers: HashMap<u8, u8> = HashMap::new();

    let mut all_segments: HashMap<u8, HashSet<u8>> = HashMap::new();
    for num in 0..=9 {
        all_segments.insert(num, HashSet::new());
        for k in 0..=6 {
            if segments[&k].contains(&num) {
                all_segments.get_mut(&num).unwrap().insert(k);
            }
        }
    }

    for number in numbers {
        let bitcount = number.count_ones();
        if let Some(_bits) = rev_segments.get(&bitcount) {
            match bitcount {
                ONE_BITS => translated_numbers.insert(1, *number),
                FOUR_BITS => translated_numbers.insert(4, *number),
                SEVEN_BITS => translated_numbers.insert(7, *number),
                EIGHT_BITS => translated_numbers.insert(8, *number),
                _ => unreachable!(),
            };
        }
    }

    for number in numbers {
        let bitcount = number.count_ones();
        let num = if bitcount == 6 {
            if translated_numbers[&4] ^ (translated_numbers[&4] & number) == 0 {
                9
            } else if translated_numbers[&1] ^ (translated_numbers[&1] & number) == 0 {
                0
            } else {
                6
            }
        } else if bitcount == 5 {
            if translated_numbers[&1] ^ (translated_numbers[&1] & number) == 0 {
                3
            } else if (*number | translated_numbers.get(&1).unwrap_or(&0))
                == *translated_numbers.get(&9).unwrap_or(&0)
            {
                5
            } else if translated_numbers.contains_key(&1) && translated_numbers.contains_key(&9) {
                2
            } else {
                continue;
            }
        } else {
            continue;
        };
        translated_numbers.insert(num, *number);
    }

    let reversed_translate_map: HashMap<u8, u8> = translated_numbers
        .into_iter()
        .map(|(k, v)| (v, k))
        .collect();
    reversed_translate_map
}

fn eight_impl(input: &[(Vec<u8>, Vec<u8>)], day_2: bool) -> usize {
    if !day_2 {
        return input
            .iter()
            .flat_map(|(_, b)| b.iter())
            .filter(|i| {
                let one_bits = i.count_ones();
                one_bits == ONE_BITS
                    || one_bits == FOUR_BITS
                    || one_bits == SEVEN_BITS
                    || one_bits == EIGHT_BITS
            })
            .count();
    }

    let mut res: usize = 0;

    for (a, b) in input {
        let numbers = a.iter().chain(b.iter()).copied().collect::<Vec<u8>>();
        let map = calc_map(&numbers);
        let mut sub_res: usize = 0;
        for n in b {
            sub_res = sub_res * 10 + (map[n] as usize);
        }
        res += sub_res;
    }

    res
}

fn parse_num<S: AsRef<str>>(s: S) -> u8 {
    s.as_ref().chars().map(|c| 1 << ((c as u8) - b'a')).sum()
}

fn parse<S: AsRef<str>>(input: &[S]) -> Vec<(Vec<u8>, Vec<u8>)> {
    let i = input
        .iter()
        .map(|s| s.as_ref().split(" | ").collect())
        .collect::<Vec<Vec<_>>>();
    i.iter()
        .map(|row| {
            (
                row[0].split(' ').map(parse_num).collect(),
                row[1].split(' ').map(parse_num).collect(),
            )
        })
        .collect()
}

pub fn eight() -> Result<(), std::io::Error> {
    let file = File::open("8_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = eight_impl(&lines, false);
    println!("Day 8 part 1: {}", res);
    let res_2 = eight_impl(&lines, true);
    println!("Day 8 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_8::{eight_impl, parse};

    #[test]
    fn it_works() {
        let lines = vec![
"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
"edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
"fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
"fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
"aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
"fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
"dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
"bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
"egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
"gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ];
        assert_eq!(26, eight_impl(&parse(&lines), false));
        assert_eq!(61229, eight_impl(&parse(&lines), true));
    }
}
