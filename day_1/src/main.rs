use itertools::*;

const INPUT: &str = include_str!("../input");

fn main() {
    println!("{}", part_1(INPUT));
    println!("{}", part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf| elf.split('\n').flat_map(|x| x.parse::<usize>()).sum())
        .max()
        .unwrap()
}

fn part_2(input: &str) -> usize {
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
