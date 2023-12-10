use nom::*;

fn parse_line1(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = bytes::complete::take_until(":")(input)?;
    let (input, _) = character::complete::char(':')(input)?;
    let (input, _) = multi::many1(character::complete::space1)(input)?;
    let (input, v) = multi::separated_list0(
        multi::many1(character::complete::char(' ')),
        character::complete::u64
    )(input)?;
    let (input, _) = character::complete::line_ending(input)?;

    Ok((input, v))
}

fn parse_line2(input: &str) -> IResult<&str, u64> {
    let (input, _) = bytes::complete::take_until(":")(input)?;
    let (input, _) = character::complete::char(':')(input)?;
    let (input, _) = multi::many1(character::complete::space1)(input)?;
    let (input, v) = multi::separated_list0(
        multi::many1(character::complete::char(' ')),
        multi::many1(character::complete::alphanumeric1).map(|x| x.concat())
    )(input)?;
    let (input, _) = character::complete::line_ending(input)?;

    let n = v.concat().parse::<u64>().unwrap();

    Ok((input, n))
}

fn single_run(t: u64, d: u64) -> u64 {
    let mut ways_to_win = 0;
    for j in 0..=t {
        let race_time = t - j;
        let travelled = j * race_time;
        if travelled > d {
            ways_to_win += 1;
        }
    }
    ways_to_win
}

fn race(times: &Vec<u64>, distances: &Vec<u64>) -> u64 {
    let n = times.len() as u64;
    let mut res = 1;
    for i in 0..n {
        let t = times[i as usize];
        let d = distances[i as usize];
        let ways_to_win = single_run(t, d);
        res *= ways_to_win;
    }

    res
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day6/input2.txt").unwrap();
    let (input, times) = parse_line1(&input).unwrap();
    let (_, distances) = parse_line1(input).unwrap();
    println!("Day 6, Part 1: {}", race(&times, &distances));
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day6/input2.txt").unwrap();
    let (input, time) = parse_line2(&input).unwrap();
    let (_, distance) = parse_line2(input).unwrap();
    println!("Day 6, Part 1: {}", single_run(time, distance));
}

#[cfg(test)]
mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day6/input1.txt").unwrap();
        let (input, times) = super::parse_line1(&input).unwrap();
        let (_, distances) = super::parse_line1(input).unwrap();
        assert_eq!(288, super::race(&times, &distances));
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day6/input2.txt").unwrap();
        let (input, times) = super::parse_line1(&input).unwrap();
        let (_, distances) = super::parse_line1(input).unwrap();
        assert_eq!(588588, super::race(&times, &distances));
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day6/input1.txt").unwrap();
        let (input, time) = super::parse_line2(&input).unwrap();
        let (_, distance) = super::parse_line2(input).unwrap();
        assert_eq!(71503, super::single_run(time, distance));
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day6/input2.txt").unwrap();
        let (input, time) = super::parse_line2(&input).unwrap();
        let (_, distance) = super::parse_line2(input).unwrap();
        assert_eq!(34655848, super::single_run(time, distance));
    }
}
