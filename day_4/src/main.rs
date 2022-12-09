use array_tool::vec::{self, Intersect};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("{}", part_1(INPUT));
    println!("{}", part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|x| x.split_once(','))
        .map(|(a, b)| (a.split_once('-').unwrap(), b.split_once('-').unwrap()))
        .map(|((a, b), (c, d))| {
            (
                (a.parse().unwrap()..(b.parse::<u8>().unwrap() + 1)).collect::<Vec<u8>>(),
                (c.parse().unwrap()..(d.parse::<u8>().unwrap() + 1)).collect::<Vec<u8>>(),
            )
        })
        .filter(|(a, b)| a.intersect(b.to_vec()).len() == std::cmp::min(b.len(), a.len()))
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .flat_map(|x| x.split_once(','))
        .map(|(a, b)| (a.split_once('-').unwrap(), b.split_once('-').unwrap()))
        .map(|((a, b), (c, d))| {
            (
                (a.parse().unwrap()..(b.parse::<u8>().unwrap() + 1)).collect::<Vec<u8>>(),
                (c.parse().unwrap()..(d.parse::<u8>().unwrap() + 1)).collect::<Vec<u8>>(),
            )
        })
        .filter(|(a, b)| !a.intersect(b.to_vec()).is_empty())
        .count()
}
