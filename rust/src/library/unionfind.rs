// unionfind
#[derive(Clone, Debug)]
pub struct UnionFind {
    par: Vec<usize>, // 各ノードの親
    siz: Vec<usize>, // 各ノードが属するグループのサイズ
    group_count: usize,
}
impl UnionFind {
    /// 初期化
    pub fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(), // 初期状態では各ノードが自分自身を親に
            siz: vec![1; n],       // 各グループのサイズは1
            group_count: n,
        }
    }

    /// 根を求める (パス圧縮を行う)
    #[inline]
    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] != x {
            // パス圧縮
            self.par[x] = self.root(self.par[x]);
        }
        self.par[x]
    }

    /// x と y が同じグループに属するかどうか
    #[inline]
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// x を含むグループと y を含むグループを併合する (サイズの併合)
    #[inline]
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let x_root = self.root(x);
        let y_root = self.root(y);
        // すでに同じグループの場合は何もしない
        if x_root == y_root {
            return false;
        }
        // union by size (小さい木を大きい木にくっつける)
        let (smaller, larger) = if self.siz[x_root] < self.siz[y_root] {
            (x_root, y_root)
        } else {
            (y_root, x_root)
        };
        // smaller を larger の子にする
        self.par[smaller] = larger;
        // larger のサイズを更新
        self.siz[larger] += self.siz[smaller];
        // グループ数を減らす
        self.group_count -= 1;
        true
    }

    /// x を含むグループのサイズ
    #[inline]
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }

    /// 全グループ取得
    #[inline]
    pub fn get_groups(&mut self) -> Vec<Vec<usize>> {
        let n = self.par.len();
        let mut res = vec![vec![]; n];

        for i in 0..n {
            res[self.root(i)].push(i);
        }

        res.into_iter().filter(|v| !v.is_empty()).collect()
    }

    /// グループの数
    #[inline]
    pub const fn group_count(&self) -> usize {
        self.group_count
    }
}
