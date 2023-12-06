#[derive(Debug)]
struct Matrix {
    rows: usize,
    columns: usize,
    data: Vec<char>,
}

#[derive(Debug)]
struct NumberWithBoundary {
    number: usize,
    boundary: Vec<char>,
    r: usize,
    c: usize,
}

impl Matrix {
    fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            data: Vec::with_capacity(rows * columns),
        }
    }

    fn from_str(input: &str) -> Self {
        let columns = input.find('\n').unwrap();
        let rows = (input.len() / columns) - 1;
        assert!(rows > 0);
        assert!(columns > 0);
        let mut matrix = Self::new(rows, columns);
        for c in input.chars() {
            if c == '\n' {
                continue;
            }
            matrix.data.push(c);
        }
        matrix
    }

    fn print(&self) {
        for r in 0..self.rows {
            for c in 0..self.columns {
                print!("{}", self.data[r * self.columns + c]);
            }
            println!();
        }
    }

    fn at(&self, row: usize, column: usize) -> char {
        self.data[row * self.columns + column]
    }

    fn clear_at(&mut self, row: usize, column: usize) {
        self.data[row * self.columns + column] = '.';
    }

    fn boundary(&self, row: usize, column: usize, left: bool, right: bool) -> Vec<char> {
        let mut boundary = Vec::new();
        // Top
        if row > 0 && column > 0 {
            boundary.push(self.at(row - 1, column - 1));
        }
        if row > 0 {
            boundary.push(self.at(row - 1, column));
        }
        if row > 0 && column < self.columns - 1 {
            boundary.push(self.at(row - 1, column + 1));
        }

        // Current
        if column > 0 && left {
            boundary.push(self.at(row, column - 1));
        }
        if column < self.columns - 1 && right {
            boundary.push(self.at(row, column + 1));
        }

        // Bottom
        if row < self.rows - 1 && column > 0 {
            boundary.push(self.at(row + 1, column - 1));
        }
        if row < self.rows - 1 {
            boundary.push(self.at(row + 1, column));
        }
        if row < self.rows - 1 && column < self.columns - 1 {
            boundary.push(self.at(row + 1, column + 1));
        }
        boundary
    }

    fn unique_symbols_in_boundary(&self) -> Vec<NumberWithBoundary> {
        let mut res = Vec::new();
        for r in 0..self.rows {
            let mut in_a_digit = false;
            let mut current_boundary = Vec::new();
            let mut current_digit = Vec::new();
            for c in 0..self.columns {
                let cell = self.at(r, c);
                if cell.is_digit(10) {
                    if !in_a_digit {
                        in_a_digit = true;
                        let left = true;
                        let mut right = true;
                        if c < self.columns - 1 && self.at(r, c + 1).is_digit(10) {
                            right = false;
                        }
                        current_boundary = self.boundary(r, c, left, right);
                        current_digit = vec![cell];
                    } else {
                        let left = false;
                        let mut right = true;
                        if c < self.columns - 1 && self.at(r, c + 1).is_digit(10) {
                            right = false;
                        }
                        current_boundary.extend(self.boundary(r, c, left, right));
                        current_digit.push(cell);
                    }
                } else {
                    if in_a_digit {
                        in_a_digit = false;
                        current_boundary.retain(|&x| x != '.');
                        current_boundary.sort();
                        current_boundary.dedup();
                        let current_digit = current_digit.iter().collect::<String>();
                        let current_digit = current_digit.parse::<usize>().unwrap();
                        res.push(NumberWithBoundary {
                            number: current_digit,
                            boundary: current_boundary.clone(),
                            r, c
                        });
                    }
                }
                if c == self.columns - 1 && in_a_digit {
                    in_a_digit = false;
                    current_boundary.retain(|&x| x != '.');
                    current_boundary.sort();
                    current_boundary.dedup();
                    let current_digit = current_digit.iter().collect::<String>();
                    let current_digit = current_digit.parse::<usize>().unwrap();
                    res.push(NumberWithBoundary {
                        number: current_digit,
                        boundary: current_boundary.clone(),
                        r, c
                    });
                }
            }
        }
        res
    }
}

fn sum_number_with_boundaries(input: &str) -> usize {
    let matrix = Matrix::from_str(input);

    let boundary = matrix.unique_symbols_in_boundary();
    let mut sum_touching = 0;
    let mut sum_not_touching = 0;
    let mut sum_all = 0;
    for n in boundary {
        if n.boundary.len() > 0 {
            sum_touching += n.number;
        } else {
            sum_not_touching += n.number;
        }
        sum_all += n.number;
    }
    sum_touching
}

fn north_west(m: &Matrix, mut r: usize, mut c: usize) -> Option<u32> {
    let mut v = Vec::new();
    loop {
        if r>0 && c>0 && m.at(r-1, c-1).is_digit(10) {
            v.push(m.at(r-1, c-1));
        } else {
            break;
        }
        c = c - 1;
    }
    if v.len() == 0 {
        return None;
    }
    v.reverse();
    Some(v.iter().collect::<String>().parse::<u32>().unwrap())
}

fn north_east(m: &Matrix, mut r: usize, mut c: usize) -> Option<u32> {
    let mut v = Vec::new();
    loop {
        if r>0 && c<m.columns && m.at(r-1, c+1).is_digit(10) {
            v.push(m.at(r-1, c+1));
        } else {
            break;
        }
        c = c + 1;
    }
    if v.len() == 0 {
        return None;
    }
    Some(v.iter().collect::<String>().parse::<u32>().unwrap())
}

