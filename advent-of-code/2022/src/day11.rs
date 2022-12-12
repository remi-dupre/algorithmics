use anyhow::{bail, Context, Result};

pub type Worry = u64;
pub type MonkeyId = usize;

#[derive(Clone, Copy)]
enum StressStrategy {
    Div(Worry),
    Mod(Worry),
}

impl StressStrategy {
    fn eval(&self, old: Worry) -> Worry {
        match self {
            StressStrategy::Div(d) => old / d,
            StressStrategy::Mod(m) => old % m,
        }
    }
}

#[derive(Clone, Copy)]
enum BinOp {
    Add,
    Mul,
}

impl BinOp {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => bail!("invalid operator {s:?}"),
        }
    }
}

#[derive(Clone, Copy)]
enum BinOpAtom {
    Old,
    Const(Worry),
}

impl BinOpAtom {
    fn eval(&self, old: Worry) -> Worry {
        match self {
            BinOpAtom::Old => old,
            BinOpAtom::Const(cst) => *cst,
        }
    }
}

impl BinOpAtom {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "old" => Ok(Self::Old),
            _ => s
                .parse()
                .map(Self::Const)
                .context("invalid atom in operation"),
        }
    }
}

#[derive(Clone, Copy)]
struct Operation {
    op: BinOp,
    val_1: BinOpAtom,
    val_2: BinOpAtom,
}

impl Operation {
    fn eval(&self, old: Worry) -> Worry {
        let x = self.val_1.eval(old);
        let y = self.val_2.eval(old);

        match self.op {
            BinOp::Add => x + y,
            BinOp::Mul => x * y,
        }
    }
}

#[derive(Clone, Copy)]
struct Test {
    divisible_by: Worry,
    if_true: MonkeyId,
    if_false: MonkeyId,
}

impl Test {
    fn throws_to(&self, worry: Worry) -> MonkeyId {
        if worry % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Clone)]
pub struct Monkey {
    items: Vec<Worry>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn inspect<'s>(
        &'s mut self,
        stress_strategy: &'s StressStrategy,
    ) -> impl Iterator<Item = (MonkeyId, Worry)> + 's {
        self.items.drain(..).map(|worry| {
            let worry = self.operation.eval(worry);
            let worry = stress_strategy.eval(worry);
            (self.test.throws_to(worry), worry)
        })
    }
}

pub fn parse(input: &str) -> Result<Vec<Monkey>> {
    let mut lines = input.lines();
    let mut res = Vec::new();

    // "Monkey X:"
    while lines.next().is_some() {
        let items = lines
            .next()
            .context("missing starting items")?
            .trim_start()
            .strip_prefix("Starting items: ")
            .context("invalid starting items prefix")?
            .split(", ")
            .map(|item| item.parse().context("invalid stating item"))
            .collect::<Result<_>>()?;

        let operation = {
            let raw = lines
                .next()
                .context("missing operation")?
                .trim_start()
                .strip_prefix("Operation: new = ")
                .context("invalid operation prefix")?;

            let mut terms = raw.split_whitespace();
            let val_1 = BinOpAtom::from_str(terms.next().context("missing value in operation")?)?;
            let op = BinOp::from_str(terms.next().context("missing operator")?)?;
            let val_2 = BinOpAtom::from_str(terms.next().context("missing value in operation")?)?;
            Operation { op, val_1, val_2 }
        };

        let test = {
            let divisible_by = lines
                .next()
                .context("missing test")?
                .trim_start()
                .strip_prefix("Test: divisible by ")
                .context("invalid test prefix")?
                .parse()
                .context("invalid quotient in test")?;

            let if_true = lines
                .next()
                .context("missing test consequence")?
                .trim_start()
                .strip_prefix("If true: throw to monkey ")
                .context("invalid test consequence")?
                .parse()
                .context("invalid value in consequence")?;

            let if_false = lines
                .next()
                .context("missing test fallback")?
                .trim_start()
                .strip_prefix("If false: throw to monkey ")
                .context("invalid test fallback")?
                .parse()
                .context("invalid value in fallback")?;

            Test {
                divisible_by,
                if_true,
                if_false,
            }
        };

        res.push(Monkey {
            items,
            operation,
            test,
        });

        lines.next(); // empty line
    }

    Ok(res)
}

fn simulate(monkeys: &[Monkey], steps: u64, stress_strategy: StressStrategy) -> u64 {
    let mut monkeys = monkeys.to_vec();
    let mut activity = vec![0u64; monkeys.len()];
    let mut throws_buffer = Vec::new();

    for _ in 0..steps {
        for i in 0..monkeys.len() {
            throws_buffer.extend(monkeys[i].inspect(&stress_strategy));

            for (throwed_to, worry) in throws_buffer.drain(..) {
                activity[i] += 1;
                monkeys[throwed_to].items.push(worry);
            }
        }
    }

    activity.sort_unstable();
    activity.into_iter().rev().take(2).product()
}

pub fn part1(monkeys: &[Monkey]) -> u64 {
    simulate(monkeys, 20, StressStrategy::Div(3))
}

pub fn part2(monkeys: &[Monkey]) -> u64 {
    let mod_by = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product();

    simulate(monkeys, 10_000, StressStrategy::Mod(mod_by))
}
