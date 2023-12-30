use grid::Grid;

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

fn is_symmetric_horizontally(g: &Grid<char>, a: usize, b: usize) -> bool{
    let mut a = a;
    let mut b = b;
    while a < b {
        let r1: Vec<&char> = g.iter_rows().nth(a).unwrap().collect();
        let r2: Vec<&char> = g.iter_rows().nth(b).unwrap().collect();
        if r1 != r2 {
            return false;
        }
        a += 1;
        b -= 1;
    }
    true
}

fn is_symmetric_vertically(g: &Grid<char>, a: usize, b: usize) -> bool{
    let mut a = a;
    let mut b = b;
    while a < b {
        let c1: Vec<&char> = g.iter_cols().nth(a).unwrap().collect();
        let c2: Vec<&char> = g.iter_cols().nth(b).unwrap().collect();
        if c1 != c2 {
            return false;
        }
        a += 1;
        b -= 1;
    }
    true
}

// fn show(g: &Grid<char>) {
//     println!("");
//     print!("     ");
//     for c in 0..g.cols() {
//         print!("{}", c%10);
//     }
//     println!("");
//     for (r, i) in g.iter_rows().enumerate() {
//         print!("{r:3}| ");
//         for e in i.into_iter() {
//             print!("{e}");
//         }
//         println!("");
//     }
// }

fn solve_horizontal_remove_top(g: &Grid<char>) -> usize {
    let mut res = 0;
    let mut n = 0;
    for i in 0..g.rows()-1 {
        if i%2 == 0 {
            n += 1;
            continue;
        }
        if is_symmetric_horizontally(g, i, g.rows()-1) {
            res = ((g.rows()-n)/2 + n) * 100;
        }
        n += 1;
    }
    res
}

fn solve_vertical_remove_left(g: &Grid<char>) -> usize {
    let mut res = 0;
    let mut n = 0;
    for i in 0..g.cols()-1 {
        if i%2 == 0 {
            n += 1;
            continue;
        }
        if is_symmetric_vertically(g, i, g.cols()-1) {
            res = ((g.cols()-n)/2 + n) * 1;
        }
        n += 1;
    }
    res
}

fn solve_vertical_remove_right(g: &Grid<char>) -> usize {
    let mut res = 0;
    let mut n = 0;
    let mut i = g.cols()-1;
    while i > 0 {
        if i%2 == 0 {
            n += 1;
            i -= 1;
            continue;
        }
        if is_symmetric_vertically(g, 0, i) {
            res = ((g.cols()-n)/2) * 1;
        }
        n += 1;
        i -= 1;
    }
    res
}

fn solve_horizontal_remove_bottom(g: &Grid<char>) -> usize {
    let mut res = 0;
    let mut n = 0;
    let mut i = g.rows()-1;
    while i > 0 {
        if i%2 == 0 {
            n += 1;
            i -= 1;
            continue;
        }
        if is_symmetric_horizontally(g, 0, i) {
            res = ((g.rows()-n)/2) * 100;
        }
        n += 1;
        i -= 1;
    }
    res
}

fn solve(grids: &Vec<Grid<char>>) -> usize {
    grids
    .iter()
    .map(|g| {
        let mut res = solve_horizontal_remove_top(g);
        res += solve_vertical_remove_left(g);
        res += solve_vertical_remove_right(g);
        res += solve_horizontal_remove_bottom(g);
        if res == 0 {
            panic!("no symmetry found");
        }
        res
    })
    .sum()
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day13/input2.txt").unwrap();
    let grids = parse(&input);
    println!("Day 13, Part 1: {}", solve(&grids));
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
}