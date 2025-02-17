#![feature(is_sorted)]

mod computer;
mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

aoc::main! {
    year 2019;
    day1 : generator => part_1, part_2;
    day2 : generator => part_1, part_2;
    day3 : generator => part_1_hashset, part_1_slice, part_2;
    day4 : generator => part_1, part_2;
    day5 : generator => part_1, part_2;
    day6 : generator => part_1, part_2;
    day7 : generator => part_1, part_2;
    day8             => part_1, part_2;
    day9 : generator => part_1, part_2;
    day10 : generator => part_1;
}
