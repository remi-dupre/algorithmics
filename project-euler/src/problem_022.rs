pub fn solve() -> u64 {
    let mut names: Vec<_> = include_str!("data/problem_022.txt")
        .split(',')
        .map(|name| name.trim().trim_matches('"'))
        .collect();
    names.sort_unstable();
    names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            (i as u64 + 1) * name.bytes().map(|b| 1 + u64::from(b - b'A')).sum::<u64>()
        })
        .sum()
}
