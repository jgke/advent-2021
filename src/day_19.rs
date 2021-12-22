use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Parsed = Vec<Vec<Point>>;
type Point = (i32, i32, i32);
type Rotation = u8;

fn flip(coord: Point, rotation: Rotation) -> Point {
    assert!(rotation < 24);

    let (x, y, z) = coord;

    match rotation {
        0 => (x, y, z),
        1 => (-y, x, z),
        2 => (-x, -y, z),
        3 => (y, -x, z),

        4 => (z, y, -x),
        5 => (z, x, y),
        6 => (z, -y, x),
        7 => (z, -x, -y),

        8 => (-x, y, -z),
        9 => (-y, -x, -z),
        10 => (x, -y, -z),
        11 => (y, x, -z),

        12 => (-z, y, x),
        13 => (-z, x, -y),
        14 => (-z, -y, -x),
        15 => (-z, -x, y),

        16 => (x, z, -y),
        17 => (-y, z, -x),
        18 => (-x, z, y),
        19 => (y, z, x),

        20 => (x, -z, y),
        21 => (y, -z, -x),
        22 => (-x, -z, -y),
        23 => (-y, -z, x),

        _ => unreachable!(),
    }
}

fn delta(pos1: Point, pos2: Point) -> Point {
    (pos2.0 - pos1.0, pos2.1 - pos1.1, pos2.2 - pos1.2)
}

fn plus(pos1: Point, pos2: Point) -> Point {
    (pos2.0 + pos1.0, pos2.1 + pos1.1, pos2.2 + pos1.2)
}

fn in_range(pos1: Point, pos2: Point) -> bool {
    (pos2.0 - pos1.0).abs() <= 1000
        && (pos2.1 - pos1.1).abs() <= 1000
        && (pos2.2 - pos1.2).abs() <= 1000
}

fn manhattan(pos1: Point, pos2: Point) -> usize {
    ((pos2.0 - pos1.0).abs() + (pos2.1 - pos1.1).abs() + (pos2.2 - pos1.2).abs()) as usize
}

fn get_fit(known_positions: &HashSet<Point>, scanner: &[Point]) -> Option<(Point, Rotation)> {
    let mut best = None;
    for rotation in 0..24 {
        let scan_positions: Vec<Point> = scanner.iter().map(|p| flip(*p, rotation)).collect();

        for known_point in known_positions {
            for scan_point in &scan_positions {
                let d = delta(*scan_point, *known_point);
                assert!(known_positions.contains(&plus(*scan_point, d)));

                let count = scan_positions
                    .iter()
                    .filter(|p| known_positions.contains(&plus(**p, d)))
                    .count();

                if let Some((best_count, _, _)) = best {
                    if count > best_count {
                        best = Some((count, d, rotation));
                    }
                } else {
                    best = Some((count, d, rotation));
                }
            }
        }
    }

    best.filter(|(c, _, _)| *c >= 12.min(known_positions.len()))
        .map(|(_, d, r)| (d, r))
}

fn run_round(known: usize, input: &[Vec<Point>], day_2: bool) -> Option<usize> {
    let mut known_positions: HashSet<Point> = input[known].iter().copied().collect();
    let mut remaining_scanners: HashSet<usize> = (0..input.len()).collect();
    remaining_scanners.remove(&known);
    let mut known_satellites = HashSet::new();
    known_satellites.insert((0, 0, 0));

    while !remaining_scanners.is_empty() {
        let mut next_scanners = HashSet::new();
        for scanner_num in &remaining_scanners {
            let scanner = &input[*scanner_num];
            if let Some((d, r)) = get_fit(&known_positions, scanner) {
                known_satellites.insert(d);
                known_positions.extend(
                    scanner
                        .iter()
                        .map(|p| plus(flip(*p, r), d))
                        .collect::<HashSet<Point>>(),
                );
                assert!(known_positions
                    .iter()
                    .all(|p| known_satellites.iter().any(|s| in_range(*s, *p))));
            } else {
                next_scanners.insert(*scanner_num);
            }
        }
        if remaining_scanners.len() == next_scanners.len() {
            return None;
        }
        remaining_scanners = next_scanners;
    }

    if !day_2 {
        Some(known_positions.len())
    } else {
        known_satellites
            .iter()
            .copied()
            .flat_map(|p| known_satellites.iter().copied().map(move |p2| (p, p2)))
            .map(|(p1, p2)| manhattan(p1, p2))
            .max()
    }
}

fn nineteen_impl(input: &[Vec<Point>], day_2: bool) -> usize {
    run_round(0, input, day_2).unwrap()
}

fn parse<S: AsRef<str>>(input: &[S]) -> Parsed {
    let mut iter = input.iter().map(|s| s.as_ref()).peekable();
    let mut reports = Vec::new();
    let mut current_report = Vec::new();

    loop {
        let _title = iter.next();
        loop {
            let row = iter.next();
            if row == None || row == Some("") {
                reports.push(current_report);
                current_report = Vec::new();
                break;
            }
            let coords = row
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();
            current_report.push((coords[0], coords[1], coords[2]));
        }
        if iter.peek().is_none() {
            return reports;
        }
    }
}

