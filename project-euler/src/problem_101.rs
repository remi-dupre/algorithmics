use num_rational::Rational64;

pub fn tchebychev(values: &[(Rational64, Rational64)]) -> impl Fn(Rational64) -> Rational64 + '_ {
    let n = values.len();

    move |x| {
        (0..n)
            .map(|k| {
                let coef = |var: Rational64| -> Rational64 {
                    (0..n)
                        .filter(|i| *i != k)
                        .map(|i| var - values[i].0)
                        .product()
                };

                values[k].1 * (coef(x) / coef(values[k].0))
            })
            .sum()
    }
}

pub fn solve() -> Rational64 {
    let u = |n: i64| -> Rational64 { (0..=10).map(|k| (-n).pow(k)).sum::<i64>().into() };
    let vals: Vec<(Rational64, Rational64)> = (1..=10).map(|k| (k.into(), u(k))).collect();

    (1..=10)
        .map(|k| tchebychev(&vals[..k as usize])((k + 1).into()))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tchevychev() {
        let vals = &[
            (1.into(), 1.into()),
            (2.into(), 8.into()),
            (3.into(), 27.into()),
            (4.into(), 64.into()),
        ];

        assert_eq!(tchebychev(&vals[..1])(2.into()), 1.into());
        assert_eq!(tchebychev(&vals[..2])(3.into()), 15.into());
        assert_eq!(tchebychev(&vals[..3])(4.into()), 58.into());
        assert_eq!(tchebychev(&vals[..4])(5.into()), 125.into());
    }
}
