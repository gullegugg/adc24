use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    error::Error,
};

use crate::Part;

fn parse<'a, T: Iterator<Item = &'a String>>(
    input: T,
) -> impl Iterator<Item = Result<(u64, u64), Box<dyn Error>>> + use<'a, T> {
    input
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter(|parts| parts.len() == 2)
        .map(|parts| -> Result<(u64, u64), Box<dyn Error>> {
            Ok((parts[0].parse()?, parts[1].parse()?))
        })
}

fn first(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    let mut first: BinaryHeap<u64> = BinaryHeap::new();
    let mut second: BinaryHeap<u64> = BinaryHeap::new();
    for parse_res in parse(input.iter()) {
        let (a, b) = parse_res?;
        first.push(a);
        second.push(b);
    }
    let mut sum: u64 = 0;
    for _ in 0..first.len() {
        sum += first.pop().unwrap().abs_diff(second.pop().unwrap());
    }
    Ok(sum)
}

fn second(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    let mut first_set = HashSet::new();
    let mut second_count: HashMap<u64, u64> = HashMap::new();
    for parse_res in parse(input.iter()) {
        let (a, b) = parse_res?;
        first_set.insert(a);
        second_count.entry(b).and_modify(|e| *e += 1).or_insert(1);
    }
    Ok(first_set
        .iter()
        .map(|i| i * second_count.get(i).unwrap_or(&0))
        .sum())
}

pub fn solve(part: Part, input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    match part {
        Part::First => first(input),
        Part::Second => second(input),
    }
}
