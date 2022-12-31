use petgraph::dot::Dot;
use petgraph::prelude::*;

use crate::graph::{build_problem_graph, GraphValve};
use crate::parse::State;

pub fn solve_part1(s: &State) -> u32 {
    // Only 15 valves have a non-zero rate.
    // Create a graph where nodes are valves, and edges are the tunnels between.
    // Each edge is annotated with the minutes required to traverse it.
    // Use the Floyd-Warshall algorithm to do so minimally by removing 0 nodes.
    // Now we've reduced the problem space greatly. For the full-size input, we
    // have only 16 important nodes.
    let graph = build_problem_graph(s);

    println!("graphviz:\n{:?}\n\n", Dot::new(&graph));

    // Perform a depth-first search bounded by length, starting from the AA node.
    let aa: Vec<_> = graph.nodes().filter(|n| n.name == "AA").collect();
    assert_eq!(aa.len(), 1);
    let aa = aa[0];

    let result = bounded_dfs(&graph, aa, 30);

    result
}

fn bounded_dfs<'a>(
    graph: &'a GraphMap<GraphValve<'a>, u32, Directed>,
    from: GraphValve<'a>,
    time_budget: u32,
) -> u32 {
    let open = im_rc::HashSet::new();
    bounded_dfs_helper(graph, from, &open, 0, time_budget)
}

/// Performs a DFS over the problem graph. Returns a pair of best score and path
/// used.
fn bounded_dfs_helper<'a>(
    graph: &'a GraphMap<GraphValve<'a>, u32, Directed>,
    from: GraphValve<'a>,
    open: &im_rc::HashSet<GraphValve<'a>>,
    mut score_so_far: u32,
    mut time_budget: u32,
) -> u32 {
    // Because of the graph reduction, we perform a greedy-ish search. If the
    // valve hasn't been opened, always open it.

    // Open this valve (if not open) as long as we have time. Skip the AA node.
    // We can do this greedily because we've made a fully-connected graph.
    let copied_open =
        if open.contains(&from) || time_budget == 0 || from.name == "AA" {
            None
        } else {
            let mut open = open.clone();
            open.insert(from);
            time_budget -= 1;
            let score = from.rate * time_budget;
            score_so_far += score;

            Some(open)
        };

    // If all the valves have been opened, return immediately.
    if open.len() == graph.node_count() {
        return score_so_far;
    }

    let open = match copied_open {
        Some(ref co) => co,
        None => open,
    };

    // Initialise best_score to score_so_far to allow for staying put.
    let mut best_score = score_so_far;

    // Then check if moving to a neighbour would be beneficial.
    for (_, neighbour, dist) in graph.edges(from) {
        // Ignore nodes that we can't reach in the remaining time.
        if time_budget < *dist {
            continue;
        }

        // And any nodes we've already visited.
        if open.contains(&neighbour) && open.len() < graph.node_count() {
            continue;
        }

        let neighbour_best_score = bounded_dfs_helper(
            graph,
            neighbour,
            open,
            score_so_far,
            time_budget - dist,
        );
        best_score = best_score.max(neighbour_best_score);
    }

    best_score
}
