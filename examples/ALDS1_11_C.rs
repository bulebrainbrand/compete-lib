// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/11/ALDS1_11_C

use compete_lib::graph::directed_graph::DirectedGraph;
use proconio::input;

fn main() {
    input! { n: usize }
    let mut edges = vec![];
    for _ in 0..n {
        input! { u: usize, k: usize, vs: [usize; k] }
        for v in vs {
            edges.push((u - 1, v - 1));
        }
    }
    let graph = DirectedGraph::from(edges);
    for i in 0..n {
        let d = match graph.get_node_distance(0, i) {
            Some(d) => d as i64,
            None if i == 0 => 0,
            None => -1,
        };
        println!("{} {}", i + 1, d);
    }
}
