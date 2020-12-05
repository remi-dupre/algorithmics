use std::collections::HashMap;

const FIELDS_REQUIRED: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn generator(input: &str) -> Vec<HashMap<&str, &str>> {
    input
        .split("\n\n")
        .map(|password| {
            password
                .split_whitespace()
                .map(
                    |field| match field.split(':').collect::<Vec<_>>().as_slice() {
                        [key, val] => (*key, *val),
                        _ => panic!("invalid field format: `{}`", field),
                    },
                )
                .collect()
        })
        .collect()
}

pub fn has_required_fields(password: &HashMap<&str, &str>) -> bool {
    FIELDS_REQUIRED
        .iter()
        .all(|&field| password.contains_key(field))
}

pub fn part_1<'a>(passwords: &[HashMap<&'a str, &'a str>]) -> usize {
    passwords
        .iter()
        .filter(|&password| has_required_fields(password))
        .count()
}

pub fn is_valid_field(key: &str, val: &str) -> bool {
    fn is_in_range(val: &str, start: u16, end: u16) -> bool {
        val.parse()
            .ok()
            .map(|as_num| (start..=end).contains(&as_num))
            .unwrap_or(false)
    }

    match key {
        "byr" => is_in_range(val, 1920, 2002),
        "iyr" => is_in_range(val, 2010, 2020),
        "eyr" => is_in_range(val, 2020, 2030),
        "hgt" => {
            val.len() > 2
                && match &val[val.len() - 2..] {
                    "cm" => is_in_range(&val[..val.len() - 2], 150, 193),
                    "in" => is_in_range(&val[..val.len() - 2], 59, 76),
                    _ => false,
                }
        }
        "hcl" => {
            val.len() == 7
                && val.bytes().next() == Some(b'#')
                && val.bytes().skip(1).all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => matches!(val, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
        "pid" => val.len() == 9 && val.bytes().all(|c| c.is_ascii_digit()),
        "cid" => true,
        _ => panic!("unknown key `{}`", key),
    }
}

pub fn part_2(passwords: &[HashMap<&str, &str>]) -> usize {
    passwords
        .iter()
        .filter(|&password| has_required_fields(password))
        .filter(|&password| password.iter().all(|(key, val)| is_valid_field(key, val)))
        .count()
}
