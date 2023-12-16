#![feature(test)]
use std::error::Error;
use std::str::FromStr;

const COLORS: [&str; 3] = ["red", "green", "blue"];
const AMOUNT: [u32; 3] = [12, 13, 14];

struct HelloWorld {
    msg: String,
}

impl FromStr for HelloWorld {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HelloWorld {
            msg: String::from(s),
        })
    }
}

// NOTE: probably should just panic, filter_mapping doesn't make much sense if you need the input to be correct.
fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string("input/2/in.txt")?;
    let cube_lines: Vec<_> = file
        .lines()
        .enumerate()
        .filter_map(|(i, line)| -> Option<(u32, Vec<(usize, u32)>)> {
            let mut fields = line.split(':');
            let id: u32 = (i + 1) as u32;
            let cubes = fields
                .nth(1)?
                .split(';')
                .flat_map(|s| {
                    s.trim()
                        .split(",")
                        .filter_map(|game| {
                            let mut fields = game.trim().split(" ");
                            let num: u32 = fields.next()?.parse().ok()?;
                            let color = fields.next()?;
                            let c = COLORS.iter().position(|col| *col == color)?;
                            Some((c, num))
                        })
                        .collect::<Vec<_>>()
                })
                .collect();
            Some((id, cubes))
        })
        .collect();

    let part1: u32 = cube_lines
        .iter()
        .filter_map(|(id, cubes)| {
            if cubes.iter().all(|(color, num)| *num <= AMOUNT[*color]) {
                Some(id)
            } else {
                None
            }
        })
        .sum();
    dbg!(part1);
    let part2: u32 = cube_lines
        .iter()
        .map(|(_, cubes)| {
            cubes
                .into_iter()
                .fold([0, 0, 0], |mut acc, (color, num)| {
                    if acc[*color] < *num {
                        acc[*color] = *num;
                    }
                    acc
                })
                .into_iter()
                .product::<u32>()
        })
        .sum();
    dbg!(part2);

    let v: HelloWorld = "hi".parse()?;
    println!("msg from impl: {}", v.msg);

    Ok(())
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn other(b: &mut Bencher) {
        b.iter(|| main());
    }
}
