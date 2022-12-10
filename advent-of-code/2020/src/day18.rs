//! Module level documentation.

use Operator::*;

#[derive(Clone, Copy)]
pub enum Operator {
    Add,
    Mul,
}

pub struct Term {
    op: Operator,
    val: Val,
}

pub enum Val {
    Const(u64),
    Expr(Vec<Term>),
}

pub fn generator(input: &str) -> Result<Vec<Val>, String> {
    fn parse(mut current: &[u8]) -> Result<(Val, &[u8]), String> {
        let mut res = Vec::new();
        let mut curr_operator = Add;

        while let [head, tail @ ..] = current {
            current = tail;

            match *head {
                b' ' => {}
                b'+' => curr_operator = Add,
                b'*' => curr_operator = Mul,
                b'0'..=b'9' => res.push(Term {
                    op: curr_operator,
                    val: Val::Const((head - b'0').into()),
                }),
                b'(' => {
                    let (expr, remain) = parse(&current)?;
                    res.push(Term {
                        op: curr_operator,
                        val: expr,
                    });

                    current = remain;
                    continue;
                }
                b')' => break,
                other => return Err(format!("unknown symbol: `{}`", char::from(other))),
            }
        }

        Ok((Val::Expr(res), current))
    }

    input
        .lines()
        .map(|line| Ok(parse(line.as_bytes())?.0))
        .collect()
}

pub fn part_1(formulas: &[Val]) -> u64 {
    formulas.iter().map(|f| f.eval_pre_left()).sum()
}

pub fn part_2(formulas: &[Val]) -> u64 {
    formulas.iter().map(|f| f.eval_pre_sum()).sum()
}

// ---
// --- Structs
// ---

impl Term {
    fn apply_to(&self, value: u64, eval: impl Fn(&Val) -> u64) -> u64 {
        match self.op {
            Operator::Add => value + eval(&self.val),
            Operator::Mul => value * eval(&self.val),
        }
    }
}

impl Val {
    fn eval_pre_left(&self) -> u64 {
        match self {
            Self::Const(x) => *x,
            Self::Expr(ops) => ops
                .iter()
                .fold(0, |acc, op| op.apply_to(acc, Self::eval_pre_left)),
        }
    }

    fn eval_pre_sum(&self) -> u64 {
        match self {
            Self::Const(x) => *x,
            Self::Expr(ops) => {
                let mut ops = ops.iter().peekable();

                std::iter::from_fn(move || {
                    let mut sum = ops.next()?.val.eval_pre_sum();

                    while matches!(ops.peek(), Some(Term { op: Add, .. })) {
                        sum = ops.next().unwrap().apply_to(sum, Self::eval_pre_sum);
                    }

                    Some(sum)
                })
                .product()
            }
        }
    }
}

// ---
// --- Tests
// ---

#[cfg(test)]
mod tests {
    use crate::day18::*;

    const EXAMPLE_1: &str = "1 + (2 * 3) + (4 * (5 + 6))";
    const EXAMPLE_2: &str = "2 * 3 + (4 * 5)";
    const EXAMPLE_3: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    const EXAMPLE_4: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    const EXAMPLE_5: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn test_part_1() {
        assert_eq!(51, part_1(&generator(EXAMPLE_1).unwrap()));
        assert_eq!(26, part_1(&generator(EXAMPLE_2).unwrap()));
        assert_eq!(437, part_1(&generator(EXAMPLE_3).unwrap()));
        assert_eq!(12240, part_1(&generator(EXAMPLE_4).unwrap()));
        assert_eq!(13632, part_1(&generator(EXAMPLE_5).unwrap()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(51, part_2(&generator(EXAMPLE_1).unwrap()));
        assert_eq!(46, part_2(&generator(EXAMPLE_2).unwrap()));
        assert_eq!(1445, part_2(&generator(EXAMPLE_3).unwrap()));
        assert_eq!(669060, part_2(&generator(EXAMPLE_4).unwrap()));
        assert_eq!(23340, part_2(&generator(EXAMPLE_5).unwrap()));
    }
}
