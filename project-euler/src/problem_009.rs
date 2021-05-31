const TARGET: u64 = 1_000;

pub fn solve() -> u64 {
    for a in 1..TARGET {
        for b in 1..(TARGET - a) {
            let c = TARGET - a - b;

            if a.pow(2) + b.pow(2) == c.pow(2) {
                return a * b * c;
            }
        }
    }

    unreachable!()
}
