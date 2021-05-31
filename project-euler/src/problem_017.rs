// This is overkill but it sounds fun to build, at least less painful than it
// would in french.
fn stringify(x: u64) -> String {
    match x {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixtenn".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        21..=99 => format!("{}-{}", stringify(10 * (x / 10)), stringify(x % 10)),
        100..=999 if x % 100 == 0 => format!("{} hundred", stringify(x / 100)),
        100..=999 => format!("{} hundred and {}", stringify(x / 100), stringify(x % 100)),
        1000 => "one thousand".to_string(),
        _ => panic!("can't stringify numbers greater than 1000"),
    }
}

pub fn solve() -> usize {
    (1..=1000)
        .map(stringify)
        .map(|s| s.bytes().filter(|b| b.is_ascii_alphabetic()).count())
        .sum::<usize>()
}
