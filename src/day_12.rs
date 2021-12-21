use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Parsed = HashMap<String, HashSet<String>>;

fn recur(map: &Parsed, visited: &mut HashSet<String>, cur: &str) -> usize {
    if cur == "start" {
        return 1;
    }
    if visited.contains(cur) {
        return 0;
    }
    if cur.chars().next().unwrap().is_ascii_lowercase() {
        visited.insert(cur.to_string());
    }
    let mut routes = 0;
    for edge in &map[cur] {
        routes += recur(map, visited, edge);
    }
    visited.remove(cur);
    routes
}

fn push(visited: &mut HashMap<String, bool>, cur: &str) {
    if visited.contains_key(cur) {
        visited.insert(cur.to_string(), true);
    } else if cur.chars().next().unwrap().is_ascii_lowercase() {
        visited.insert(cur.to_string(), false);
    }
}

fn pop(visited: &mut HashMap<String, bool>, cur: &str) {
    if visited.get(cur) == Some(&true) {
        visited.insert(cur.to_string(), false);
    } else {
        visited.remove(cur);
    }
}

fn recur2(map: &Parsed, visited: &mut HashMap<String, bool>, cur: &str) -> usize {
    if cur == "start" {
        return 1;
    }
    if visited.contains_key(cur)
        && (cur == "start" || cur == "end" || visited.values().any(|v| v == &true))
    {
        return 0;
    }

    push(visited, cur);
    let mut routes = 0;
    for edge in &map[cur] {
        routes += recur2(map, visited, edge);
    }
    pop(visited, cur);

    routes
}

fn twelve_impl(input: &Parsed, day_2: bool) -> usize {
    if !day_2 {
        return recur(input, &mut HashSet::new(), "end");
    }

    recur2(input, &mut HashMap::new(), "end")
}

fn parse<S: AsRef<str>>(input: &[S]) -> Parsed {
    let mut map: Parsed = HashMap::new();
    input.iter().for_each(|s| {
        let pair = s
            .as_ref()
            .split('-')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let start = pair[0].clone();
        let end = pair[1].clone();
        map.entry(start.clone()).or_default().insert(end.clone());
        map.entry(end).or_default().insert(start);
    });
    map
}

pub fn twelve() -> Result<(), std::io::Error> {
    let file = File::open("12_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = twelve_impl(&lines, false);
    println!("Day 12 part 1: {}", res);
    let res_2 = twelve_impl(&lines, true);
    println!("Day 12 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_12::{parse, twelve_impl};

    #[test]
    fn it_works_ex1() {
        let lines = vec!["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
        assert_eq!(10, twelve_impl(&parse(&lines), false));
        assert_eq!(36, twelve_impl(&parse(&lines), true));
    }

    #[test]
    fn it_works_ex2() {
        let lines = vec![
            "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa",
            "kj-HN", "kj-dc",
        ];
        assert_eq!(19, twelve_impl(&parse(&lines), false));
        assert_eq!(103, twelve_impl(&parse(&lines), true));
    }

    #[test]
    fn it_works_ex3() {
        let lines = vec![
            "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
            "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
        ];
        assert_eq!(226, twelve_impl(&parse(&lines), false));
        assert_eq!(3509, twelve_impl(&parse(&lines), true));
    }
}
