use anyhow::Result;
use itertools::*;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let output = p1(&file);
    let output2 = p2(&file);
    println!("{}", output);
    println!("{}", output2)
}

fn p1(input: &String) -> usize {
    input
        .split("\n\n")
        .map(|elf| elf.split('\n').flat_map(|x| x.parse::<usize>()).sum())
        .max()
        .unwrap()
}

fn p2(input: &String) -> usize {
    input
        .split("\n\n")
        .map(|elf| elf.split('\n').flat_map(|x| x.parse::<usize>()).sum())
        .collect::<Vec<usize>>()
        .iter()
        .sorted()
        .rev()
        .take(3)
        .sum()
}