fn north(m: &Matrix, mut r: usize, mut c: usize) -> Option<u32> {
    if r>0 && m.at(r-1, c).is_digit(10) {
        // there is a number above
        let mut v = Vec::new();
        loop {
            if c>0 && m.at(r-1, c).is_digit(10) {
                c = c - 1;
            } else {
                c = c + 1;
                break;
            }
        }
        loop {
            if m.at(r-1, c).is_digit(10) {
                v.push(m.at(r-1, c));
            } else {
                break;
            }
            c = c + 1;
        }
        return Some(v.iter().collect::<String>().parse::<u32>().unwrap());
    }

    None
}

fn south(m: &Matrix, mut r: usize, mut c: usize) -> Option<u32> {
    if r<m.rows && m.at(r+1, c).is_digit(10) {
        // there is a number below
        let mut v = Vec::new();
        loop {
            if c>0 && m.at(r+1, c).is_digit(10) {
                c = c - 1;
            } else {
                c = c + 1;
                break;
            }
        }
        loop {
            if m.at(r+1, c).is_digit(10) {
                v.push(m.at(r+1, c));
            } else {
                break;
            }
            c = c + 1;
        }
        return Some(v.iter().collect::<String>().parse::<u32>().unwrap());
    }

    None
}

fn west(m: &Matrix, mut r: usize, mut c: usize) -> Option<u32> {
    let mut v = Vec::new();
    loop {
        if c>0 && m.at(r, c-1).is_digit(10) {
            v.push(m.at(r, c-1));
        } else {
            break;
        }
        c = c - 1;
    }
    v.reverse();
    if v.len() == 0 {
        return None;
    }
    Some(v.iter().collect::<String>().parse::<u32>().unwrap())
}

fn east(m: &Matrix, mut r: usize, mut c: usize) -> Option<u32> {
    let mut v = Vec::new();
    loop {
        if c<m.columns && m.at(r, c+1).is_digit(10) {
            v.push(m.at(r, c+1));
        } else {
            break;
        }
        c = c + 1;
    }
    if v.len() == 0 {
        return None;
    }
    Some(v.iter().collect::<String>().parse::<u32>().unwrap())
}

fn south_west(m: &Matrix, mut r: usize, mut c: usize) -> Option<u32> {
    let mut v = Vec::new();
    loop {
        if r<m.rows && c>0 && m.at(r+1, c-1).is_digit(10) {
            v.push(m.at(r+1, c-1));
        } else {
            break;
        }
        c = c - 1;
    }
    v.reverse();
    if v.len() == 0 {
        return None;
    }
    Some(v.iter().collect::<String>().parse::<u32>().unwrap())
}

fn south_east(m: &Matrix, mut r: usize, mut c: usize) -> Option<u32> {
    let mut v = Vec::new();
    loop {
        if r<m.rows && c<m.columns && m.at(r+1, c+1).is_digit(10) {
            v.push(m.at(r+1, c+1));
        } else {
            break;
        }
        c = c + 1;
    }
    if v.len() == 0 {
        return None;
    }
    Some(v.iter().collect::<String>().parse::<u32>().unwrap())
}

fn sum_gears_power(input: &str) -> u32 {
    let matrix = Matrix::from_str(input);
    let mut sum = 0;
    for r in 0..matrix.rows {
        for c in 0..matrix.columns {
            if matrix.at(r, c) == '*' {
                let mut nums = Vec::new();
                let n = north(&matrix, r, c);
                if n.is_some() {
                    nums.push(n.unwrap());
                } else {
                    let nw = north_west(&matrix, r, c);
                    if nw.is_some() {
                        nums.push(nw.unwrap());
                    }
                    let ne = north_east(&matrix, r, c);
                    if ne.is_some() {
                        nums.push(ne.unwrap());
                    }
                }
                let s = south(&matrix, r, c);
                if s.is_some() {
                    nums.push(s.unwrap());
                } else {
                    let sw = south_west(&matrix, r, c);
                    if sw.is_some() {
                        nums.push(sw.unwrap());
                    }
                    let se = south_east(&matrix, r, c);
                    if se.is_some() {
                        nums.push(se.unwrap());
                    }
                }
                let w = west(&matrix, r, c);
                if w.is_some() {
                    nums.push(w.unwrap());
                }
                let e = east(&matrix, r, c);
                if e.is_some() {
                    nums.push(e.unwrap());
                }
                if nums.len() == 2 {
                    let power = nums[0] * nums[1];
                    sum += power;
                }
            }
        }
    }

    sum
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day3/input2.txt").unwrap();
    println!("Day 3, Part 1: {}", sum_number_with_boundaries(&input));
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day3/input4.txt").unwrap();
    println!("Day 3, Part 2: {}", sum_gears_power(&input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day3/input1.txt").unwrap();
        let n = super::sum_number_with_boundaries(&input);
        assert_eq!(n, 4361);
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day3/input2.txt").unwrap();
        let n = super::sum_number_with_boundaries(&input);
        assert_eq!(n, 514969);
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day3/input3.txt").unwrap();
        let n = super::sum_gears_power(&input);
        assert_eq!(n, 467835);
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day3/input4.txt").unwrap();
        let n = super::sum_gears_power(&input);
        assert_eq!(n, 78915902);
    }
}
