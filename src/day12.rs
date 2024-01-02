use std::collections::HashMap;

type Cache = HashMap::<(String, Vec<usize>), usize>;

#[derive(Debug, Default)]
struct Record {
    seed: String,
    groups: Vec<usize>,
}

fn parse(input: &str) -> Vec<Record> {
    let mut records = Vec::new();
    for line in input.lines() {
        let mut v = line.split(" ");
        let seed = v.next().expect("should have seed");
        let groups = v.next().expect("should have groups");
        let groups = groups.split(",").map(|s| s.parse::<usize>().expect("should be a number")).collect::<Vec<_>>();
        records.push(Record {
            seed: seed.to_string(),
            groups,
        });
    }
    records
}

fn parse2(input: &str) -> Vec<Record> {
    let mut records = Vec::new();
    for line in input.lines() {
        let mut v = line.split(" ");
        let seed = v.next().expect("should have seed");
        let groups = v.next().expect("should have groups");
        let groups = format!("{},{},{},{},{}", groups, groups, groups, groups, groups);
        let groups = groups.split(",").map(|s| s.parse::<usize>().expect("should be a number")).collect::<Vec<_>>();

        let seed = format!("{}?{}?{}?{}?{}", seed, seed, seed, seed, seed);

        records.push(Record {
            seed: seed.to_string(),
            groups,
        });
    }
    records
}

fn solve(records: &mut Vec<Record>) -> usize {
    records
    // .par_iter_mut()
    .iter_mut()
    .map(|record|{
        let mut memo = Cache::new();
        recurse(&record.seed, &record.groups, &mut memo)
    })
    .sum()
}

fn on_dot(
    seed: &str,
    groups: &[usize],
    cache: &mut Cache,
) -> usize {
    if seed.len() >= 1 {
        let res = recurse(&seed[1..], groups, cache);
        cache.insert((seed[1..].to_owned(), groups.to_vec()), res);
        return res;
    }

    if groups.len() == 0 {
        cache.insert((seed.to_owned(), groups.to_vec()), 1);
        return 1;
    }

    cache.insert((seed.to_owned(), groups.to_vec()), 0);
    return 0;
}

fn on_hash(
    seed: &str,
    groups: &[usize],
    cache: &mut Cache,
) -> usize {

    // # ()
    if groups.len() == 0 {
        cache.insert((seed.to_owned(), groups.to_vec()), 0);
        return 0;
    }

    // ## (4)
    if groups.iter().sum::<usize>() > seed.len() {
        // no way to fit a group here
        cache.insert((seed.to_owned(), groups.to_vec()), 0);
        return 0;
    }

    // ##.# (4)
    if seed[0..groups[0]].chars().into_iter().any(|c| c == '.') {
        // There is a dot in the group, no way to fit
        cache.insert((seed.to_owned(), groups.to_vec()), 0);
        return 0;
    }

    // #### (3)
    if seed.len() > groups[0] && seed.chars().nth(groups[0]).unwrap()  == '#' {
        // Group is not closed, invalid config
        cache.insert((seed.to_owned(), groups.to_vec()), 0);
        return 0;
    }

    // ## (3)
    if groups.len() == 1 {
        if seed.chars().count() < groups[0] {
            cache.insert((seed.to_owned(), groups.to_vec()), 0);
            return 0;
        }
    }

    // ###.## (3, 2)
    if seed.len() > groups[0] {
        let res = recurse(&seed[groups[0]+1..], &groups[1..], cache);
        cache.insert((seed[groups[0]+1..].to_owned(), groups[1..].to_vec()), res);
        return res;
    }

    // ### (3)
    cache.insert((seed.to_owned(), groups.to_vec()), 1);
    return 1
}

fn recurse(
    seed: &str,
    groups: &[usize],
    cache: &mut Cache,
) -> usize {

    let k = (seed.to_owned(), groups.to_vec());
    if cache.contains_key(&k) {
        return cache.get(&k).unwrap().clone();
    }

    match seed.chars().next() {
        Some('.') => {
            return on_dot(seed, groups, cache);
        },
        Some('#') => {
            return on_hash(seed, groups, cache);
        },
        Some('?') => {
            return
                // consider this a '.': 
                on_dot(seed, groups, cache) + 
                // consider this a '#':
                on_hash(seed, groups, cache);
        },
        None => {
            if groups.len() == 0 {
                return 1;
            }
            return 0;
        }
        _ => panic!("Invalid symbol"),
    }
}

pub fn solve1() -> usize {
    let input = std::fs::read_to_string("data/day12/input2.txt").unwrap();
    let mut records = parse(&input);
    solve(&mut records)
}

pub fn solve2() -> usize {
    let input = std::fs::read_to_string("data/day12/input2.txt").unwrap();
    let mut records = parse2(&input);
    solve(&mut records)
}

pub fn part1() {
    println!("Day 12, Part 1: {}", solve1());
}

pub fn part2() {
    println!("Day 12, Part 2: {}", solve2());
}

mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day12/input1.txt").unwrap();
        let mut records = super::parse(&input);
        let res = super::solve(&mut records);
        assert_eq!(res, 21);
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day12/input2.txt").unwrap();
        let mut records = super::parse(&input);
        let res = super::solve(&mut records);
        assert_eq!(res, 7541);
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day12/input1.txt").unwrap();
        let mut records = super::parse2(&input);
        let res = super::solve(&mut records);
        assert_eq!(res, 525152);
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day12/input2.txt").unwrap();
        let mut records = super::parse2(&input);
        let res = super::solve(&mut records);
        assert_eq!(res, 17485169859432);
    }
}