use itertools::Itertools;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<usize> {
    let graph = build_graph(input);
    stoer_wagner_min_cut(&graph, |_| Ok::<usize, Vec<NodeIndex>>(1))
        .map(|cut| cut.map(|(_, nodes)| (graph.node_count() - nodes.len()) * nodes.len()))
        .unwrap()
}

fn build_graph(input: &str) -> UnGraph<&str, usize> {
    let mut graph: UnGraph<&str, usize> = UnGraph::new_undirected();
    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();

    input
        .lines()
        .filter_map(|line| line.split_once(":"))
        .map(|(n, c)| (n, c.split_ascii_whitespace().collect_vec()))
        .for_each(|(node, connections)| {
            for connection in connections {
                let left = *nodes.entry(node).or_insert_with(|| graph.add_node(node));
                let right = *nodes
                    .entry(connection)
                    .or_insert_with(|| graph.add_node(connection));
                graph.add_edge(left, right, 1);
                graph.add_edge(right, left, 1);
            }
        });

    graph
}
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
