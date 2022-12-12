use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NumOrAddr {
    Num(i32),
    Addr(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Instr {
    Inp(u8),
    Add(u8, NumOrAddr),
    Mul(u8, NumOrAddr),
    Div(u8, NumOrAddr),
    Mod(u8, NumOrAddr),
    Eql(u8, NumOrAddr),
}

type Parsed = Vec<Instr>;

fn read(memory: [i32; 4], addr: NumOrAddr) -> i32 {
    match addr {
        NumOrAddr::Num(num) => num,
        NumOrAddr::Addr(addr) => memory[addr as usize],
    }
}

fn write(mut memory: [i32; 4], addr: u8, val: i32) -> [i32; 4] {
    memory[addr as usize] = val;
    memory
}

fn run_program(
    program: &[Instr],
    history: &mut HashSet<(usize, [i32; 4])>,
    ip: usize,
    code: usize,
    memory: [i32; 4],
) -> usize {
    if ip == program.len() {
        if code % 10000000000 == 9999999999 {
            println!("{}", code);
        }
        if memory[3] == 0 {
            return code;
        } else {
            return 0;
        }
    }

    if history.contains(&(ip, memory)) {
        return 0;
    }
    history.insert((ip, memory));

    match program[ip] {
        Instr::Inp(addr) => {
            if code == 0 {
                for n in (1..=9).rev() {
                    let c = run_program(
                        program,
                        history,
                        ip + 1,
                        code * 10 + n,
                        write(memory, 0, n as i32),
                    );
                    if c != 0 {
                        return c;
                    }
                    history.clear();
                }
            } else {
                for n in (1..=9).rev() {
                    let c = run_program(
                        program,
                        history,
                        ip + 1,
                        code * 10 + n,
                        write(memory, 0, n as i32),
                    );
                    if c != 0 {
                        return c;
                    }
                }
            }
            0
        }
        Instr::Add(addr, val) => run_program(
            program,
            history,
            ip + 1,
            code,
            write(memory, addr, memory[addr as usize] + read(memory, val)),
        ),
        Instr::Mul(addr, val) => run_program(
            program,
            history,
            ip + 1,
            code,
            write(memory, addr, memory[addr as usize] * read(memory, val)),
        ),
        Instr::Div(addr, val) => run_program(
            program,
            history,
            ip + 1,
            code,
            write(memory, addr, memory[addr as usize] / read(memory, val)),
        ),
        Instr::Mod(addr, val) => run_program(
            program,
            history,
            ip + 1,
            code,
            write(memory, addr, memory[addr as usize] % read(memory, val)),
        ),
        Instr::Eql(addr, val) => run_program(
            program,
            history,
            ip + 1,
            code,
            write(
                memory,
                addr,
                if memory[addr as usize] == read(memory, val) {
                    1
                } else {
                    0
                },
            ),
        ),
    }
}

fn run_program_2(
    program: &[Instr],
    history: &mut HashSet<(usize, [i32; 4])>,
    ip: usize,
    code: usize,
    memory: [i32; 4],
) -> usize {
    if ip == program.len() {
        if code % 10000000000 == 1111111111 {
            println!("{}", code);
        }
        if memory[3] == 0 {
            return code;
        } else {
            return 0;
        }
    }

    if history.contains(&(ip, memory)) {
        return 0;
    }
    history.insert((ip, memory));

    match program[ip] {
        Instr::Inp(addr) => {
            if code == 0 {
                for n in 1..=9 {
                    let c = run_program_2(
                        program,
                        history,
                        ip + 1,
                        code * 10 + n,
                        write(memory, 0, n as i32),
                    );
                    if c != 0 {
                        return c;
                    }
                    history.clear();
                }
            } else {
                for n in 1..=9 {
                    let c = run_program_2(
                        program,
                        history,
                        ip + 1,
                        code * 10 + n,
                        write(memory, 0, n as i32),
                    );
                    if c != 0 {
                        return c;
                    }
                }
            }
            0
        }
        Instr::Add(addr, val) => run_program_2(
            program,
            history,
            ip + 1,
            code,
            write(memory, addr, memory[addr as usize] + read(memory, val)),
        ),
        Instr::Mul(addr, val) => run_program_2(
            program,
            history,
            ip + 1,
            code,
            write(memory, addr, memory[addr as usize] * read(memory, val)),
        ),
        Instr::Div(addr, val) => run_program_2(
            program,
            history,
            ip + 1,
            code,
            write(memory, addr, memory[addr as usize] / read(memory, val)),
        ),
        Instr::Mod(addr, val) => run_program_2(
            program,
            history,
            ip + 1,
            code,
            write(memory, addr, memory[addr as usize] % read(memory, val)),
        ),
        Instr::Eql(addr, val) => run_program_2(
            program,
            history,
            ip + 1,
            code,
            write(
                memory,
                addr,
                if memory[addr as usize] == read(memory, val) {
                    1
                } else {
                    0
                },
            ),
        ),
    }
}

fn twentyfour_impl(input: &Parsed, day_2: bool) -> usize {
    if day_2 {
        run_program_2(input, &mut HashSet::new(), 0, 0, [0, 0, 0, 0])
    } else {
        run_program(input, &mut HashSet::new(), 0, 0, [0, 0, 0, 0])
    }
}

fn parse<S: AsRef<str>>(input: &[S]) -> Parsed {
    fn parse_addr(s: &str) -> u8 {
        let c = s.chars().next().unwrap();
        match c {
            'x' | 'y' | 'z' | 'w' => (c as u8) - b'w',
            _ => panic!(),
        }
    }
    fn parse_num_or_addr(s: &str) -> NumOrAddr {
        let c = s.chars().next().unwrap();
        match c {
            'x' | 'y' | 'z' | 'w' => NumOrAddr::Addr((c as u8) - b'w'),
            _ => NumOrAddr::Num(s.parse().unwrap()),
        }
    }

    input
        .iter()
        .map(|s| {
            let parts = s.as_ref().split(' ').collect::<Vec<_>>();
            let addr = parse_addr(parts[1]);
            if parts[0] == "inp" {
                return Instr::Inp(addr);
            }
            let operand = parse_num_or_addr(parts[2]);
            match parts[0] {
                "add" => Instr::Add(addr, operand),
                "mul" => Instr::Mul(addr, operand),
                "div" => Instr::Div(addr, operand),
                "mod" => Instr::Mod(addr, operand),
                "eql" => Instr::Eql(addr, operand),
                op => unimplemented!("{}", op),
            }
        })
        .collect()
}

pub fn twentyfour() -> Result<(), std::io::Error> {
    let file = File::open("24_input")?;
    let reader = BufReader::new(file);
    let lines = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    let res_2 = twentyfour_impl(&lines, true);
    println!("Day 24 part 2: {}", res_2);
    let res = twentyfour_impl(&lines, false);
    println!("Day 24 part 1: {}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
