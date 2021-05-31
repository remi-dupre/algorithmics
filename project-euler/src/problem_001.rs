const MAX: u32 = 1_000;

pub fn solve() -> u32 {
    (0..MAX).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}
