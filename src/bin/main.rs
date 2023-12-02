use std::{error::Error, iter::Peekable, str::Chars};

#[derive(Debug)]
struct Node {
    char: char,
    children: Vec<Node>,
    num: Option<i32>,
}

fn build_trees(words: Vec<&str>) -> Vec<Node> {
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
    dbg!(&roots);
    roots
}

impl Node {
    pub fn find_match(&mut self, s: &mut Peekable<Chars>) -> Option<i32> {
        if let Some(c) = s.peek() {
            if self.char == *c {
                if self.num.is_some() {
                    return self.num;
                }
                s.next();
                return self.children.iter_mut().find_map(|n| n.find_match(s));
            }
        }
        None
    }
}

fn find_match(trees: &mut Vec<Node>, s: &str) -> Option<i32> {
    trees
        .iter_mut()
        .find_map(|t| t.find_match(&mut s.chars().peekable()))
}

fn main() -> Result<(), Box<dyn Error>> {
    let words = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let mut trees = build_trees(words);

    let mut sum = 0;
    for line in std::fs::read_to_string("input/1/in.txt")?.lines() {
        dbg!(&line);
        let left = (0..line.len())
            .find_map(|i| find_match(&mut trees, &line[i..]))
            .unwrap_or(0);
        let right = (0..line.len())
            .rev()
            .find_map(|i| find_match(&mut trees, &line[i..]))
            .unwrap_or(0);
        sum += 10 * left + right;
        dbg!(10 * left + right);
    }
    dbg!(sum);
    Ok(())
}
