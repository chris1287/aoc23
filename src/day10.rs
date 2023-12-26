#[allow(dead_code)]

use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    multi::separated_list1,
    multi::many1,
    branch::alt,
    combinator::map,
    IResult
};
use petgraph::graph::Graph;
use petgraph::visit::Dfs;
use std::fmt;

#[derive(Clone, Eq, PartialEq)]
enum Connections {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start
}

#[derive(Clone)]
struct Cell {
    connections: Connections,
    row: usize,
    col: usize,
}

impl fmt::Display for Connections {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Connections::NorthSouth => write!(f, "│"),
            Connections::EastWest => write!(f, "─"),
            Connections::NorthEast => write!(f, "└"),
            Connections::NorthWest => write!(f, "┘"),
            Connections::SouthWest => write!(f, "┐"),
            Connections::SouthEast => write!(f, "┌"),
            Connections::Ground => write!(f, "."),
            Connections::Start => write!(f, "S"),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.connections)
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:2},{:2})", self.row, self.col)
    }
}

fn parse(input: &str) -> IResult<&str, Graph<Cell, usize, petgraph::Undirected>> {
    let (input, lines) = separated_list1(
        line_ending,
        many1(
            alt((
                map(tag("|"), |_| Connections::NorthSouth),
                map(tag("-"), |_| Connections::EastWest),
                map(tag("L"), |_| Connections::NorthEast),
                map(tag("J"), |_| Connections::NorthWest),
                map(tag("7"), |_| Connections::SouthWest),
                map(tag("F"), |_| Connections::SouthEast),
                map(tag("."), |_| Connections::Ground),
                map(tag("S"), |_| Connections::Start),
            ))
        )
    )(input)?;

    let mut graph = Graph::<Cell, usize, petgraph::Undirected>::new_undirected();
    for (row, rows) in lines.iter().enumerate() {
        for (col, connections) in rows.iter().enumerate() {
            graph.add_node(Cell {
                connections: connections.clone(),
                row,
                col,
            });
        }
    }

    for idx1 in graph.node_indices() {
        match &graph[idx1].connections {
            Connections::NorthSouth => {
                if let Some(node) = graph.node_indices().find(|idx2|
                        graph[idx1].row > 0 &&
                        graph[*idx2].row == graph[idx1].row - 1 &&
                        graph[*idx2].col == graph[idx1].col && (
                            graph[*idx2].connections == Connections::Start ||
                            graph[*idx2].connections == Connections::NorthSouth ||
                            graph[*idx2].connections == Connections::SouthEast ||
                            graph[*idx2].connections == Connections::SouthWest
                        )
                    ) {
                        graph.add_edge(idx1, node, 1);
                    };
                if let Some(node) = graph.node_indices().find(|idx2|
                    graph[idx1].row < graph.node_count() - 1 &&
                        graph[*idx2].row == graph[idx1].row + 1 &&
                        graph[*idx2].col == graph[idx1].col && (
                            graph[*idx2].connections == Connections::Start ||
                            graph[*idx2].connections == Connections::NorthSouth ||
                            graph[*idx2].connections == Connections::NorthEast ||
                            graph[*idx2].connections == Connections::NorthWest
                        )
                    ) {
                        graph.add_edge(idx1, node, 1);
                    };
                },

            Connections::EastWest => {
                if let Some(node) = graph.node_indices().find(|idx2|
                        graph[idx1].col > 0 &&
                        graph[*idx2].row == graph[idx1].row &&
                        graph[*idx2].col == graph[idx1].col - 1 && (
                            graph[*idx2].connections == Connections::Start ||
                            graph[*idx2].connections == Connections::EastWest ||
                            graph[*idx2].connections == Connections::NorthEast ||
                            graph[*idx2].connections == Connections::NorthWest
                        )
                    ) {
                        graph.add_edge(idx1, node, 1);
                    };
                if let Some(node) = graph.node_indices().find(|idx2|
                        graph[idx1].col < graph.node_count() - 1 &&
                        graph[*idx2].row == graph[idx1].row &&
                        graph[*idx2].col == graph[idx1].col + 1 && (
                            graph[*idx2].connections == Connections::Start ||
                            graph[*idx2].connections == Connections::EastWest ||
                            graph[*idx2].connections == Connections::NorthWest ||
                            graph[*idx2].connections == Connections::SouthWest
                        )
                    ) {
                        graph.add_edge(idx1, node, 1);
                    };
                },


            Connections::NorthEast => {
                if let Some(node) = graph.node_indices().find(|idx2|
                        graph[idx1].row > 0 &&
                        graph[*idx2].row == graph[idx1].row - 1 &&
                        graph[*idx2].col == graph[idx1].col && (
                            graph[*idx2].connections == Connections::Start ||
                            graph[*idx2].connections == Connections::NorthSouth ||
                            graph[*idx2].connections == Connections::SouthWest ||
                            graph[*idx2].connections == Connections::SouthEast
                        )
                    ) {
                        graph.add_edge(idx1, node, 1);
                    };
                if let Some(node) = graph.node_indices().find(|idx2|
                        graph[idx1].col < graph.node_count() - 1 &&
                        graph[*idx2].row == graph[idx1].row &&
                        graph[*idx2].col == graph[idx1].col + 1 && (
                            graph[*idx2].connections == Connections::Start ||
                            graph[*idx2].connections == Connections::EastWest ||
                            graph[*idx2].connections == Connections::NorthWest ||
                            graph[*idx2].connections == Connections::SouthWest
                        )
                    ) {
                        graph.add_edge(idx1, node, 1);
                    };
                },

            Connections::NorthWest => {
                if let Some(node) = graph.node_indices().find(|idx2|
                        graph[idx1].row > 0 &&
                        graph[*idx2].row == graph[idx1].row - 1 &&
                        graph[*idx2].col == graph[idx1].col && (
                            graph[*idx2].connections == Connections::Start ||
                            graph[*idx2].connections == Connections::NorthSouth ||
                            graph[*idx2].connections == Connections::SouthEast ||
                            graph[*idx2].connections == Connections::SouthWest
                        )
                    ) {
                        graph.add_edge(idx1, node, 1);
                    };
                if let Some(node) = graph.node_indices().find(|idx2|
                        graph[idx1].col > 0 &&
                        graph[*idx2].row == graph[idx1].row &&
                        graph[*idx2].col == graph[idx1].col - 1 && (
                            graph[*idx2].connections == Connections::Start ||
                            graph[*idx2].connections == Connections::EastWest ||
                            graph[*idx2].connections == Connections::NorthEast ||
                            graph[*idx2].connections == Connections::SouthEast
                        )
                    ) {
                        graph.add_edge(idx1, node, 1);
                    };
                },

            Connections::SouthWest => {
                if let Some(node) = graph.node_indices().find(|idx2|
                        graph[idx1].row < graph.node_count() - 1 &&
                        graph[*idx2].row == graph[idx1].row + 1 &&
                        graph[*idx2].col == graph[idx1].col && (
                            graph[*idx2].connections == Connections::Start ||
                            graph[*idx2].connections == Connections::NorthSouth ||
                            graph[*idx2].connections == Connections::NorthEast ||
                            graph[*idx2].connections == Connections::NorthWest
                        )
                    ) {
                        graph.add_edge(idx1, node, 1);
                    };
                if let Some(node) = graph.node_indices().find(|idx2|
                        graph[idx1].col > 0 &&
                        graph[*idx2].row == graph[idx1].row &&
                        graph[*idx2].col == graph[idx1].col - 1 && (
                            graph[*idx2].connections == Connections::Start ||
                            graph[*idx2].connections == Connections::EastWest ||
                            graph[*idx2].connections == Connections::NorthEast ||
                            graph[*idx2].connections == Connections::SouthEast
                        )
                    ) {
                        graph.add_edge(idx1, node, 1);
                    };
                },

            Connections::SouthEast => {
                if let Some(node) = graph.node_indices().find(|idx2|
                        graph[idx1].row < graph.node_count() - 1 &&
                        graph[*idx2].row == graph[idx1].row + 1 &&
                        graph[*idx2].col == graph[idx1].col && (
                            graph[*idx2].connections == Connections::Start ||
                            graph[*idx2].connections == Connections::NorthSouth ||
                            graph[*idx2].connections == Connections::NorthEast ||
                            graph[*idx2].connections == Connections::NorthWest
                        )
                    ) {
                        graph.add_edge(idx1, node, 1);
                    };
                if let Some(node) = graph.node_indices().find(|idx2|
                        graph[idx1].col < graph.node_count() - 1 &&
                        graph[*idx2].row == graph[idx1].row &&
                        graph[*idx2].col == graph[idx1].col + 1 && (
                            graph[*idx2].connections == Connections::Start ||
                            graph[*idx2].connections == Connections::EastWest ||
                            graph[*idx2].connections == Connections::NorthWest ||
                            graph[*idx2].connections == Connections::SouthWest
                        )
                    ) {
                        graph.add_edge(idx1, node, 1);
                    };
                },

            _ => {}
        }
    }

    Ok((input, graph))
}

