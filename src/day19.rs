use nom::IResult;

#[derive(Debug, Eq, PartialEq)]
enum Operator {
    GreaterThan,
    LessThan,
}

#[derive(Debug)]
struct Rule {
    category: char,
    operator: Operator,
    value: u32,
}

#[derive(Debug)]
struct Jump {
    destination: String,
    condition: Option<Rule>,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    jumps: Vec<Jump>,
}

#[derive(Debug)]
struct Category {
    value: u32,
}


#[derive(Debug)]
struct Data {
    workflows: Vec<Workflow>,
    categories: Vec<Vec<Category>>,
}

fn parse_conditional_jump(input: &str) -> IResult<&str, Jump> {
    let (input, category) = nom::character::complete::one_of("xmas")(input)?;
    let (input, operator) = nom::combinator::map(
        nom::character::complete::one_of("<>"),
        |c| match c {
            '<' => Operator::LessThan,
            '>' => Operator::GreaterThan,
            _ => unreachable!(),
        }
    )(input)?;
    let (input, value) = nom::character::complete::u32(input)?;
    let (input, _) = nom::bytes::complete::tag(":")(input)?;
    let (input, destination) = nom::character::complete::alpha1(input)?;

    Ok((input, Jump{
        destination: destination.to_string(),
        condition: Some(Rule{
            category,
            operator,
            value,
        }),
    }))
}

fn parse_unconditional_jump(input: &str) -> IResult<&str, Jump> {
    let (input, destination) = nom::character::complete::alpha1(input)?;
    Ok((input, Jump{
        destination: destination.to_string(),
        condition: None
    }))
}

fn parse_jump(input: &str) -> IResult<&str, Jump> {
    let (input, jump) = nom::branch::alt((
        parse_conditional_jump,
        parse_unconditional_jump
    ))(input)?;

    Ok((input,jump))
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = nom::bytes::complete::take_until("{")(input)?;
    let (input, _) = nom::bytes::complete::tag("{")(input)?;
    let (input, jumps) = nom::multi::separated_list1(
        nom::bytes::complete::tag(","),
        parse_jump
    )(input)?;
    let (input, _) = nom::bytes::complete::tag("}")(input)?;

    Ok((input, Workflow{
        name: name.to_string(),
        jumps,
    }))
}

fn parse_workflows(input: &str) -> IResult<&str, Vec<Workflow>> {
    let (input, v) = nom::multi::separated_list1(
        nom::character::complete::line_ending,
        parse_workflow
    )(input)?;
    Ok((input, v))
}

fn parse_category(input: &str) -> IResult<&str, Category> {
    let (input, _category) = nom::character::complete::one_of("xmas")(input)?;
    let (input, _) = nom::bytes::complete::tag("=")(input)?;
    let (input, value) = nom::character::complete::u32(input)?;
    Ok((input, Category{
        value,
    }))
}

fn parse_category_set(input: &str) -> IResult<&str, Vec<Category>> {
    let (input, _) = nom::bytes::complete::tag("{")(input)?;
    let (input, v) = nom::multi::separated_list1(
        nom::bytes::complete::tag(","),
        parse_category
    )(input)?;
    let (input, _) = nom::bytes::complete::tag("}")(input)?;
    Ok((input, v))
}

fn parse_categories(input: &str) -> IResult<&str, Vec<Vec<Category>>> {
    let (input, v) = nom::multi::separated_list1(
        nom::character::complete::line_ending,
        parse_category_set
    )(input)?;
    Ok((input, v))
}

fn parse(input: &str) -> IResult<&str, Data> {
    let (input, workflows) = parse_workflows(input)?;
    let (input, _) = nom::character::complete::line_ending(input)?;
    let (input, _) = nom::character::complete::line_ending(input)?;
    let (input, categories) = parse_categories(input)?;

    Ok((input, Data{
        workflows,
        categories,
    }))
}

fn solve(data: &Data) -> u32 {
    data.categories.iter().map(|c|{
        let x = c[0].value;
        let m = c[1].value;
        let a = c[2].value;
        let s = c[3].value;
        let mut w = data.workflows.iter().find(|a| a.name == "in").expect("'in' must exist");
        let mut accept = false;
        let mut done = false;
        while !done {
            for j in w.jumps.iter() {
                if let Some(c) = &j.condition {
                    let n = match c.category {
                        'x' => x,
                        'm' => m,
                        'a' => a,
                        's' => s,
                        _ => unreachable!()
                    };
                    let success;
                    if c.operator == Operator::GreaterThan && n > c.value {
                        success = true;
                    } else if c.operator == Operator::LessThan && n < c.value {
                        success = true;
                    } else {
                        success = false;
                    }
                    if success {
                        match j.destination.as_str() {
                            "A" => {
                                accept = true;
                                done = true;
                                break;
                            },
                            "R" => {
                                accept = false;
                                done = true;
                                break;
                            },
                            _ => {
                                w = data.workflows.iter().find(|a| a.name == j.destination).expect("conditional destination must exist");
                                break;
                            }
                        }
                    } else {
                        // continue with the next jump
                    }
                } else {
                    // unconditional jump
                    match j.destination.as_str() {
                        "A" => {
                            accept = true;
                            done = true;
                            break;
                        },
                        "R" => {
                            accept = false;
                            done = true;
                            break;
                        },
                        _ => {
                            w = data.workflows.iter().find(|a| a.name == j.destination).expect("unconditional destination must exist");
                            break;
                        }
                    }
                }
            };
        }
        if accept {
            x + m + a + s
        } else {
            0
        }
    })
    .sum()
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day19/input2.txt").unwrap();
    let (_, data) = parse(&input).unwrap();
    println!("Day 19, Part 1: {}", solve(&data));
}

mod tests {
    use super::*;

    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day19/input1.txt").unwrap();
        let (_, data) = parse(&input).unwrap();
        assert_eq!(19114, solve(&data));
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day19/input2.txt").unwrap();
        let (_, data) = parse(&input).unwrap();
        assert_eq!(263678, solve(&data));
    }
}
