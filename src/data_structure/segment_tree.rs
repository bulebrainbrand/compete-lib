use std::usize;

#[derive(Debug, Clone)]
///
/// e: 単位元
/// op: `|&D,&D| -> D`の関数
/// (D,op)がmonoidをなす
/// Dはclone traitが必要(単位元を配列全体に満たすため)
/// # Example
/// ```rust
/// # use crate::compete_lib::data_structure::segment_tree::SegmentTree;
/// // 加算セグ木
/// let mut tree = SegmentTree::new(8,0,|a,b| a + b);
/// tree.set(0, 1);
/// tree.set(1, 2);
/// assert_eq!(3, tree.prod(0,8));
/// tree.set(0, 3);
/// assert_eq!(5, tree.prod(0,8));
/// ```
///
///
pub struct SegmentTree<D, F> {
    size: usize,
    data: Vec<D>,
    e: D,
    op: F,
}

impl<D: Clone, F: Fn(&D, &D) -> D> SegmentTree<D, F> {
    pub fn new(size: usize, e: D, op: F) -> Self {
        SegmentTree {
            size,
            data: vec![e.clone(); size * 2],
            e: e,
            op,
        }
    }
    /// O(n)
    /// - nは`v.len()`
    /// Vecから構築します。newしてsetをn回呼ぶのに比べてlog nだけ高速です。
    /// 配列の長さがそのままsizeになるので可変長な場合は最大の長さまでeで埋めて伸ばしてください
    /// # Example
    /// ```rust
    /// # use crate::compete_lib::data_structure::segment_tree::SegmentTree;
    /// let vector = vec![1,2,3,4,5];
    /// let tree = SegmentTree::from_vec(vector, 0, |a, b| a +b);
    /// assert_eq!(15, tree.prod(0,5));
    /// ```
    pub fn from_vec(v: Vec<D>, e: D, op: F) -> Self {
        let size = v.len();
        let mut data = vec![e.clone(); size];
        data.extend(v);
        for i in (1..size).rev() {
            data[i] = op(&data[2 * i], &data[2 * i + 1]);
        }
        SegmentTree { size, data, e, op }
    }
    /// O(log n * p)
    /// - nは`SegmentTree.size / 2`
    /// - pは`SegmentTree.op`の最悪計算量
    /// # Examples
    /// ```rust
    /// # use crate::compete_lib::data_structure::segment_tree::SegmentTree;
    /// let mut tree = SegmentTree::new(8,0,|a,b| a + b);
    /// tree.set(0, 1); // 0番目の要素を1に更新
    /// ```
    ///
    pub fn set(&mut self, i: usize, d: D) {
        assert!(i < self.size);
        let mut i = self.size + i;
        self.data[i] = d;
        while i > 1 {
            i >>= 1;
            self.data[i] = (self.op)(&self.data[i * 2], &self.data[i * 2 + 1]);
        }
    }
    /// O(log n * p + f)
    /// - nは`SegmentTree.size / 2`
    /// - pは`SegmentTree.op`の最悪計算量
    /// - fは引数`f`の最悪計算量
    /// # Examples
    /// ```rust
    /// # use crate::compete_lib::data_structure::segment_tree::SegmentTree;
    /// let mut tree = SegmentTree::new(8,0,|a,b| a + b);
    /// tree.set(0, 1); // 0番目の要素を1に更新
    /// ```
    ///
    pub fn update<M: FnOnce(&D) -> D>(&mut self, i: usize, f: M) {
        assert!(i < self.size);
        let mut i = self.size + i;
        self.data[i] = f(&self.data[i]);
        while i > 1 {
            i >>= 1;
            self.data[i] = (self.op)(&self.data[i * 2], &self.data[i * 2 + 1]);
        }
    }
    /// `O(log n)`
    /// - nは`SegmentTree.size / 2`
    ///
    /// `[l,r)`の半開区間であることに注意
    ///
    /// # Examples
    /// ```rust
    /// # use crate::compete_lib::data_structure::segment_tree::SegmentTree;
    /// let mut tree = SegmentTree::new(8,0,|a,b| a + b);
    ///
    /// tree.set(0,1); // [1,0,0,0,0,0,0,0]
    /// tree.set(2,8); // [1,0,8,0,0,0,0,0]
    /// tree.set(3,1); // [1,0,8,1,0,0,0,0]
    ///
    /// assert_eq!(0, tree.prod(0,0)); // [] 単位元eが返る
    /// assert_eq!(1, tree.prod(0,1)); // [1]
    /// assert_eq!(9, tree.prod(0,3)); // [1,0,8]
    /// assert_eq!(9, tree.prod(2,4)); // [8,1]
    /// ```
    pub fn prod(&self, mut l: usize, mut r: usize) -> D {
        assert!(l <= r && r <= self.size);
        let mut sml = self.e.clone();
        let mut smr = self.e.clone();
        l += self.size;
        r += self.size;
        while l < r {
            if l & 1 == 1 {
                sml = (self.op)(&sml, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = (self.op)(&self.data[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(&sml, &smr)
    }
}
