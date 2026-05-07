    use crate::library::graph::*;

    // =========================================================================
    // 1. 最小全域木 (Prim) のテスト
    // =========================================================================

    #[test]
    #[should_panic(expected = "Prim's algorithm requires an undirected graph")]
    fn test_prim_directed_panic() {
        let mut g = Graph::new(3, true); // 有向グラフ
        g.add_edge(0, 1, 1);
        g.prim(); // パニックになることを確認
    }

    #[test]
    fn test_prim_mst() {
        let mut g = Graph::new(4, false);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 4);
        g.add_edge(1, 2, 2);
        g.add_edge(1, 3, 6);
        g.add_edge(2, 3, 3);

        // 0-1 (cost 1), 1-2 (cost 2), 2-3 (cost 3) の辺が選ばれて合計6
        assert_eq!(g.prim(), Some(6));
    }

    #[test]
    fn test_prim_disconnected() {
        let mut g = Graph::new(4, false);
        g.add_edge(0, 1, 1);
        g.add_edge(2, 3, 1); // 0,1 と 2,3 が分断されている
        assert_eq!(g.prim(), None, "非連結グラフならNoneを返す");
    }

    // =========================================================================
    // 2. 最短経路・経路復元のテスト
    // =========================================================================

    #[test]
    fn test_dijkstra() {
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 2);
        g.add_edge(0, 2, 5);
        g.add_edge(2, 3, 1);

        let (dist, _) = g.dijkstra(0);
        assert_eq!(dist, vec![0, 1, 3, 4]);
    }

    #[test]
    fn test_undirected_dijkstra() {
        let mut g = Graph::new(3, false);
        g.add_edge(0, 1, 10);
        g.add_edge(1, 2, 20);

        let (dist, pre) = g.dijkstra(2);
        assert_eq!(dist[0], 30);
        assert_eq!(get_path(&pre, 2, 0), vec![2, 1, 0]);
    }

    #[test]
    fn test_bellman_ford_negative_edges() {
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1, 5);
        g.add_edge(1, 2, -2);
        g.add_edge(2, 3, 3);
        g.add_edge(0, 3, 10);

        let res = g.bellman_ford(0);
        assert!(res.is_some());
        let (dist, pre) = res.unwrap();
        assert_eq!(dist[3], 6);
        assert_eq!(get_path(&pre, 0, 3), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_bellman_ford_negative_cycle() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, -1);
        g.add_edge(2, 1, -1);

        assert!(g.bellman_ford(0).is_none());
    }

    #[test]
    fn test_warshall_floyd() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 2);
        g.add_edge(1, 2, 3);

        let res = g.warshall_floyd();
        assert!(res.is_some());
        let dist = res.unwrap();
        assert_eq!(dist[0][2], 5);
        assert_eq!(dist[1][0], INF);
    }

    #[test]
    fn test_warshall_floyd_negative_cycle() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, -1);
        g.add_edge(2, 1, -1);

        assert!(g.warshall_floyd().is_none());
    }

    #[test]
    fn test_get_path_safety() {
        let pre_broken = vec![NONE, 2, 1, NONE];
        assert!(get_path(&pre_broken, 0, 2).is_empty());
    }

    // =========================================================================
    // 3. 構造・閉路 (SCC, トポロジカルソート) のテスト
    // =========================================================================

    #[test]
    fn test_scc_and_topological_sort() {
        let mut g_cycle = Graph::new(3, true);
        g_cycle.add_edge(0, 1, 1);
        g_cycle.add_edge(1, 2, 1);
        g_cycle.add_edge(2, 0, 1);

        let (count, _) = g_cycle.scc();
        assert_eq!(count, 1);
        assert_eq!(g_cycle.topological_sort(), None);

        let mut g_dag = Graph::new(3, true);
        g_dag.add_edge(0, 1, 1);
        g_dag.add_edge(1, 2, 1);
        assert_eq!(g_dag.topological_sort(), Some(vec![0, 1, 2]));
    }

    // =========================================================================
    // 4. エッジケース・コーナーケースのテスト
    // =========================================================================

    #[test]
    fn test_single_vertex() {
        let g = Graph::new(1, true);
        assert_eq!(g.dijkstra(0).0, vec![0]);
        assert_eq!(g.bellman_ford(0).unwrap().0, vec![0]);
        assert_eq!(g.warshall_floyd().unwrap(), vec![vec![0]]);
        assert_eq!(g.scc().0, 1);
        assert_eq!(g.topological_sort(), Some(vec![0]));
    }

    #[test]
    fn test_disconnected_components() {
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1, 10);
        g.add_edge(2, 3, 20);

        let (dist, pre) = g.dijkstra(0);
        assert_eq!(dist, vec![0, 10, INF, INF]);
        assert!(get_path(&pre, 0, 2).is_empty());
    }

    #[test]
    fn test_multiple_edges_and_self_loops() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 10);
        g.add_edge(0, 1, 2); // 最短の多重辺
        g.add_edge(1, 1, 5); // 自己ループ
        g.add_edge(1, 2, 10);

        let (dist, _) = g.dijkstra(0);
        assert_eq!(dist[1], 2);
        assert_eq!(dist[2], 12);
    }

    #[test]
    fn test_negative_self_loop() {
        let mut g = Graph::new(1, true);
        g.add_edge(0, 0, -1);
        assert!(g.bellman_ford(0).is_none());
    }

    #[test]
    fn test_start_equals_goal_path() {
        let g = Graph::new(3, true);
        let (_, pre) = g.dijkstra(0);
        assert_eq!(get_path(&pre, 0, 0), vec![0]);
    }

    #[test]
    fn test_scc_complex_disconnected() {
        let mut g = Graph::new(5, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 0, 1);
        g.add_edge(3, 4, 1);
        g.add_edge(4, 3, 1);

        let (count, group) = g.scc();
        assert_eq!(count, 3);
        assert_eq!(group[0], group[1]);
        assert_eq!(group[3], group[4]);
        assert_ne!(group[0], group[2]);
        assert_ne!(group[2], group[3]);
    }

    // =========================================================================
    // 5. 極限ストレステスト（悪意のあるケース・限界突破）
    // =========================================================================

    #[test]
    fn test_scc_deep_recursion_prevention() {
        // 深さ10万の直線グラフ（非再帰実装の恩恵確認）
        let n = 100_000;
        let mut g = Graph::new(n, true);
        for i in 0..n - 1 {
            g.add_edge(i, i + 1, 1);
        }
        let (count, _) = g.scc();
        assert_eq!(count, n, "10万頂点でStack Overflowせずに完走できるか");
    }

    #[test]
    fn test_zero_weight_cycle() {
        // 合計コストがピッタリ『0』の閉路の誤検知防止
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, -1);
        g.add_edge(2, 0, 0); // 1 - 1 + 0 = 0

        assert!(
            g.bellman_ford(0).is_some(),
            "コスト0の閉路を負の閉路と誤検知していないか"
        );
        assert!(
            g.warshall_floyd().is_some(),
            "コスト0の閉路を負の閉路と誤検知していないか"
        );
    }

    #[test]
    fn test_near_overflow_weights() {
        // INFに近い巨大な重みでのオーバーフロー耐性
        let mut g = Graph::new(3, true);
        let huge = INF / 3;
        g.add_edge(0, 1, huge);
        g.add_edge(1, 2, huge);

        let (dist, _) = g.dijkstra(0);
        assert_eq!(
            dist[2],
            huge * 2,
            "巨大な重みの加算でパニックを起こさないか"
        );
    }

    #[test]
    fn test_dense_graph_prim() {
        // 完全グラフ（密グラフ）での最小全域木
        let n = 50;
        let mut g = Graph::new(n, false);
        for i in 0..n {
            for j in i + 1..n {
                g.add_edge(i, j, (j - i) as i64);
            }
        }
        assert_eq!(
            g.prim(),
            Some((n - 1) as i64),
            "密グラフで正しく最小コストを計算できるか"
        );
    }
