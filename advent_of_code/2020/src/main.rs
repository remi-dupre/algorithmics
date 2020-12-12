#![feature(array_windows)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod utils;

aoc_main::main! {
    year 2020;
    day01 : generator? => part_1_array?, part_1_hashset?, part_2?;
    day02 : generator? => part_1, part_2;
    day03 : generator? => part_1, part_2;
    day04 : generator? => part_1, part_2;
    day05 : generator? => part_1?, part_2?;
    day06 : generator  => part_1_u32, part_1_slice, part_1_bitset, part_1_hashset, part_2_u32, part_2_slice;
    day07 : generator? => part_1, part_2;
    day08 : generator? => part_1, part_2, part_2_naive?;
    day09 : generator? => part_1?, part_2?;
    day10 : generator? => part_1, part_2;
    day11 : generator? => part_1, part_2;
    day12 : generator? => part_1, part_2;
}
