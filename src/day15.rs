fn parse(input: &str) -> Vec<Vec<char>> {
    let v = input.split(',').map(|x| x.chars().collect()).collect();

    v
}

fn aoc_hash(v: &Vec<char>) -> usize {
    v.iter().fold(0, |acc, &c|{
        (acc + c as usize) * 17 % 256
    })
}

fn solve(data: &Vec<Vec<char>>) -> usize {
    data
    .iter()
    .map(|block|{
        aoc_hash(block)
    })
    .sum()
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Remove,
    Insert,
}

#[derive(Debug)]
struct Chest {
    chest_id: usize,
    label: String,
    focal_length: usize,
}

#[derive(Debug)]
struct Step {
    operation: Operation,
    chest: Chest,
}


fn get_steps(data: &Vec<Vec<char>>) -> Vec<Step>
{
    let mut v = Vec::new();
    for block in data {
        let seed: Vec<char> = block.iter().take_while(|&&c| c != '=' && c != '-').map(|c| c.clone()).collect();
        let chest_id = aoc_hash(&seed);
        let operation = if block[seed.len()] == '=' {
            Operation::Insert
        } else {
            Operation::Remove
        };
        let mut focal_length = 0;
        if operation == Operation::Insert {
            focal_length = block[seed.len()+1..].into_iter().collect::<String>().parse().unwrap();
        }

        v.push(Step {
            operation,
            chest: Chest {
                chest_id,
                label: seed.into_iter().collect(),
                focal_length,
            }
        })
    } 

    v
}

fn solve2(data: &Vec<Vec<char>>) -> usize {
    let steps = get_steps(data);

    let mut chests: Vec<Vec<Chest>> = Vec::with_capacity(256);
    chests.resize_with(256, || Vec::new());

    
    for step in steps {
        let entry = chests.get_mut(step.chest.chest_id).unwrap();
        match step.operation {
            Operation::Insert => {
                let mut found = false;
                for chest in entry.iter_mut() {
                    if chest.label == step.chest.label {
                        chest.focal_length = step.chest.focal_length;
                        found = true;
                        break;
                    }
                }
                if !found {
                    entry.push(step.chest);
                }
            },
            Operation::Remove => {
                if let Some(index) = entry.iter().position(|chest| chest.label == step.chest.label) {
                    entry.remove(index);
                }
            }
        }
    }
    
   chests
   .iter()
   .map(|entry| {
       entry
       .iter()
       .enumerate()
       .map(|(slot, chest)| {
            (chest.chest_id+1) * (slot+1) * chest.focal_length 
       })
       .sum::<usize>()
   })
   .sum()
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day15/input2.txt").unwrap();
    let data = parse(&input);
    println!("Day 15, Part 1: {}", solve(&data));
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day15/input2.txt").unwrap();
    let data = parse(&input);
    println!("Day 15, Part 2: {}", solve2(&data));
}

mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day15/input1.txt").unwrap();
        let data = super::parse(&input);
        assert_eq!(1320, super::solve(&data));
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day15/input2.txt").unwrap();
        let data = super::parse(&input);
        assert_eq!(514639, super::solve(&data));
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day15/input1.txt").unwrap();
        let data = super::parse(&input);
        assert_eq!(145, super::solve2(&data));
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day15/input2.txt").unwrap();
        let data = super::parse(&input);
        assert_eq!(279470, super::solve2(&data));
    }
}