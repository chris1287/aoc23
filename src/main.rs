use nom::{IResult, character, multi, bytes};

#[derive(Debug)]
struct Match {
    red: u32,
    green: u32,
    blue: u32
}

#[derive(Debug)]
struct Game {
    gid: u32,
    matches: Vec<Match>
}

#[derive(Debug)]
struct Games {
    games: Vec<Game>
}

fn fancy_str_to_number_part1(input: &str) -> Option<char> {
    let input = input.to_lowercase();
    let mut result = None;
    if input.starts_with("0") {
        result = Some('0');
    } else if input.starts_with("1") {
        result = Some('1');
    } else if input.starts_with("2") {
        result = Some('2');
    } else if input.starts_with("3") {
        result = Some('3');
    } else if input.starts_with("4") {
        result = Some('4');
    } else if input.starts_with("5") {
        result = Some('5');
    } else if input.starts_with("6") {
        result = Some('6');
    } else if input.starts_with("7") {
        result = Some('7');
    } else if input.starts_with("8") {
        result = Some('8');
    } else if input.starts_with("9") {
        result = Some('9');
    }

    result
}

fn fancy_str_to_number_part2(input: &str) -> Option<char> {
    let input = input.to_lowercase();
    let mut result = None;
    if input.starts_with("zero") || input.starts_with("0") {
        result = Some('0');
    } else if input.starts_with("one") || input.starts_with("1") {
        result = Some('1');
    } else if input.starts_with("two") || input.starts_with("2") {
        result = Some('2');
    } else if input.starts_with("three") || input.starts_with("3") {
        result = Some('3');
    } else if input.starts_with("four") || input.starts_with("4") {
        result = Some('4');
    } else if input.starts_with("five") || input.starts_with("5") {
        result = Some('5');
    } else if input.starts_with("six") || input.starts_with("6") {
        result = Some('6');
    } else if input.starts_with("seven") || input.starts_with("7") {
        result = Some('7');
    } else if input.starts_with("eight") || input.starts_with("8") {
        result = Some('8');
    } else if input.starts_with("nine") || input.starts_with("9") {
        result = Some('9');
    }

    result
}

fn compute_calibration_value_part1(input: &str) -> i32 {
    let mut calibration_value = 0;
    for line in input.lines() {
        let mut a = Option::None;
        let mut b = Option::None;
        for i in 0..line.len() {
            if a.is_none() {
                a = fancy_str_to_number_part1(&line[i..]);
            }
            let tmp = fancy_str_to_number_part1(&line[i..]);
            if tmp.is_some() {
                b = tmp;
            }
        }
        if a.is_some() && b.is_some() {
            let line_value = format!("{}{}", a.unwrap(), b.unwrap());
            calibration_value += line_value.parse::<i32>().unwrap();
        }
    }

    calibration_value
}

fn compute_calibration_value_part2(input: &str) -> i32 {
    let mut calibration_value = 0;
    for line in input.lines() {
        let mut a = Option::None;
        let mut b = Option::None;
        for i in 0..line.len() {
            if a.is_none() {
                a = fancy_str_to_number_part2(&line[i..]);
            }
            let tmp = fancy_str_to_number_part2(&line[i..]);
            if tmp.is_some() {
                b = tmp;
            }
        }
        if a.is_some() && b.is_some() {
            let line_value = format!("{}{}", a.unwrap(), b.unwrap());
            calibration_value += line_value.parse::<i32>().unwrap();
        }
    }

    calibration_value
}

fn d2p1_parse_color(input: &str) -> IResult<&str, (u32, &str)> {
    // 3 blue
    let (input, count) = character::complete::u32(input)?;
    let (input, _) = bytes::complete::tag(" ")(input)?;
    let (input, color) = character::complete::alpha1(input)?;

    Ok((input, (count, color)))
}

fn d2p1_parse_colors(input: &str) -> IResult<&str, Match> {
    // 3 blue, 4 red
    let (input, v) = multi::separated_list0(
        bytes::complete::tag(", "),
        d2p1_parse_color
    )(input)?;
    
    let mut m = Match {
        red: 0,
        green: 0,
        blue: 0
    };

    for (count, color) in v {
        match color {
            "red" => m.red = count,
            "green" => m.green = count,
            "blue" => m.blue = count,
            _ => {
                panic!("Unknown color: {}", color);
            }
        }
    }

    Ok((input, m))
}

