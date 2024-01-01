use grid::Grid;

fn parse(input: &str) -> Grid<char> {
    let cols = input.find('\n').expect("expected at least one newline");
    let v: Vec<char> =
    input
    .chars()
    .filter(|&c| c != '\n')
    .collect();
    Grid::from_vec(v, cols)
}

fn rotate_north(g: &mut Grid<char>) {
    for r in 1..g.rows() {
        for c in 0..g.cols() {
            if let Some(&obj) = g.get(r, c) {
                if obj == 'O' {
                    let mut cur_row = r;
                    while cur_row > 0 {
                        if let Some(&obj_above) = g.get(cur_row-1, c) {
                            if obj_above == '.' {
                                // keep moving
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                        cur_row -= 1;
                    }
                    if cur_row != r {
                        *(g.get_mut(cur_row, c).unwrap()) = 'O';
                        *(g.get_mut(r, c).unwrap()) = '.';
                    }
                }
            }
        }
    }
}

fn rotate_south(g: &mut Grid<char>) {
    for r in (0..g.rows()-1).rev() {
        for c in 0..g.cols() {
            if let Some(&obj) = g.get(r, c) {
                if obj == 'O' {
                    let mut cur_row = r;
                    while cur_row < g.rows()-1 {
                        if let Some(&obj_below) = g.get(cur_row+1, c) {
                            if obj_below == '.' {
                                // keep moving
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                        cur_row += 1;
                    }
                    if cur_row != r {
                        *(g.get_mut(cur_row, c).unwrap()) = 'O';
                        *(g.get_mut(r, c).unwrap()) = '.';
                    }
                }
            }
        }
    }
}

fn rotate_west(g: &mut Grid<char>) {
    for r in 0..g.rows() {
        for c in 1..g.cols() {
            if let Some(&obj) = g.get(r, c) {
                if obj == 'O' {
                    let mut cur_col = c;
                    while cur_col > 0 {
                        if let Some(&obj_left) = g.get(r, cur_col-1) {
                            if obj_left == '.' {
                                // keep moving
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                        cur_col -= 1;
                    }
                    if cur_col != c {
                        *(g.get_mut(r, cur_col).unwrap()) = 'O';
                        *(g.get_mut(r, c).unwrap()) = '.';
                    }
                }
            }
        }
    }
}

fn rotate_east(g: &mut Grid<char>) {
    for r in 0..g.rows() {
        for c in (0..g.cols()-1).rev() {
            if let Some(&obj) = g.get(r, c) {
                if obj == 'O' {
                    let mut cur_col = c;
                    while cur_col < g.cols()-1 {
                        if let Some(&obj_right) = g.get(r, cur_col+1) {
                            if obj_right == '.' {
                                // keep moving
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                        cur_col += 1;
                    }
                    if cur_col != c {
                        *(g.get_mut(r, cur_col).unwrap()) = 'O';
                        *(g.get_mut(r, c).unwrap()) = '.';
                    }
                }
            }
        }
    }
}

fn solve(g: &mut Grid<char>) -> usize {
    let mut weight = g.cols();
    rotate_north(g);
    g
    .iter_rows()
    .map(|r|{
        let w = weight * r
        .filter(|&&e| e == 'O')
        .count();
        weight -= 1;
        w
    })
    .sum()
}

fn compute_weight(g: &Grid<char>) -> usize {
    let mut weight = g.cols();
    g
    .iter_rows()
    .map(|r|{
        let w = weight * r
        .filter(|&&e| e == 'O')
        .count();
        weight -= 1;
        w
    })
    .sum()
}

fn tortoise(g: &mut Grid<char>) {
    rotate_north(g);
    rotate_west(g);
    rotate_south(g);
    rotate_east(g);
}

fn hare(g: &mut Grid<char>) {
    tortoise(g);
    tortoise(g);
}

fn solve2(g: &mut Grid<char>) -> usize {
    // println!("\n\n");
    let mut t = g.clone();
    let mut h = g.clone();
    hare(&mut h);
    tortoise(&mut t);
    while t != h {
        hare(&mut h);
        tortoise(&mut t);
    }
    // println!("Cycle detected after {} iterations", i);
    let mut mu = 0;
    t = g.clone();
    while t != h {
        tortoise(&mut t);
        tortoise(&mut h);
        mu += 1;
    }
    // println!("mu = {}", mu);

    let mut lam = 1;
    tortoise(&mut t);
    while t != h {
        tortoise(&mut t);
        lam += 1;
    }
    // println!("lam = {}", lam);

    // perform mu iterations
    for _ in 0..mu {
        tortoise(g);
    }
    // perform 1000000000 % lam iterations
    let n = (1000000000 - mu) % lam;
    // println!("n = {}", n);
    for _ in 0..n {
        tortoise(g);
    }

    compute_weight(&g)
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day14/input2.txt").unwrap();
    let mut grid = parse(&input);
    println!("Day 14, Part 1: {}", solve(&mut grid));
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day14/input2.txt").unwrap();
    let mut grid = parse(&input);
    println!("Day 14, Part 2: {}", solve2(&mut grid));
}

mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day14/input1.txt").unwrap();
        let mut grid = super::parse(&input);
        let res = super::solve(&mut grid);
        assert_eq!(136, res);
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day14/input2.txt").unwrap();
        let mut grid = super::parse(&input);
        let res = super::solve(&mut grid);
        assert_eq!(105249, res);
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day14/input1.txt").unwrap();
        let mut grid = super::parse(&input);
        let res = super::solve2(&mut grid);
        assert_eq!(64, res);
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day14/input2.txt").unwrap();
        let mut grid = super::parse(&input);
        let res = super::solve2(&mut grid);
        assert_eq!(88680, res);
    }
}