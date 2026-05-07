    use crate::library::graph::*;

    // =========================================================================
    // bfs
    // =========================================================================

    #[test]
    fn test_bfs_basic() {
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(0, 2, 1);
        g.add_edge(2, 3, 1);
        assert_eq!(g.bfs(0), vec![0, 1, 1, 2]);
    }

    #[test]
    fn test_bfs_unreachable() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        assert_eq!(g.bfs(0)[2], INF);
    }

    #[test]
    fn test_bfs_undirected() {
        let mut g = Graph::new(3, false);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 1);
        assert_eq!(g.bfs(2), vec![2, 1, 0]);
    }

    // =========================================================================
    // dijkstra
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
    fn test_dijkstra_disconnected() {
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1, 10);
        g.add_edge(2, 3, 20);
        let (dist, pre) = g.dijkstra(0);
        assert_eq!(dist, vec![0, 10, INF, INF]);
        assert!(get_path(&pre, 0, 2).is_empty());
    }

    #[test]
    fn test_dijkstra_multiple_edges_and_self_loops() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 10);
        g.add_edge(0, 1, 2);
        g.add_edge(1, 1, 5);
        g.add_edge(1, 2, 10);
        let (dist, _) = g.dijkstra(0);
        assert_eq!(dist[1], 2);
        assert_eq!(dist[2], 12);
    }

    #[test]
    fn test_dijkstra_near_overflow_weights() {
        let mut g = Graph::new(3, true);
        let huge = INF / 3;
        g.add_edge(0, 1, huge);
        g.add_edge(1, 2, huge);
        let (dist, _) = g.dijkstra(0);
        assert_eq!(dist[2], huge * 2);
    }

    // =========================================================================
    // bellman_ford
    // =========================================================================

    #[test]
    fn test_bellman_ford_negative_edges() {
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1, 5);
        g.add_edge(1, 2, -2);
        g.add_edge(2, 3, 3);
        g.add_edge(0, 3, 10);
        let (dist, pre) = g.bellman_ford(0).unwrap();
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
    fn test_bellman_ford_negative_self_loop() {
        let mut g = Graph::new(1, true);
        g.add_edge(0, 0, -1);
        assert!(g.bellman_ford(0).is_none());
    }

    #[test]
    fn test_bellman_ford_zero_weight_cycle() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, -1);
        g.add_edge(2, 0, 0);
        assert!(g.bellman_ford(0).is_some());
    }

    // =========================================================================
    // warshall_floyd
    // =========================================================================

    #[test]
    fn test_warshall_floyd() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 2);
        g.add_edge(1, 2, 3);
        let dist = g.warshall_floyd().unwrap();
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
    fn test_warshall_floyd_zero_weight_cycle() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, -1);
        g.add_edge(2, 0, 0);
        assert!(g.warshall_floyd().is_some());
    }

    // =========================================================================
    // prim
    // =========================================================================

    #[test]
    #[should_panic(expected = "Prim's algorithm requires an undirected graph")]
    fn test_prim_directed_panic() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        g.prim();
    }

    #[test]
    fn test_prim_mst() {
        let mut g = Graph::new(4, false);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 4);
        g.add_edge(1, 2, 2);
        g.add_edge(1, 3, 6);
        g.add_edge(2, 3, 3);
        assert_eq!(g.prim(), Some(6));
    }

    #[test]
    fn test_prim_disconnected() {
        let mut g = Graph::new(4, false);
        g.add_edge(0, 1, 1);
        g.add_edge(2, 3, 1);
        assert_eq!(g.prim(), None);
    }

    #[test]
    fn test_prim_dense_graph() {
        let n = 50;
        let mut g = Graph::new(n, false);
        for i in 0..n {
            for j in i + 1..n {
                g.add_edge(i, j, (j - i) as i64);
            }
        }
        assert_eq!(g.prim(), Some((n - 1) as i64));
    }

    // =========================================================================
    // is_bipartite
    // =========================================================================

    #[test]
    fn test_bipartite_yes() {
        let mut g = Graph::new(4, false);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 3, 1);
        let color = g.is_bipartite().unwrap();
        assert_ne!(color[0], color[1]);
        assert_ne!(color[1], color[2]);
    }

    #[test]
    fn test_bipartite_no() {
        let mut g = Graph::new(3, false);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 0, 1);
        assert!(g.is_bipartite().is_none());
    }

    #[test]
    fn test_bipartite_disconnected() {
        let mut g = Graph::new(4, false);
        g.add_edge(0, 1, 1);
        g.add_edge(2, 3, 1);
        assert!(g.is_bipartite().is_some());
    }

    // =========================================================================
    // scc
    // =========================================================================

    #[test]
    fn test_scc_cycle() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 0, 1);
        assert_eq!(g.scc().0, 1);
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
    }

    #[test]
    fn test_scc_deep_recursion_prevention() {
        let n = 100_000;
        let mut g = Graph::new(n, true);
        for i in 0..n - 1 {
            g.add_edge(i, i + 1, 1);
        }
        assert_eq!(g.scc().0, n);
    }

    // =========================================================================
    // topological_sort
    // =========================================================================

    #[test]
    fn test_topological_sort_dag() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 1);
        assert_eq!(g.topological_sort(), Some(vec![0, 1, 2]));
    }

    #[test]
    fn test_topological_sort_cycle() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 0, 1);
        assert_eq!(g.topological_sort(), None);
    }

    #[test]
    fn test_single_vertex() {
        let g = Graph::new(1, true);
        assert_eq!(g.dijkstra(0).0, vec![0]);
        assert_eq!(g.bellman_ford(0).unwrap().0, vec![0]);
        assert_eq!(g.warshall_floyd().unwrap(), vec![vec![0]]);
        assert_eq!(g.scc().0, 1);
        assert_eq!(g.topological_sort(), Some(vec![0]));
    }

    // =========================================================================
    // get_path
    // =========================================================================

    #[test]
    fn test_get_path_safety() {
        let pre_broken = vec![NONE, 2, 1, NONE];
        assert!(get_path(&pre_broken, 0, 2).is_empty());
    }

    #[test]
    fn test_start_equals_goal_path() {
        let g = Graph::new(3, true);
        let (_, pre) = g.dijkstra(0);
        assert_eq!(get_path(&pre, 0, 0), vec![0]);
    }
