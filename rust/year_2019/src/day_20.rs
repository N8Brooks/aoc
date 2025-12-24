use std::{iter::successors, mem};

use hashbrown::{HashMap, HashSet};
use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let map: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let mut portals = map
        .iter()
        .enumerate()
        .flat_map(|(i, line)| line.iter().enumerate().map(move |(j, &b)| (i, j, b)))
        .filter(|&(_, _, b)| b == b'.')
        .filter_map(|(i, j, _)| {
            [
                [map[i - 2][j], map[i - 1][j]],
                [map[i][j + 1], map[i][j + 2]],
                [map[i + 1][j], map[i + 2][j]],
                [map[i][j - 2], map[i][j - 1]],
            ]
            .into_iter()
            .find(|[a, b]| a.is_ascii_uppercase() && b.is_ascii_uppercase())
            .map(|label| (label, (i, j)))
        })
        .into_group_map();
    let [start, end] = [b"AA", b"ZZ"].map(|label| {
        portals
            .remove(label)
            .unwrap()
            .into_iter()
            .exactly_one()
            .unwrap()
    });
    let portals: HashMap<_, _> = portals
        .into_values()
        .flat_map(|positions| {
            let [p1, p2] = positions.try_into().unwrap();
            [(p1, p2), (p2, p1)]
        })
        .collect();
    let mut new = vec![vec![true; map[0].len()]; map.len()];
    successors(Some(vec![start]), |frontier| {
        let frontier: Vec<_> = frontier
            .iter()
            .flat_map(|&v @ (i1, j1)| {
                [(i1 + 1, j1), (i1 - 1, j1), (i1, j1 + 1), (i1, j1 - 1)]
                    .into_iter()
                    .map(move |w| (v, w))
                    .filter_map(|(v, w @ (i2, j2))| match map[i2][j2] {
                        b'#' => None,
                        b'.' => Some(w),
                        b'A'..=b'Z' => portals.get(&v).copied(),
                        b => panic!("unexpected map character: {}", b as char),
                    })
            })
            .filter(|&(i, j)| mem::replace(&mut new[i][j], false))
            .collect();
        (!frontier.is_empty()).then_some(frontier)
    })
    .enumerate()
    .find(|(_, frontier)| frontier.contains(&end))
    .unwrap()
    .0
}

pub fn part_2(input: &str) -> usize {
    let map: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let mut portals = map
        .iter()
        .enumerate()
        .flat_map(|(i, line)| line.iter().enumerate().map(move |(j, &b)| (i, j, b)))
        .filter(|&(_, _, b)| b == b'.')
        .filter_map(|(i, j, _)| {
            [
                [map[i - 2][j], map[i - 1][j]],
                [map[i][j + 1], map[i][j + 2]],
                [map[i + 1][j], map[i + 2][j]],
                [map[i][j - 2], map[i][j - 1]],
            ]
            .into_iter()
            .find(|[a, b]| a.is_ascii_uppercase() && b.is_ascii_uppercase())
            .map(|label| (label, (i, j)))
        })
        .into_group_map();
    let [start, end] = [b"AA", b"ZZ"].map(|label| {
        portals
            .remove(label)
            .unwrap()
            .into_iter()
            .exactly_one()
            .unwrap()
    });
    let portals: HashMap<_, _> = portals
        .into_values()
        .flat_map(|positions| {
            let [p1, p2] = positions.try_into().unwrap();
            [(p1, p2), (p2, p1)]
        })
        .collect();
    let is_outer = |(i, j)| i == 2 || j == 2 || i == map.len() - 3 || j == map[0].len() - 3;
    let mut seen = HashSet::new();
    successors(Some(vec![(0_usize, start)]), |frontier| {
        let frontier: Vec<_> = frontier
            .iter()
            .flat_map(|&(d, v @ (i, j))| {
                [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]
                    .into_iter()
                    .map(move |w| (d, v, w))
                    .filter_map(|(d, v, w @ (i, j))| match map[i][j] {
                        b'#' => None,
                        b'.' => Some((d, w)),
                        b'A'..=b'Z' => {
                            if is_outer(v) && d == 0 {
                                None
                            } else {
                                portals
                                    .get(&v)
                                    .map(|&w| if is_outer(v) { (d - 1, w) } else { (d + 1, w) })
                            }
                        }
                        b => panic!("unexpected map character: {}", b as char),
                    })
            })
            .filter(|&key| seen.insert(key))
            .collect();
        (!frontier.is_empty()).then_some(frontier)
    })
    .enumerate()
    .find(|(_, frontier)| frontier.contains(&(0, end)))
    .unwrap()
    .0
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_20.txt");

    const EXAMPLE_1: &str = "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ";

    const EXAMPLE_2: &str = "                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";

    #[test_case(EXAMPLE_1 => 23)]
    #[test_case(EXAMPLE_2 => 58)]
    #[test_case(INPUT => 568)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    const EXAMPLE_3: &str = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";

    #[test_case(EXAMPLE_3 => 396)]
    #[test_case(INPUT => 6546)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
