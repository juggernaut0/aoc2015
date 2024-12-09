#![warn(clippy::pedantic)]

use aoc::Solution;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

const DAYS: [&dyn Solution; 25] = [
    &day1::Solution,
    &day2::Solution,
    &day3::Solution,
    &day4::Solution,
    &day5::Solution,
    &day6::Solution,
    &day7::Solution,
    &day8::Solution,
    &day9::Solution,
    &day10::Solution,
    &day11::Solution,
    &day12::Solution,
    &day13::Solution,
    &day14::Solution,
    &day15::Solution,
    &day16::Solution,
    &day17::Solution,
    &day18::Solution,
    &day19::Solution,
    &day20::Solution,
    &day21::Solution,
    &day22::Solution,
    &day23::Solution,
    &day24::Solution,
    &day25::Solution,
];

fn main() {
    aoc::run("2015", DAYS);
}

#[cfg(test)]
mod answers {
    aoc::generate_answer_tests!(crate::DAYS);
}
