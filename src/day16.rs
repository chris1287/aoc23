#[derive(Debug, Clone)]
enum CellType {
    Empty,
    ForwardSlash,
    BackwardSlash,
    Dash,
    VerticalBar,
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Cell {
    cell_type: CellType,
    energized: bool,
}

#[derive(Debug, Clone)]
struct Maze {
    rows: usize,
    cols: usize,
    cells: Vec<Vec<Cell>>,
}

#[derive(Debug, Clone)]
struct VisitedCell {
    row: usize,
    col: usize,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Beam {
    row: usize,
    col: usize,
    direction: Direction,
}

fn recurse(beam: &mut Beam, maze: &mut Maze, visited_cells: &mut Vec<VisitedCell>) {
    loop {
        if visited_cells.iter().any(|vc| vc.row == beam.row && vc.col == beam.col && vc.direction == beam.direction) {
            return;
        }

        // dbg!(beam.row, beam.col, &beam.direction, beam.visited_cells.len());

        maze.cells[beam.row][beam.col].energized = true;

        visited_cells.push(VisitedCell {
            row: beam.row,
            col: beam.col,
            direction: beam.direction.clone(),
        });

        let mut new_beam = None;

        match maze.cells[beam.row][beam.col].cell_type {
            CellType::Empty => {
                match beam.direction {
                    Direction::Up => {
                        if beam.row > 0 {
                            beam.row -= 1;
                        } else {
                            return;
                        }
                    }
                    Direction::Down => {
                        if beam.row < maze.rows - 1 {
                            beam.row += 1;
                        } else {
                            return;
                        }
                    }
                    Direction::Left => {
                        if beam.col > 0 {
                            beam.col -= 1;
                        } else {
                            return;
                        }
                    }
                    Direction::Right => {
                        if beam.col < maze.cols - 1 {
                            beam.col += 1;
                        } else {
                            return;
                        }
                    }
                }
            }
            CellType::ForwardSlash => {
                match beam.direction {
                    Direction::Up => {
                        if beam.col < maze.cols - 1 {
                            beam.col += 1;
                            beam.direction = Direction::Right;
                        } else {
                            return;
                        }
                    }
                    Direction::Down => {
                        if beam.col > 0 {
                            beam.col -= 1;
                            beam.direction = Direction::Left;
                        } else {
                            return;
                        }
                    }
                    Direction::Left => {
                        if beam.row < maze.rows - 1 {
                            beam.row += 1;
                            beam.direction = Direction::Down;
                        } else {
                            return;
                        }
                    }
                    Direction::Right => {
                        if beam.row > 0 {
                            beam.row -= 1;
                            beam.direction = Direction::Up;
                        } else {
                            return;
                        }
                    }
                }
            }
            CellType::BackwardSlash => {
                match beam.direction {
                    Direction::Up => {
                        if beam.col > 0 {
                            beam.col -= 1;
                            beam.direction = Direction::Left;
                        } else {
                            return;
                        }
                    }
                    Direction::Down => {
                        if beam.col < maze.cols - 1 {
                            beam.col += 1;
                            beam.direction = Direction::Right;
                        } else {
                            return;
                        }
                    }
                    Direction::Left => {
                        if beam.row > 0 {
                            beam.row -= 1;
                            beam.direction = Direction::Up;
                        } else {
                            return;
                        }
                    }
                    Direction::Right => {
                        if beam.row < maze.rows - 1 {
                            beam.row += 1;
                            beam.direction = Direction::Down;
                        } else {
                            return;
                        }
                    }
                }
            }
            CellType::Dash => {
                match beam.direction {
                    Direction::Up | Direction::Down => {
                        new_beam = Some(beam.clone());
                        if beam.col > 0 {
                            beam.direction = Direction::Left;
                            beam.col -= 1;
                        }
                        if let Some(x) = new_beam.as_mut() {
                            if x.col < maze.cols - 1 {
                                x.direction = Direction::Right;
                                x.col += 1;
                            }
                        }
                    },
                    Direction::Left => {
                        if beam.col > 0 {
                            beam.col -= 1;
                        } else {
                            return;
                        }
                    },
                    Direction::Right => {
                        if beam.col < maze.cols - 1 {
                            beam.col += 1;
                        } else {
                            return;
                        }
                    },
                }
            }
            CellType::VerticalBar => {
                match beam.direction {
                    Direction::Left | Direction::Right => {
                        new_beam = Some(beam.clone());
                        if beam.row > 0 {
                            beam.direction = Direction::Up;
                            beam.row -= 1;
                        }
                        if let Some(x) = new_beam.as_mut() {
                            if x.row < maze.rows - 1 {
                                x.direction = Direction::Down;
                                x.row += 1;
                            }
                        }
                    },
                    Direction::Up => {
                        if beam.row > 0 {
                            beam.row -= 1;
                        } else {
                            return;
                        }
                    },
                    Direction::Down => {
                        if beam.row < maze.rows - 1 {
                            beam.row += 1;
                        } else {
                            return;
                        }
                    },
                }
            }
        }

        if let Some(x) = new_beam.as_mut() {
            recurse(x, maze, visited_cells);
        }
    }
}

fn parse(input: &str) -> Maze {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();
    
    Maze {
        rows,
        cols,
        cells: input.lines().map(|line|{
            line.chars().map(|c|{
                Cell {
                    cell_type: match c {
                        '.' => CellType::Empty,
                        '/' => CellType::ForwardSlash,
                        '\\' => CellType::BackwardSlash,
                        '-' => CellType::Dash,
                        '|' => CellType::VerticalBar,
                        _ => panic!("Unknown cell type: {}", c),
                    },
                    energized: false,
                }
            }).collect()
        }).collect()
    }
}

fn solve(maze: &mut Maze, beam: &mut Beam) -> usize {
    recurse(beam, maze, &mut Vec::new());
    
    maze.cells.iter().map(|row|{
        row.iter().filter(|cell| cell.energized).count()
    }).sum()
}

fn solve2(maze: &mut Maze) -> usize {
    let mut max = 0;
    for row in 0..maze.rows {
        let mut beam = Beam {
            row,
            col: 0,
            direction: Direction::Right,
        };
        let n = solve(&mut maze.clone(), &mut beam);
        if n > max {
            max = n;
        }
        let mut beam = Beam {
            row,
            col: maze.cols - 1,
            direction: Direction::Left,
        };
        let n = solve(&mut maze.clone(), &mut beam);
        if n > max {
            max = n;
        }
    }
    for col in 0..maze.cols {
        let mut beam = Beam {
            row: 0,
            col,
            direction: Direction::Down,
        };
        let n = solve(&mut maze.clone(), &mut beam);
        if n > max {
            max = n;
        }
        let mut beam = Beam {
            row: maze.rows - 1,
            col,
            direction: Direction::Up,
        };
        let n = solve(&mut maze.clone(), &mut beam);
        if n > max {
            max = n;
        }
    }
    max
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day16/input2.txt").unwrap();
    let mut data = parse(&input);
    let mut beam = Beam {
        row: 0,
        col: 0,
        direction: Direction::Right
    };
    println!("Day 16, Part 1: {}", solve(&mut data, &mut beam));
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day16/input2.txt").unwrap();
    let mut data = parse(&input);
    println!("Day 16, Part 2: {}", solve2(&mut data));
}

mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day16/input1.txt").unwrap();
        let mut data = super::parse(&input);
        let mut beam = super::Beam {
            row: 0,
            col: 0,
            direction: super::Direction::Right
        };
        assert_eq!(46, super::solve(&mut data, &mut beam));
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day16/input2.txt").unwrap();
        let mut data = super::parse(&input);
        let mut beam = super::Beam {
            row: 0,
            col: 0,
            direction: super::Direction::Right
        };
        assert_eq!(8901, super::solve(&mut data, &mut beam));
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day16/input1.txt").unwrap();
        let mut data = super::parse(&input);
        assert_eq!(51, super::solve2(&mut data));
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day16/input2.txt").unwrap();
        let mut data = super::parse(&input);
        assert_eq!(9064, super::solve2(&mut data));
    }
}
