use rustc_hash::FxHashMap;

const FIELDS_REQUIRED: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn generator(input: &str) -> Result<Vec<FxHashMap<&str, &str>>, String> {
    input
        .split("\n\n")
        .map(|password| {
            password
                .split_whitespace()
                .map(
                    |field| match *field.split(':').collect::<Vec<_>>().as_slice() {
                        [key, val] => Ok((key, val)),
                        _ => Err(format!("invalid field format: `{}`", field)),
                    },
                )
                .collect()
        })
        .collect()
}

pub fn has_required_fields(password: &FxHashMap<&str, &str>) -> bool {
    FIELDS_REQUIRED
        .iter()
        .all(|&field| password.contains_key(field))
}

pub fn part_1<'a>(passwords: &[FxHashMap<&'a str, &'a str>]) -> usize {
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

pub fn part_2(passwords: &[FxHashMap<&str, &str>]) -> usize {
    passwords
        .iter()
        .filter(|&password| has_required_fields(password))
        .filter(|&password| password.iter().all(|(key, val)| is_valid_field(key, val)))
        .count()
}

// ---
// --- Tests
// ---

#[cfg(test)]
mod tests {
    use crate::day04::*;

    const EXAMPLE_1: &str = crate::lines! {
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd"
        "byr:1937 iyr:2017 cid:147 hgt:183cm"
        ""
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884"
        "hcl:#cfa07d byr:1929"
        ""
        "hcl:#ae17e1 iyr:2013"
        "eyr:2024"
        "ecl:brn pid:760753108 byr:1931"
        "hgt:179cm"
        ""
        "hcl:#cfa07d eyr:2025 pid:166559648"
        "iyr:2011 ecl:brn hgt:59in"
    };

    const EXAMPLE_2_INVALID: &str = crate::lines! {
        "eyr:1972 cid:100"
        "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"
        ""
        "iyr:2019"
        "hcl:#602927 eyr:1967 hgt:170cm"
        "ecl:grn pid:012533040 byr:1946"
        ""
        "hcl:dab227 iyr:2012"
        "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"
        ""
        "hgt:59cm ecl:zzz"
        "eyr:2038 hcl:74454a iyr:2023"
        "pid:3556412378 byr:2007"
    };

    const EXAMPLE_2_VALID: &str = crate::lines! {
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980"
        "hcl:#623a2f"
        ""
        "eyr:2029 ecl:blu cid:129 byr:1989"
        "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"
        ""
        "hcl:#888785"
        "hgt:164cm byr:2001 iyr:2015 cid:88"
        "pid:545766238 ecl:hzl"
        "eyr:2022"
        ""
        "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
    };

    #[test]
    fn test_part_1() {
        assert_eq!(2, part_1(&generator(EXAMPLE_1).unwrap()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(0, part_2(&generator(EXAMPLE_2_INVALID).unwrap()));
        assert_eq!(4, part_2(&generator(EXAMPLE_2_VALID).unwrap()));
    }
}
