use aoc::{Dir, Point};
use std::collections::HashSet;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let mut res = HashSet::new();
        let mut current = Point::zero();
        res.insert(current);
        for c in input.trim().chars() {
            let dir = match c {
                '^' => Dir::N,
                '>' => Dir::E,
                'v' => Dir::S,
                '<' => Dir::W,
                _ => unreachable!(),
            };
            current = current + dir.diff();
            res.insert(current);
        }
        res.len().to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let mut res = HashSet::new();
        let mut a = Point::zero();
        let mut b = Point::zero();
        res.insert(a);
        for (i, c) in input.trim().chars().enumerate() {
            let dir = match c {
                '^' => Dir::N,
                '>' => Dir::E,
                'v' => Dir::S,
                '<' => Dir::W,
                _ => unreachable!(),
            };
            if i % 2 == 0 {
                a = a + dir.diff();
                res.insert(a);
            } else {
                b = b + dir.diff();
                res.insert(b);
            }
        }
        res.len().to_string()
    }
}