fn d2p1_parse_game(input: &str) -> IResult<&str, Game> {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let (input, _) = bytes::complete::tag("Game ")(input)?;
    let (input, gid) = character::complete::u32(input)?;
    let (input, _) = bytes::complete::tag(": ")(input)?;
    let (input, m) = multi::separated_list0(
        bytes::complete::tag("; "),
        d2p1_parse_colors
    )(input)?;

    let game = Game {
        gid: gid,
        matches: m
    };

    Ok((input, game))
}

fn d2p1_parse_games(input: &str) -> IResult<&str, Games> {
    let (_, games) = multi::separated_list0(
        bytes::complete::tag("\n"),
        d2p1_parse_game
    )(input)?;

    Ok((input, Games { games }))
}

fn sum_possibile_games(input: &str, available_cubes: Match) -> u32 {
    let games = d2p1_parse_games(input).unwrap().1;
    let mut res = 0;
    for game in games.games {
        let mut possible = true;
        for m in game.matches {
            if m.red > available_cubes.red || m.green > available_cubes.green || m.blue > available_cubes.blue {
                possible = false;
            }
        }
        if possible {
            res += game.gid;
        }
    }

    res
}

fn fewers_number_of_cubes_to_make_it_possible(input: &str) -> u32 {
    let games = d2p1_parse_games(input).unwrap().1;
    let mut res = 0;
    for game in games.games {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for m in game.matches {
            if m.red > min_red {
                min_red = m.red;
            }
            if m.green > min_green {
                min_green = m.green;
            }
            if m.blue > min_blue {
                min_blue = m.blue;
            }
        }
        let power = min_red * min_green * min_blue;
        res += power;
    }

    res
}

fn day1part1() {
    let input = std::fs::read_to_string("data/day1/input2.txt").unwrap();
    println!("Day 1, Part 1: {}", compute_calibration_value_part1(&input));
}

fn day1part2() {
    let input = std::fs::read_to_string("data/day1/input4.txt").unwrap();
    println!("Day 1, Part 2: {}", compute_calibration_value_part2(&input));
}

fn day2part1() {
    let input = std::fs::read_to_string("data/day2/input2.txt").unwrap();
    println!("Day 2, Part 1: {}", sum_possibile_games(&input, Match { red: 12, green: 13, blue: 14 }));
}

fn day2part2() {
    let input = std::fs::read_to_string("data/day2/input4.txt").unwrap();
    println!("Day 2, Part 2: {}", fewers_number_of_cubes_to_make_it_possible(&input));
}

fn main() {
    day1part1();
    day1part2();
    day2part1();
    day2part2();
}

#[cfg(test)]
mod tests {
    #[test]
    fn d1t1() {
        let input = std::fs::read_to_string("data/day1/input1.txt").unwrap();
        assert_eq!(super::compute_calibration_value_part1(&input), 142);
    }

    #[test]
    fn d1t2() {
        let input = std::fs::read_to_string("data/day1/input2.txt").unwrap();
        assert_eq!(super::compute_calibration_value_part1(&input), 55108);
    }

    #[test]
    fn d1t3() {
        let input = std::fs::read_to_string("data/day1/input3.txt").unwrap();
        assert_eq!(super::compute_calibration_value_part2(&input), 281);
    }

    #[test]
    fn d1t4() {
        let input = std::fs::read_to_string("data/day1/input4.txt").unwrap();
        assert_eq!(super::compute_calibration_value_part2(&input), 56324);
    }

    #[test]
    fn d2t1() {
        let input = std::fs::read_to_string("data/day2/input1.txt").unwrap();
        let sum = super::sum_possibile_games(&input, super::Match { red: 12, green: 13, blue: 14 });
        assert_eq!(sum, 8);
    }

    #[test]
    fn d2t2() {
        let input = std::fs::read_to_string("data/day2/input2.txt").unwrap();
        let sum = super::sum_possibile_games(&input, super::Match { red: 12, green: 13, blue: 14 });
        assert_eq!(sum, 2149);
    }

    #[test]
    fn d2t3() {
        let input = std::fs::read_to_string("data/day2/input3.txt").unwrap();
        let sum = super::fewers_number_of_cubes_to_make_it_possible(&input);
        assert_eq!(sum, 2286);
    }

    #[test]
    fn d2t4() {
        let input = std::fs::read_to_string("data/day2/input4.txt").unwrap();
        let sum = super::fewers_number_of_cubes_to_make_it_possible(&input);
        assert_eq!(sum, 71274);
    }
}

