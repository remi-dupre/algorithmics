use crate::util::arithmetic::Digits;
use crate::util::sequences::fibonacci;
use num_bigint::BigUint;

fn check(x: &BigUint) -> bool {
    let start = || {
        let mut digits = [
            true, false, false, false, false, false, false, false, false, false,
        ];

        for d in x.digits(10).take(9).map(usize::from) {
            if std::mem::replace(&mut digits[d], true) {
                return false;
            }
        }

        true
    };

    let end = || {
        let mut digits = [
            true, false, false, false, false, false, false, false, false, false,
        ];

        for d in x.digits_rev(10).take(9).map(usize::from) {
            if std::mem::replace(&mut digits[d], true) {
                return false;
            }
        }

        true
    };

    end() && start()
}

// An optimisation would be to truncate numbers with > 21 digits, but it was
// more fun to optimize the `digits` function.
pub fn solve() -> usize {
    1 + fibonacci()
        .enumerate()
        .skip(100)
        .find(|(_, x)| check(x))
        .unwrap()
        .0
}
