use nom::{
    bytes::complete::tag,
    character::complete::{
        line_ending,
        alphanumeric1
    },
    multi::{
        many1,
        separated_list1
    },
    branch::alt,
    IResult
};
use std::collections::BTreeMap;
use num::integer::lcm;

fn parse_directions(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, directions) = many1(alt((tag("L"), tag("R"))))(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, directions))
}

fn parse_element(input: &str) -> IResult<&str, &str> {
    let (input, element) = alphanumeric1(input)?;
    Ok((input, element))
}

fn parse_entry(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, start) = parse_element(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = parse_element(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = parse_element(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, (start, left, right)))
}

fn parse_entries(input: &str) -> IResult<&str, (Vec<&str>, BTreeMap<&str, (&str, &str)>)> {
    let (input, directions) = parse_directions(input)?;
    let (input, _) = line_ending(input)?;
    let (input, entries) = separated_list1(line_ending, parse_entry)(input)?;
    let mut jump_table = BTreeMap::new();
    for (start, left, right) in entries {
        jump_table.insert(start, (left, right));
    }
    Ok((input, (directions, jump_table)))
}

fn walk(start: &str, directions: Vec<&str>, jump_table: &BTreeMap<&str, (&str, &str)>) -> usize {
    let mut current = start;
    let mut steps = 0;
    loop {
        for step in 0..directions.len() {
            let (left, right) = jump_table.get(current).unwrap();
            if directions[step] == "L" {
                current = left;
            } else {
                current = right;
            }
            steps += 1;
            if current == "ZZZ" {
                return steps;
            }
        }
    }
}

fn walk2(start: &str, directions: Vec<&str>, jump_table: &BTreeMap<&str, (&str, &str)>) -> usize {
    let mut current = start;
    let mut steps = 0;
    loop {
        for step in 0..directions.len() {
            let (left, right) = jump_table.get(current).unwrap();
            if directions[step] == "L" {
                current = left;
            } else {
                current = right;
            }
            steps += 1;
            if current.ends_with("Z") {
                return steps;
            }
        }
    }
}

fn walk3(directions: Vec<&str>, jump_table: &BTreeMap<&str, (&str, &str)>) -> usize {
    let mut currents = Vec::new();
    let mut steps = Vec::new();
    for (start, (_, _)) in jump_table {
        if start.ends_with("A") {
            currents.push(start);
        }
    }

    for current in currents {
        steps.push(walk2(current, directions.clone(), jump_table));
    }

    lcmm(steps)
}

fn lcmm(nums: Vec<usize>) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    if nums.len() == 2 {
        return lcm(nums[0], nums[1]);
    }

    return lcm(nums[0], lcmm(nums[1..].to_vec()));
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day8/input2.txt").unwrap();
    let (_, data) = parse_entries(&input).unwrap();
    let steps = walk("AAA", data.0, &data.1);
    println!("Day 8, Part 1: {}", steps);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day8/input2.txt").unwrap();
    let (_, data) = parse_entries(&input).unwrap();
    let steps = walk3(data.0, &data.1);
    println!("Day 8, Part 2: {}", steps);
}

mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day8/input1.txt").unwrap();
        let (_, data) = super::parse_entries(&input).unwrap();
        let steps = super::walk("AAA", data.0, &data.1);
        assert_eq!(steps, 6);
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day8/input2.txt").unwrap();
        let (_, data) = super::parse_entries(&input).unwrap();
        let steps = super::walk("AAA", data.0, &data.1);
        assert_eq!(steps, 20093);
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day8/input3.txt").unwrap();
        let (_, data) = super::parse_entries(&input).unwrap();
        let steps = super::walk3(data.0, &data.1);
        assert_eq!(steps, 6);
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day8/input2.txt").unwrap();
        let (_, data) = super::parse_entries(&input).unwrap();
        let steps = super::walk3(data.0, &data.1);
        assert_eq!(steps, 22103062509257);
    }
}
