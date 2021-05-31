use cached::proc_macro::cached;

pub fn divisors(n: u64) -> impl Iterator<Item = u64> {
    let sqrt = (n as f64).sqrt() as u64;

    ((1..sqrt).filter(move |d| n % d == 0))
        .chain(if n % sqrt == 0 { Some(sqrt) } else { None })
        .chain(
            (1..sqrt)
                .rev()
                .filter(move |d| n % d == 0)
                .map(move |d| n / d),
        )
}

#[cached]
pub fn binomial(k: u64, n: u64) -> u64 {
    if k == 0 || k == n {
        1
    } else {
        binomial(k - 1, n - 1) + binomial(k, n - 1)
    }
}
