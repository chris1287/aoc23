use core::panic;
use std::collections::VecDeque;
use rayon::prelude::*;

#[derive(Debug, Default)]
struct Record {
    seed: String,
    groups: Vec<usize>,
}

fn parse(input: &str) -> Vec<Record> {
    let mut records = Vec::new();
    for line in input.lines() {
        let mut v = line.split(" ");
        let seed = v.next().expect("should have seed");
        let groups = v.next().expect("should have groups");
        let groups = groups.split(",").map(|s| s.parse::<usize>().expect("should be a number")).collect::<Vec<_>>();
        records.push(Record {
            seed: seed.to_string(),
            groups,
        });
    }
    records
}

fn is_combination_valid(combination: &str, groups: &Vec<usize>) -> bool {
    let mut current_group_len = 0;
    let mut in_a_group = false;
    let mut partial_combination = false;
    let mut wannabe_group = Vec::new();
    let mut current_group_id : i32 = -1;
    for c in combination.chars() {
        match c {
            '.' => {
                // Functional
                if in_a_group {
                    in_a_group = false;
                    wannabe_group.push(current_group_len);
                    current_group_len = 0;
                }
            },
            '#' => {
                // Not functional
                if !in_a_group {
                    current_group_id += 1;
                    in_a_group = true;
                }
                current_group_len += 1;
            },
            _ => {
                partial_combination = true;
                if in_a_group {
                    if current_group_id as usize >= groups.len() {
                        // Cannot be valid
                        return false;
                    }
                    if current_group_id >= 0 && current_group_len > groups[current_group_id as usize] {
                        // Cannot be valid
                        return false;
                    } else {
                        // Cannot decide yet, maybe valid
                        in_a_group = false;
                        wannabe_group.pop();
                    }
                }
                // Cannot decide yet, maybe valid
                break;
            },
        }
    }
    if in_a_group {
        wannabe_group.push(current_group_len);
    }

    if wannabe_group.len() > groups.len() {
        // Cannot be valid
        return false;
    }

    if partial_combination {
        for (i, g) in wannabe_group.iter().enumerate() {
            if *g != groups[i] {
                // Cannot be valid
                return false;
            }
        }
        // Might be valid
        return true;
    } else {
        // Must be exactly equal
        return wannabe_group == *groups;
    }
}

fn generate_arrangements(record: &mut Record) -> usize {
    let mut arrangements = VecDeque::new();
    arrangements.push_front(record.seed.clone());
    for (i, _e) in record.seed.chars().enumerate() {
        let elements = arrangements.len();
        for _j in 0..elements {
            let r = arrangements.pop_back().unwrap();
            match r.chars().nth(i) {
                Some('.') | Some('#') => arrangements.push_front(r),
                Some('?') => {
                    let mut r1 = r.clone();
                    r1.replace_range(i..i+1, ".");
                    let mut r2 = r.clone();
                    r2.replace_range(i..i+1, "#");
                    if is_combination_valid(&r1, &record.groups) {
                        arrangements.push_front(r1);
                    }
                    if is_combination_valid(&r2, &record.groups) {
                        arrangements.push_front(r2);
                    }
                },
                _ => panic!("Should not happen"),
            }
        }
    }
    arrangements.len()
}

fn solve(records: &mut Vec<Record>) -> usize {
    records
    .par_iter_mut()
    .map(|record|{
        generate_arrangements(record)
    })
    .sum()
}

pub fn solve1() -> usize {
    let input = std::fs::read_to_string("data/day12/input2.txt").unwrap();
    let mut records = parse(&input);
    solve(&mut records)
}

pub fn part1() {
    println!("Day 12, Part 1: {}", solve1());
}

pub fn part2() {
    println!("Day 12, Part 2: NA");
}

mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day12/input1.txt").unwrap();
        let mut records = super::parse(&input);
        let res = super::solve(&mut records);
        assert_eq!(res, 21);
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day12/input2.txt").unwrap();
        let mut records = super::parse(&input);
        let res = super::solve(&mut records);
        assert_eq!(res, 7541);
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day12/input1.txt").unwrap();
        let mut records = super::parse(&input);
        let res = super::solve(&mut records);
        assert_eq!(res, 525152);
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day12/input2.txt").unwrap();
        let mut records = super::parse(&input);
        let res = super::solve(&mut records);
        assert_eq!(res, 0);
    }

    #[test]
    fn t5() {
        let input = std::fs::read_to_string("data/day12/input3.txt").unwrap();
        let mut records = super::parse(&input);
        let res = super::solve(&mut records);
        assert_eq!(res, 0);
    }
}