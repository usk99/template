    use crate::library::unionfind::*;
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

    /// get_groups の内容を HashSet で正確に検証
    #[test]
    fn test_get_groups_exact() {
        let n = 6;
        let mut uf = UnionFind::new(n);
        uf.unite(0, 1);
        uf.unite(2, 3);
        uf.unite(3, 4);

        let groups = uf.get_groups();
        let groups_as_sets: Vec<HashSet<usize>> =
            groups.iter().map(|g| g.iter().cloned().collect()).collect();

        assert!(groups_as_sets.contains(&HashSet::from([0, 1])));
        assert!(groups_as_sets.contains(&HashSet::from([2, 3, 4])));
        assert!(groups_as_sets.contains(&HashSet::from([5])));
    }

    /// チェーン状 unite（0-1-2-...-N-1）のテスト
    #[test]
    fn test_chain_unite() {
        let n = 10;
        let mut uf = UnionFind::new(n);

        for i in 0..n - 1 {
            uf.unite(i, i + 1);
        }

        assert_eq!(uf.group_count(), 1);
        assert_eq!(uf.size(0), n);
        for i in 0..n {
            assert!(uf.is_same(0, i));
        }
    }

    /// 経路圧縮後も is_same / size が正しいか
    #[test]
    fn test_path_compression_consistency() {
        let n = 8;
        let mut uf = UnionFind::new(n);

        // チェーン状に結合して経路圧縮を発生させる
        for i in 0..n - 1 {
            uf.unite(i, i + 1);
        }

        // root を何度呼んでも結果が一致する
        let root = uf.root(0);
        for i in 0..n {
            assert_eq!(uf.root(i), root);
            assert_eq!(uf.size(i), n);
        }
    }

    /// reset 後に再利用できるか
    #[test]
    fn test_reset_and_reuse() {
        let n = 5;
        let mut uf = UnionFind::new(n);

        uf.unite(0, 1);
        uf.unite(2, 3);
        uf.unite(3, 4);
        assert_eq!(uf.group_count(), 2);

        uf.reset();

        // 再利用: 別の組み合わせで結合
        uf.unite(0, 2);
        uf.unite(0, 4);
        assert_eq!(uf.group_count(), 3); // {0,2,4}, {1}, {3}
        assert!(uf.is_same(0, 2));
        assert!(uf.is_same(0, 4));
        assert!(!uf.is_same(0, 1));
        assert!(!uf.is_same(0, 3));
    }

    /// ストレステスト（愚直解との比較）
    #[test]
    fn stress_test() {
        struct Rng(u64);
        impl Rng {
            fn next(&mut self) -> u64 {
                self.0 ^= self.0 << 13;
                self.0 ^= self.0 >> 7;
                self.0 ^= self.0 << 17;
                self.0
            }
            fn gen_range(&mut self, n: usize) -> usize {
                self.next() as usize % n
            }
        }

        let n = 50;
        let q = 500;
        let mut rng = Rng(12345);

        let mut uf = UnionFind::new(n);
        // 愚直: ノードごとに「属するグループID」を管理
        let mut group = (0..n).collect::<Vec<_>>();

        for _ in 0..q {
            let u = rng.gen_range(n);
            let v = rng.gen_range(n);

            let same_expected = group[u] == group[v];
            assert_eq!(uf.is_same(u, v), same_expected);

            if !same_expected {
                let old = group[v];
                let new = group[u];
                for g in group.iter_mut() {
                    if *g == old {
                        *g = new;
                    }
                }
                uf.unite(u, v);
            }
        }

        // 最終的なグループ数の一致確認
        let distinct: HashSet<usize> = group.iter().cloned().collect();
        assert_eq!(uf.group_count(), distinct.len());
    }
