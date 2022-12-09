use std::collections::VecDeque;

const INPUT: &str = include_str!("../input");

fn main() {
    println!("{}", part_1(INPUT));
    println!("{}", part_2(INPUT));
}

type Stacks = Vec<VecDeque<char>>;

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_stacks(input: &str) -> Stacks {
    let mut stacks: Stacks = vec![VecDeque::new(); 9];
    input.lines().take(8).for_each(|l| {
        let mut elements = [None; 9];
        for (i, p) in (1..l.len()).step_by(4).enumerate() {
            let c = l.as_bytes()[p] as char;
            elements[i] = match c {
                ' ' => None,
                x => Some(x),
            }
        }
        for (i, e) in elements.iter().enumerate().filter(|(_, e)| e.is_some()) {
            stacks[i].push_back(e.unwrap());
        }
    });
    stacks
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|words| Move {
            count: words.get(1).unwrap().parse().unwrap(),
            from: words.get(3).unwrap().parse().unwrap(),
            to: words.get(5).unwrap().parse().unwrap(),
        })
        .collect()
}

fn parse_stacks_and_moves(input: &str) -> (Stacks, Vec<Move>) {
    let (stacks_string, moves_string) = input.split_once("\n\n").unwrap();
    let stacks = parse_stacks(stacks_string);
    let moves = parse_moves(moves_string);
    (stacks, moves)
}

fn part_1(input: &str) -> String {
    let (mut stacks, moves) = parse_stacks_and_moves(input);

    moves.iter().for_each(|m| {
        for _ in 0..m.count {
            let elem = stacks[m.from - 1].pop_front().unwrap();
            stacks[m.to - 1].push_front(elem);
        }
    });

    stacks.iter().map(|stack| stack.get(0).unwrap()).collect()
}

fn part_2(input: &str) -> String {
    let (mut stacks, moves) = parse_stacks_and_moves(input);

    moves.iter().for_each(|m| {
        let mut crates = Vec::new();
        for _ in 0..m.count {
            crates.push(stacks[m.from - 1].pop_front().unwrap())
        }
        crates.reverse();
        for c in crates {
            stacks[m.to - 1].push_front(c);
        }
    });

    stacks.iter().map(|stack| stack.get(0).unwrap()).collect()
}
