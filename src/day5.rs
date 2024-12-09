use std::collections::HashMap;
use std::hash::Hash;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        input.lines().filter(|s| is_nice_1(s)).count().to_string()
    }

    fn solve_2(&self, input: String) -> String {
        input.lines().filter(|s| is_nice_2(s)).count().to_string()
    }
}

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn is_nice_1(s: &str) -> bool {
    let three_vowels = s.chars().filter(|c| VOWELS.contains(c)).count() >= 3;
    let dup_char = s.as_bytes().windows(2).any(|w| w[0] == w[1]);
    let no_bad_substrings =
        !s.contains("ab") && !s.contains("cd") && !s.contains("pq") && !s.contains("xy");
    three_vowels && dup_char && no_bad_substrings
}

fn is_nice_2(s: &str) -> bool {
    let pair_groups = s
        .as_bytes()
        .windows(2)
        .enumerate()
        .map(|(a, b)| (b, a))
        .group();
    let non_overlapping_pair = pair_groups
        .into_iter()
        .any(|(_, group)| group.iter().max().unwrap() - group.iter().min().unwrap() > 1);
    let sandwich = s.as_bytes().windows(3).any(|w| w[0] == w[2]);
    non_overlapping_pair && sandwich
}

trait Group<K, V> {
    fn group(self) -> HashMap<K, Vec<V>>;
}

impl<K: Hash + Eq, V, T: Iterator<Item = (K, V)>> Group<K, V> for T {
    fn group(self) -> HashMap<K, Vec<V>> {
        let mut res: HashMap<K, Vec<V>> = HashMap::new();
        for (k, v) in self {
            res.entry(k).or_default().push(v);
        }
        res
    }
}
