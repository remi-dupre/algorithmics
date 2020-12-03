use std::convert::TryInto;
use std::fmt::Debug;
use std::iter;

pub struct Computer<I>
where
    I: FnMut() -> isize,
{
    memory: Vec<isize>,
    run: bool,
    ptr: usize,
    rbase: usize,
    input: I,
}

impl<I> Computer<I>
where
    I: FnMut() -> isize,
{
    pub fn new<P: Into<Vec<isize>>>(program: P, input: I) -> Self {
        Self {
            memory: program.into(),
            run: true,
            ptr: 0,
            rbase: 0,
            input,
        }
    }

    pub fn replace_input(&mut self, new_input: I) -> I {
        std::mem::replace(&mut self.input, new_input)
    }

    pub fn run(mut self) -> Vec<isize> {
        while self.next().is_some() {}
        self.memory
    }

    fn apply<O>(&mut self, inst: Instruction, mut output: O)
    where
        O: FnMut(isize),
    {
        assert!(self.run);
        let params = inst.params;

        match inst.op {
            Operation::Add => *params[2].get_mut(self) = params[0].get(self) + params[1].get(self),
            Operation::Mul => *params[2].get_mut(self) = params[0].get(self) * params[1].get(self),
            Operation::Input => {
                let input = (self.input)();
                *params[0].get_mut(self) = input;
            }
            Operation::Output => output(params[0].get(self)),
            Operation::End => {
                self.run = false;
                return;
            }
            Operation::JumpIf(true) => {
                if params[0].get(self) != 0 {
                    self.ptr = params[1].get(self).try_into().expect("invalid jump");
                    return;
                }
            }
            Operation::JumpIf(false) => {
                if params[0].get(self) == 0 {
                    self.ptr = params[1].get(self).try_into().expect("invalid jump");
                    return;
                }
            }
            Operation::Less => {
                *params[2].get_mut(self) = (params[0].get(self) < params[1].get(self)) as isize
            }
            Operation::Equals => {
                *params[2].get_mut(self) = (params[0].get(self) == params[1].get(self)) as isize
            }
            Operation::AdjustRBase => {
                self.rbase = {
                    (self.rbase as isize + params[0].get(self))
                        .try_into()
                        .expect("negative rbase")
                }
            }
        };

        self.ptr += 1 + inst.op.nb_parameters();
    }
}

impl<I> Iterator for Computer<I>
where
    I: FnMut() -> isize,
{
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut output = None;

        while output.is_none() && self.run {
            let instruction = Instruction::parse(&self.memory[self.ptr..]);
            self.apply(instruction, |val| output = Some(val))
        }

        output
    }
}

#[derive(Debug)]
struct Instruction {
    op: Operation,
    params: Vec<Parameter>,
}

impl Instruction {
    fn parse(program: &[isize]) -> Self {
        let instruction_code = program[0];
        let op = Operation::parse(instruction_code % 100);

        let params_val = program[1..].iter().copied();
        let params_mode = {
            let mut encoded = instruction_code / 100;

            iter::from_fn(move || {
                let res = encoded % 10;
                encoded /= 10;
                Some(res)
            })
            .map(Mode::parse)
        };

        let params = params_mode
            .zip(params_val)
            .map(|(mode, val)| Parameter { mode, val })
            .take(op.nb_parameters())
            .collect();

        Self { op, params }
    }
}

#[derive(Copy, Clone, Debug)]
enum Operation {
    Add,
    Mul,
    Input,
    Output,
    End,
    JumpIf(bool),
    Less,
    Equals,
    AdjustRBase,
}

impl Operation {
    fn parse(opcode: isize) -> Self {
        match opcode {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIf(true),
            6 => Self::JumpIf(false),
            7 => Self::Less,
            8 => Self::Equals,
            9 => Self::AdjustRBase,
            99 => Self::End,
            _ => panic!("invalid opcode `{}`", opcode),
        }
    }

    fn nb_parameters(self) -> usize {
        match self {
            Self::End => 0,
            Self::Input | Self::Output | Self::AdjustRBase => 1,
            Self::JumpIf(_) => 2,
            Self::Add | Self::Mul | Self::Less | Self::Equals => 3,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Parameter {
    mode: Mode,
    val: isize,
}

#[derive(Copy, Clone, Debug)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Mode {
    fn parse(mode: isize) -> Self {
        match mode {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => panic!("invalid mode `{}`", mode),
        }
    }
}

impl Parameter {
    fn get<I: FnMut() -> isize>(self, computer: &Computer<I>) -> isize {
        match self.mode {
            Mode::Position => {
                let ptr: usize = self.val.try_into().expect("invalid negative pointer");
                computer.memory.get(ptr).copied().unwrap_or(0)
            }
            Mode::Immediate => self.val,
            Mode::Relative => {
                let as_pos = Self {
                    mode: Mode::Position,
                    val: computer.rbase as isize + self.val,
                };
                as_pos.get(computer)
            }
        }
    }

    fn get_mut<I: FnMut() -> isize>(self, computer: &mut Computer<I>) -> &mut isize {
        match self.mode {
            Mode::Position => {
                let ptr: usize = self.val.try_into().expect("invalid negative pointer");

                if ptr >= computer.memory.len() {
                    computer.memory.resize(ptr + 1, 0);
                }

                computer.memory.get_mut(ptr).unwrap()
            }
            Mode::Immediate => panic!("can't mutate in immediate mode"),
            Mode::Relative => {
                let as_pos = Self {
                    mode: Mode::Position,
                    val: computer.rbase as isize + self.val,
                };
                as_pos.get_mut(computer)
            }
        }
    }
}
