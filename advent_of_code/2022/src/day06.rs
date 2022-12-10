use std::collections::{hash_map, HashMap};

use fxhash::FxBuildHasher;

pub fn find_marker_hashmap(marker_size: usize, signal: &[u8]) -> Option<usize> {
    let mut count_for_win: HashMap<u8, usize, FxBuildHasher> = Default::default();

    for b in &signal[..marker_size] {
        *count_for_win.entry(*b).or_default() += 1;
    }

    if count_for_win.len() == marker_size {
        return Some(0);
    }

    for (i, win) in signal.windows(marker_size + 1).enumerate() {
        let removed = *win.first().unwrap();
        let added = *win.last().unwrap();
        *count_for_win.entry(added).or_default() += 1;

        match count_for_win.entry(removed) {
            hash_map::Entry::Occupied(mut e) if *e.get() > 1 => *e.get_mut() -= 1,
            hash_map::Entry::Occupied(e) if *e.get() == 1 => {
                count_for_win.remove(&removed);
            }
            _ => unreachable!(),
        };

        if count_for_win.len() == marker_size {
            return Some(i + 1);
        }
    }

    None
}

struct Counter {
    for_char: [usize; u8::MAX as usize],
    non_zero: usize,
}

impl Counter {
    pub fn get(&self) -> usize {
        self.non_zero
    }

    pub fn insert(&mut self, val: u8) {
        let val: usize = val.into();
        let for_char = &mut self.for_char[val];

        if *for_char == 0 {
            self.non_zero += 1;
        }

        *for_char += 1;
    }

    pub fn remove(&mut self, val: u8) {
        let val: usize = val.into();
        let for_char = &mut self.for_char[val];

        if *for_char == 1 {
            self.non_zero -= 1;
        }

        *for_char -= 1;
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            for_char: [0; u8::MAX as usize],
            non_zero: 0,
        }
    }
}

pub fn find_marker(marker_size: usize, signal: &[u8]) -> Option<usize> {
    let mut counter = Counter::default();

    for b in &signal[..marker_size] {
        counter.insert(*b);
    }

    if counter.get() == marker_size {
        return Some(0);
    }

    for (i, win) in signal.windows(marker_size + 1).enumerate() {
        let removed = *win.first().unwrap();
        let inserted = *win.last().unwrap();
        counter.insert(inserted);
        counter.remove(removed);

        if counter.get() == marker_size {
            return Some(i + 1);
        }
    }

    None
}

pub fn part1_hashmap(signal: &str) -> Option<usize> {
    Some(find_marker_hashmap(4, signal.as_bytes())? + 4)
}

pub fn part2_hashmap(signal: &str) -> Option<usize> {
    Some(find_marker_hashmap(14, signal.as_bytes())? + 14)
}

pub fn part1(signal: &str) -> Option<usize> {
    Some(find_marker(4, signal.as_bytes())? + 4)
}

pub fn part2(signal: &str) -> Option<usize> {
    Some(find_marker(14, signal.as_bytes())? + 14)
}