pub fn nineteen() -> Result<(), std::io::Error> {
    let file = File::open("19_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = nineteen_impl(&lines, false);
    println!("Day 19 part 1: {}", res);
    let res_2 = nineteen_impl(&lines, true);
    println!("Day 19 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_19::{in_range, manhattan, nineteen_impl, parse};

    #[test]
    fn range() {
        assert_eq!(true, in_range((500, 0, -500), (-500, 1000, -1500)));
        assert_eq!(false, in_range((500, 0, -500), (1501, 0, -500)));
    }

    #[test]
    fn test_manhattan() {
        assert_eq!(3621, manhattan((1105, -1205, 1229), (-92, -2380, -20)));
    }

    #[test]
    fn it_works() {
        assert_eq!(
            6,
            nineteen_impl(
                &parse(&vec![
                    "--- scanner 0 ---",
                    "-1,-1,1",
                    "-2,-2,2",
                    "-3,-3,3",
                    "-2,-3,1",
                    "5,6,-4",
                    "8,0,7",
                    "",
                    "--- scanner 0 ---",
                    "1,-1,1",
                    "2,-2,2",
                    "3,-3,3",
                    "2,-1,3",
                    "-5,4,-6",
                    "-8,-7,0",
                    "",
                    "--- scanner 0 ---",
                    "-1,-1,-1",
                    "-2,-2,-2",
                    "-3,-3,-3",
                    "-1,-3,-2",
                    "4,6,5",
                    "-7,0,8",
                    "",
                    "--- scanner 0 ---",
                    "1,1,-1",
                    "2,2,-2",
                    "3,3,-3",
                    "1,3,-2",
                    "-4,-6,5",
                    "7,0,8",
                    "",
                    "--- scanner 0 ---",
                    "1,1,1",
                    "2,2,2",
                    "3,3,3",
                    "3,1,2",
                    "-6,-4,-5",
                    "0,7,-8",
                ]),
                false
            )
        );
        assert_eq!(
            79,
            nineteen_impl(
                &parse(&vec![
                    "--- scanner 0 ---",
                    "404,-588,-901",
                    "528,-643,409",
                    "-838,591,734",
                    "390,-675,-793",
                    "-537,-823,-458",
                    "-485,-357,347",
                    "-345,-311,381",
                    "-661,-816,-575",
                    "-876,649,763",
                    "-618,-824,-621",
                    "553,345,-567",
                    "474,580,667",
                    "-447,-329,318",
                    "-584,868,-557",
                    "544,-627,-890",
                    "564,392,-477",
                    "455,729,728",
                    "-892,524,684",
                    "-689,845,-530",
                    "423,-701,434",
                    "7,-33,-71",
                    "630,319,-379",
                    "443,580,662",
                    "-789,900,-551",
                    "459,-707,401",
                    "",
                    "--- scanner 1 ---",
                    "686,422,578",
                    "605,423,415",
                    "515,917,-361",
                    "-336,658,858",
                    "95,138,22",
                    "-476,619,847",
                    "-340,-569,-846",
                    "567,-361,727",
                    "-460,603,-452",
                    "669,-402,600",
                    "729,430,532",
                    "-500,-761,534",
                    "-322,571,750",
                    "-466,-666,-811",
                    "-429,-592,574",
                    "-355,545,-477",
                    "703,-491,-529",
                    "-328,-685,520",
                    "413,935,-424",
                    "-391,539,-444",
                    "586,-435,557",
                    "-364,-763,-893",
                    "807,-499,-711",
                    "755,-354,-619",
                    "553,889,-390",
                    "",
                    "--- scanner 2 ---",
                    "649,640,665",
                    "682,-795,504",
                    "-784,533,-524",
                    "-644,584,-595",
                    "-588,-843,648",
                    "-30,6,44",
                    "-674,560,763",
                    "500,723,-460",
                    "609,671,-379",
                    "-555,-800,653",
                    "-675,-892,-343",
                    "697,-426,-610",
                    "578,704,681",
                    "493,664,-388",
                    "-671,-858,530",
                    "-667,343,800",
                    "571,-461,-707",
                    "-138,-166,112",
                    "-889,563,-600",
                    "646,-828,498",
                    "640,759,510",
                    "-630,509,768",
                    "-681,-892,-333",
                    "673,-379,-804",
                    "-742,-814,-386",
                    "577,-820,562",
                    "",
                    "--- scanner 3 ---",
                    "-589,542,597",
                    "605,-692,669",
                    "-500,565,-823",
                    "-660,373,557",
                    "-458,-679,-417",
                    "-488,449,543",
                    "-626,468,-788",
                    "338,-750,-386",
                    "528,-832,-391",
                    "562,-778,733",
                    "-938,-730,414",
                    "543,643,-506",
                    "-524,371,-870",
                    "407,773,750",
                    "-104,29,83",
                    "378,-903,-323",
                    "-778,-728,485",
                    "426,699,580",
                    "-438,-605,-362",
                    "-469,-447,-387",
                    "509,732,623",
                    "647,635,-688",
                    "-868,-804,481",
                    "614,-800,639",
                    "595,780,-596",
                    "",
                    "--- scanner 4 ---",
                    "727,592,562",
                    "-293,-554,779",
                    "441,611,-461",
                    "-714,465,-776",
                    "-743,427,-804",
                    "-660,-479,-426",
                    "832,-632,460",
                    "927,-485,-438",
                    "408,393,-506",
                    "466,436,-512",
                    "110,16,151",
                    "-258,-428,682",
                    "-393,719,612",
                    "-211,-452,876",
                    "808,-476,-593",
                    "-575,615,604",
                    "-485,667,467",
                    "-680,325,-822",
                    "-627,-443,-432",
                    "872,-547,-609",
                    "833,512,582",
                    "807,604,487",
                    "839,-516,451",
                    "891,-625,532",
                    "-652,-548,-490",
                    "30,-46,-14",
                ]),
                false
            )
        );
        assert_eq!(
            3621,
            nineteen_impl(
                &parse(&vec![
                    "--- scanner 0 ---",
                    "404,-588,-901",
                    "528,-643,409",
                    "-838,591,734",
                    "390,-675,-793",
                    "-537,-823,-458",
                    "-485,-357,347",
                    "-345,-311,381",
                    "-661,-816,-575",
                    "-876,649,763",
                    "-618,-824,-621",
                    "553,345,-567",
                    "474,580,667",
                    "-447,-329,318",
                    "-584,868,-557",
                    "544,-627,-890",
                    "564,392,-477",
                    "455,729,728",
                    "-892,524,684",
                    "-689,845,-530",
                    "423,-701,434",
                    "7,-33,-71",
                    "630,319,-379",
                    "443,580,662",
                    "-789,900,-551",
                    "459,-707,401",
                    "",
                    "--- scanner 1 ---",
                    "686,422,578",
                    "605,423,415",
                    "515,917,-361",
                    "-336,658,858",
                    "95,138,22",
                    "-476,619,847",
                    "-340,-569,-846",
                    "567,-361,727",
                    "-460,603,-452",
                    "669,-402,600",
                    "729,430,532",
                    "-500,-761,534",
                    "-322,571,750",
                    "-466,-666,-811",
                    "-429,-592,574",
                    "-355,545,-477",
                    "703,-491,-529",
                    "-328,-685,520",
                    "413,935,-424",
                    "-391,539,-444",
                    "586,-435,557",
                    "-364,-763,-893",
                    "807,-499,-711",
                    "755,-354,-619",
                    "553,889,-390",
                    "",
                    "--- scanner 2 ---",
                    "649,640,665",
                    "682,-795,504",
                    "-784,533,-524",
                    "-644,584,-595",
                    "-588,-843,648",
                    "-30,6,44",
                    "-674,560,763",
                    "500,723,-460",
                    "609,671,-379",
                    "-555,-800,653",
                    "-675,-892,-343",
                    "697,-426,-610",
                    "578,704,681",
                    "493,664,-388",
                    "-671,-858,530",
                    "-667,343,800",
                    "571,-461,-707",
                    "-138,-166,112",
                    "-889,563,-600",
                    "646,-828,498",
                    "640,759,510",
                    "-630,509,768",
                    "-681,-892,-333",
                    "673,-379,-804",
                    "-742,-814,-386",
                    "577,-820,562",
                    "",
                    "--- scanner 3 ---",
                    "-589,542,597",
                    "605,-692,669",
                    "-500,565,-823",
                    "-660,373,557",
                    "-458,-679,-417",
                    "-488,449,543",
                    "-626,468,-788",
                    "338,-750,-386",
                    "528,-832,-391",
                    "562,-778,733",
                    "-938,-730,414",
                    "543,643,-506",
                    "-524,371,-870",
                    "407,773,750",
                    "-104,29,83",
                    "378,-903,-323",
                    "-778,-728,485",
                    "426,699,580",
                    "-438,-605,-362",
                    "-469,-447,-387",
                    "509,732,623",
                    "647,635,-688",
                    "-868,-804,481",
                    "614,-800,639",
                    "595,780,-596",
                    "",
                    "--- scanner 4 ---",
                    "727,592,562",
                    "-293,-554,779",
                    "441,611,-461",
                    "-714,465,-776",
                    "-743,427,-804",
                    "-660,-479,-426",
                    "832,-632,460",
                    "927,-485,-438",
                    "408,393,-506",
                    "466,436,-512",
                    "110,16,151",
                    "-258,-428,682",
                    "-393,719,612",
                    "-211,-452,876",
                    "808,-476,-593",
                    "-575,615,604",
                    "-485,667,467",
                    "-680,325,-822",
                    "-627,-443,-432",
                    "872,-547,-609",
                    "833,512,582",
                    "807,604,487",
                    "839,-516,451",
                    "891,-625,532",
                    "-652,-548,-490",
                    "30,-46,-14",
                ]),
                true
            )
        );
    }
}
