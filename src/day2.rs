use aoc::parse_lines_with;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        parse_lines_with(&input, parse_line)
            .map(|[w, h, l]| w * h + 2 * l * w + 2 * w * h + 2 * h * l)
            .sum::<u32>()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        parse_lines_with(&input, parse_line)
            .map(|[w, h, l]| w * h * l + 2 * w + 2 * h)
            .sum::<u32>()
            .to_string()
    }
}

fn parse_line(line: &str) -> [u32; 3] {
    let mut dims = [0; 3];
    let mut iter = line.split('x').map(|it| it.parse().unwrap());
    dims[0] = iter.next().unwrap();
    dims[1] = iter.next().unwrap();
    dims[2] = iter.next().unwrap();
    dims.sort_unstable();
    dims
}
