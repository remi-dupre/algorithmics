// const STEP: u128 = 1504170715041707;
// const LIMIT: u128 = 4503599627370517;
//
// fn u(n: u128) -> u128 {
//     (n * STEP) % LIMIT
// }
//
// fn next_coin(prev: u128) -> Option<u128> {
//     fn search_next_coin(step: u128, limit: u128, max: u128) -> Option<u128> {
//         if limit == 1 {
//             None
//         } else if step < limit {
//             Some(step)
//         } else {
//             let shift = max - (max / step) * step;
//             search_next_coin(shift, limit, step).map(|c| limit - c)
//         }
//     }
//
//     search_next_coin(STEP, prev, LIMIT)
// }
//
// pub fn solve() -> u128 {
//     let mut limit = 4503599627370517;
//     let mut step = 1504170715041707;
//     let mut max = LIMIT;
//     let mut n = 1;
//
//     std::iter::from_fn(move || {
//         if max > 1 {
//             while u(n) >= max {
//                 println!("n = {}", n);
//                 // We need to walk at least LIMIT - u(n) until we walk past 0 again.
//                 // This means ceil((LIMIT - u(n)) / STEP) steps.
//                 n += (LIMIT - u(n) + STEP - 1) / STEP;
//             }
//
//             max = u(n);
//             Some(u(n))
//         } else {
//             None
//         }
//     })
//     .inspect(|x| println!("{}", x))
//     .sum()
// }

pub fn solve() -> &'static str {
    "todo"
}
