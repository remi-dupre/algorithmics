use std::convert::{TryFrom, TryInto};
use std::iter;

pub struct Computer<I>
where
    I: FnMut() -> isize,
{
    program: Vec<isize>,
    curr_ptr: Option<usize>,
    input: I,
}

impl<I> Computer<I>
where
    I: FnMut() -> isize,
{
    pub fn new<P: Into<Vec<isize>>>(program: P, input: I) -> Self {
        Self {
            program: program.into(),
            curr_ptr: Some(0),
            input,
        }
    }

    pub fn run(mut self) -> Vec<isize> {
        while self.next().is_some() {}
        self.program
    }

    pub fn replace_input(&mut self, new_input: I) -> I {
        std::mem::replace(&mut self.input, new_input)
    }
}

impl<I> Iterator for Computer<I>
where
    I: FnMut() -> isize,
{
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut output = None;

        while output.is_none() && self.curr_ptr.is_some() {
            let ptr = self.curr_ptr.unwrap();
            let instruction = Instruction::parse(&self.program[ptr..]);
            self.curr_ptr = instruction.apply(ptr, &mut self.program, &mut self.input, |val| {
                output = Some(val)
            })
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

    fn apply<I, O>(
        &self,
        ptr: usize,
        memory: &mut [isize],
        mut input: I,
        mut output: O,
    ) -> Option<usize>
    where
        I: FnMut() -> isize,
        O: FnMut(isize),
    {
        let params: Vec<_> = self.params.iter().map(|param| param.get(memory)).collect();

        let set_mem = |memory: &mut [isize], addr, val| {
            memory[usize::try_from(addr).expect("out of memory")] = val
        };

        match self.op {
            Operation::Add => set_mem(memory, self.params[2].val, params[0] + params[1]),
            Operation::Mul => set_mem(memory, self.params[2].val, params[0] * params[1]),
            Operation::Input => set_mem(memory, self.params[0].val, input()),
            Operation::Output => output(params[0]),
            Operation::End => return None,
            Operation::JumpIf(true) => {
                if params[0] != 0 {
                    return Some(params[1].try_into().expect("invalid jump"));
                }
            }
            Operation::JumpIf(false) => {
                if params[0] == 0 {
                    return Some(params[1].try_into().expect("invalid jump"));
                }
            }
            Operation::Less => set_mem(
                memory,
                self.params[2].val,
                if params[0] < params[1] { 1 } else { 0 },
            ),
            Operation::Equals => set_mem(
                memory,
                self.params[2].val,
                if params[0] == params[1] { 1 } else { 0 },
            ),
        };

        Some(1 + ptr + self.op.nb_parameters())
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
            99 => Self::End,
            _ => panic!("invalid opcode `{}`", opcode),
        }
    }

    fn nb_parameters(self) -> usize {
        match self {
            Self::End => 0,
            Self::Input | Self::Output => 1,
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
}

impl Mode {
    fn parse(mode: isize) -> Self {
        match mode {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => panic!("invalid mode `{}`", mode),
        }
    }
}

impl Parameter {
    fn get(self, memory: &mut [isize]) -> isize {
        match self.mode {
            Mode::Position => *memory
                .get(usize::try_from(self.val).expect("invalid pointer"))
                .expect("out of memory"),
            Mode::Immediate => self.val,
        }
    }
}
