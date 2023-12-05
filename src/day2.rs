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

fn parse_color(input: &str) -> IResult<&str, (u32, &str)> {
    // 3 blue
    let (input, count) = character::complete::u32(input)?;
    let (input, _) = bytes::complete::tag(" ")(input)?;
    let (input, color) = character::complete::alpha1(input)?;

    Ok((input, (count, color)))
}

fn parse_colors(input: &str) -> IResult<&str, Match> {
    // 3 blue, 4 red
    let (input, v) = multi::separated_list0(
        bytes::complete::tag(", "),
        parse_color
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

fn parse_game(input: &str) -> IResult<&str, Game> {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let (input, _) = bytes::complete::tag("Game ")(input)?;
    let (input, gid) = character::complete::u32(input)?;
    let (input, _) = bytes::complete::tag(": ")(input)?;
    let (input, m) = multi::separated_list0(
        bytes::complete::tag("; "),
        parse_colors
    )(input)?;

    let game = Game {
        gid: gid,
        matches: m
    };

    Ok((input, game))
}

fn parse_games(input: &str) -> IResult<&str, Games> {
    let (_, games) = multi::separated_list0(
        bytes::complete::tag("\n"),
        parse_game
    )(input)?;

    Ok((input, Games { games }))
}

fn sum_possibile_games(input: &str, available_cubes: Match) -> u32 {
    let games = parse_games(input).unwrap().1;
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
    let games = parse_games(input).unwrap().1;
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

pub fn part1() {
    let input = std::fs::read_to_string("data/day2/input2.txt").unwrap();
    println!("Day 2, Part 1: {}", sum_possibile_games(&input, Match { red: 12, green: 13, blue: 14 }));
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day2/input4.txt").unwrap();
    println!("Day 2, Part 2: {}", fewers_number_of_cubes_to_make_it_possible(&input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day2/input1.txt").unwrap();
        let sum = super::sum_possibile_games(&input, super::Match { red: 12, green: 13, blue: 14 });
        assert_eq!(sum, 8);
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day2/input2.txt").unwrap();
        let sum = super::sum_possibile_games(&input, super::Match { red: 12, green: 13, blue: 14 });
        assert_eq!(sum, 2149);
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day2/input3.txt").unwrap();
        let sum = super::fewers_number_of_cubes_to_make_it_possible(&input);
        assert_eq!(sum, 2286);
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day2/input4.txt").unwrap();
        let sum = super::fewers_number_of_cubes_to_make_it_possible(&input);
        assert_eq!(sum, 71274);
    }
}

