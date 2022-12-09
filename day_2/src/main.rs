use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    input.split('\n').fold(0, |score, round| {
        score
            + match round.split_once(' ') {
                Some(("A", "Y")) => 8,
                Some(("B", "Y")) => 5,
                Some(("C", "Y")) => 2,
                Some(("C", "X")) => 7,
                Some(("A", "X")) => 4,
                Some(("B", "X")) => 1,
                Some(("B", "Z")) => 9,
                Some(("C", "Z")) => 6,
                Some(("A", "Z")) => 3,
                _ => 0,
            }
    })
}

fn part_2(input: &str) -> usize {
    input.split('\n').fold(0, |score, round| {
        score
            + match round.split_once(' ') {
                Some(("A", "X")) => 3,
                Some(("B", "X")) => 1,
                Some(("C", "X")) => 2,
                Some(("A", "Y")) => 4,
                Some(("B", "Y")) => 5,
                Some(("C", "Y")) => 6,
                Some(("A", "Z")) => 8,
                Some(("B", "Z")) => 9,
                Some(("C", "Z")) => 7,
                _ => 0,
            }
    })
}
