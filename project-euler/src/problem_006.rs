const MAX: u64 = 100;

pub fn solve() -> u64 {
    let square_sum = (1..=MAX).map(|x| x * x).sum::<u64>();
    let sum_square = (1..=MAX).sum::<u64>().pow(2);
    sum_square - square_sum
}
