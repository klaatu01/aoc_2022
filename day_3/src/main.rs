use array_tool::vec::Intersect;

const INPUT: &str = include_str!("../input");

fn main() {
    println!("{}", part_1(INPUT));
    println!("{}", part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(a, b)| {
            (
                a.chars().collect::<Vec<char>>(),
                b.chars().collect::<Vec<char>>(),
            )
        })
        .map(|(a, b)| (a.intersect(b)))
        .fold(0, |acc, i| {
            acc + i
                .first()
                .map(|i| *i as u8 - if i.is_uppercase() { 38 } else { 96 })
                .unwrap_or(0) as usize
        })
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            chunk[0]
                .chars()
                .collect::<Vec<char>>()
                .intersect(chunk[1].chars().collect())
                .intersect(
                    chunk[0]
                        .chars()
                        .collect::<Vec<char>>()
                        .intersect(chunk[2].chars().collect()),
                )
        })
        .fold(0, |acc, i| {
            acc + i
                .first()
                .map(|i| *i as u8 - if i.is_uppercase() { 38 } else { 96 })
                .unwrap_or(0) as usize
        })
}
