use nom::{IResult, character, multi, bytes};
use indicatif::*;

#[derive(Debug)]
struct SeedMap {
    ranges: Vec<Vec<u64>>,
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    // seeds: 79 14 55 13
    let (input, _) = bytes::complete::tag("seeds: ")(input)?;
    let (input, v) = multi::separated_list0(
        character::complete::space1,
        character::complete::u64
    )(input)?;
    let (input, _) = character::complete::line_ending(input)?;
    let (input, _) = character::complete::line_ending(input)?;

    Ok((input, v))
}

fn parse_map(input: &str) -> IResult<&str, SeedMap> {
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    let (input, _name) = character::complete::not_line_ending(input)?;
    let (input, _) = character::complete::line_ending(input)?;
    let (input, ranges) = multi::separated_list0(
        character::complete::line_ending,
        multi::separated_list1(
            character::complete::space1,
            character::complete::u64
        )
    )(input)?;

    Ok((input, SeedMap{
        ranges
    }))
}

fn parse_maps(input: &str) -> IResult<&str, Vec<SeedMap>> {
    let (input, m) = multi::separated_list1(
        bytes::complete::tag("\n\n"),
        parse_map
    )(input)?;

    Ok((input, m))
}

fn find_closest_location(seeds: &Vec<u64>, maps: &Vec<SeedMap>) -> u64 {
    let mut closest_location = u64::MAX;
    for seed in seeds {
        let mut key = *seed;
        for map in maps {
            let mut mapping = key;
            for range in &map.ranges {
                let dst = range[0];
                let src = range[1];
                let len = range[2];
                if key >= src && key < src+len {
                    let distance = key - src;
                    mapping = dst + distance;
                    break;
                }
            }
            key = mapping;
        }
        if key < closest_location {
            closest_location = key;
        }
    }
    closest_location
}

fn find_closest_location2(seeds: &Vec<u64>, maps: &Vec<SeedMap>) -> u64 {
    let mut location = 0;
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    pb.set_message("Processing...");
    for _ in 0..u64::MAX {
        let mut key = location;
        for map in maps.iter().rev() {
            for range in &map.ranges {
                let dst = range[1];
                let src = range[0];
                let len = range[2];
                if key >= src && key < src+len {
                    let distance = key - src;
                    key = dst + distance;
                    break;
                }
            }
        }
        let iter = seeds.chunks(2);
        for v in iter {
            let start = v[0];
            let end = v[0] + v[1];
            if key >= start && key < end {
                pb.finish_with_message("Location found");
                return location;
            }
        }
        location += 1;
    }
    panic!("Location not found");
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day5/input2.txt").unwrap();
    let (input, seeds) = parse_seeds(&input).unwrap();
    let (_, maps) = parse_maps(input).unwrap();
    println!("Day 5, Part 1: {}", find_closest_location(&seeds, &maps));
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day5/input2.txt").unwrap();
    let (input, seeds) = parse_seeds(&input).unwrap();
    let (_, maps) = parse_maps(input).unwrap();
    println!("Day 5, Part 2: {}", find_closest_location2(&seeds, &maps));
}

#[cfg(test)]
mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day5/input1.txt").unwrap();
        let (input, seeds) = super::parse_seeds(&input).unwrap();
        let (_, maps) = super::parse_maps(input).unwrap();
        assert_eq!(35, super::find_closest_location(&seeds, &maps));
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day5/input2.txt").unwrap();
        let (input, seeds) = super::parse_seeds(&input).unwrap();
        let (_, maps) = super::parse_maps(input).unwrap();
        assert_eq!(31599214, super::find_closest_location(&seeds, &maps));
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day5/input1.txt").unwrap();
        let (input, seeds) = super::parse_seeds(&input).unwrap();
        let (_, maps) = super::parse_maps(input).unwrap();
        assert_eq!(46, super::find_closest_location2(&seeds, &maps));
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day5/input2.txt").unwrap();
        let (input, seeds) = super::parse_seeds(&input).unwrap();
        let (_, maps) = super::parse_maps(input).unwrap();
        assert_eq!(20358599, super::find_closest_location2(&seeds, &maps));
    }
}

