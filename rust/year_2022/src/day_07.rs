use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default, Debug)]
struct Node<'a> {
    size: usize,
    out_dir: Option<Rc<RefCell<Node<'a>>>>,
    in_dirs: HashMap<&'a str, Rc<RefCell<Node<'a>>>>,
}

impl<'a> Node<'a> {
    fn new(size: usize, out_dir: &Rc<RefCell<Node<'a>>>) -> Node<'a> {
        Node {
            size,
            out_dir: Some(Rc::clone(out_dir)),
            in_dirs: HashMap::new(),
        }
    }

    fn from_input(input: &'a str) -> Rc<RefCell<Node<'a>>> {
        let root = Rc::new(RefCell::new(Node::default()));
        let mut node = Rc::clone(&root);
        for line in input.lines() {
            if let Some(command) = line.strip_prefix("$ ") {
                node = match command.strip_prefix("cd ") {
                    Some("/") => Rc::clone(&root),
                    Some("..") => Rc::clone(node.borrow().out_dir.as_ref().unwrap()),
                    Some(path) => Rc::clone(node.borrow().in_dirs.get(path).unwrap()),
                    None if command == "ls" => node,
                    _ => panic!("unknown command {command}"),
                }
            } else {
                let (size, name) = line.split_once(' ').unwrap();
                let size = size.parse().unwrap_or_default();
                let next_node = Rc::new(RefCell::new(Node::new(size, &node)));
                node.borrow_mut().in_dirs.insert(name, next_node);
            }
        }
        root.borrow_mut().update_totals();
        root
    }

    fn update_totals(&mut self) -> usize {
        self.size += self
            .in_dirs
            .values()
            .map(|node| node.borrow_mut().update_totals())
            .sum::<usize>();
        self.size
    }

    fn part_1(&self) -> usize {
        if self.in_dirs.is_empty() {
            return 0;
        }
        let size = if self.size > 100_000 { 0 } else { self.size };
        self.in_dirs
            .values()
            .map(|node| node.borrow().part_1())
            .sum::<usize>()
            + size
    }

    fn part_2(&self, min_size: usize) -> Option<usize> {
        if self.in_dirs.is_empty() || self.size < min_size {
            return None;
        }
        self.in_dirs
            .values()
            .flat_map(|node| node.borrow().part_2(min_size))
            .chain(Some(self.size))
            .min()
    }
}

pub fn part_1(input: &str) -> usize {
    Node::from_input(input).borrow_mut().part_1()
}

pub fn part_2(input: &str) -> usize {
    let node = Node::from_input(input);
    let node = node.borrow_mut();
    let min_size = node.size.saturating_sub(40_000_000);
    node.part_2(min_size).unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    const INPUT: &str = include_str!("../test_data/day_07.txt");

    #[test_case(EXAMPLE => 95437)]
    #[test_case(INPUT => 1443806)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 24933642)]
    #[test_case(INPUT => 942298)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
