use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Cost {
    row: usize,
    col: usize,
    cost: usize,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    row: usize,
    col: usize,
    direction: Direction,
    steps_done: usize,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position2 {
    row: usize,
    col: usize,
    direction: Direction,
    steps_done: usize,
    steps_to_do: usize,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Visitor {
    costs: Vec<Cost>,
    rows: usize,
    cols: usize,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Visitor2 {
    costs: Vec<Cost>,
    rows: usize,
    cols: usize,
}

impl Visitor {
    fn cost(&self, r: usize, c: usize) -> usize {
        self.costs.iter().find(|x| x.row == r && x.col == c).unwrap().cost
    }

    fn successors(&self, p: &Position) -> Vec<(Position, usize)> {
        let mut positions = vec![];
        match p.direction {
            Direction::Up => {
                if p.row > 0 && p.steps_done < 2 {
                    positions.push((Position {
                        row: p.row - 1,
                        col: p.col,
                        direction: Direction::Up,
                        steps_done: p.steps_done + 1
                        },
                        self.cost(p.row - 1, p.col)));
                }
                if p.col > 0 {
                    positions.push((Position {
                        row: p.row,
                        col: p.col - 1,
                        direction: Direction::Left,
                        steps_done: 0
                        },
                        self.cost(p.row, p.col - 1)));
                }
                if p.col < self.cols - 1 {
                    positions.push((Position {
                        row: p.row,
                        col: p.col + 1,
                        direction: Direction::Right,
                        steps_done: 0
                        },
                        self.cost(p.row, p.col + 1)));
                }
            },
            Direction::Down => {
                if p.row < self.rows - 1 && p.steps_done < 2 {
                    positions.push((Position {
                        row: p.row + 1,
                        col: p.col,
                        direction: Direction::Down,
                        steps_done: p.steps_done + 1
                        },
                        self.cost(p.row + 1, p.col)));
                }
                if p.col > 0 {
                    positions.push((Position {
                        row: p.row,
                        col: p.col - 1,
                        direction: Direction::Left,
                        steps_done: 0
                        },
                        self.cost(p.row, p.col - 1)));
                }
                if p.col < self.cols - 1 {
                    positions.push((Position {
                        row: p.row,
                        col: p.col + 1,
                        direction: Direction::Right,
                        steps_done: 0
                        },
                        self.cost(p.row, p.col + 1)));
                }
            },
            Direction::Left => {
                if p.col > 0 && p.steps_done < 2 {
                    positions.push((Position {
                        row: p.row,
                        col: p.col - 1,
                        direction: Direction::Left,
                        steps_done: p.steps_done + 1
                        },
                        self.cost(p.row, p.col - 1)));
                }
                if p.row > 0 {
                    positions.push((Position {
                        row: p.row - 1,
                        col: p.col,
                        direction: Direction::Up,
                        steps_done: 0
                        },
                        self.cost(p.row - 1, p.col)));
                }
                if p.row < self.rows - 1 {
                    positions.push((Position {
                        row: p.row + 1,
                        col: p.col,
                        direction: Direction::Down,
                        steps_done: 0
                        },
                        self.cost(p.row + 1, p.col)));
                }
            },
            Direction::Right => {
                if p.col < self.cols - 1 && p.steps_done < 2 {
                    positions.push((Position {
                        row: p.row,
                        col: p.col + 1,
                        direction: Direction::Right,
                        steps_done: p.steps_done + 1
                        },
                        self.cost(p.row, p.col + 1)));
                }
                if p.row > 0 {
                    positions.push((Position {
                        row: p.row - 1,
                        col: p.col,
                        direction: Direction::Up,
                        steps_done: 0
                        },
                        self.cost(p.row - 1, p.col)));
                }
                if p.row < self.rows - 1 {
                    positions.push((Position {
                        row: p.row + 1,
                        col: p.col,
                        direction: Direction::Down,
                        steps_done: 0
                        },
                        self.cost(p.row + 1, p.col)));
                }
            },
        }
        positions
    }

    // fn show_path(&self, path: &Vec<Position>) {
    //     println!("\n\n");
    //     for x in self.costs.iter() {
    //         if path.iter().any(|p| p.row == x.row && p.col == x.col) {
    //             print!("#");
    //         } else {
    //             print!("{}", x.cost);
    //         }
    //         if x.col == self.cols - 1 {
    //             println!();
    //         }
    //     }
    // }
}

impl Visitor2 {
    fn cost(&self, r: usize, c: usize) -> usize {
        self.costs.iter().find(|x| x.row == r && x.col == c).unwrap().cost
    }

    fn successors(&self, p: &Position2) -> Vec<(Position2, usize)> {
        let mut positions = vec![];
        match p.direction {
            Direction::Up => {
                if p.row > 0 && p.steps_done < 9 {
                    positions.push((Position2 {
                        row: p.row - 1,
                        col: p.col,
                        direction: Direction::Up,
                        steps_done: p.steps_done + 1,
                        steps_to_do: if p.steps_to_do == 0 { 0 } else { p.steps_to_do-1 },
                        },
                        self.cost(p.row - 1, p.col)));
                }
                if p.col > 0 && p.steps_to_do == 0 {
                    positions.push((Position2 {
                        row: p.row,
                        col: p.col - 1,
                        direction: Direction::Left,
                        steps_done: 0,
                        steps_to_do: 3,
                        },
                        self.cost(p.row, p.col - 1)));
                }
                if p.col < self.cols - 1 && p.steps_to_do == 0 {
                    positions.push((Position2 {
                        row: p.row,
                        col: p.col + 1,
                        direction: Direction::Right,
                        steps_done: 0,
                        steps_to_do: 3,
                        },
                        self.cost(p.row, p.col + 1)));
                }
            },
            Direction::Down => {
                if p.row < self.rows - 1 && p.steps_done < 9 {
                    positions.push((Position2 {
                        row: p.row + 1,
                        col: p.col,
                        direction: Direction::Down,
                        steps_done: p.steps_done + 1,
                        steps_to_do: if p.steps_to_do == 0 { 0 } else { p.steps_to_do-1 },
                        },
                        self.cost(p.row + 1, p.col)));
                }
                if p.col > 0 && p.steps_to_do == 0 {
                    positions.push((Position2 {
                        row: p.row,
                        col: p.col - 1,
                        direction: Direction::Left,
                        steps_done: 0,
                        steps_to_do: 3,
                        },
                        self.cost(p.row, p.col - 1)));
                }
                if p.col < self.cols - 1 && p.steps_to_do == 0 {
                    positions.push((Position2 {
                        row: p.row,
                        col: p.col + 1,
                        direction: Direction::Right,
                        steps_done: 0,
                        steps_to_do: 3,
                        },
                        self.cost(p.row, p.col + 1)));
                }
            },
            Direction::Left => {
                if p.col > 0 && p.steps_done < 9 {
                    positions.push((Position2 {
                        row: p.row,
                        col: p.col - 1,
                        direction: Direction::Left,
                        steps_done: p.steps_done + 1,
                        steps_to_do: if p.steps_to_do == 0 { 0 } else { p.steps_to_do-1 },
                        },
                        self.cost(p.row, p.col - 1)));
                }
                if p.row > 0 && p.steps_to_do == 0 {
                    positions.push((Position2 {
                        row: p.row - 1,
                        col: p.col,
                        direction: Direction::Up,
                        steps_done: 0,
                        steps_to_do: 3,
                        },
                        self.cost(p.row - 1, p.col)));
                }
                if p.row < self.rows - 1 && p.steps_to_do == 0 {
                    positions.push((Position2 {
                        row: p.row + 1,
                        col: p.col,
                        direction: Direction::Down,
                        steps_done: 0,
                        steps_to_do: 3,
                        },
                        self.cost(p.row + 1, p.col)));
                }
            },
            Direction::Right => {
                if p.col < self.cols - 1 && p.steps_done < 9 {
                    positions.push((Position2 {
                        row: p.row,
                        col: p.col + 1,
                        direction: Direction::Right,
                        steps_done: p.steps_done + 1,
                        steps_to_do: if p.steps_to_do == 0 { 0 } else { p.steps_to_do-1 },
                        },
                        self.cost(p.row, p.col + 1)));
                }
                if p.row > 0 && p.steps_to_do == 0 {
                    positions.push((Position2 {
                        row: p.row - 1,
                        col: p.col,
                        direction: Direction::Up,
                        steps_done: 0,
                        steps_to_do: 3,
                        },
                        self.cost(p.row - 1, p.col)));
                }
                if p.row < self.rows - 1 && p.steps_to_do == 0 {
                    positions.push((Position2 {
                        row: p.row + 1,
                        col: p.col,
                        direction: Direction::Down,
                        steps_done: 0,
                        steps_to_do: 3,
                        },
                        self.cost(p.row + 1, p.col)));
                }
            },
        }
        positions
    }

    // fn show_path(&self, path: &Vec<Position2>) {
    //     println!("\n\n");
    //     for x in self.costs.iter() {
    //         if path.iter().any(|p| p.row == x.row && p.col == x.col) {
    //             print!("#");
    //         } else {
    //             print!("{}", x.cost);
    //         }
    //         if x.col == self.cols - 1 {
    //             println!();
    //         }
    //     }
    // }
}


fn parse(input: &str) -> Visitor {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();

    Visitor {
        costs: input.lines().enumerate().map(|(row, line)| {
            line.chars().enumerate().map(move |(col, c)| {
                Cost {
                    row,
                    col,
                    cost: c.to_digit(10).unwrap() as usize,
                }
            })
        })
        .flatten()
        .collect(),
        rows,
        cols,
    }
}

fn parse2(input: &str) -> Visitor2 {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();

    Visitor2 {
        costs: input.lines().enumerate().map(|(row, line)| {
            line.chars().enumerate().map(move |(col, c)| {
                Cost {
                    row,
                    col,
                    cost: c.to_digit(10).unwrap() as usize,
                }
            })
        })
        .flatten()
        .collect(),
        rows,
        cols,
    }
}

fn solve(v: &Visitor) -> usize {
    let start = Position {
        row: 0,
        col: 0,
        direction: Direction::Right,
        steps_done: 0,
    };

    let stop = Position {
        row: v.rows - 1,
        col: v.cols - 1,
        direction: Direction::Right,
        steps_done: 0,
    };

    if let Some(res) = dijkstra(&start, |p| v.successors(&p), |p| p.row == stop.row && p.col == stop.col) {
        // v.show_path(&res.0);
        return res.1;
    } else {
        panic!("no path found");
    }
}

fn solve2(v: &Visitor2) -> usize {
    let start = Position2 {
        row: 0,
        col: 0,
        direction: Direction::Right,
        steps_done: 0,
        steps_to_do: 3,
    };

    let stop = Position2 {
        row: v.rows - 1,
        col: v.cols - 1,
        direction: Direction::Right,
        steps_done: 0,
        steps_to_do: 0,
    };

    if let Some(res) = dijkstra(&start, |p| v.successors(&p), |p| p.row == stop.row && p.col == stop.col && p.steps_to_do == 0) {
        // v.show_path(&res.0);
        return res.1;
    } else {
        panic!("no path found");
    }
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day17/input2.txt").unwrap();
    let data = parse(&input);
    println!("Day 17, Part 1: {}", solve(&data));
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day17/input2.txt").unwrap();
    let data = parse2(&input);
    println!("Day 17, Part 2: {}", solve2(&data));
}

mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day17/input1.txt").unwrap();
        let data = super::parse(&input);
        assert_eq!(102, super::solve(&data));
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day17/input2.txt").unwrap();
        let data = super::parse(&input);
        assert_eq!(1044, super::solve(&data));
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day17/input1.txt").unwrap();
        let data = super::parse2(&input);
        assert_eq!(94, super::solve2(&data));
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day17/input2.txt").unwrap();
        let data = super::parse2(&input);
        assert_eq!(1227, super::solve2(&data));
    }
}