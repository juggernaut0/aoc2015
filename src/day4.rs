pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        find(input.trim(), "00000").to_string()
    }

    fn solve_2(&self, input: String) -> String {
        find(input.trim(), "000000").to_string()
    }
}

fn find(input: &str, prefix: &str) -> u32 {
    for i in 0.. {
        let digest = md5_hash_hex(&format!("{input}{i}"));
        if digest.starts_with(prefix) {
            return i;
        }
    }
    unreachable!()
}

fn md5_hash_hex(s: &str) -> String {
    format!("{:x}", md5::compute(s))
}
