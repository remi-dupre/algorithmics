use std::iter;

pub fn generator(input: &str) -> (u64, u64) {
    let mut passwords = input.split('-');
    (
        passwords.next().unwrap().parse().unwrap(),
        passwords.next().unwrap().parse().unwrap(),
    )
}

fn groups<T: Eq>(mut vals: impl Iterator<Item = T>) -> impl Iterator<Item = usize> {
    let mut item: Option<T> = vals.next();
    let mut count = 1;

    iter::from_fn(move || {
        if item.is_some() {
            loop {
                let new = vals.next();

                if new == item {
                    count += 1;
                } else {
                    item = new;
                    break;
                }
            }

            let res = count;
            count = 1;
            Some(res)
        } else {
            None
        }
    })
}

pub fn part_1(input: &(u64, u64)) -> usize {
    (input.0..=input.1)
        .filter(|candidate| {
            let as_str = candidate.to_string();
            let has_pair = groups(as_str.bytes()).any(|group_size| group_size >= 2);
            as_str.as_bytes().is_sorted() && has_pair
        })
        .count()
}

pub fn part_2(input: &(u64, u64)) -> usize {
    (input.0..=input.1)
        .filter(|candidate| {
            let as_str = candidate.to_string();
            let has_pair = groups(as_str.bytes()).any(|group_size| group_size == 2);
            as_str.bytes().is_sorted() && has_pair
        })
        .count()
}
