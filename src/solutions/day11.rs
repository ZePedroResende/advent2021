use crate::{load_file, parse_lines, AoCDay};
pub struct Code;


fn neighbors(
    y: usize,
    x: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    println!("{} {} {} {}", y, x, width, height);
    let mut vector = vec![(y < height - 1, (y + 1, x)), (x < width - 1, (y, x + 1))];
    if y != 0 {
        vector.push((y > 0, (y - 1, x)));
    }

    if x != 0 {
        vector.push((x > 0, (y, x - 1)));
    }

    vector
        .into_iter()
        .filter_map(|(cond, value)| if cond { Some(value) } else { None })
}

fn tick(matrix: &mut Vec<Vec<u32>>) {

    for row in matrix {
        for item in row {
            item +=1;
        }
    }

    for row in matrix {
        for item in row {
            item +=1;
        }
    }


}



impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);
        let lines: Vec<Vec<u32>> = parse_lines::<String>(&data)
            .map(|str| str.chars().map(|char| char.to_digit(10).unwrap()).collect())
            .collect();


        lines


        let number = 10;

        number.to_string()
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        todo!()
    }
}
