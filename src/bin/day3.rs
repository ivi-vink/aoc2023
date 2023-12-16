#![feature(test)]
use std::error::Error;
use std::str::FromStr;
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

#[derive(Debug)]
struct Number {
    value: u32,
    begin: usize,
    end: usize,
}

#[derive(Debug)]
struct SchematicLine {
    numbers: Vec<Number>,
    symbols: Vec<(usize, char)>,
}

impl FromStr for SchematicLine {
    type Err = std::convert::Infallible;
    fn from_str(line: &str) -> Result<Self, <Self as FromStr>::Err> {
        let numbers = (1..4).rev().fold(vec![], |mut acc, window_size| {
            acc.extend(
                line.as_bytes()
                    .windows(window_size as usize)
                    .enumerate()
                    .filter(|(i, _)| !acc.iter().any(|n: &Number| n.begin <= *i && *i <= n.end))
                    .filter_map(|(i, w)| {
                        if w[0] as char == '+' {
                            return None;
                        }
                        if let Some(num) = std::str::from_utf8(w).ok()?.parse().ok() {
                            Some(Number {
                                value: num,
                                begin: i,
                                end: i + window_size - 1,
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>(),
            );
            acc
        });
        let symbols = line
            .char_indices()
            .filter(|(_, c)| !c.is_digit(10) && *c != '.')
            .collect::<Vec<_>>();
        Ok(SchematicLine { numbers, symbols })
    }
}

impl SchematicLine {
    pub fn check_against_symbols<'a>(&'a self, mut symbols: impl Iterator<Item=&'a (usize, char)> + 'a + Clone) -> impl Iterator<Item=&Number> + 'a {
        let s = symbols.collect::<Vec<_>>();
        self.numbers
            .iter()
            .filter(move |n| {
                s.iter().any(|&&(pos, _)| {
                    (pos as i32 - n.begin as i32).abs() <= 1
                        || (pos as i32 - n.end as i32).abs() <= 1
                }) || self
                    .symbols
                    .iter()
                    .find(|&&(s, _)| {
                        s as isize == n.begin as isize - 1 || s as isize == n.end as isize + 1
                    })
                    .is_some()
            })
    }

    pub fn get_gear_products(&self, mut numbers:  impl Iterator<Item=&Number>) -> impl Iterator<Item=&u32> {
        self.symbols
            .filter(|(i, symbol)| symbol == '*')
            .map(|(i, _)| i as u32)
    }
}

// 467..114..   ...*......   ..35..633.   ......#...
fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string("input/3/in.txt")?;
    let lines: &[SchematicLine] = &file
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>();
    let numbers = lines
        .windows(3)
        .enumerate()
        .fold(vec![], |mut machine_part_numbers, (i, w)| {
            if i == 0 {
                machine_part_numbers
                    .extend(w[0].check_against_symbols(w[1].symbols.iter()));
            }
            machine_part_numbers
                .extend(w[1].check_against_symbols(w[0].symbols.iter().chain(w[2].symbols.iter())));
            if i == lines.len() - 3 {
                machine_part_numbers
                    .extend(w[2].check_against_symbols(w[1].symbols.iter()));
            }
            machine_part_numbers
        });
    let part1: u32 = numbers.iter().map(|n| n.value).sum();

    let part2 = lines
        .windows(3)
        .enumerate()
        .fold(vec![], |mut gear_products, (i, w)| {
            gear_products.extend();
            gear_products
        });
    dbg!(&part2);

    Ok(())
}
