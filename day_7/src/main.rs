use anyhow::Result;
use ptree::{print_tree, TreeBuilder};
use std::{borrow::BorrowMut, cell::RefCell, collections::VecDeque, rc::Rc, str::FromStr};

const INPUT: &str = include_str!("../input");

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    contents: Vec<Rc<RefCell<FileSystem>>>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Directory {
            name: name.to_string(),
            contents: Vec::new(),
        }
    }

    fn add_content(&mut self, content: FileSystem) {
        self.contents.push(Rc::new(RefCell::new(content)));
    }

    fn size(&self) -> usize {
        self.contents
            .iter()
            .flat_map(|x| match &*x.as_ref().borrow() {
                FileSystem::File(size) => vec![*size],
                FileSystem::Dir(dir) => dir.as_ref().borrow().sizes(),
            })
            .sum()
    }

    fn sizes(&self) -> Vec<usize> {
        self.contents
            .iter()
            .flat_map(|x| match &*x.as_ref().borrow() {
                FileSystem::File(size) => vec![*size],
                FileSystem::Dir(dir) => dir.as_ref().borrow().sizes(),
            })
            .collect()
    }

    fn tree<'a>(&self, t: &'a mut TreeBuilder) -> &'a mut TreeBuilder {
        t.begin_child(self.name.clone());
        self.contents
            .iter()
            .for_each(|x| match &*x.as_ref().borrow() {
                FileSystem::File(size) => {
                    t.add_empty_child(format!("{}", size));
                }
                FileSystem::Dir(dir) => {
                    dir.as_ref().borrow().tree(t);
                }
            });
        t.end_child();
        t
    }

    fn to_string_pretty(&self, indent: usize) -> String {
        let indent_string = (0..(indent - 1)).map(|_| ' ').collect::<String>();
        let indented_more_string = (0..(indent)).map(|_| ' ').collect::<String>();
        format!(
            "{}{} - {}\n{}",
            indent_string.clone(),
            self.name,
            self.size(),
            self.contents
                .iter()
                .map(|x| match &*x.as_ref().borrow() {
                    FileSystem::File(size) => {
                        "".to_string()
                    }
                    FileSystem::Dir(dir) => {
                        let tmp_dir = dir.as_ref().borrow();
                        format!(
                            "{}{}\n",
                            indented_more_string,
                            tmp_dir.to_string_pretty(indent + 2)
                        )
                    }
                })
                .collect::<String>()
        )
    }
}

#[derive(Debug)]
enum FileSystem {
    File(usize),
    Dir(Rc<RefCell<Directory>>),
}

impl FromStr for FileSystem {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.split_once(' ').unwrap() {
            ("dir", name) => Ok(Self::Dir(Rc::new(RefCell::new(Directory::new(name))))),
            (size, name) if size.parse::<usize>().is_ok() => Ok(Self::File(size.parse().unwrap())),
            _ => Err(anyhow::Error::msg("uh oh")),
        }
    }
}

#[derive(Debug)]
enum Commands {
    CdIn(String),
    CdUp,
    Ls,
}

impl FromStr for Commands {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["$", "cd", ".."] => Ok(Self::CdUp),
            ["$", "cd", name] => Ok(Self::CdIn(name.to_string())),
            ["$", "ls"] => Ok(Self::Ls),
            _ => Err(anyhow::Error::msg("uh oh")),
        }
    }
}

fn main() {
    println!("{}", part_1(INPUT));
    println!("{}", part_2(INPUT));
}

fn parse_file_system(input: &str) -> Rc<RefCell<Directory>> {
    let mut stack: VecDeque<Rc<RefCell<Directory>>> = VecDeque::new();
    let root = Rc::new(RefCell::new(Directory::new("/")));
    stack.push_back(root.clone());

    input.lines().skip(1).for_each(|line| {
        let curr = stack.back_mut().unwrap().clone();

        match (Commands::from_str(line), FileSystem::from_str(line)) {
            (Ok(cmd), _) => match cmd {
                Commands::CdUp => {
                    stack.pop_back();
                }
                Commands::CdIn(dir) => {
                    let t = curr.as_ref().borrow_mut();
                    let d = t.contents.iter().find(|d| match &*d.as_ref().borrow() {
                        FileSystem::Dir(directory) => dir == directory.as_ref().borrow().name,
                        _ => false,
                    });
                    if let FileSystem::Dir(directory) = &*d.expect("").borrow() {
                        stack.push_back(directory.clone())
                    };
                }
                Commands::Ls => (),
            },
            (_, Ok(fs)) => curr.as_ref().borrow_mut().add_content(fs),
            (_, _) => panic!("UH OHHH"),
        }
    });

    root
}

fn part_1(input: &str) -> usize {
    let fs = parse_file_system(input);
    let dir = fs.as_ref().borrow();
    let tree = dir.tree(&mut TreeBuilder::new("".to_string())).build();
    print_tree(&tree).unwrap();
    dir.sizes().iter().filter(|x| **x >= 100000).sum()
}

fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use crate::part_1;

    #[test]
    fn example() {
        let input: &str = include_str!("../example_input");
        assert_eq!(part_1(input), 48381165);
    }
}
