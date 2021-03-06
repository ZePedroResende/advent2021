use std::io::Read;
use std::str::FromStr;

/// Trait for every solution
///
/// All methods take a `dyn Read`. It can be given a file object (actual input) or a string (test cases)
///
/// Extra args are for cases where there are extra details in the question that are not part of the input itself
pub trait AoCDay {
    fn part1(&self, input: &mut dyn Read, extra_args: &[String]) -> String;
    fn part2(&self, input: &mut dyn Read, extra_args: &[String]) -> String;
    /// This method should be implemented if solving both parts together is more efficient than doing them one at a time
    fn both(&self, input: &mut dyn Read, extra_args: &[String]) -> String {
        let p1 = self.part1(input, extra_args);
        let p2 = self.part2(input, extra_args);
        format!(
            "Part1: {}\n\
            Part2: {}",
            p1, p2
        )
    }
}

pub fn load_file(_input: &mut dyn std::io::Read) -> String {
    let mut string = String::new();
    _input.read_to_string(&mut string).expect("Unable to read to string");

    string
}

pub fn load_file_bytes(_input: &mut dyn std::io::Read) -> String {
    let mut string = String::new();
    _input.read_to_string(&mut string).expect("Unable to read to string");

    string
}

pub fn parse_lines<T>(input: &str) -> impl Iterator<Item = T> + '_
    where
        T: FromStr + std::fmt::Debug,
        <T as FromStr>::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.parse().unwrap_or_else(|_| panic!("Expected to be able to parse `{:?}` as `{:?}`",
                l,
                std::any::type_name::<T>()))
        })
}


pub mod solutions {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
    pub mod day8;
    pub mod day9;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
}
pub use solutions::*;
