// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use compete_lib::data_structure::segment_tree::SegmentTree;
use proconio::input;

fn main() {
    input! {
        n:usize,
        q:usize,
        a:[usize;n]
    }
    let mut tree = SegmentTree::from_vec(a, 0, |a, b| a + b);
    for _ in 0..q {
        input! {
            t:usize,
        }
        if t == 0 {
            input! {
                p:usize,
                x:usize
            }
            tree.update(p, |d| d + x);
        } else {
            input! {
                l:usize,
                r:usize
            }
            println!("{}", tree.prod(l, r))
        }
    }
}
