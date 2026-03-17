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

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashSet;

    /// 基本的な機能（unite, is_same, size, group_count）のテスト
    #[test]
    fn test_basic_operations() {
        let n = 10;
        let mut uf = UnionFind::new(n);

        // 初期状態の確認
        assert_eq!(uf.group_count(), n, "初期状態のグループ数はNであるべき");
        for i in 0..n {
            assert_eq!(uf.size(i), 1, "初期状態の各要素のサイズは1であるべき");
            assert!(uf.is_same(i, i), "自分自身とは同じグループであるべき");
        }

        // 0と1を結合
        assert!(uf.unite(0, 1), "異なるグループのuniteはtrueを返すべき");
        assert!(uf.is_same(0, 1), "unite後は同じグループになるべき");
        assert_eq!(uf.size(0), 2, "結合後のサイズは合算されるべき");
        assert_eq!(uf.size(1), 2, "同じグループならサイズは同じ値を返すべき");
        assert_eq!(uf.group_count(), n - 1, "結合後はグループ数が減るべき");

        // 既に同じグループを結合
        assert!(!uf.unite(0, 1), "同じグループのuniteはfalseを返すべき");
        assert_eq!(
            uf.group_count(),
            n - 1,
            "無効なuniteでグループ数は変わらないべき"
        );
    }

    /// 推移律（A=B かつ B=C ならば A=C）のテスト
    #[test]
    fn test_transitivity() {
        let n = 5;
        let mut uf = UnionFind::new(n);

        uf.unite(0, 1);
        uf.unite(1, 2);

        assert!(uf.is_same(0, 2), "A=B, B=C ならば A=C であるべき");
        assert_eq!(uf.size(0), 3);
        assert_eq!(uf.size(1), 3);
        assert_eq!(uf.size(2), 3);

        // 無関係な要素には影響しない
        assert!(!uf.is_same(0, 3));
        assert!(!uf.is_same(2, 4));
    }

    /// Union by Size (小さい木が大きい木の下につく) の挙動テスト
    #[test]
    fn test_union_by_size_logic() {
        let n = 10;
        let mut uf = UnionFind::new(n);

        // グループA: {0, 1, 2} (サイズ3) を作る
        uf.unite(0, 1);
        uf.unite(1, 2);
        let root_a = uf.root(0);
        assert_eq!(uf.size(root_a), 3);

        // グループB: {3} (サイズ1)
        let root_b = uf.root(3);
        assert_eq!(uf.size(root_b), 1);

        // AとBを結合 -> サイズの大きいAが親になるはず
        uf.unite(0, 3);

        let new_root = uf.root(3);
        assert_eq!(
            new_root, root_a,
            "小さいグループが大きいグループの子になるべき"
        );
        assert_eq!(uf.size(root_a), 4);
    }

    /// リセット機能のテスト
    #[test]
    fn test_reset() {
        let n = 5;
        let mut uf = UnionFind::new(n);

        // 状態を変更する
        uf.unite(0, 1);
        uf.unite(2, 3);
        uf.unite(3, 4);
        assert_ne!(uf.group_count(), n);

        // リセット実行
        uf.reset();

        // 初期状態に戻っているか確認
        assert_eq!(uf.group_count(), n, "リセット後はグループ数がNに戻るべき");
        for i in 0..n {
            assert_eq!(uf.size(i), 1, "リセット後はサイズが1に戻るべき");
            assert_eq!(uf.root(i), i, "リセット後は自分が根に戻るべき");
            // 他の要素と切れているか確認（代表として隣とチェック）
            if i + 1 < n {
                assert!(!uf.is_same(i, i + 1), "リセット後は結合が解除されるべき");
            }
        }
    }

    /// get_groups のテスト
    #[test]
    fn test_get_groups() {
        let n = 6;
        let mut uf = UnionFind::new(n);

        // {0, 1}, {2, 3, 4}, {5} の3グループを作る
        uf.unite(0, 1);
        uf.unite(2, 3);
        uf.unite(3, 4);

        let groups = uf.get_groups();

        // グループ数の確認
        assert_eq!(groups.len(), 3);

        // 内容の確認（順序不同に対応するためHashSetで比較しても良いが、
        // ここでは単純に全要素が含まれているか、サイズが正しいかを確認）
        let mut count_check = vec![0; n];
        for group in &groups {
            for &elem in group {
                count_check[elem] += 1;
            }
        }
        // 全要素がちょうど1回ずつ現れるか
        assert!(
            count_check.iter().all(|&c| c == 1),
            "全要素が漏れなく重複なく含まれるべき"
        );

        // 各グループのサイズ確認
        let sizes: Vec<usize> = groups.iter().map(|g| g.len()).collect();
        assert!(sizes.contains(&1)); // {5}
        assert!(sizes.contains(&2)); // {0, 1}
        assert!(sizes.contains(&3)); // {2, 3, 4}
    }

    /// コーナーケース（N=0, N=1）のテスト
    #[test]
    fn test_corner_cases() {
        // N=0
        let mut uf0 = UnionFind::new(0);
        assert_eq!(uf0.group_count(), 0);
        assert_eq!(uf0.get_groups().len(), 0);
        // unite等はインデックス外参照になるので呼べないが、生成できることが重要

        // N=1
        let mut uf1 = UnionFind::new(1);
        assert_eq!(uf1.group_count(), 1);
        assert_eq!(uf1.size(0), 1);
        assert!(!uf1.unite(0, 0)); // 自分自身とのunite
        uf1.reset();
        assert_eq!(uf1.group_count(), 1);
    }

    /// ランダムテスト（整合性チェック）
    #[test]
    fn test_random_consistency() {
        use rand::Rng;
        let mut rng = rand::rng();
        let n = 100;
        let mut uf = UnionFind::new(n);

        // 正解データを管理するための愚直な実装（HashSetのベクタ）
        let mut sets: Vec<HashSet<usize>> = (0..n)
            .map(|i| {
                let mut s = HashSet::new();
                s.insert(i);
                s
            })
            .collect();

        for _ in 0..1000 {
            let a = rng.random_range(0..n);
            let b = rng.random_range(0..n);

            if rng.random_bool(0.5) {
                // unite操作
                uf.unite(a, b);

                // 正解データの更新（aを含む集合とbを含む集合をマージ）
                let mut set_indices = vec![];
                for (i, s) in sets.iter().enumerate() {
                    if s.contains(&a) || s.contains(&b) {
                        set_indices.push(i);
                    }
                }
                if set_indices.len() == 2 {
                    let idx1 = set_indices[0];
                    let idx2 = set_indices[1];
                    let set2 = sets[idx2].clone(); // 退避
                    sets[idx1].extend(set2);
                    sets.remove(idx2);
                }
            } else {
                // is_same操作の検証
                let actual = uf.is_same(a, b);

                let expected = sets.iter().any(|s| s.contains(&a) && s.contains(&b));
                assert_eq!(
                    actual, expected,
                    "ランダムテストで不整合: unite({}, {})",
                    a, b
                );
            }
        }
    }
}
