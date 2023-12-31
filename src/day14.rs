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

fn solve(g: &mut Grid<char>) -> usize {
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

pub fn part1() {
    let input = std::fs::read_to_string("data/day14/input2.txt").unwrap();
    let mut grid = parse(&input);
    println!("Day 14, Part 1: {}", solve(&mut grid));
}

pub fn part2() {
    println!("Day 14, Part 2: NA");
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
}