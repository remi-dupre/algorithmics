type Error = Box<dyn std::error::Error>;

pub const VAL_BOUND: u64 = 1_000_000_007;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Update {
    X,
    F,
    O,
    Dot,
}

impl Update {
    pub fn try_from_byte(c: u8) -> Result<Self, Error> {
        match c {
            b'X' => Ok(Self::X),
            b'F' => Ok(Self::F),
            b'O' => Ok(Self::O),
            b'.' => Ok(Self::Dot),
            _ => Err(format!("Invalid update char '{}'", c as char).into()),
        }
    }
}

// Compute the last index of value `u` in text[..i] for all i.
fn compute_last_index_for(u: Update, text: &[Update]) -> Vec<Option<usize>> {
    text.iter()
        .enumerate()
        .map({
            let mut prev = None;

            move |(i, curr)| {
                if *curr == u {
                    prev.replace(i)
                } else {
                    prev
                }
            }
        })
        .collect()
}

// Compute the first index of value `u` in text[..i] for all i.
fn compute_first_index_for(u: Update, text: &[Update]) -> Vec<Option<usize>> {
    text.iter()
        .enumerate()
        .map({
            let mut prev = None;

            move |(i, curr)| {
                if prev.is_none() && *curr == u {
                    prev.replace(i)
                } else {
                    prev
                }
            }
        })
        .collect()
}

// Compute the expanded length of text[..=i] for all i.
fn compute_actual_length(text: &[Update]) -> Vec<usize> {
    text.iter()
        .map({
            let mut val = 0;

            move |curr| {
                match curr {
                    Update::Dot => val *= 2,
                    _ => val += 1,
                }

                val %= VAL_BOUND as usize;
                val
            }
        })
        .collect()
}

// Compute the first of either X or O in text[..=i] for all i.
fn compute_first_xo(text: &[Update]) -> Vec<Option<Update>> {
    text.iter()
        .map({
            let mut first = None;

            move |curr| {
                if first.is_none() && [Update::X, Update::O].contains(curr) {
                    first = Some(*curr);
                }

                first
            }
        })
        .collect()
}

// Compute the last of either X or O in text[..=i] for all i.
fn compute_last_xo(text: &[Update]) -> Vec<Option<Update>> {
    text.iter()
        .map({
            let mut last = None;

            move |curr| {
                if [Update::X, Update::O].contains(curr) {
                    last = Some(*curr);
                }

                last
            }
        })
        .collect()
}

pub fn minimum_switches(text: &[Update]) -> Vec<u64> {
    let last_o = compute_last_index_for(Update::O, text);
    let last_x = compute_last_index_for(Update::X, text);

    text.iter()
        .enumerate()
        .map({
            let mut val = 0;

            move |(i, curr)| {
                match curr {
                    Update::F => {}
                    Update::Dot => {
                        if val > 0 {
                            val = 2 * val + 1
                        }
                    }
                    Update::X => {
                        if last_o[i] > last_x[i] {
                            val += 1
                        }
                    }
                    Update::O => {
                        if last_x[i] > last_o[i] {
                            val += 1
                        }
                    }
                }

                val %= VAL_BOUND;
                val
            }
        })
        .collect()
}

fn sum_switches_for_suffixes(text: &[Update]) -> Vec<u64> {
    let length = compute_actual_length(text);
    let last_o = compute_last_index_for(Update::O, text);
    let last_x = compute_last_index_for(Update::X, text);
    let first_xo = compute_first_xo(text);
    let for_complete = minimum_switches(text);

    text.iter()
        .enumerate()
        .map({
            let mut val = 0;

            move |(i, curr)| {
                match curr {
                    Update::F => {}
                    Update::Dot => {
                        if val > 0 {
                            // As val > 0, there is at least an X and an O
                            let first_xo = first_xo[i].unwrap();

                            // If current prefix ends with O and starts with X, we must pay and
                            // extra switch for each long enough suffix.
                            let cost_joining = {
                                if first_xo == Update::X && last_o[i] > last_x[i] {
                                    length[last_o[i].unwrap()] as u64
                                } else if first_xo == Update::O && last_x[i] > last_o[i] {
                                    length[last_x[i].unwrap()] as u64
                                } else {
                                    0
                                }
                            };

                            val =
                                2 * val + length[i - 1] as u64 * for_complete[i - 1] + cost_joining
                        }
                    }
                    Update::X => {
                        if last_o[i] > last_x[i] {
                            val += last_o[i].map(|d| length[d] as u64).unwrap_or(0)
                        }
                    }
                    Update::O => {
                        if last_x[i] > last_o[i] {
                            val += last_x[i].map(|d| length[d] as u64).unwrap_or(0)
                        }
                    }
                }

                val %= VAL_BOUND;
                val as u64
            }
        })
        .collect()
}

fn sum_switches_for_prefixes(text: &[Update]) -> Vec<u64> {
    let length = compute_actual_length(text);
    let first_o = compute_first_index_for(Update::O, text);
    let first_x = compute_first_index_for(Update::X, text);
    let last_xo = compute_last_xo(text);
    let for_complete = minimum_switches(text);

    text.iter()
        .enumerate()
        .map({
            let mut val = 0;

            move |(i, curr)| {
                match curr {
                    Update::Dot => {
                        if val > 0 {
                            let last_xo = last_xo[i].unwrap();

                            let cost_joining = {
                                if last_xo == Update::X && first_o[i] < first_x[i] {
                                    (1 + length[i - 1] - length[first_o[i].unwrap()]) as u64
                                } else if last_xo == Update::O && first_x[i] < first_o[i] {
                                    (1 + length[i - 1] - length[first_x[i].unwrap()]) as u64
                                } else {
                                    0
                                }
                            };

                            val =
                                2 * val + length[i - 1] as u64 * for_complete[i - 1] + cost_joining
                        }
                    }
                    _ => val += for_complete[i],
                }

                val %= VAL_BOUND;
                val as u64
            }
        })
        .collect()
}

pub fn sum_switches(text: &[Update]) -> u64 {
    let length = compute_actual_length(text);
    let cost_suffix = sum_switches_for_suffixes(text);
    let cost_prefix = sum_switches_for_prefixes(text);
    let last_xo = compute_last_xo(text);
    let last_o = compute_last_index_for(Update::O, text);
    let last_x = compute_last_index_for(Update::X, text);
    let first_o = compute_first_index_for(Update::O, text);
    let first_x = compute_first_index_for(Update::X, text);

    text.iter()
        .enumerate()
        .map(|(i, curr)| match curr {
            Update::Dot if i > 0 => {
                let cost_join = {
                    if let Some(last_xo) = last_xo[i] {
                        if last_xo == Update::X && first_o[i] < first_x[i] && first_o[i].is_some() {
                            let options_left = length[last_x[i].unwrap()];
                            let options_right = 1 + length[i - 1] - length[first_o[i].unwrap()];
                            options_left * options_right
                        } else if first_x[i] < first_o[i] && first_x[i].is_some() {
                            let options_left = length[last_o[i].unwrap()];
                            let options_right = 1 + length[i - 1] - length[first_x[i].unwrap()];
                            options_left * options_right
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                };

                (cost_suffix[i - 1] + cost_prefix[i - 1]) * length[i - 1] as u64 + cost_join as u64
            }
            _ => cost_suffix[i],
        })
        .sum::<u64>()
        % VAL_BOUND
}
