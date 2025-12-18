use super::monoid::*;

// ノード定義
struct Node<M: Monoid> {
    left: usize,
    right: usize,
    parent: usize,
    val: M::S, // ノード自身の値
    acc: M::S, // 部分木の集約値
    rev: bool, // 反転フラグ
}

impl<M: Monoid> Node<M> {
    fn new(val: M::S) -> Self {
        Self {
            left: !0,
            right: !0,
            parent: !0,
            val,
            acc: val, // 初期状態ではaccは自身の値
            rev: false,
        }
    }
}

pub struct LinkCutTree<M: Monoid> {
    nodes: Vec<Node<M>>,
}

impl<M: Monoid> LinkCutTree<M> {
    pub fn new(n: usize) -> Self {
        let mut nodes = Vec::with_capacity(n);
        for _ in 0..n {
            nodes.push(Node::new(M::identity()));
        }
        Self { nodes }
    }

    /// 初期値配列から構築する場合
    pub fn new_with_values(values: Vec<M::S>) -> Self {
        let nodes = values.into_iter().map(Node::new).collect();
        Self { nodes }
    }

    /// 値の更新
    pub fn set(&mut self, v: usize, x: M::S) {
        self.access(v);
        self.splay(v);
        self.nodes[v].val = x;
        self.pull(v);
    }

    /// 親的な
    pub fn root(&mut self, v: usize) -> usize {
        self.access(v);
        self.splay(v);
        let mut cur = v;
        while self.nodes[cur].left != !0 {
            cur = self.nodes[cur].left;
            self.push(cur);
        }
        self.splay(cur);
        cur
    }

    pub fn unite(&mut self, u: usize, v: usize) {
        self.make_root(u);
        if self.root(v) == u {
            return;
        }
        self.nodes[u].parent = v;
    }

    pub fn cut(&mut self, u: usize, v: usize) {
        self.make_root(u);
        self.access(v);
        self.splay(v);
        if self.nodes[v].left == u && self.nodes[u].right == !0 {
            self.nodes[v].left = !0;
            self.nodes[u].parent = !0;
            self.pull(v);
        }
    }

    pub fn query(&mut self, u: usize, v: usize) -> M::S {
        self.make_root(u);
        self.access(v);
        self.splay(v);
        self.nodes[v].acc
    }

    pub fn make_root(&mut self, v: usize) {
        self.access(v);
        self.splay(v);
        self.reverse(v);
        self.push(v);
    }

    pub fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    // -----------------------------------------------------------
    // Internal Logic
    // -----------------------------------------------------------

    fn is_root(&self, idx: usize) -> bool {
        let p = self.nodes[idx].parent;
        p == !0 || (self.nodes[p].left != idx && self.nodes[p].right != idx)
    }

    fn reverse(&mut self, idx: usize) {
        let n = &mut self.nodes[idx];
        std::mem::swap(&mut n.left, &mut n.right);
        n.rev = !n.rev;
        // Monoidにtoggleがないため、accの反転処理は行わない
        // (可換な演算であることを前提とする)
    }

    fn push(&mut self, idx: usize) {
        if self.nodes[idx].rev {
            let left = self.nodes[idx].left;
            let right = self.nodes[idx].right;
            if left != !0 {
                self.reverse(left);
            }
            if right != !0 {
                self.reverse(right);
            }
            self.nodes[idx].rev = false;
        }
    }

    fn pull(&mut self, idx: usize) {
        let left = self.nodes[idx].left;
        let right = self.nodes[idx].right;

        // 演算順序: left -> self -> right
        let mut res = if left != !0 {
            self.nodes[left].acc
        } else {
            M::identity()
        };
        res = M::binary_operation(res, self.nodes[idx].val);
        if right != !0 {
            res = M::binary_operation(res, self.nodes[right].acc);
        }
        self.nodes[idx].acc = res;
    }

    fn rotate(&mut self, idx: usize) {
        let p = self.nodes[idx].parent;
        let g = self.nodes[p].parent;
        let is_left = self.nodes[p].left == idx;

        if !self.is_root(p) {
            if self.nodes[g].left == p {
                self.nodes[g].left = idx;
            } else {
                self.nodes[g].right = idx;
            }
        }
        self.nodes[idx].parent = g;

        if is_left {
            let r = self.nodes[idx].right;
            self.nodes[p].left = r;
            if r != !0 {
                self.nodes[r].parent = p;
            }
            self.nodes[idx].right = p;
        } else {
            let l = self.nodes[idx].left;
            self.nodes[p].right = l;
            if l != !0 {
                self.nodes[l].parent = p;
            }
            self.nodes[idx].left = p;
        }
        self.nodes[p].parent = idx;
        self.pull(p);
        self.pull(idx);
    }