fn find_cycle_length(graph: &Graph<Cell, usize, petgraph::Undirected>) -> usize {
    if let Some(s) = graph.node_indices().find(|node| {
        graph[*node].connections == Connections::Start
    }) {
        for nbor in graph.neighbors(s) {
            let mut dfs = Dfs::new(graph, nbor);
            let mut steps = 0;
            let mut back_to_start = false;
            while let Some(node) = dfs.next(graph) {
                steps += 1;
                if node.index() == s.index() {
                    back_to_start = true;
                    break;
                }
            }
            if back_to_start {
                return steps/2;
            }
        }
        panic!("no loops found");
    } else {
        panic!("no start found");
    }
}

fn count_interior_points(graph: &Graph<Cell, usize, petgraph::Undirected>) -> i32 {
    if let Some(s) = graph.node_indices().find(|node| {
        graph[*node].connections == Connections::Start
    }) {
        for nbor in graph.neighbors(s) {
            let mut area: i32 = 0;
            let mut dfs = Dfs::new(graph, nbor);
            let mut this = s;
            let mut n_points = 0;
            while let Some(node) = dfs.next(graph) {
                // Find area with shoelace algorithm
                n_points += 1; // Keep track of boundary points
                let x1 = graph[this].row as i32;
                let y1 = graph[this].col as i32;
                let x2 = graph[node].row as i32;
                let y2 = graph[node].col as i32;
                let n = x1*y2 - x2*y1;
                area += n;
                if node.index() == s.index() {
                    // this is a closed loop
                    area /= 2;
                    if area < 0 {
                        area = -area;
                    }
                    // Pick theorem:
                    // Area = interior_points + boundary_points/2 - 1
                    // We already have Area and boundary_points, we need to compute interior points.
                    // So: 
                    // res = Area - n_points/2 - 1
                    return area - (n_points/2) + 1;
                }
                this = node;
            }
        }
        panic!("no loops found");
    } else {
        panic!("no start found");
    }
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day10/input2.txt").unwrap();
    let (_, g) = parse(&input).unwrap();
    let steps = find_cycle_length(&g);
    println!("Day 10, Part 1: {}", steps);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day10/input2.txt").unwrap();
    let (_, g) = parse(&input).unwrap();
    let n = count_interior_points(&g);
    println!("Day 10, Part 2: {}", n);
}

