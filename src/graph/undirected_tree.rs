use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Edge {
    to_node: usize,
}

pub struct Tree {
    nodes: Vec<Vec<Edge>>,
}

impl Tree {
    pub fn new(edges: &Vec<Vec<Edge>>) -> Tree {
        Tree {
            nodes: (&edges).to_vec(),
        }
    }
    pub fn from(edges: &Vec<(usize, usize)>) -> Tree {
        let mut nodes: Vec<Vec<Edge>> = vec![];
        for (from_number, to_number) in edges {
            let to = Edge {
                to_node: *to_number,
            };
            let from = Edge {
                to_node: *from_number,
            };
            if *from_number >= nodes.len() {
                nodes.resize(*from_number + 1, vec![]);
            };
            nodes[*from_number].push(to);
            if *to_number >= nodes.len() {
                nodes.resize(*to_number + 1, vec![]);
            }
            nodes[*to_number].push(from);
        }
        Tree { nodes }
    }
    /// 木の直径を取得します
    /// O(n) # nは木の大きさ
    /// # example
    /// ```rust
    /// # use compete_lib::graph::undirected_tree::Tree;
    /// // 1 -> 2 -> 3
    /// // |-> 4
    /// let tree = Tree::from(&vec![(1,2),(2,3),(1,4)]);
    /// assert_eq!(tree.get_diameter(),3)
    /// ```
    ///
    ///
    pub fn get_diameter(&self) -> usize {
        let start = self.nodes.iter().position(|e| !e.is_empty()).unwrap_or(0);
        let farthest = self.get_fartest_node(start);
        self.get_fartest_node(farthest.0).1
    }
    pub fn get_node_distance(&self, node: usize) -> Vec<Option<usize>> {
        let mut stack = VecDeque::new();
        let mut distance = vec![None; self.nodes.len()];
        stack.push_back((node, 0));
        while let Some((node, dis)) = stack.pop_front() {
            distance[node] = Some(dis);
            for edge in &self.nodes[node] {
                if distance[edge.to_node].is_some() {
                    continue;
                };
                stack.push_back((edge.to_node, dis + 1));
            }
        }
        distance
    }
    /// 引数に渡されたnodeから最も遠い点を求めます
    /// O(n) nは木の大きさ
    /// # Examples
    /// ```rust
    /// # use compete_lib::graph::undirected_tree::Tree;
    /// // 1 -> 2 -> 3
    /// // |-> 4
    /// let tree = Tree::from(&vec![(1,2),(2,3),(1,4)]);
    /// assert_eq!(tree.get_fartest_node(1),(3,2))
    /// ```
    pub fn get_fartest_node(&self, node: usize) -> (usize, usize) {
        let distance = &self.get_node_distance(node);
        distance
            .iter()
            .enumerate()
            .filter_map(|(i, d)| d.map(|d| (i, d)))
            .max_by_key(|&(_, d)| d)
            .unwrap()
    }
}
