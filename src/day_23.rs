use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AmphipodState {
    Start,
    Middle,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Amphipod {
    pos: Point,
    state: AmphipodState,
}

impl Amphipod {
    fn new(pos: Point) -> Amphipod {
        Amphipod {
            pos,
            state: AmphipodState::Start,
        }
    }

    fn move_to(&mut self, to: Point) {
        self.pos = to;

        self.state = match self.state {
            AmphipodState::Start if to.1 != 1 => AmphipodState::End,
            AmphipodState::Start => AmphipodState::Middle,
            AmphipodState::Middle => AmphipodState::End,
            AmphipodState::End => panic!(),
        };
    }
}

impl std::fmt::Display for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}) ({})",
            self.pos.0,
            self.pos.1,
            match self.state {
                AmphipodState::Start => "S",
                AmphipodState::Middle => "M",
                AmphipodState::End => "E",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AmphipodPair {
    Part1([Amphipod; 2]),
    Part2([Amphipod; 4]),
}

impl AmphipodPair {
    fn is_at(&self, p: Point) -> bool {
        match self {
            AmphipodPair::Part1([n1, n2]) => n1.pos == p || n2.pos == p,
            AmphipodPair::Part2([n1, n2, n3, n4]) => {
                n1.pos == p || n2.pos == p || n3.pos == p || n4.pos == p
            }
        }
    }

    fn move_to(&mut self, from: Point, to: Point) {
        assert!(self.is_at(from));
        match self {
            AmphipodPair::Part1([n1, n2]) => {
                if n1.pos == from {
                    n1.move_to(to);
                } else {
                    n2.move_to(to);
                }
            }
            AmphipodPair::Part2([n1, n2, n3, n4]) => {
                if n1.pos == from {
                    n1.move_to(to);
                } else if n2.pos == from {
                    n2.move_to(to);
                } else if n3.pos == from {
                    n3.move_to(to);
                } else {
                    n4.move_to(to);
                }
            }
        }
    }

    fn get_pods(&self) -> &[Amphipod] {
        match self {
            AmphipodPair::Part1(pods) => pods,
            AmphipodPair::Part2(pods) => pods,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    pods: [AmphipodPair; 4],
    part2: bool,
}

impl State {
    fn _is_accessible(&self, from: Point, to: Point, len: usize) -> Option<usize> {
        if len > 0 {
            for pod in &self.pods {
                if pod.is_at(from) {
                    return None;
                }
            }
        }

        if from == to {
            return Some(len);
        }

        if from.0 != to.0 {
            if from.1 != 1 {
                self._is_accessible((from.0, from.1 - 1), to, len + 1)
            } else if from.0 < to.0 {
                self._is_accessible((from.0 + 1, from.1), to, len + 1)
            } else {
                self._is_accessible((from.0 - 1, from.1), to, len + 1)
            }
        } else if from.1 < to.1 {
            self._is_accessible((from.0, from.1 + 1), to, len + 1)
        } else {
            self._is_accessible((from.0, from.1 - 1), to, len + 1)
        }
    }

    fn is_accessible(&self, from: Point, to: Point) -> Option<usize> {
        self._is_accessible(from, to, 0)
    }

    fn get_final_destination_list(&self, i: usize) -> Vec<Point> {
        if self.part2 {
            vec![
                (2 * i + 3, 5),
                (2 * i + 3, 4),
                (2 * i + 3, 3),
                (2 * i + 3, 2),
            ]
        } else {
            vec![(2 * i + 3, 3), (2 * i + 3, 2)]
        }
    }

    fn ready(&self) -> bool {
        for (i, pod) in self.pods.iter().enumerate() {
            for target in self.get_final_destination_list(i) {
                if !pod.is_at(target) {
                    return false;
                }
            }
        }
        true
    }

    fn get_lowest_empty_target(&self, i: usize) -> Point {
        for target in self.get_final_destination_list(i) {
            if !self.pods[i].is_at(target) {
                return target;
            }
        }

        unreachable!()
    }

    fn get_targets(&self) -> Vec<(usize, Point, Point)> {
        self.pods
            .iter()
            .enumerate()
            .flat_map(|(i, pod)| {
                pod.get_pods()
                    .iter()
                    .flat_map(move |pod| {
                        let mut res = Vec::new();
                        if pod.state == AmphipodState::Start {
                            res.push((1, 1));
                            res.push((2, 1));
                            res.push((4, 1));
                            res.push((6, 1));
                            res.push((8, 1));
                            res.push((10, 1));
                            res.push((11, 1));
                        };
                        if pod.state != AmphipodState::End {
                            res.push(self.get_lowest_empty_target(i));
                        }
                        res.into_iter().map(move |new_pos| (i, pod.pos, new_pos))
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .collect()
    }

    fn state_change(mut self, from: Point, to: Point) -> State {
        //println!("{}", self);
        //println!("{:?} -> {:?}", from, to);
        for pod in self.pods {
            assert!(!pod.is_at(to));
        }
        for pod in &mut self.pods {
            if pod.is_at(from) {
                pod.move_to(from, to);
            }
        }

        self
    }

    fn day_2(&mut self) {
        let mut realpods = vec![vec![], vec![], vec![], vec![]];
        for (i, pod) in &mut self.pods.iter().enumerate() {
            for mut realpod in pod.get_pods().to_vec() {
                if realpod.pos.1 == 3 {
                    realpod.pos = (realpod.pos.0, 5);
                }
                realpods[i].push(realpod);
            }
        }

        self.pods = [
            AmphipodPair::Part2([
                realpods[0][0],
                Amphipod::new((9, 3)),
                Amphipod::new((7, 4)),
                realpods[0][1],
            ]),
            AmphipodPair::Part2([
                realpods[1][0],
                Amphipod::new((7, 3)),
                Amphipod::new((5, 4)),
                realpods[1][1],
            ]),
            AmphipodPair::Part2([
                realpods[2][0],
                Amphipod::new((5, 3)),
                Amphipod::new((9, 4)),
                realpods[2][1],
            ]),
            AmphipodPair::Part2([
                realpods[3][0],
                Amphipod::new((3, 3)),
                Amphipod::new((3, 4)),
                realpods[3][1],
            ]),
        ];

        self.part2 = true;
    }

    fn approx_dist(&self, g: usize) -> usize {
        g - self
            .pods
            .iter()
            .enumerate()
            .flat_map(|(i, pod)| {
                pod.get_pods().iter().map(move |p| {
                    if p.pos.0 != 2 * i + 3 {
                        (((p.pos.0 as i32) - (2 * (i as i32) + 3)).abs()) as usize
                    } else {
                        0
                    }
                })
            })
            .sum::<usize>()
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (ty, pod) in self.pods.iter().enumerate() {
            writeln!(
                f,
                "{}: {}",
                (ty as u8 + b'A') as char,
                pod.get_pods()
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        write!(f, "")
    }
}

type Point = (usize, usize);
type Parsed = State;

fn calc_cost(ty: usize, len: usize) -> usize {
    match ty {
        0 => len,
        1 => 10 * len,
        2 => 100 * len,
        3 => 1000 * len,
        _ => unreachable!(),
    }
}

fn find_solution_cost(state: &State) -> Option<usize> {
    let mut q: BinaryHeap<(usize, usize, State)> = BinaryHeap::new();
    let mut visited = HashSet::new();

    q.push((usize::MAX, usize::MAX, *state));

    while let Some((_, cost, s)) = q.pop() {
        if s.ready() {
            return Some(usize::MAX - cost);
        }

        if visited.contains(&s) {
            continue;
        }
        visited.insert(s);

        for (ty, from, to) in s.get_targets() {
            if let Some(len) = s.is_accessible(from, to) {
                let new_cost = cost - calc_cost(ty, len);
                let new_state = s.state_change(from, to);
                q.push((new_state.approx_dist(new_cost), new_cost, new_state))
            }
        }
    }

    unreachable!()
}

fn twentythree_impl(input: &Parsed, day_2: bool) -> usize {
    let mut state = *input;
    if day_2 {
        state.day_2();
    }
    find_solution_cost(&state).unwrap()
}

fn parse<S: AsRef<str>>(input: &[S]) -> Parsed {
    let mut creatures = vec![vec![], vec![], vec![], vec![]];

    for (y, s) in input.iter().enumerate() {
        for (x, c) in s.as_ref().chars().enumerate() {
            match c {
                'A' | 'B' | 'C' | 'D' => {
                    let ty = ((c as u8) - b'A') as usize;
                    let pos = (x, y);
                    let state = if (x, y) == (2 * ty + 3, 3) {
                        AmphipodState::End
                    } else {
                        AmphipodState::Start
                    };
                    creatures[ty].push(Amphipod { pos, state });
                }
                _ => {}
            }
        }
    }

    State {
        pods: [
            AmphipodPair::Part1([creatures[0][0], creatures[0][1]]),
            AmphipodPair::Part1([creatures[1][0], creatures[1][1]]),
            AmphipodPair::Part1([creatures[2][0], creatures[2][1]]),
            AmphipodPair::Part1([creatures[3][0], creatures[3][1]]),
        ],
        part2: false,
    }
}

pub fn twentythree() -> Result<(), std::io::Error> {
    let file = File::open("23_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = twentythree_impl(&lines, false);
    println!("Day 23 part 1: {}", res);
    let res_2 = twentythree_impl(&lines, true);
    println!("Day 23 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_23::{parse, twentythree_impl};

    #[test]
    fn it_works() {
        assert_eq!(
            12521,
            twentythree_impl(
                &parse(&vec![
                    "#############",
                    "#...........#",
                    "###B#C#B#D###",
                    "  #A#D#C#A#  ",
                    "  #########  ",
                ]),
                false
            )
        );
        assert_eq!(
            44169,
            twentythree_impl(
                &parse(&vec![
                    "#############",
                    "#...........#",
                    "###B#C#B#D###",
                    "  #A#D#C#A#  ",
                    "  #########  ",
                ]),
                true
            )
        );
    }
}
