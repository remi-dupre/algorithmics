#![feature(array_chunks)]
#![feature(array_windows)]
#![feature(byte_slice_trim_ascii)]
#![feature(ptr_sub_ptr)]

pub mod util;

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
mod day13;
mod day14;
mod day15;

aoc_main::main! {
    year 2022;
    day01: parse? => part1, part2;
    day02: parse? => part1, part2;
    day03: parse? => part1, part2?;
    day04: parse? => part1, part2;
    day05: parse? => part1?, part2?;
    day06         => part1?, part1_hashmap?, part2?, part2_hashmap?;
    day07: parse? => part1?, part2?;
    day08: parse? => part1, part2?;
    day09: parse? => part1, part2;
    day10: parse? => part1, part2;
    day11: parse? => part1, part2;
    day12: parse  => part1?, part2?;
    day13: parse? => part1, part2;
    day14: parse? => part1, part2;
    day15: parse? => part1, part2?;
}
