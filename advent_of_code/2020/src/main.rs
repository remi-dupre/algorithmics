#![feature(array_windows)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

aoc::main! {
    year 2020;
    day1 : generator => part_1_array, part_1_hashset, part_2;
    day2 : generator => part_1, part_2;
    day3 : generator => part_1, part_2;
    day4 : generator => part_1, part_2;
    day5 : generator => part_1, part_2;
}
