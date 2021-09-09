use crate::util::arithmetic::Digits;

fn fractional_digit(mut n: u32) -> u8 {
    fn nb_digits_until_bigger(digits: u32) -> u32 {
        (10u32.pow(digits) - 10u32.pow(digits - 1)) * digits
    }

    let mut digits = 1;

    while nb_digits_until_bigger(digits) < n {
        n -= nb_digits_until_bigger(digits);
        digits += 1;
    }

    let num = 10u32.pow(digits - 1) + (n - 1) / digits;
    let digit = num.digits(10).nth(((n - 1) % digits) as usize).unwrap();
    digit
}

pub fn solve() -> u64 {
    (0..=6)
        .map(|k| fractional_digit(10u32.pow(k)))
        .map(u64::from)
        .product()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::arithmetic::NbDigits;

    #[test]
    fn test_fractional_digit() {
        fn fractional_digit_slow(mut n: usize) -> u8 {
            let mut x = 0u64;

            loop {
                x += 1;

                if n <= usize::from(x.nb_digits(10)) {
                    return x.digits(10).nth(n - 1).unwrap();
                }

                n -= usize::from(x.nb_digits(10));
            }
        }

        assert_eq!(fractional_digit(1), 1);
        assert_eq!(fractional_digit(6), 6);
        assert_eq!(fractional_digit(10), 1);
        assert_eq!(fractional_digit(11), 0);
        assert_eq!(fractional_digit(12), 1);
        assert_eq!(fractional_digit(13), 1);
        assert_eq!(fractional_digit(14), 1);
        assert_eq!(fractional_digit(15), 2);

        for d in 1..1000 {
            println!("({}) d = {}", fractional_digit(d), d);
            assert_eq!(fractional_digit(d), fractional_digit_slow(d as usize),);
        }
    }
}
