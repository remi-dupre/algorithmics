use crate::utils::SignedAdd;
use std::error::Error;

pub fn generator(input: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            Ok(match *line.split(' ').collect::<Vec<_>>().as_slice() {
                ["nop", x] => Instruction::Nop(x.parse()?),
                ["acc", x] => Instruction::Acc(x.parse()?),
                ["jmp", x] => Instruction::Jmp(x.parse()?),
                _ => return Err(format!("invalid instruction format: `{}`", line).into()),
            })
        })
        .collect()
}

pub fn part_1(program: &[Instruction]) -> isize {
    let mut computer = Computer::new(program);
    let mut seen = vec![false; program.len()];

    while !seen[computer.ptr] {
        seen[computer.ptr] = true;
        computer.step();
    }

    computer.acc
}

fn swap(instruction: Instruction) -> Option<Instruction> {
    match instruction {
        Instruction::Nop(x) => Some(Instruction::Jmp(x)),
        Instruction::Jmp(x) => Some(Instruction::Nop(x)),
        Instruction::Acc(_) => None,
    }
}

pub fn part_2_naive(program: &[Instruction]) -> Option<isize> {
    let mut program = program.to_vec();

    for swaped in 0..program.len() {
        if let Some(swap_val) = swap(program[swaped]) {
            program[swaped] = swap_val;

            let mut computer = Computer::new(&program);
            let mut seen = vec![false; program.len()];

            while computer.ptr < program.len() && !seen[computer.ptr] {
                seen[computer.ptr] = true;
                computer.step();
            }

            if computer.ptr >= program.len() {
                return Some(computer.acc);
            }

            program[swaped] = swap(program[swaped]).unwrap();
        }
    }

    None
}

pub fn part_2(program: &[Instruction]) -> isize {
    fn apply_offset(ptr: usize, instruction: Instruction) -> usize {
        match instruction {
            Instruction::Jmp(x) => ptr.signed_add(x).expect("bad ptr value"),
            _ => ptr + 1,
        }
    }

    let will_end_from = {
        let mut visited = vec![false; program.len()];
        let mut will_end_from = vec![false; program.len()];

        fn run(
            program: &[Instruction],
            node: usize,
            visited: &mut [bool],
            will_end_from: &mut [bool],
        ) -> bool {
            let target = apply_offset(node, program[node]);
            visited[node] = true;

            will_end_from[node] = {
                if target < program.len() && !visited[target] {
                    run(program, target, visited, will_end_from)
                } else if target < program.len() && visited[target] {
                    will_end_from[target]
                } else {
                    target >= program.len()
                }
            };

            will_end_from[node]
        }

        for start in 0..program.len() {
            if !visited[start] {
                run(program, start, &mut visited, &mut will_end_from);
            }
        }

        will_end_from
    };

    // Add shortcircuits to winning section whenever possible

    let program_len = program.len();
    let mut program = program.to_vec();

    for (node, inst) in program
        .iter_mut()
        .enumerate()
        .filter(|&(node, _)| !will_end_from[node])
    {
        if let Some(swaped) = swap(*inst) {
            let swap_target = apply_offset(node, swaped);

            if swap_target >= program_len || will_end_from[swap_target] {
                *inst = swaped;
            }
        }
    }

    // Run program

    let mut computer = Computer::new(&program);

    while computer.ptr < program.len() {
        computer.step();
    }

    computer.acc
}

// ---
// --- Structs
// ---

struct Computer<'p> {
    program: &'p [Instruction],
    ptr: usize,
    acc: isize,
}

impl<'p> Computer<'p> {
    fn new(program: &'p [Instruction]) -> Self {
        Self {
            program,
            ptr: 0,
            acc: 0,
        }
    }

    fn step(&mut self) {
        match self.program[self.ptr] {
            Instruction::Nop(_) => {}
            Instruction::Acc(x) => self.acc += x,
            Instruction::Jmp(x) => {
                self.ptr = self.ptr.signed_add(x).expect("bad ptr value");
                return;
            }
        }

        self.ptr += 1;
    }
}

#[derive(Copy, Clone)]
pub enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

// ---
// --- Tests
// ---

#[cfg(test)]
mod tests {
    use crate::day08::*;

    const EXAMPLE: &str = crate::lines! {
        "nop +0"
        "acc +1"
        "jmp +4"
        "acc +3"
        "jmp -3"
        "acc -99"
        "acc +1"
        "jmp -4"
        "acc +6"
    };

    #[test]
    fn test_part_1() {
        assert_eq!(5, part_1(&generator(EXAMPLE).unwrap()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(8, part_2(&generator(EXAMPLE).unwrap()));
        assert_eq!(Some(8), part_2_naive(&generator(EXAMPLE).unwrap()));
    }
}
