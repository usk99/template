#[derive(Clone, Debug)]
pub struct UnionFind {
    parent_or_size: Vec<i32>,
    group_count: usize,
}

impl UnionFind {
    /// 初期化 : O(N)
    pub fn new(n: usize) -> Self {
        Self {
            parent_or_size: vec![-1; n],
            group_count: n,
        }
    }

    /// リセット(メモリ確保コストを回避) : O(N)
    pub fn reset(&mut self) {
        self.group_count = self.parent_or_size.len();
        self.parent_or_size.fill(-1);
    }

    /// 根を求める(経路圧縮あり) : O(α(N))
    #[inline]
    pub fn root(&mut self, mut index: usize) -> usize {
        debug_assert!(index < self.parent_or_size.len());
        let mut root = index;
        while self.parent_or_size[root] >= 0 {
            root = self.parent_or_size[root] as usize;
        }
        // 経路圧縮
        while self.parent_or_size[index] >= 0 {
            let next = self.parent_or_size[index] as usize;
            self.parent_or_size[index] = root as i32;
            index = next;
        }
        root
    }

    /// 同じグループか判定
    #[inline]
    pub fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    /// グループの結合(Union by Size) : O(α(N))
    /// 返り値: 結合が行われたら true, 既に同じなら false
    #[inline]
    pub fn unite(&mut self, u: usize, v: usize) -> bool {
        let mut x = self.root(u);
        let mut y = self.root(v);
        if x == y {
            return false;
        }
        // Union by Size (サイズが大きい方が親になる)
        // parent_or_size は負の値(サイズ * -1)なので、値が小さいほうがサイズが大きい
        if self.parent_or_size[x] > self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        self.group_count -= 1;
        true
    }

    /// グループのサイズを取得
    #[inline]
    pub fn size(&mut self, index: usize) -> usize {
        let root = self.root(index);
        -self.parent_or_size[root] as usize
    }

    /// 全グループの内容を取得 : O(N)
    pub fn get_groups(&mut self) -> Vec<Vec<usize>> {
        let n = self.parent_or_size.len();
        let mut result = vec![Vec::new(); n];
        for i in 0..n {
            result[self.root(i)].push(i);
        }
        result.retain(|v| !v.is_empty());
        result
    }

    /// 連結成分の個数を取得 : O(1)
    #[inline]
    pub const fn group_count(&self) -> usize {
        self.group_count
    }
}