mod tests {
    #[test]
    fn t1() {
        let input = std::fs::read_to_string("data/day10/input1.txt").unwrap();
        let (_, g) = super::parse(&input).unwrap();
        let steps = super::find_cycle_length(&g);
        assert_eq!(steps, 8);
    }

    #[test]
    fn t2() {
        let input = std::fs::read_to_string("data/day10/input2.txt").unwrap();
        let (_, g) = super::parse(&input).unwrap();
        let steps = super::find_cycle_length(&g);
        assert_eq!(steps, 6733);
    }

    #[test]
    fn t3() {
        let input = std::fs::read_to_string("data/day10/input5.txt").unwrap();
        let (_, g) = super::parse(&input).unwrap();
        let n = super::count_interior_points(&g);
        assert_eq!(n, 4);

        let input = std::fs::read_to_string("data/day10/input4.txt").unwrap();
        let (_, g) = super::parse(&input).unwrap();
        let n = super::count_interior_points(&g);
        assert_eq!(n, 10);

        let input = std::fs::read_to_string("data/day10/input3.txt").unwrap();
        let (_, g) = super::parse(&input).unwrap();
        let n = super::count_interior_points(&g);
        assert_eq!(n, 8);
    }

    #[test]
    fn t4() {
        let input = std::fs::read_to_string("data/day10/input2.txt").unwrap();
        let (_, g) = super::parse(&input).unwrap();
        let n = super::count_interior_points(&g);
        assert_eq!(n, 435);
    }
}

