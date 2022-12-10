use anyhow::{bail, Context, Result};

type Cycle = u64;
type Register = i64;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    AddX(Register),
    NoOp,
}

impl Instruction {
    pub fn duration_cycles(&self) -> Cycle {
        match self {
            Instruction::AddX(_) => 2,
            Instruction::NoOp => 1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub cycle: Cycle,
    pub register: Register,
}

impl Default for State {
    fn default() -> Self {
        Self {
            cycle: 0,
            register: 1,
        }
    }
}

pub fn execute(program: &[Instruction]) -> impl Iterator<Item = State> + '_ {
    let mut state = State::default();

    program.iter().filter_map(move |inst| {
        state.cycle += inst.duration_cycles();

        match inst {
            Instruction::AddX(val) => state.register += val,
            Instruction::NoOp => return None,
        };

        Some(state)
    })
}

pub fn parse(input: &str) -> Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            let mut tokens = line.split(' ');

            let op = match tokens.next() {
                Some("addx") => {
                    let val = tokens
                        .next()
                        .context("missing parameter for addx")?
                        .parse()
                        .context("invalid parameter for addx")?;

                    Instruction::AddX(val)
                }
                Some("noop") => Instruction::NoOp,
                Some(other) => bail!("unknown instruction {other:?}"),
                None => bail!("line without any instruction"),
            };

            if tokens.next().is_some() {
                bail!("too many arguments for {op:?}");
            }

            Ok(op)
        })
        .collect()
}

pub fn part1(program: &[Instruction]) -> i128 {
    let mut runtime = execute(program).peekable();
    let mut state = runtime.next().unwrap();

    (20..)
        .step_by(40)
        .map_while(|cycle| {
            while runtime.peek()?.cycle < cycle {
                state = runtime.next()?;
            }

            Some(i128::from(state.register) * i128::from(cycle))
        })
        .sum()
}

pub fn part2(program: &[Instruction]) -> String {
    let height = 6;
    let width = 40;

    let mut runtime = execute(program).peekable();
    let mut state = runtime.next().unwrap();
    let mut res = String::new();

    for line in 0..height {
        let start = width * line;
        let end = start + width;

        for cycle in start..end {
            if matches!(runtime.peek(), Some(x) if x.cycle <= cycle) {
                state = runtime.next().unwrap();
            }

            res.push({
                if (state.register).abs_diff((cycle - start) as _) <= 1 {
                    '░'
                } else {
                    ' '
                }
            })
        }

        res.push('\n');
    }

    res
}
