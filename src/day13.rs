use grid::Grid;
use itertools::Itertools;

fn parse(input: &str) -> Vec<Grid<char>> {
    let mut grids = Vec::new();
    let mut v: Vec<char> = Vec::new();
    let mut columns = 0;
    for line in input.lines() {
        if columns == 0 {
            columns = line.len();
            if columns % 2 != 1 {
                panic!("columns should be odd");
            }
        }
        if line.len() == 0 {
            // end of current grid
            grids.push(Grid::from_vec(v.clone(), columns));
            if grids.last().unwrap().rows() % 2 != 1 {
                panic!("rows should be odd");
            }
            v.clear();
            columns = 0;
        } else {
            v.extend(line.chars());
        }
    }
    // last grid
    grids.push(Grid::from_vec(v.clone(), columns));
    if grids.last().unwrap().rows() % 2 != 1 {
        panic!("rows should be odd");
    }
    grids
}

fn _show(g: &Grid<char>) {
    println!("");
    print!("     ");
    for c in 0..g.cols() {
        print!("{}", c%10);
    }
    println!("");
    for (r, i) in g.iter_rows().enumerate() {
        print!("{r:3}| ");
        for e in i.into_iter() {
            print!("{e}");
        }
        println!("");
    }
}

fn solve(grids: &Vec<Grid<char>>) -> usize {
    grids
    .iter()
    .map(|g| {
        for ((r1, a), (r2, b)) in g
        .iter_rows()
        .enumerate()
        .tuple_windows() {
            let different_elements = a.zip(b).filter(|(e1, e2)| e1 != e2).count();
            if different_elements == 0 {
                let mut i = r1;
                let mut j = r2;
                let mut symmetric = true;
                while i > 0 && j < g.rows()-1 {
                    i -= 1;
                    j += 1;
                    if g.iter_rows().nth(i).unwrap().collect::<Vec<&char>>() != g.iter_rows().nth(j).unwrap().collect::<Vec<&char>>() {
                        symmetric = false;
                        break;
                    }
                }
                if symmetric {
                    return (r1 + 1) * 100;
                }
            }
        }

        for ((c1, a), (c2, b)) in g
        .iter_cols()
        .enumerate()
        .tuple_windows() {
            let different_elements = a.zip(b).filter(|(e1, e2)| e1 != e2).count();
            if different_elements == 0 {
                let mut i = c1;
                let mut j = c2;
                let mut symmetric = true;
                while i > 0 && j < g.cols()-1 {
                    i -= 1;
                    j += 1;
                    if g.iter_cols().nth(i).unwrap().collect::<Vec<&char>>() != g.iter_cols().nth(j).unwrap().collect::<Vec<&char>>() {
                        symmetric = false;
                        break;
                    }
                }
                if symmetric {
                    return c1 + 1;
                }
            }
        }
        
        panic!("no symmetry found");
    })
    .sum()
}

fn solve2(grids: &Vec<Grid<char>>) -> usize {
    grids
    .iter()
    .map(|g| {
        for ((r1, a), (r2, b)) in g
        .iter_rows()
        .enumerate()
        .tuple_windows() {
            let mut smudge_used = false;
            let different_elements = a.zip(b).filter(|(e1, e2)| e1 != e2).count();
            if different_elements == 1 {
                smudge_used = true;
            }
            if different_elements == 0 || smudge_used {
                let mut i = r1;
                let mut j = r2;
                let mut symmetric = true;
                while i > 0 && j < g.rows()-1 {
                    i -= 1;
                    j += 1;
                    let different_elements_in_row = g.iter_rows().nth(i).unwrap().zip(g.iter_rows().nth(j).unwrap()).filter(|(e1, e2)| e1 != e2).count();
                    if different_elements_in_row == 1 && !smudge_used {
                        smudge_used = true;
                        continue;
                    }
                    if different_elements_in_row == 0 {
                        continue;
                    }
                    symmetric = false;
                    break;   
                }
                if symmetric && smudge_used {
                    return (r1 + 1) * 100;
                }
            }
        }

        for ((c1, a), (c2, b)) in g
        .iter_cols()
        .enumerate()
        .tuple_windows() {
            let mut smudge_used = false;
            let different_elements = a.zip(b).filter(|(e1, e2)| e1 != e2).count();
            if different_elements == 1 {
                smudge_used = true;
            }
            if different_elements == 0 || smudge_used {
                let mut i = c1;
                let mut j = c2;
                let mut symmetric = true;
                while i > 0 && j < g.cols()-1 {
                    i -= 1;
                    j += 1;
                    let different_elements_in_col = g.iter_cols().nth(i).unwrap().zip(g.iter_cols().nth(j).unwrap()).filter(|(e1, e2)| e1 != e2).count();
                    if different_elements_in_col == 1 && !smudge_used {
                        smudge_used = true;
                        continue;
                    }
                    if different_elements_in_col == 0 {
                        continue;
                    }
                    symmetric = false;
                    break;   
                }
                if symmetric && smudge_used {
                    return c1 + 1;
                }
            }
        }
        
        panic!("no symmetry found");
    })
    .sum()
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day13/input2.txt").unwrap();
    let grids = parse(&input);
    println!("Day 13, Part 1: {}", solve(&grids));
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day13/input2.txt").unwrap();
    let grids = parse(&input);
    println!("Day 13, Part 2: {}", solve2(&grids));
}

mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day13/input1.txt").unwrap();
        let grids = super::parse(&input);
        let res = super::solve(&grids);
        assert_eq!(405, res);
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day13/input2.txt").unwrap();
        let grids = super::parse(&input);
        let res = super::solve(&grids);
        assert_eq!(43614, res);
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day13/input1.txt").unwrap();
        let grids = super::parse(&input);
        let res = super::solve2(&grids);
        assert_eq!(400, res);
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day13/input2.txt").unwrap();
        let grids = super::parse(&input);
        let res = super::solve2(&grids);
        assert_eq!(36771, res);
    }
}