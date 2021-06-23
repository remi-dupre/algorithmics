pub fn solve() -> u64 {
    (1..1000)
        .max_by_key(|p| {
            (1..=*p)
                .flat_map(|a| (1..(1000 - a)).map(move |b| (a, b)))
                .filter(|(a, b)| {
                    let c = p - a - b;
                    a * a + b * b == c * c
                })
                .count()
        })
        .unwrap()
}
