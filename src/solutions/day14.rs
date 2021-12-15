use crate::{load_file, AoCDay};
use std::collections::HashMap;
pub struct Code;
impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);
        let (template, rules_str) = data.split_once("\n\n").unwrap();
        let rules: HashMap<(char, char), char> = rules_str.lines().fold(
            HashMap::new(),
            |mut acc: HashMap<(char, char), char>, l: &str| {
                let (element_a, element_b) = l.rsplit_once(" -> ").unwrap();
                let mut chars = element_a.chars();
                acc.insert(
                    (chars.next().unwrap(), chars.next().unwrap()),
                    element_b.chars().next().unwrap(),
                );
                acc
            },
        );

        let pairs: Vec<(char, char)> = template
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .map(|c| (c[0], c[1]))
            .collect();

        let mut counter_chars: HashMap<char, u64> = pairs.iter().fold(
            HashMap::new(),
            |mut acc: HashMap<char, u64>, (a, b): &(char, char)| {
                *acc.entry(*a).or_insert(0) += 1 as u64;
                *acc.entry(*b).or_insert(0) += 1 as u64;
                acc
            },
        );

        let mut counter_pairs: HashMap<(char, char), u64> = pairs.iter().fold(
            HashMap::new(),
            |mut acc: HashMap<(char, char), u64>, pair: &(char, char)| {
                *acc.entry(pair.clone()).or_insert(0) += 1 as u64;
                acc
            },
        );

        for _ in 0..10 {
            for (pair, times) in counter_pairs.clone().iter() {
                let value: &char = rules.get(pair).unwrap();
                *counter_pairs.entry((pair.0, *value)).or_insert(0) += *times as u64;
                *counter_pairs.entry((*value, pair.1)).or_insert(0) += *times as u64;
                *counter_pairs.entry(*pair).or_insert(0) -= *times as u64;
                *counter_chars.entry(*value).or_insert(0) += *times as u64;
            }
        }

        let values: Vec<&u64> = counter_chars.values().collect();
        let max = values.iter().max().unwrap();
        let min = values.iter().min().unwrap();

        let number = **max - **min;

        number.to_string()
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);
        let (template, rules_str) = data.split_once("\n\n").unwrap();
        let rules: HashMap<(char, char), char> = rules_str.lines().fold(
            HashMap::new(),
            |mut acc: HashMap<(char, char), char>, l: &str| {
                let (element_a, element_b) = l.rsplit_once(" -> ").unwrap();
                let mut chars = element_a.chars();
                acc.insert(
                    (chars.next().unwrap(), chars.next().unwrap()),
                    element_b.chars().next().unwrap(),
                );
                acc
            },
        );

        let pairs: Vec<(char, char)> = template
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .map(|c| (c[0], c[1]))
            .collect();

        let mut counter_chars: HashMap<char, u64> = pairs.iter().fold(
            HashMap::new(),
            |mut acc: HashMap<char, u64>, (a, b): &(char, char)| {
                *acc.entry(*a).or_insert(0) += 1 as u64;
                *acc.entry(*b).or_insert(0) += 1 as u64;
                acc
            },
        );

        let mut counter_pairs: HashMap<(char, char), u64> = pairs.iter().fold(
            HashMap::new(),
            |mut acc: HashMap<(char, char), u64>, pair: &(char, char)| {
                *acc.entry(pair.clone()).or_insert(0) += 1 as u64;
                acc
            },
        );

        for _ in 0..40 {
            for (pair, times) in counter_pairs.clone().iter() {
                let value: &char = rules.get(pair).unwrap();
                *counter_pairs.entry((pair.0, *value)).or_insert(0) += *times as u64;
                *counter_pairs.entry((*value, pair.1)).or_insert(0) += *times as u64;
                *counter_pairs.entry(*pair).or_insert(0) -= *times as u64;
                *counter_chars.entry(*value).or_insert(0) += *times as u64;
            }
        }

        let values = counter_chars.values();
        let max = values.clone().max().unwrap();
        let min = values.min().unwrap();

        let number = *max - *min;

        number.to_string()
    }
}
