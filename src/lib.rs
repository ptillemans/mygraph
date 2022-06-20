use anyhow::{bail, Result};
use std::collections::HashMap;
use std::hash::Hash;

pub struct Graph<NId: Eq, N, E> {
    nodes: HashMap<NId, N>,
    edges: HashMap<NId, Vec<(NId, E)>>,
}

impl<NId: Eq + Hash + Clone, N: Clone, E: Clone> Graph<NId, N, E> {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(self: &mut Self, id: NId, value: N) {
        self.nodes.insert(id, value);
    }

    pub fn all_nodes(self: &Self) -> Vec<(NId, N)> {
        self.nodes
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn add_edge(self: &mut Self, from: NId, to: NId, value: E) -> Result<()> {
        if self.nodes.contains_key(&from) && self.nodes.contains_key(&to) {
            self.edges.insert(from, vec![(to, value)]);
            return Ok(());
        }
        bail!("Cannot add edge with unknown nodes");
    }

    pub fn all_edges(self: &Self) -> Vec<(NId, NId, E)> {
        self.edges
            .iter()
            .flat_map(|(from, tos)| {
                tos.into_iter()
                    .map(|(to, value)| (from.clone(), to.clone(), value.clone()))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_nodes() {
        let mut graph = Graph::<&str, &str, ()>::new();
        graph.add_node("A", "Aap");
        graph.add_node("B", "Beer");
        graph.add_node("C", "Chocolade");
        graph.add_node("D", "Das");

        let result = graph.all_nodes().len();
        assert_eq!(result, 4);
    }

    #[test]
    fn add_edges() {
        let mut graph = Graph::<&str, &str, ()>::new();
        graph.add_node("A", "Aap");
        graph.add_node("B", "Beer");
        graph.add_node("C", "Chocolade");
        graph.add_node("D", "Das");

        graph.add_edge("A", "B", ())?;
        graph.add_edge("B", "C", ())?;
        graph.add_edge("C", "D", ())?;
        graph.add_edge("A", "D", ())?;

        let result = graph.all_edges().len();
        assert_eq!(result, 3);
    }

    #[test]
    fn add_edges_illegal_from() {
        let mut graph = Graph::<&str, &str, ()>::new();
        graph.add_node("A", "Aap");
        graph.add_node("B", "Beer");
        graph.add_node("C", "Chocolade");
        graph.add_node("D", "Das");

        assert!(graph.add_edge("E", "B", ()).is_err());
    }

    #[test]
    fn add_edges_illegal_to() {
        let mut graph = Graph::<&str, &str, ()>::new();
        graph.add_node("A", "Aap");
        graph.add_node("B", "Beer");
        graph.add_node("C", "Chocolade");
        graph.add_node("D", "Das");

        assert!(graph.add_edge("C", "E", ()).is_err());
    }
}