    fn splay(&mut self, idx: usize) {
        let mut cur = idx;
        let mut stack = vec![cur];
        while !self.is_root(cur) {
            cur = self.nodes[cur].parent;
            stack.push(cur);
        }
        while let Some(v) = stack.pop() {
            self.push(v);
        }

        while !self.is_root(idx) {
            let p = self.nodes[idx].parent;
            if self.is_root(p) {
                self.rotate(idx);
            } else {
                let g = self.nodes[p].parent;
                if (self.nodes[p].left == idx) == (self.nodes[g].left == p) {
                    self.rotate(p);
                    self.rotate(idx);
                } else {
                    self.rotate(idx);
                    self.rotate(idx);
                }
            }
        }
    }

    fn access(&mut self, mut idx: usize) {
        let mut last = !0;
        while idx != !0 {
            self.splay(idx);
            self.nodes[idx].right = last;
            self.pull(idx);
            last = idx;
            idx = self.nodes[idx].parent;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lct_sum_basic() {
        let n = 10;
        let mut lct = LCTSum::new(n);

        // 初期化: 全ノードの値を 1 に設定
        for i in 0..n {
            lct.set(i, 1);
        }

        // --- 連結とクエリのテスト ---
        // グラフ: 0 - 1 - 2 - 3
        lct.unite(0, 1);
        lct.unite(1, 2);
        lct.unite(2, 3);

        // 連結判定
        assert!(lct.is_same(0, 3));
        assert!(!lct.is_same(0, 4));

        // パスクエリ (0から3までの和: 1+1+1+1 = 4)
        assert_eq!(lct.query(0, 3), 4);
        // 部分的なパスクエリ (1から2までの和: 1+1 = 2)
        assert_eq!(lct.query(1, 2), 2);

        // --- 値の更新テスト ---
        // 頂点 2 の値を 10 に変更
        lct.set(2, 10);

        // パス 0-3 の和: 1(v0) + 1(v1) + 10(v2) + 1(v3) = 13
        assert_eq!(lct.query(0, 3), 13);
    }

    #[test]
    fn test_lct_cut_and_relink() {
        let n = 5;
        let mut lct = LCTSum::new(n);
        for i in 0..n {
            lct.set(i, 1);
        }

        // 0 - 1 - 2
        lct.unite(0, 1);
        lct.unite(1, 2);
        assert_eq!(lct.query(0, 2), 3);

        // 切断: 1 - 2 を切る
        lct.cut(1, 2);

        // 0-1 は繋がっているが、0-2 は切れている
        assert!(lct.is_same(0, 1));
        assert!(!lct.is_same(0, 2));

        // 繋ぎ変え: 0 - 1    2 - 3
        //           |________|
        // 新たに 1 - 3 を繋ぐ (0-1-3-2 というパスを作るため、まず3-2をつなぐ)
        lct.unite(2, 3);
        lct.unite(1, 3);

        // 構造: 0 - 1 - 3 - 2
        assert!(lct.is_same(0, 2));
        // パス長 4 (ノード数)
        assert_eq!(lct.query(0, 2), 4);
    }

    #[test]
    fn test_lct_tree_query() {
        // 木構造でのパスクエリ（make_rootが正しく機能するか）
        let n = 5;
        let mut lct = LCTSum::new(n);
        // 値: index * 10 (0, 10, 20, 30, 40)
        for i in 0..n {
            lct.set(i, i * 10);
        }

        // 構成:
        //      1
        //     / \
        //    0   2
        //         \
        //          3
        lct.unite(1, 0);
        lct.unite(1, 2);
        lct.unite(2, 3);

        // パス 0 - 3 (0 -> 1 -> 2 -> 3)
        // 和: 0 + 10 + 20 + 30 = 60
        assert_eq!(lct.query(0, 3), 60);

        // 根を変えてみる (3を根にする操作は内部で行われるが、ユーザー側からもクエリが変わらないことを確認)
        lct.make_root(3);
        // パス 3 - 0 (3 -> 2 -> 1 -> 0)
        assert_eq!(lct.query(3, 0), 60);
    }

    #[test]
    fn test_cycle_handling() {
        // 閉路を作ろうとしたときに無視されるか
        let mut lct = LCTSum::new(3);
        lct.unite(0, 1);
        lct.unite(1, 2);

        // 0-1-2 が繋がっている状態で 2-0 を繋ごうとする
        lct.unite(2, 0);

        // 無限ループせずに正しく処理が戻ってくるか確認
        assert!(lct.is_same(0, 2));
    }
}
