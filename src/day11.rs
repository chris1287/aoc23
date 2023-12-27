use transpose::transpose;
use std::fmt;

struct Universe {
    data: Vec<u8>,
    r: usize,
    c: usize
}

struct Universe2 {
    data: Vec<u8>,
    r: usize,
    c: usize,
    row_costs: Vec<usize>,
    col_costs: Vec<usize>
}

impl fmt::Debug for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}\n", self.r, self.c)?;
        for row in self.data.chunks(self.c) {
            for c in row {
                write!(f, "{}", *c as char)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Universe2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}\n", self.r, self.c)?;
        write!(f, "Row costs: {:?}\n", self.row_costs)?;
        write!(f, "Col costs: {:?}\n", self.col_costs)?;
        for row in self.data.chunks(self.c) {
            for c in row {
                write!(f, "{}", *c as char)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Universe {
    fn rotate(&mut self) {
        let mut transposed = vec![0; self.data.len()];
        transpose(&mut self.data, &mut transposed, self.c, self.r);
        self.data = transposed;
        let r = self.r;
        self.r = self.c;
        self.c = r;
    }

    fn expand_rows(&mut self) {
        let mut expanded = Vec::new();
        for row in self.data.chunks(self.c) {
            expanded.extend_from_slice(row);
            if !row.contains(&b'#') {
                // expand
                expanded.extend_from_slice(row);
                self.r += 1;
            }
        }
        self.data = expanded;
    }

    fn expand(&mut self) {
        self.expand_rows();
        self.rotate();
        self.expand_rows();
        self.rotate();
    }
}

fn parse_input(input: &str) -> Universe {
    let n_cols = input.split("\n").next().unwrap().len();
    let n_rows = input.split("\n").count();
    let v = input.as_bytes();
    let mut data = Vec::new();
    for c in v {
        if *c != b'\n' {
            data.push(*c);
        }
    }
    Universe {
        data,
        r: n_rows,
        c: n_cols
    }
}

fn parse_input2(input: &str, extra_cost: usize) -> Universe2 {
    let n_cols = input.split("\n").next().unwrap().len();
    let n_rows = input.split("\n").count();
    let v = input.as_bytes();
    let mut data = Vec::new();
    for c in v {
        if *c != b'\n' {
            data.push(*c);
        }
    }

    let mut row_costs = vec![0; n_rows];
    let mut col_costs = vec![0; n_cols];
    let mut current_row = 0;
    for row in data.chunks(n_cols) {
        if row.contains(&b'#') {
            row_costs[current_row] = 1;
        } else {
            row_costs[current_row] = extra_cost;
        }
        current_row += 1;
    }

    let mut current_col = 0;
    let mut transposed = vec![0; data.len()];
    transpose(&mut data, &mut transposed, n_cols, n_rows);
    for col in transposed.chunks(n_rows) {
        if col.contains(&b'#') {
            col_costs[current_col] = 1;
        } else {
            col_costs[current_col] = extra_cost;
        }
        current_col += 1;
    }

    Universe2 {
        data,
        r: n_rows,
        c: n_cols,
        row_costs,
        col_costs
    }
}

fn solve(universe: &Universe) -> usize {
    let mut sum = 0;
    let mut positions = Vec::new();
    for (idx, c) in universe.data
        .iter()
        .enumerate() {
        if *c == b'#' {
            let row = idx / universe.c;
            let col = idx % universe.c;
            // println!("Galaxy: {}, {}", row, col);
            positions.push((row, col));
        }
    }

    for i in 0..positions.len() {
        for j in i+1..positions.len() {
            let a = positions[i];
            let b = positions[j];
            let d_row;
            let d_col;
            if a.0 > b.0 {
                d_row = a.0 - b.0;
            } else {
                d_row = b.0 - a.0;
            }
            if a.1 > b.1 {
                d_col = a.1 - b.1;
            } else {
                d_col = b.1 - a.1;
            }
            let d = d_row + d_col;
            // println!("Distance between {:?} and {:?} is {}", a, b, d);
            sum += d;
        }
    }

    sum
}

fn solve2(universe: &Universe2) -> usize {
    let mut sum = 0;
    let mut positions = Vec::new();
    for (idx, c) in universe.data
        .iter()
        .enumerate() {
        if *c == b'#' {
            let row = idx / universe.c;
            let col = idx % universe.c;
            // println!("Galaxy: {}, {}", row, col);
            positions.push((row, col));
        }
    }

    for i in 0..positions.len() {
        for j in i+1..positions.len() {
            let a = positions[i];
            let b = positions[j];
            let mut d_row = 0;
            let mut d_col = 0;
            // compute row costs
            if a.0 < b.0 {
                for cost in universe.row_costs[a.0..b.0].iter() {
                    d_row += cost;
                }
            } else {
                for cost in universe.row_costs[b.0..a.0].iter() {
                    d_row += cost;
                }
            }
            // compute col costs
            if a.1 < b.1 {
                for cost in universe.col_costs[a.1..b.1].iter() {
                    d_col += cost;
                }
            } else {
                for cost in universe.col_costs[b.1..a.1].iter() {
                    d_col += cost;
                }
            }
            let d = d_row + d_col;
            // println!("Distance between {:?} and {:?} is {}", a, b, d);
            sum += d;
        }
    }

    sum
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day11/input2.txt").unwrap();
    let mut universe = parse_input(&input);
    universe.expand();
    let res = solve(&universe);
    println!("Day 11, Part 1: {}", res);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day11/input2.txt").unwrap();
    let universe = parse_input2(&input, 1000000);
    let res = solve2(&universe);
    println!("Day 11, Part 2: {}", res);
}

mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day11/input1.txt").unwrap();
        let mut universe = super::parse_input(&input);
        universe.expand();
        let res = super::solve(&universe);
        assert_eq!(res, 374);
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day11/input2.txt").unwrap();
        let mut universe = super::parse_input(&input);
        universe.expand();
        let res = super::solve(&universe);
        assert_eq!(res, 9521550);
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day11/input1.txt").unwrap();
        let universe = super::parse_input2(&input, 10);
        let res = super::solve2(&universe);
        assert_eq!(res, 1030);

        let universe = super::parse_input2(&input, 100);
        let res = super::solve2(&universe);
        assert_eq!(res, 8410);
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day11/input2.txt").unwrap();
        let universe = super::parse_input2(&input, 1000000);
        let res = super::solve2(&universe);
        assert_eq!(res, 298932923702);
    }
}
