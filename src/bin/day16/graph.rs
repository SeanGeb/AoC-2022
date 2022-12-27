use petgraph::algo::floyd_warshall;
use petgraph::prelude::*;

use crate::parse::State;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct GraphValve<'a> {
    pub name: &'a String,
    pub rate: u32,
}

pub fn build_problem_graph(s: &State) -> GraphMap<GraphValve, u32, Directed> {
    let mut graph: GraphMap<GraphValve, u32, Undirected> = GraphMap::new();

    // Add nodes and edges in one go.
    for (v1_name, v1) in s.valves.iter() {
        for v2_name in v1.tunnels_to.iter() {
            let v2 = &s.valves[v2_name];

            let v1 = GraphValve {
                name: v1_name,
                rate: v1.rate,
            };
            let v2 = GraphValve {
                name: v2_name,
                rate: v2.rate,
            };

            graph.add_edge(v1, v2, 1);
        }
    }

    // Run the Floyd-Warshall algorithm to get the shortest path length between
    // each pair of valves.
    let fw_pair_distances = floyd_warshall(&graph, |(_, _, n)| *n).unwrap();

    // Build a new graph. Make it directed so we don't return to the start state
    // AA (which has rate=0 in both cases).
    let mut fw_graph: GraphMap<GraphValve, u32, Directed> = GraphMap::new();

    for ((from, to), dist) in fw_pair_distances {
        // Ignore nodes where rate=0 - these are useless to us - or self-edges,
        // other than out-edges from the AA node.
        // Include an assert to handle a petgraph bug (#524).
        assert_ne!(dist, u32::MAX, "encountered an upstream bug: petgraph#524");
        if dist > 0 && (from.rate != 0 || from.name == "AA") && to.rate != 0 {
            fw_graph.add_edge(from, to, dist);
        }
    }

    fw_graph
}
