/*
    graph
    This problem requires you to implement a basic graph function
*/

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    // 在无向图中添加一条边（自动添加两个方向）
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;

        // 确保两个节点都存在于图中
        self.add_node(from);
        self.add_node(to);

        // 添加 from -> to
        self.adjacency_table_mutable()
            .get_mut(from)
            .unwrap()
            .push((to.to_string(), weight));

        // 添加 to -> from（因为是无向图）
        self.adjacency_table_mutable()
            .get_mut(to)
            .unwrap()
            .push((from.to_string(), weight));
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    // 添加节点，如果已存在则返回 false
    fn add_node(&mut self, node: &str) -> bool {
        if self.contains(node) {
            return false; // 节点已存在
        }
        self.adjacency_table_mutable()
            .insert(node.to_string(), Vec::new());
        true
    }

    // 添加边（由具体类型实现）
    fn add_edge(&mut self, edge: (&str, &str, i32));

    // 检查图是否包含某个节点
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    // 返回所有节点的集合
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    // 返回所有边的列表（每条边只算一次方向，但测试要求双向列出）
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
        ];

        for edge in expected_edges.iter() {
            assert!(
                graph.edges().contains(edge),
                "Expected edge {:?} not found",
                edge
            );
        }
    }
}