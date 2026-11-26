use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Edge {
    to_node: usize,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    edges: Vec<Edge>,
}
pub struct DirectedGraph {
    nodes: Vec<Node>,
}

impl DirectedGraph {
    pub fn new(edges: &Vec<Vec<Edge>>) -> DirectedGraph {
        DirectedGraph {
            nodes: edges.iter().map(|x| Node { edges: x.to_vec() }).collect(),
        }
    }

    /// bfsでnode間の最短距離を求めます。
    /// O(頂点の数 + 辺の数)
    /// # Example
    /// ```rust
    /// # use::compete_lib::graph::directed_graph::DirectedGraph;
    /// // 0 -> 1 -> 2 -> 3   4 -> 5
    /// // |              ^
    /// // ---------------|
    /// let graph = DirectedGraph::from(vec![(0,1),(1,2),(2,3),(0,3),(4,5)]);
    /// assert_eq!(Some(1), graph.get_node_distance(0,3)); // 0 -> 3
    /// assert_eq!(Some(2), graph.get_node_distance(0,2)); // 0 -> 1 -> 2
    /// assert_eq!(None, graph.get_node_distance(0,4)); // None,行く道がない
    /// assert_eq!(None, graph.get_node_distance(1,0)); // None,逆向きには進めない
    /// assert_eq!(Some(0), graph.get_node_distance(0,0))
    /// ```
    pub fn get_node_distance(&self, from_node_index: usize, to_node_index: usize) -> Option<usize> {
        if from_node_index >= self.nodes.len() || to_node_index >= self.nodes.len() {
            return None;
        }
        let mut queue = VecDeque::new();
        let mut distance = vec![None; self.nodes.len()];
        queue.push_back(from_node_index);
        distance[from_node_index] = Some(0);
        while let Some(node) = queue.pop_front() {
            let dis = distance[node].unwrap();
            if node == to_node_index {
                return Some(dis);
            }
            for edge in &self.nodes[node].edges {
                if distance[edge.to_node].is_some() {
                    continue;
                };
                distance[edge.to_node] = Some(dis + 1);
                queue.push_back(edge.to_node);
            }
        }
        None
    }
    /// bfs+経路復元でnode間の最短距離での経路を求めます。
    /// O(頂点の数 + 辺の数)
    /// # Example
    /// ```rust
    /// # use::compete_lib::graph::directed_graph::DirectedGraph;
    /// // 0 -> 1 -> 2 -> 3   4 -> 5
    /// // |              ^
    /// // ---------------|
    /// let graph = DirectedGraph::from(vec![(0,1),(1,2),(2,3),(0,3),(4,5)]);
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
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.nodes.len()];
        let mut prev = vec![None; self.nodes.len()];
        visited[from_node_index] = true;
        queue.push_back((from_node_index, 0));

        while let Some((node, dis)) = queue.pop_front() {
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
                if visited[edge.to_node] {
                    continue;
                };
                prev[edge.to_node] = Some(node);
                visited[node] = true;
                queue.push_back((edge.to_node, dis + 1));
            }
        }
        None
    }
}

impl From<Vec<(usize, usize)>> for DirectedGraph {
    fn from(item: Vec<(usize, usize)>) -> Self {
        let mut nodes = vec![];
        for (from_index, to_index) in item {
            if nodes.len() <= from_index {
                nodes.resize(from_index + 1, Node { edges: vec![] });
            }
            if nodes.len() <= to_index {
                nodes.resize(to_index + 1, Node { edges: vec![] });
            }
            nodes[from_index].edges.push(Edge { to_node: to_index });
        }
        DirectedGraph { nodes }
    }
}
