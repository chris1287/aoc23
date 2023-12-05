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

pub fn part1() {
    let input = std::fs::read_to_string("data/day1/input2.txt").unwrap();
    println!("Day 1, Part 1: {}", compute_calibration_value_part1(&input));
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day1/input4.txt").unwrap();
    println!("Day 1, Part 2: {}", compute_calibration_value_part2(&input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day1/input1.txt").unwrap();
        assert_eq!(super::compute_calibration_value_part1(&input), 142);
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day1/input2.txt").unwrap();
        assert_eq!(super::compute_calibration_value_part1(&input), 55108);
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day1/input3.txt").unwrap();
        assert_eq!(super::compute_calibration_value_part2(&input), 281);
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day1/input4.txt").unwrap();
        assert_eq!(super::compute_calibration_value_part2(&input), 56324);
    }
}

