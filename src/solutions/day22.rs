use crate::{load_file, AoCDay};
use std::cmp::{max, min};
use std::collections::HashMap;
pub struct Code;

fn reboot(
    cubes: &mut HashMap<((i64, i64), (i64, i64), (i64, i64)), i64>,
    coords: (bool, (i64, i64), (i64, i64), (i64, i64)),
) -> HashMap<((i64, i64), (i64, i64), (i64, i64)), i64> {
    let (status, (x0, x1), (y0, y1), (z0, z1)) = coords;

    let status_i = if status { 1 } else { -1 };

    for (((kx0, kx1), (ky0, ky1), (kz0, kz1)), value) in cubes.clone().into_iter() {
        let ix0 = max(x0, kx0);
        let ix1 = min(x1, kx1);

        let iy0 = max(y0, ky0);
        let iy1 = min(y1, ky1);

        let iz0 = max(z0, kz0);
        let iz1 = min(z1, kz1);

        if ix0 <= ix1 && iy0 <= iy1 && iz0 <= iz1 {
            *cubes
                .entry(((ix0, ix1), (iy0, iy1), (iz0, iz1)))
                .or_insert(0) -= value;
        }
    }

    if status_i > 0 {
        *cubes.entry(((x0, x1), (y0, y1), (z0, z1))).or_insert(0) += status_i;
    }

    cubes.clone()
}

fn subaxis((a, b): (i64, i64), (low, high): (i64, i64)) -> bool {
    a > high && b < low
}

fn cube_comparator(
    c1: &(bool, (i64, i64), (i64, i64), (i64, i64)),
    c2: (bool, (i64, i64), (i64, i64), (i64, i64)),
) -> bool {
    let xr = subaxis(c1.1, c2.1);
    let yr = subaxis(c1.2, c2.2);
    let zr = subaxis(c1.3, c2.3);
    xr && yr && zr
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let lines = load_file(_input);

        let cubes: Vec<(bool, (i64, i64), (i64, i64), (i64, i64))> = lines
            .lines()
            .map(|l| {
                let (status, coords) = l.split_once(' ').unwrap();
                let coords: Vec<(i64, i64)> = coords
                    .split(',')
                    .map(|s| {
                        let values: Vec<i64> =
                            s[2..].split("..").map(|x| x.parse().unwrap()).collect();
                        (values[0], values[1])
                    })
                    .collect();

                (status == "on", coords[0], coords[1], coords[2])
            })
            .collect::<Vec<(bool, (i64, i64), (i64, i64), (i64, i64))>>();

        let filtered_cubes: Vec<(bool, (i64, i64), (i64, i64), (i64, i64))> = cubes
            .into_iter()
            .filter(|cube| cube_comparator(cube, (true, (-50, 50), (-50, 50), (-50, 50))))
            .collect::<Vec<(bool, (i64, i64), (i64, i64), (i64, i64))>>();

        let out = filtered_cubes
            .iter()
            .fold(HashMap::new(), |mut acc, cube| reboot(&mut acc, *cube));
        out.into_iter()
            .fold(0, |acc, (((x0, x1), (y0, y1), (z0, z1)), value)| {
                acc + (x1 - x0 + 1) * (y1 - y0 + 1) * (z1 - z0 + 1) * value
            })
            .to_string()
    }

    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        todo!()
    }
}
