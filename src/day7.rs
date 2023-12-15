use nom::{
    bytes::complete::tag,
    character::complete::{
        space1,
        digit1,
        line_ending
    },
    multi::{
        many1,
        separated_list1
    },
    branch::alt,
    IResult
};

use std::cmp::Ordering;
use std::fmt;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    value: u8,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    points: u32,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", 
            self.cards.iter()
               .map(|x| symbolic_value(x.value))
               .collect::<String>(),
            self.bid
        )
    }
}

fn symbolic_value(value: u8) -> String {
    match value {
        1 => "*".to_string(),
        2 => "2".to_string(),
        3 => "3".to_string(),
        4 => "4".to_string(),
        5 => "5".to_string(),
        6 => "6".to_string(),
        7 => "7".to_string(),
        8 => "8".to_string(),
        9 => "9".to_string(),
        10 => "T".to_string(),
        11 => "J".to_string(),
        12 => "Q".to_string(),
        13 => "K".to_string(),
        14 => "A".to_string(),
        _ => "X".to_string()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FiveOfAKind {
    value: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FourOfAKind {
    value: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FullHouse {
    three: u8,
    two: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ThreeOfAKind {
    value: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TwoPair {
    high: u8,
    low: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct OnePair {
    value: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HighCard {
    value: u8,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct HandPoints {
    five_of_a_kind: Option<FiveOfAKind>,
    four_of_a_kind: Option<FourOfAKind>,
    full_house: Option<FullHouse>,
    three_of_a_kind: Option<ThreeOfAKind>,
    two_pair: Option<TwoPair>,
    one_pair: Option<OnePair>,
    high_card: Option<HighCard>,
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, value) = alt((
        tag("2"),
        tag("3"),
        tag("4"),
        tag("5"),
        tag("6"),
        tag("7"),
        tag("8"),
        tag("9"),
        tag("T"),
        tag("J"),
        tag("Q"),
        tag("K"),
        tag("A"),
    ))(input)?;

    let value = match value {
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "T" => 10,
        "J" => 11,
        "Q" => 12,
        "K" => 13,
        "A" => 14,
        _ => unreachable!(),
    };

    Ok((input, Card { value }))
}

fn parse_card2(input: &str) -> IResult<&str, Card> {
    let (input, value) = alt((
        tag("2"),
        tag("3"),
        tag("4"),
        tag("5"),
        tag("6"),
        tag("7"),
        tag("8"),
        tag("9"),
        tag("T"),
        tag("J"),
        tag("Q"),
        tag("K"),
        tag("A"),
    ))(input)?;

    let value = match value {
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "T" => 10,
        "J" => 1,
        "Q" => 12,
        "K" => 13,
        "A" => 14,
        _ => unreachable!(),
    };

    Ok((input, Card { value }))
}

fn parse_line(input: &str) -> IResult<&str, Hand> {
    // 32T3K 765
    let (input, cards) = many1(parse_card)(input)?;
    let (input, _) = space1(input)?;
    let (input, bid) = digit1(input)?;

    let bid = bid.parse::<u32>().unwrap();
    let hand = Hand { cards, bid, points: 0};

    Ok((input, hand))
}

fn parse_line2(input: &str) -> IResult<&str, Hand> {
    // 32T3K 765
    let (input, cards) = many1(parse_card2)(input)?;
    let (input, _) = space1(input)?;
    let (input, bid) = digit1(input)?;

    let bid = bid.parse::<u32>().unwrap();
    let hand = Hand { cards, bid, points: 0};

    Ok((input, hand))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = separated_list1(
        line_ending,
        parse_line
    )(input)?;

    Ok((input, hands))
}

fn parse_lines2(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = separated_list1(
        line_ending,
        parse_line2
    )(input)?;

    Ok((input, hands))
}

fn compute_hands(hands: &mut Vec<Hand>) -> usize {
    let mut total_winnings = 0;

    for hand in hands.iter_mut() {
        let values = hand.cards
            .iter()
            .counts_by(|x| x.value)
            .values()
            .sorted()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .concat();
        let points = match values.as_str() {
            "11111" => 1,
            "1112" => 2,
            "122" => 3,
            "113" => 4,
            "23" => 5,
            "14" => 6,
            "5" => 7,
            _ => unreachable!(),
        };
        hand.points = points;
    }
    hands.sort_by(|a, b|{
        match b.points.cmp(&a.points) {
            Ordering::Equal => {
                for (x, y) in a.cards.iter().zip(b.cards.iter()) {
                    if x.value != y.value {
                        return y.value.cmp(&x.value);
                    } else {
                        continue;
                    }
                }
                Ordering::Equal
            },
            x => x
        }
    });
    hands
        .iter()
        .rev()
        .enumerate()
        .for_each(|(i, x)| {
            total_winnings += (i + 1) * (x.bid as usize);
        });

    total_winnings
}

fn compute_hands2(hands: &mut Vec<Hand>) -> usize {
    let mut total_winnings = 0;

    for hand in hands.iter_mut() {
        let values = hand.cards
            .iter()
            .filter(|x| x.value != 1)
            .counts_by(|x| x.value)
            .values()
            .sorted()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .concat();
        let points = match values.as_str() {
            // 5 cards, no jokers
            "11111" => 1,
            "1112" => 2, // One Pair
            "122" => 3, // Two Pair
            "113" => 4, // Three of a Kind
            "23" => 5, // Full House
            "14" => 6, // Four of a Kind
            "5" => 7, // Five of a Kind
            // 4 cards, 1 joker
            "1111" => 2, // One Pair
            "112" => 4, // Three of a Kind
            "22" => 5, // Full House
            "13" => 6, // Four of a Kind
            "4" => 7, // Five of a Kind
            // 3 cards, 2 jokers
            "111" => 4, // Three of a Kind
            "12" => 6, // Four of a Kind
            "3" => 7, // Five of a Kind
            // 2 cards, 3 jokers
            "11" => 6, // Four of a Kind
            "2" => 7, // Five of a Kind
            // 1 card, 4 jokers
            "1" => 7, // Five of a Kind
            // 0 cards, 5 jokers
            "" => 7, // Five of a Kind
            _ => unreachable!(),
        };
        hand.points = points;
    }
    hands.sort_by(|a, b|{
        match b.points.cmp(&a.points) {
            Ordering::Equal => {
                for (x, y) in a.cards.iter().zip(b.cards.iter()) {
                    if x.value != y.value {
                        return y.value.cmp(&x.value);
                    } else {
                        continue;
                    }
                }
                Ordering::Equal
            },
            x => x
        }
    });

    hands
        .iter()
        .rev()
        .enumerate()
        .for_each(|(i, x)| {
            total_winnings += (i + 1) * (x.bid as usize);
        });

    total_winnings
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day7/input2.txt").unwrap();
    let (_, mut hands) = parse_lines(&input).unwrap();
    println!("Day 7, Part 1: {}", compute_hands(&mut hands));
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day7/input2.txt").unwrap();
    let (_, mut hands) = parse_lines2(&input).unwrap();
    println!("Day 7, Part 2: {}", compute_hands2(&mut hands));
}

#[cfg(test)]
mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day7/input1.txt").unwrap();
        let (_, mut hands) = super::parse_lines(&input).unwrap();
        assert_eq!(6440, super::compute_hands(&mut hands));
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day7/input2.txt").unwrap();
        let (_, mut hands) = super::parse_lines(&input).unwrap();
        assert_eq!(253954294, super::compute_hands(&mut hands));
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day7/input1.txt").unwrap();
        let (_, mut hands) = super::parse_lines2(&input).unwrap();
        assert_eq!(5905, super::compute_hands2(&mut hands));
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day7/input2.txt").unwrap();
        let (_, mut hands) = super::parse_lines2(&input).unwrap();
        assert_eq!(254837398, super::compute_hands2(&mut hands));
    }
}
