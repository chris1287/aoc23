#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction:  Direction,
    distance:   usize,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

// fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
//   u8::from_str_radix(input, 16)
// }

// fn is_hex_digit(c: char) -> bool {
//   c.is_digit(16)
// }

// fn hex_primary(input: &str) -> nom::IResult<&str, u8> {
//     nom::combinator::map_res(
//         nom::bytes::complete::take_while_m_n(2, 2, is_hex_digit),
//         from_hex
//   )(input)
// }

// fn hex_color(input: &str) -> nom::IResult<&str, Color> {
//   let (input, _) = nom::bytes::complete::tag("#")(input)?;
//   let (input, (red, green, blue)) = nom::sequence::tuple((hex_primary, hex_primary, hex_primary))(input)?;

//   Ok((input, Color { red, green, blue }))
// }

fn parse_line(input: &str) -> nom::IResult<&str, Instruction> {
    let (input, direction) = nom::combinator::map_res(
        nom::character::complete::one_of("UDLR"),
        |c| match c {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err("Invalid direction"),
        }
    )(input)?;
    let (input, _) = nom::character::complete::space1(input)?;
    let (input, distance) = nom::combinator::map_res(
        nom::character::complete::digit1,
        |s: &str| s.parse::<usize>()
    )(input)?;
    let (input, _) = nom::bytes::complete::take_until(")")(input)?;
    let (input, _) = nom::bytes::complete::tag(")")(input)?;

    Ok((input, 
        Instruction{
        direction,
        distance,
    }))
}

fn parse_line2(input: &str) -> nom::IResult<&str, Instruction> {
    let (input, _) = nom::bytes::complete::take_until("#")(input)?;
    let (input, _) = nom::bytes::complete::tag("#")(input)?;
    let (input, distance) = nom::combinator::map_res(
        nom::bytes::complete::take_while_m_n(5, 5, |_|true),
        |s: &str| usize::from_str_radix(s, 16)
    )(input)?;
    let (input, direction) = nom::combinator::map_res(
        nom::character::complete::one_of("0123"),
        |c| match c {
            '0' => Ok(Direction::Right),
            '1' => Ok(Direction::Down),
            '2' => Ok(Direction::Left),
            '3' => Ok(Direction::Up),
            _ => Err("Invalid direction"),
        }
    )(input)?;
    let (input, _) = nom::bytes::complete::tag(")")(input)?;

    Ok((input, 
        Instruction{
        direction,
        distance,
    }))
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Instruction>> {
    let (input, v) = nom::multi::separated_list1(
        nom::character::complete::line_ending,
        parse_line
    )(input)?;

    Ok((input, v))
}

fn parse2(input: &str) -> nom::IResult<&str, Vec<Instruction>> {
    let (input, v) = nom::multi::separated_list1(
        nom::character::complete::line_ending,
        parse_line2
    )(input)?;

    Ok((input, v))
}

fn shoelace(data: &Vec<Position>) -> i64 {
    data
    .windows(2)
    .fold(0, |acc, v| {
        let x1 = v[0].x;
        let y1 = v[0].y;
        let x2 = v[1].x;
        let y2 = v[1].y;
        acc + (x1*y2 - x2*y1)
    })
    .abs() / 2
}

fn pick_theorem(area: i64, n_points: i64) -> i64 {
    area - (n_points/2) + 1
}

fn positions(data: &Vec<Instruction>) -> Vec<Position> {
    let mut v = Vec::new();
    let mut pos = Position { x: 0, y: 0 };
    v.push(pos.clone());
    for i in data {
        match i.direction {
            Direction::Up => {
                for _ in 0..i.distance {
                    pos.y += 1;
                    v.push(pos);
                }
            },
            Direction::Down => {
                for _ in 0..i.distance {
                    pos.y -= 1;
                    v.push(pos);
                }
            },
            Direction::Left => {
                for _ in 0..i.distance {
                    pos.x -= 1;
                    v.push(pos);
                }
            },
            Direction::Right => {
                for _ in 0..i.distance {
                    pos.x += 1;
                    v.push(pos);
                }
            },
        }
    }
    v
}

fn solve(data: &Vec<Instruction>) -> i64 {
    let positions = positions(data);
    let area = shoelace(&positions);
    let n_points = (positions.len()-1) as i64;
    let interior = pick_theorem(area, n_points); // should be 24
    interior + n_points
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day18/input2.txt").unwrap();
    let data = parse(&input).unwrap().1;
    println!("Day 18, Part 1: {}", solve(&data));
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day18/input2.txt").unwrap();
    let data = parse2(&input).unwrap().1;
    println!("Day 18, Part 2: {}", solve(&data));
}

mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day18/input1.txt").unwrap();
        let (_, v) = super::parse(&input).unwrap();
        assert_eq!(62, super::solve(&v));
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day18/input2.txt").unwrap();
        let (_, v) = super::parse(&input).unwrap();
        assert_eq!(62365, super::solve(&v));
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day18/input1.txt").unwrap();
        let (_, v) = super::parse2(&input).unwrap();
        assert_eq!(952408144115, super::solve(&v));
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day18/input2.txt").unwrap();
        let (_, v) = super::parse2(&input).unwrap();
        assert_eq!(159485361249806, super::solve(&v));
    }
}
