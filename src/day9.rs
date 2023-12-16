use nom::{
    bytes::complete::tag,
    character::complete::{
        line_ending,
        i32
    },
    multi::separated_list1 ,
    IResult
};

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, lines) = separated_list1(
        line_ending,
        separated_list1(
            tag(" "),
            i32
        )
    )(input)?;
    Ok((input, lines))
}

fn consume(data: &Vec<i32>) -> i32 {
    let mut res = Vec::new();
    for (b, a) in data.iter().skip(1).zip(data) {
        res.push(b - a);
    }
    if res.iter().any(|x| *x != 0) {
        return res.last().unwrap() + consume(&res);
    }

    0
}

fn start_consuming(data: &Vec<i32>) -> i32 {
    data.last().unwrap() + consume(&data)
}


fn consume2(data: &Vec<i32>) -> i32 {
    let mut res = Vec::new();
    for (b, a) in data.iter().skip(1).zip(data) {
        res.push(b - a);
    }
    if res.iter().any(|x| *x != 0) {
        return res.first().unwrap() - consume2(&res);
    }

    0
}

fn start_consuming2(data: &Vec<i32>) -> i32 {
    data.first().unwrap() - consume2(&data)
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day9/input2.txt").unwrap();
    let (_, data) = parse_lines(&input).unwrap();
    let res = data
        .iter()
        .map(|x| start_consuming(x))
        .sum::<i32>();
    println!("Day 9, Part 1: {}", res);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day9/input2.txt").unwrap();
    let (_, data) = parse_lines(&input).unwrap();
    let res = data
        .iter()
        .map(|x| start_consuming2(x))
        .sum::<i32>();
    println!("Day 9, Part 2: {}", res);
}

mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day9/input1.txt").unwrap();
        let (_, data) = super::parse_lines(&input).unwrap();
        let res = data
            .iter()
            .map(|x| super::start_consuming(x))
            .sum::<i32>();
        assert_eq!(res, 114);
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day9/input2.txt").unwrap();
        let (_, data) = super::parse_lines(&input).unwrap();
        let res = data
            .iter()
            .map(|x| super::start_consuming(x))
            .sum::<i32>();
        assert_eq!(res, 1901217887);
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day9/input1.txt").unwrap();
        let (_, data) = super::parse_lines(&input).unwrap();
        let res = data
            .iter()
            .map(|x| super::start_consuming2(x))
            .sum::<i32>();
        assert_eq!(res, 2);
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day9/input2.txt").unwrap();
        let (_, data) = super::parse_lines(&input).unwrap();
        let res = data
            .iter()
            .map(|x| super::start_consuming2(x))
            .sum::<i32>();
        assert_eq!(res, 905);
    }
}
