// This code was made to try and touch as many different rust concepts as possible
// To make it faster I would probably use slice equality somehow
#![feature(test)]
use std::error::Error;

const WORDS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];
const REVERSE_WORDS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "eno", "owt", "eerht", "ruof", "evif", "xis",
    "neves", "thgie", "enin",
];

#[derive(Debug)]
struct Node {
    char: char,
    children: Vec<Node>,
    num: Option<i32>,
}

fn build_trees(words: [&str; 18]) -> Vec<Node> {
    let mut roots: Vec<Node> = words
        .iter()
        .map(|w| w.chars().nth(0).unwrap())
        .collect::<std::collections::HashSet<char>>()
        .into_iter()
        .map(|c| Node {
            char: c,
            children: vec![],
            num: None,
        })
        .collect();

    for iw in 0..words.len() {
        let w = words[iw];
        let mut node: &mut Node = roots
            .iter_mut()
            .find(|n| n.char == w.chars().nth(0).unwrap())
            .unwrap();

        for c in w.chars().skip(1) {
            if let Some(idx) = node.children.iter().position(|n| n.char == c) {
                node = &mut node.children[idx];
            } else {
                node.children.push(Node {
                    char: c,
                    children: vec![],
                    num: None,
                });
                node = node.children.last_mut().unwrap();
            }
        }
        node.num = Some(((iw % 9) + 1).try_into().unwrap())
    }
    roots
}

fn find_match<I: Iterator<Item = char> + Clone>(trees: &Vec<Node>, iter: I) -> Option<i32> {
    let mut it = iter.peekable();
    let mut node: Option<&Node> = None;
    while let Some(ch) = it.next() {
        if node.is_none() {
            node = trees.iter().find(|t| t.char == ch);
        }
        if let Some(mut r) = node {
            let orig = it.clone();
            loop {
                if r.num.is_some() {
                    return r.num;
                }
                if let Some(c) = it.peek() {
                    if let Some(n) = r.children.iter().find(|t| t.char == *c) {
                        if n.num.is_some() {
                            return n.num;
                        }
                        it.next();
                        r = n;
                        continue;
                    } else {
                        node = None;
                        it = orig;
                        break;
                    }
                }
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let trees = build_trees(WORDS);
    let rev_trees = build_trees(REVERSE_WORDS);

    let mut sum = 0;
    for line in std::fs::read_to_string("input/1/in.txt")?.lines() {
        let left = find_match(&trees, line.chars()).unwrap_or(0);
        let right = find_match(&rev_trees, line.chars().rev()).unwrap_or(0);
        sum += 10 * left + right;
    }
    println!("{}", sum);
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
