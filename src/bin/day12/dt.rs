use std::cmp::min;
use std::error::Error;
use std::fmt::{self, Display, Write};
use std::io;

use aoc2022::types::grid::{FixedWidthDisplay, Grid};
use petgraph::algo::astar::astar;
use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use petgraph::{Directed, Graph};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    height: u8,
    pos: (usize, usize),
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(char::from_u32('a' as u32 + self.height as u32).unwrap())
    }
}

impl FixedWidthDisplay for Point {}

impl TryFrom<(char, (usize, usize))> for Point {
    type Error = &'static str;

    fn try_from(value: (char, (usize, usize))) -> Result<Self, Self::Error> {
        Ok(Point {
            height: match value.0 {
                'S' => 0,
                'E' => 25,
                'a'..='z' => (value.0 as u32 - 'a' as u32).try_into().unwrap(),
                _ => return Err("unrecognised input char"),
            },
            pos: value.1,
        })
    }
}

#[derive(Debug)]
pub struct HMap {
    idx_grid: Grid<NodeIndex>,
    grid: Graph<Point, (), Directed>,
    start: (usize, usize),
    end: (usize, usize),
}

impl HMap {
    pub fn find_part_one_dist(&self) -> usize {
        let s_idx = *self.idx_grid.get(self.start.0, self.start.1).unwrap();
        let e_idx = *self.idx_grid.get(self.end.0, self.end.1).unwrap();
        let r = astar(
            &self.grid,
            s_idx,
            |f| f == e_idx,
            |_| 1, // edge cost is always 1
            |from_ni| {
                // This function returns the minimum possible cost to go from
                // the node at from_ni to the goal node.
                let from = self.grid.node_weight(from_ni).unwrap();
                self.idx_grid.taxicab_dist(from.pos, self.end).unwrap()
            },
        )
        .unwrap();
        r.0
    }

    pub fn find_part_two_dist(mut self) -> i32 {
        // Trick: Dijkstra's algorithm will allow us to efficiently find the
        // distance from the summit to any reachable point, if we start the
        // search at the summit and descend instead.

        // First, flip all edges in the graph: A -> B becomes B -> A.
        let edges: Vec<(NodeIndex, NodeIndex)> = self
            .grid
            .edge_indices()
            .map(|e| self.grid.edge_endpoints(e).unwrap())
            .collect();
        self.grid.clear_edges();
        for (from, to) in edges {
            self.grid.update_edge(to, from, ()); // NB: reversed direction!
        }

        // Then perform the search from the summit.
        let summit_idx = *self.idx_grid.get(self.end.0, self.end.1).unwrap();
        let r = dijkstra(&self.grid, summit_idx, None, |_| 1);

        // And finally search all the node indexes to find a 0 node with the
        // shortest distance.
        let mut shortest_dist: Option<i32> = None;
        for (_, poss_start) in self.idx_grid.enumerate() {
            let poss_start = *poss_start;
            let start_node = self.grid.node_weight(poss_start).unwrap();
            if start_node.height != 0 {
                // Ignore nodes that aren't candidate start nodes.
                continue;
            }

            if let Some(d) = r.get(&poss_start) {
                shortest_dist = Some(match shortest_dist {
                    None => *d,
                    Some(d_2) => min(*d, d_2),
                });
            }
        }

        shortest_dist.unwrap()
    }

    pub fn parse_from_lines(
        lines: impl Iterator<Item = Result<String, io::Error>>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut grid: Graph<Point, (), Directed> = Graph::<Point, ()>::new();
        let mut idx_grid: Vec<Vec<NodeIndex>> = Vec::new();

        let mut start = None;
        let mut end = None;

        // Add nodes, recording the NodeIndexes into a Grid for efficient
        // random access into the graph.
        for (y, line) in lines.enumerate() {
            let line = line?;
            let mut idx_row: Vec<NodeIndex> = Vec::new();

            for (x, c) in line.chars().enumerate() {
                let p: Point = (c, (x, y)).try_into()?;
                idx_row.push(grid.add_node(p));

                if c == 'S' {
                    assert!(start.is_none());
                    start = Some((x, y));
                } else if c == 'E' {
                    assert!(end.is_none());
                    end = Some((x, y));
                }
            }

            idx_grid.push(idx_row);
        }

        let idx_grid: Grid<NodeIndex> = idx_grid.try_into()?;

        // Add edges using the Grid<NodeIndex>.
        for (pos, node_idx) in idx_grid.enumerate() {
            let node_idx = *node_idx;
            let node_height = grid.node_weight(node_idx).unwrap().height;

            let neighbours = idx_grid.enumerate_n4(pos).map(|(_, ni)| ni);
            for neighbour_idx in neighbours {
                let neighbour_idx = *neighbour_idx;
                let neighbour_node_height = grid.node_weight(neighbour_idx).unwrap().height;

                if (0..=(node_height + 1)).contains(&neighbour_node_height) {
                    grid.update_edge(node_idx, neighbour_idx, ());
                }
            }
        }

        grid.shrink_to_fit();

        Ok(Self {
            idx_grid,
            grid,
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}

impl fmt::Display for HMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}\n  start = {:?}\n  end = {:?}",
            self.grid, self.start, self.end
        )
    }
}
