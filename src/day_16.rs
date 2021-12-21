use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Peekable;

type Parsed = Vec<bool>;

#[derive(Debug)]
struct Packet {
    id: u8,
    version: u8,
    content: PacketContent,
}

#[derive(Debug)]
enum PacketContent {
    Literal(usize),
    Operator(Vec<Packet>),
}

impl Packet {
    fn version_sum(&self) -> usize {
        self.version as usize
            + match &self.content {
                PacketContent::Literal(_) => 0,
                PacketContent::Operator(v) => v.iter().map(|p| p.version_sum()).sum(),
            }
    }

    fn value(&self) -> usize {
        match &self.content {
            PacketContent::Literal(v) => *v,
            PacketContent::Operator(ps) => match self.id {
                0 => ps.iter().map(|p| p.value()).sum(),
                1 => ps.iter().map(|p| p.value()).product(),
                2 => ps.iter().map(|p| p.value()).min().unwrap(),
                3 => ps.iter().map(|p| p.value()).max().unwrap(),
                5 => {
                    if ps[0].value() > ps[1].value() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if ps[0].value() < ps[1].value() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if ps[0].value() == ps[1].value() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

fn read_num<I: Iterator<Item = bool>>(iter: &mut Peekable<I>, bitcount: usize) -> usize {
    let mut res = 0;
    for _ in 0..bitcount {
        res = (res << 1) | iter.next().unwrap() as usize;
    }
    res
}

fn literal<I: Iterator<Item = bool>>(iter: &mut Peekable<I>) -> (PacketContent, usize) {
    let mut num = 0;
    let mut stop = !iter.next().unwrap();
    let mut read_bits = 0;
    loop {
        read_bits += 5;
        num = (num << 4) | read_num(iter, 4);
        if stop {
            break;
        }
        stop = !iter.next().unwrap();
    }
    (PacketContent::Literal(num), read_bits)
}

fn operator<I: Iterator<Item = bool>>(iter: &mut Peekable<I>) -> (PacketContent, usize) {
    if !iter.next().unwrap() {
        let subpacket_bits = read_num(iter, 15);
        let mut read_bits = 1 + 15;
        let mut res = Vec::new();
        while read_bits - 12 < subpacket_bits {
            let (packet, bits) = packet(iter);
            res.push(packet);
            read_bits += bits;
        }
        (PacketContent::Operator(res), read_bits)
    } else {
        let subpacket_count = read_num(iter, 11);
        let mut read_bits = 1 + 11;
        let mut res = Vec::new();
        for _ in 0..subpacket_count {
            let (packet, bits) = packet(iter);
            res.push(packet);
            read_bits += bits;
        }
        (PacketContent::Operator(res), read_bits)
    }
}

fn packet<I: Iterator<Item = bool>>(iter: &mut Peekable<I>) -> (Packet, usize) {
    let version: u8 = read_num(iter, 3) as u8;
    let ty: u8 = read_num(iter, 3) as u8;

    let (content, read_bits) = match ty {
        4 => literal(iter),
        _ => operator(iter),
    };

    (
        Packet {
            id: ty,
            version,
            content,
        },
        read_bits + 6,
    )
}

fn sixteen_impl(input: &[bool], day_2: bool) -> usize {
    let mut iter = input.iter().copied().peekable();

    let _extras = 0;

    let mut res = Vec::new();

    let mut cont = true;
    while cont {
        res.push(packet(&mut iter).0);
        cont = iter.peek() == Some(&true);
    }

    if !day_2 {
        res.iter().map(|p| p.version_sum()).sum()
    } else {
        res[0].value()
    }
}

fn parse<S: AsRef<str>>(input: &[S]) -> Parsed {
    input
        .iter()
        .flat_map(|s| {
            s.as_ref().chars().flat_map(|c| {
                format!("{:04b}", c.to_digit(16).unwrap())
                    .chars()
                    .map(|c| c == '1')
                    .collect::<Vec<_>>()
                    .into_iter()
            })
        })
        .collect()
}

pub fn sixteen() -> Result<(), std::io::Error> {
    let file = File::open("16_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res = sixteen_impl(&lines, false);
    println!("Day 15 part 1: {}", res);
    let res_2 = sixteen_impl(&lines, true);
    println!("Day 15 part 2: {}", res_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_16::{parse, sixteen_impl};

    #[test]
    fn it_works() {
        assert_eq!(6, sixteen_impl(&parse(&vec!["D2FE28"]), false));
        assert_eq!(16, sixteen_impl(&parse(&vec!["8A004A801A8002F478"]), false));
        assert_eq!(
            12,
            sixteen_impl(&parse(&vec!["620080001611562C8802118E34"]), false)
        );
        assert_eq!(
            23,
            sixteen_impl(&parse(&vec!["C0015000016115A2E0802F182340"]), false)
        );
        assert_eq!(
            31,
            sixteen_impl(&parse(&vec!["A0016C880162017C3686B18A3D4780"]), false)
        );

        assert_eq!(3, sixteen_impl(&parse(&vec!["C200B40A82"]), true));
        assert_eq!(54, sixteen_impl(&parse(&vec!["04005AC33890"]), true));
        assert_eq!(7, sixteen_impl(&parse(&vec!["880086C3E88112"]), true));
        assert_eq!(9, sixteen_impl(&parse(&vec!["CE00C43D881120"]), true));
        assert_eq!(1, sixteen_impl(&parse(&vec!["D8005AC2A8F0"]), true));
        assert_eq!(0, sixteen_impl(&parse(&vec!["F600BC2D8F"]), true));
        assert_eq!(0, sixteen_impl(&parse(&vec!["9C005AC2F8F0"]), true));
        assert_eq!(
            1,
            sixteen_impl(&parse(&vec!["9C0141080250320F1802104A08"]), true)
        );
    }
}
