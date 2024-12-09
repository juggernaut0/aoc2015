use std::str::FromStr;
use aoc::{parse_lines, Point};
use once_cell::sync::Lazy;
use regex::Regex;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let mut grid = [[false; 1000]; 1000];
        for instr in parse_lines::<Instruction>(&input) {
            instr.apply(&mut grid);
        }
        // for some reason this stack overflows:
        //grid.into_iter().flat_map(|it| it).filter(|it| *it).count().to_string()
        grid.into_iter().map(|it| it.into_iter().filter(|it| *it).count()).sum::<usize>().to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let mut grid = [[0; 1000]; 1000];
        for instr in parse_lines::<Instruction>(&input) {
            instr.apply_2(&mut grid);
        }
        grid.into_iter().map(|it| it.into_iter().sum(|it| *it).count()).sum::<usize>().to_string()
    }
}

struct Instruction {
    mode: Mode,
    start: Point<usize>,
    end: Point<usize>,
}

enum Mode {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Instruction {
    fn apply(&self, grid: &mut [[bool; 1000]; 1000]) {
        for y in self.start.1..=self.end.1 {
            for x in self.start.0..=self.end.0 {
                grid[x][y] = match self.mode {
                    Mode::TurnOn => true,
                    Mode::TurnOff => false,
                    Mode::Toggle => !grid[x][y],
                }
            }
        }
    }

    fn apply_2(&self, grid: &mut [[u32; 1000]; 1000]) {
        for y in self.start.1..=self.end.1 {
            for x in self.start.0..=self.end.0 {
                match self.mode {
                    Mode::TurnOn => grid[x][y] += 1,
                    Mode::TurnOff => grid[x][y].saturating_sub(1),
                    Mode::Toggle => grid[x][y] += 2,
                }
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static INPUT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap());
        let groups = INPUT_PATTERN.captures(s).unwrap();
        let mode = match &groups[1] {
            "turn on" => Mode::TurnOn,
            "turn off" => Mode::TurnOff,
            "toggle" => Mode::Toggle,
            _ => panic!(),
        };
        let start = Point(groups[2].parse().unwrap(), groups[3].parse().unwrap());
        let end = Point(groups[4].parse().unwrap(), groups[5].parse().unwrap());
        Ok(Instruction { mode, start, end })
    }
}
