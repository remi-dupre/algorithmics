#![feature(array_windows)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

aoc::main! {
    year 2020;
    day1 : generator => part_1_array, part_1_hashset, part_2;
    day2 : generator => part_1, part_2;
    day3 : generator => part_1, part_2;
    day4 : generator => part_1, part_2;
    day5 : generator => part_1, part_2;
    day6 : generator => part_1_slice, part_1_bitset, part_1_hashset, part_2;
    day7 : generator => part_1, part_2;
}
