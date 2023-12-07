use nom::{IResult, character, multi, bytes};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Card {
    cid: u32,
    value: u32,
    winning_numbers: Vec<u32>,
    available_numbers: Vec<u32>
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let (input, _) = bytes::complete::tag("Card")(input)?;
    let (input, _) = character::complete::space1(input)?;
    let (input, cid) = character::complete::u32(input)?;
    let (input, _) = bytes::complete::tag(":")(input)?;
    let (input, _) = character::complete::space1(input)?;
    let (input, winning_numbers) = multi::separated_list0(
        character::complete::multispace1,
        character::complete::u32
    )(input)?;
    let (input, _) = character::complete::space0(input)?;
    let (input, _) = bytes::complete::tag("|")(input)?;
    let (input, _) = character::complete::space0(input)?;
    let (input, available_numbers) = multi::separated_list0(
        character::complete::multispace1,
        character::complete::u32
    )(input)?;

    let mut value = 0;
    for available_number in &available_numbers {
        if winning_numbers.contains(&available_number) {
            value += 1;
        }
    }

    Ok((input, Card {
        cid,
        value,
        winning_numbers,
        available_numbers
    }))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, v) = multi::separated_list0(
        character::complete::newline,
        parse_card
    )(input)?;


    Ok((input, v))
}

fn compute_score(input: &str) -> u32 {
    let (_, cards) = parse_cards(input).unwrap();
    let mut res = 0;
    for card in cards {
        let mut n = 0;
        for available_number in card.available_numbers {
            if card.winning_numbers.contains(&available_number) {
                if n == 0 {
                    n = 1;
                } else {
                    n *= 2;
                }
            }
        }
        res += n;
    }
    res
}

fn duplicate_cards_then_count(input: &str) -> u32 {
    let (_, cards) = parse_cards(input).unwrap();
    let mut deck = VecDeque::from(cards.clone());
    let mut card_count = deck.len() as u32;
    loop {
        let card = deck.pop_front().unwrap();
        for i in 0..card.value {
            let id_to_copy = card.cid + i;
            card_count += 1;
            deck.push_back(cards[id_to_copy as usize].clone());
        }
        if deck.is_empty() {
            break;
        }
    }
    card_count
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day4/input2.txt").unwrap();
    println!("Day 4, Part 1: {}", compute_score(&input));
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day4/input4.txt").unwrap();
    println!("Day 4, Part 2: {}", duplicate_cards_then_count(&input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day4/input1.txt").unwrap();
        assert_eq!(super::compute_score(&input), 13);
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day4/input2.txt").unwrap();
        assert_eq!(super::compute_score(&input), 21138);
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day4/input3.txt").unwrap();
        assert_eq!(super::duplicate_cards_then_count(&input), 30);
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day4/input4.txt").unwrap();
        assert_eq!(super::duplicate_cards_then_count(&input), 7185540);
    }
}
