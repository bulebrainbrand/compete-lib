use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Edge {
    to_node: usize,
    weight: usize,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    edges: Vec<Edge>,
}
pub struct WeightedDirectedGraph {
    nodes: Vec<Node>,
}

impl WeightedDirectedGraph {
    pub fn new(edges: &Vec<Vec<Edge>>) -> WeightedDirectedGraph {
        WeightedDirectedGraph {
            nodes: edges.iter().map(|x| Node { edges: x.to_vec() }).collect(),
        }
    }

    /// ダイクストラでnode間の最短距離を求めます。
    /// O((頂点の数 + 辺の数) log 頂点の数)
    /// # Example
    /// ```rust
    /// # use::compete_lib::graph::weighted_directed_graph::WeightedDirectedGraph;
    /// // 0 -1-> 1 -2-> 2 -2-> 3   4 -2-> 5
    /// // |              ^
    /// // --------1------|
    /// let graph = WeightedDirectedGraph::from(vec![(0,1,1),(1,2,2),(2,3,2),(0,3,1),(4,5,2)]);
    /// assert_eq!(Some(1), graph.get_node_distance(0,3)); // 0 -> 3
    /// assert_eq!(Some(3), graph.get_node_distance(0,2)); // 0 -> 1 -> 2
    /// assert_eq!(None, graph.get_node_distance(0,4)); // None,行く道がない
    /// assert_eq!(None, graph.get_node_distance(1,0)); // None,逆向きには進めない
    /// assert_eq!(Some(0), graph.get_node_distance(0,0))
    /// ```
    pub fn get_node_distance(&self, from_node_index: usize, to_node_index: usize) -> Option<usize> {
        if from_node_index >= self.nodes.len() || to_node_index >= self.nodes.len() {
            return None;
        }
        let mut heap = BinaryHeap::new();
        let mut distance = vec![None; self.nodes.len()];
        heap.push(Reverse((0, from_node_index)));
        distance[from_node_index] = Some(0);
        while let Some(Reverse((dis, node))) = heap.pop() {
            if distance[node].map_or(false, |d| d < dis) {
                continue;
            }
            if node == to_node_index {
                return Some(dis);
            }
            for edge in &self.nodes[node].edges {
                let next = dis + edge.weight;
                if distance[edge.to_node].map_or(false, |d| d <= next) {
                    continue;
                };
                distance[edge.to_node] = Some(next);
                heap.push(Reverse((next, edge.to_node)));
            }
        }
        None
    }
    /// ダイクストラ+経路復元でnode間の最短距離での経路を求めます。
    /// O((頂点の数 + 辺の数) log 頂点の数)
    /// # Example
    /// ```rust
    /// # use::compete_lib::graph::weighted_directed_graph::WeightedDirectedGraph;
    /// // 0 -> 1 -> 2 -> 3   4 -> 5
    /// // |              ^
    /// // ---------------|
    /// let graph = WeightedDirectedGraph::from(vec![(0,1,1),(1,2,1),(2,3,1),(0,3,1),(4,5,1)]);
    /// assert_eq!(Some(vec![0,3]), graph.get_node_route(0,3)); // 0 -> 3
    /// assert_eq!(Some(vec![0,1,2]), graph.get_node_route(0,2)); // 0 -> 1 -> 2
    /// assert_eq!(None, graph.get_node_route(0,4)); // None,行く道がない
    /// assert_eq!(None, graph.get_node_route(1,0)); // None,逆向きには進めない
    /// assert_eq!(Some(vec![0]), graph.get_node_route(0,0))
    /// ```
    pub fn get_node_route(
        &self,
        from_node_index: usize,
        to_node_index: usize,
    ) -> Option<Vec<usize>> {
        if from_node_index >= self.nodes.len() || to_node_index >= self.nodes.len() {
            return None;
        }
        let mut heap = BinaryHeap::new();
        let mut distance = vec![None; self.nodes.len()];
        let mut prev = vec![None; self.nodes.len()];
        distance[from_node_index] = Some(0);
        heap.push(Reverse((0, from_node_index)));

        while let Some(Reverse((dis, node))) = heap.pop() {
            if distance[node].map_or(false, |d| d < dis) {
                continue;
            }
            if node == to_node_index {
                // 到着、経路復元
                let mut route = vec![to_node_index];
                let mut curr = to_node_index;
                while let Some(num) = prev[curr] {
                    route.push(num);
                    curr = num;
                }
                route.reverse();
                return Some(route);
            }
            for edge in &self.nodes[node].edges {
                let next = dis + edge.weight;
                if distance[edge.to_node].map_or(false, |d| d <= next) {
                    continue;
                };
                prev[edge.to_node] = Some(node);
                distance[edge.to_node] = Some(next);
                heap.push(Reverse((dis + edge.weight, edge.to_node)));
            }
        }
        None
    }
}

impl From<Vec<(usize, usize, usize)>> for WeightedDirectedGraph {
    fn from(item: Vec<(usize, usize, usize)>) -> Self {
        let mut nodes = vec![];
        for (from_index, to_index, weight) in item {
            if nodes.len() <= from_index {
                nodes.resize(from_index + 1, Node { edges: vec![] });
            }
            if nodes.len() <= to_index {
                nodes.resize(to_index + 1, Node { edges: vec![] });
            }
            nodes[from_index].edges.push(Edge {
                to_node: to_index,
                weight,
            });
        }
        WeightedDirectedGraph { nodes }
    }
}
