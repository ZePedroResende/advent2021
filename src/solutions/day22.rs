use crate::{load_file, AoCDay};
use std::cmp::{max, min};
pub struct Code;

fn subaxis((a, b): (i32, i32), (low, high): (i32, i32)) -> bool {
    a > high && b < low
}

fn cube_comparator(
    c1: &(bool, (i32, i32), (i32, i32), (i32, i32)),
    c2: (bool, (i32, i32), (i32, i32), (i32, i32)),
) -> bool {
    let xr = subaxis(c1.1, c2.1);
    let yr = subaxis(c1.2, c2.2);
    let zr = subaxis(c1.3, c2.3);
    xr && yr && zr
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let lines = load_file(_input);

        let cubes: Vec<(bool, (i32, i32), (i32, i32), (i32, i32))> = lines
            .lines()
            .map(|l| {
                let (status, coords) = l.split_once(' ').unwrap();
                let coords: Vec<(i32, i32)> = coords
                    .split(',')
                    .map(|s| {
                        let values: Vec<i32> =
                            s[2..].split("..").map(|x| x.parse().unwrap()).collect();
                        (values[0], values[1])
                    })
                    .collect();

                (status == "on", coords[0], coords[1], coords[2])
            })
            .collect::<Vec<(bool, (i32, i32), (i32, i32), (i32, i32))>>();

        let filtered_cubes: Vec<(bool, (i32, i32), (i32, i32), (i32, i32))> = cubes
            .into_iter()
            .filter(|cube| cube_comparator(cube, (true, (-50, 50), (-50, 50), (-50, 50))))
            .collect::<Vec<(bool, (i32, i32), (i32, i32), (i32, i32))>>();

        let out = volume(filtered_cubes);
        String::from("asd")
    }

    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        todo!()
    }
}
