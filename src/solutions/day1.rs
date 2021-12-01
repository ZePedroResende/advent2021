use std::io::Read;
use crate::{AoCDay, load_file, parse_lines};
pub struct Code;
impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn Read, _extra_argss: &[String]) -> String {


        let data = load_file(_input);
        let numbers: Vec<u64> = parse_lines::<u64>(&data).collect();

        numbers.windows(2).filter(|n| n[0] < n[1]).count().to_string()

    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);
        let numbers: Vec<u64> = parse_lines::<u64>(&data).collect();

        numbers.windows(4).filter(|n| n[0]< n[3]).count().to_string()
    }
}
