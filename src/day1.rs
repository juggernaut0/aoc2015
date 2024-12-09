pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        input
            .trim()
            .chars()
            .map(|c| if c == '(' { 1 } else { -1 })
            .sum::<i32>()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let i = input
            .trim()
            .chars()
            .map(|c| if c == '(' { 1 } else { -1 })
            .scan(0, |n, d| {
                *n += d;
                if *n >= 0 {
                    Some(())
                } else {
                    None
                }
            })
            .count();
        (i + 1).to_string()
    }
}
