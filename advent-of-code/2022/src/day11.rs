use anyhow::{bail, Context, Result};

pub type Worry = u64;
pub type MonkeyId = usize;

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
    starting_items: Vec<Worry>,
    operation: Operation,
    test: Test,
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
            starting_items: items,
            operation,
            test,
        });

        lines.next(); // empty line
    }

    Ok(res)
}

fn simulate(monkeys: &[Monkey], steps: u64, regulate_stress: impl Fn(Worry) -> Worry) -> usize {
    struct Item {
        monkey: MonkeyId,
        worry: Worry,
    }

    let mut items: Vec<_> = monkeys
        .iter()
        .enumerate()
        .flat_map(|(monkey_id, monkey)| {
            monkey.starting_items.iter().map(move |worry| Item {
                monkey: monkey_id,
                worry: *worry,
            })
        })
        .collect();

    let mut activity = vec![0; monkeys.len()];

    for _ in 0..steps {
        for (monkey_id, (monkey, activity)) in monkeys.iter().zip(&mut activity).enumerate() {
            *activity += items
                .iter_mut()
                .filter(|item| item.monkey == monkey_id)
                .map(|item| {
                    item.worry = regulate_stress(monkey.operation.eval(item.worry));
                    item.monkey = monkey.test.throws_to(item.worry);
                })
                .count();
        }
    }

    activity.sort_unstable();
    activity.into_iter().rev().take(2).product()
}

pub fn part1(monkeys: &[Monkey]) -> usize {
    simulate(monkeys, 20, |stress| stress / 3)
}

pub fn part2(monkeys: &[Monkey]) -> usize {
    /// Computed at compile time to allow compiler optimizations
    const MAX_WORRY: Worry = 2 * 3 * 5 * 7 * 9 * 11 * 13 * 17 * 19;

    for monkey in monkeys {
        // Ensure all divisibility rules will apply after stress regulation
        assert!(MAX_WORRY % monkey.test.divisible_by == 0);
    }

    simulate(monkeys, 10_000, |stress| stress % MAX_WORRY)
}
