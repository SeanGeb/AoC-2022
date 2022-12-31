use std::sync::atomic::{AtomicU32, Ordering};

use petgraph::prelude::*;
use rayon::prelude::*;

use crate::graph::{build_problem_graph, GraphValve};
use crate::parse::State;

pub fn solve_part2(s: &State) -> u32 {
    let graph = build_problem_graph(s);

    let aa: Vec<_> = graph.nodes().filter(|n| n.name == "AA").collect();
    assert_eq!(aa.len(), 1);
    let aa = aa[0];

    // For this one, try something a bit different.
    // Partition the graph in two and use the part1 algorithm on each partition.

    partition_dfs(&graph, aa, 26)
}

fn partition_dfs<'a>(
    graph: &'a GraphMap<GraphValve<'a>, u32, Directed>,
    from: GraphValve<'a>,
    time_budget: u32,
) -> u32 {
    let nodes: Vec<_> = graph.nodes().filter(|n| n.name != "AA").collect();

    let n_nodes: u32 = graph.nodes().len().try_into().unwrap();

    let progress = AtomicU32::new(0);
    let bitvec_max = 1u64.checked_shl(n_nodes - 1).unwrap() - 1;

    // NB: can halve the search space to avoid the human and elephant doing the
    // same work.
    let best_score = (0..=bitvec_max)
        .into_par_iter()
        .map(|partition_bitvec| -> u32 {
            let nodes_human: im_rc::HashSet<GraphValve> =
                im_rc::HashSet::from_iter(
                    nodes
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| (partition_bitvec & 1 << i) == 0)
                        .map(|(_, n)| *n),
                );
            let nodes_elephant: im_rc::HashSet<GraphValve> =
                im_rc::HashSet::from_iter(
                    nodes
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| (partition_bitvec & 1 << i) != 0)
                        .map(|(_, n)| *n),
                );

            let score_human =
                partition_dfs_helper(graph, from, &nodes_human, 0, time_budget);
            let score_elephant = partition_dfs_helper(
                graph,
                from,
                &nodes_elephant,
                0,
                time_budget,
            );

            let progress = progress.fetch_add(1, Ordering::Relaxed);
            if progress % 256 == 0 {
                println!("Progress: {progress}/{bitvec_max}");
            }

            score_human + score_elephant
        })
        .max();

    best_score.unwrap()
}

fn partition_dfs_helper<'a>(
    graph: &'a GraphMap<GraphValve<'a>, u32, Directed>,
    from: GraphValve<'a>,
    partition: &im_rc::HashSet<GraphValve<'a>>,
    mut score_so_far: u32,
    mut time_budget: u32,
) -> u32 {
    // Open this valve.
    let copied_partition = if time_budget >= 1 && from.name != "AA" {
        time_budget -= 1;
        score_so_far += from.rate * time_budget;
        Some(partition.without(&from))
    } else {
        None
    };

    // If all valves are open, bail.
    if partition.is_empty() {
        return score_so_far;
    }

    let partition = match copied_partition {
        Some(ref cp) => cp,
        None => partition,
    };

    let mut best_score = score_so_far;

    for (_, neighbour, dist) in graph.edges(from) {
        // Ignore nodes that we can't reach in the remaining time.
        if time_budget < *dist {
            continue;
        }

        // or nodes that have been visted, or are outside the partition.
        if !partition.contains(&neighbour) {
            continue;
        }

        best_score = best_score.max(partition_dfs_helper(
            graph,
            neighbour,
            partition,
            score_so_far,
            time_budget - dist,
        ));
    }

    best_score
}
