use itertools::Itertools;

const INPUT: &str = include_str!("../input");

fn main() {
    println!("{}", part_1(INPUT));
    println!("{}", part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let mut p = 4;
    for i in 0..input.len() {
        if input[i..p].chars().unique().count() == 4 {
            return p;
        }
        p += 1;
    }
    p
}

fn part_2(input: &str) -> usize {
    let mut p = 14;
    for i in 0..input.len() {
        if input[i..p].chars().unique().count() == 14 {
            return p;
        }
        p += 1;
    }
    p
}
