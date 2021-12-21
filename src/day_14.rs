use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Parsed = (Vec<char>, Vec<Rule>);

struct Rule {
    from: (char, char),
    to: char,
}

fn fourteen_impl(input: &Parsed, day_2: bool) -> usize {
    let mut state: HashMap<(char, char), usize> = HashMap::new();

    let mut prev = input.0[0];
    for c in &input.0[1..] {
        *state.entry((prev, *c)).or_default() += 1;
        prev = *c;
    }

    let mut rules = HashMap::new();
    for rule in &input.1 {
        rules.insert(rule.from, rule.to);
    }

    let steps = if !day_2 { 10 } else { 40 };
    for _ in 0..steps {
        let mut next_state: HashMap<(char, char), usize> = HashMap::new();
        for (pair, count) in state {
            if let Some(to) = rules.get(&pair) {
                *next_state.entry((pair.0, *to)).or_default() += count;
                *next_state.entry((*to, pair.1)).or_default() += count;
            } else {
                *next_state.entry(pair).or_default() += count;
            }
        }
        state = next_state;
    }
    let mut counts: HashMap<char, usize> = HashMap::new();
    for ((a, b), count) in state {
        *counts.entry(a).or_default() += count;
        *counts.entry(b).or_default() += count;
    }
    let mut real_counts = counts
        .iter()
        .map(|(c, count)| (count / 2, c))
        .collect::<Vec<_>>();
    real_counts.sort();
    return real_counts.last().unwrap().0 - real_counts[1].0;
}

fn parse<S: AsRef<str>>(input: &[S]) -> Parsed {
    let start_state = std::iter::once('#')
        .chain(input[0].as_ref().chars())
        .chain(std::iter::once('#'))
        .collect();
    let rules = input[2..]
        .iter()
        .map(|s| {
            let parts = s.as_ref().split(" -> ").collect::<Vec<_>>();
            let from = parts[0].chars().collect::<Vec<_>>();
            Rule {
                from: (from[0], from[1]),
                to: parts[1].chars().next().unwrap(),
            }
        })
        .collect();

    (start_state, rules)
}

pub fn fourteen() -> Result<(), std::io::Error> {
    let file = File::open("14_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = fourteen_impl(&lines, false);
    println!("Day 14 part 1: {}", res);
    let res_2 = fourteen_impl(&lines, true);
    println!("Day 14 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_14::{fourteen_impl, parse};

    #[test]
    fn it_works() {
        let lines = vec![
            "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B",
            "HN -> C", "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B",
            "CC -> N", "CN -> C",
        ];
        assert_eq!(1588, fourteen_impl(&parse(&lines), false));
        assert_eq!(2188189693529, fourteen_impl(&parse(&lines), true));
    }
}
